use aes_gcm_siv::{
	aead,
	aead::{Aead, KeyInit},
	Aes256GcmSiv, Nonce,
};
use argon2::Argon2;
use base64::{engine::general_purpose, Engine as _};
use zeroize::{Zeroize, Zeroizing};

#[derive(thiserror::Error, Debug)]
pub enum CryptError {
	#[error("Failed to create hash from password.")]
	Hash(#[from] argon2::Error),
	#[error("Failed to decode base64")]
	Base64Decode(#[from] base64::DecodeError),
	#[error("Failed to create crypto cipher, likely invalid length nonce")]
	Cipher(#[from] crypto_common::InvalidLength),
	#[error("Encrypted data is too short to contain a nonce")]
	Truncated,
	#[error("Failed to read nonce from payload")]
	Nonce(#[from] std::array::TryFromSliceError),
	#[error("Incorrect password.")]
	Decryption(#[from] aead::Error),
	#[error("Failed to decode utf8.")]
	UTF8(#[from] std::str::Utf8Error),
}

/// Decrypt the vault; the plaintext is wiped when the returned value is dropped.
pub fn decrypt_vault(
	payload: &str,
	key: &[u8; 32],
) -> Result<Zeroizing<String>, CryptError> {
	let cipher = Aes256GcmSiv::new_from_slice(key)?;

	let cyphertext_from_string =
		general_purpose::STANDARD_NO_PAD.decode(payload)?;
	let (nonce_bytes, cyphertext) =
		cyphertext_from_string.split_at_checked(12).ok_or(CryptError::Truncated)?;
	let nonce = Nonce::try_from(nonce_bytes)?;

	// decrypted in place, so this is the only copy of the plaintext
	let plaintext = cipher.decrypt(&nonce, cyphertext)?;
	match String::from_utf8(plaintext) {
		Ok(plaintext) => Ok(Zeroizing::new(plaintext)),
		Err(err) => {
			let utf8_error = err.utf8_error();
			err.into_bytes().zeroize();
			Err(utf8_error.into())
		},
	}
}

#[cfg(not(test))]
fn generate_nonce() -> Nonce {
	use aes_gcm_siv::aead::Generate;
	Nonce::generate()
}

#[cfg(test)]
fn generate_nonce() -> Nonce {
	Nonce::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])
}

/// Encrypt the plaintext; the cipher encrypts its own copy in place.
pub fn encrypt_vault(
	payload: &[u8],
	key: &[u8; 32],
) -> Result<String, CryptError> {
	let cipher = Aes256GcmSiv::new_from_slice(key)?;
	let nonce = generate_nonce();

	let ciphertext = cipher.encrypt(&nonce, payload)?;
	let payload = [nonce.as_slice(), ciphertext.as_slice()].concat();
	let b64_payload =
		general_purpose::STANDARD_NO_PAD.encode(payload).to_string();
	Ok(b64_payload)
}

/// Derive the vault key; it is wiped when the returned value is dropped.
pub fn password_hash(
	password: &[u8],
	salt: &str,
) -> Result<Zeroizing<[u8; 32]>, CryptError> {
	let mut okm = Zeroizing::new([0u8; 32]);
	Argon2::default().hash_password_into(
		password,
		salt.as_bytes(),
		okm.as_mut_slice(),
	)?;
	Ok(okm)
}

#[cfg(test)]
mod tests {
	use super::*;

	fn get_decrypted_vault() -> String {
		String::from(
			r#"[[contents]]
id = 1
title = \"Bank\"

[[contents.fields]]
id = 0
kind = \"Url\"
title = \"URL\"
visible = true
value = [[1702851212, \"https://bankofaustralia.com.au\"]]

[[contents.fields]]
id = 1
kind = \"TextLineSecret\"
title = \"Username\"
visible = true
value = [[1702851212, \"ano85\"]]

[[contents.fields]]
id = 2
kind = \"TextLineSecret\"
title = \"Password\"
visible = true
value = [[1702851212, \"totally_secure_password!1\"]]

[[contents.fields]]
id = 3
kind = \"TextLineSecret\"
title = \"Notes\"
visible = true
value = [[1702851212, \"My secret notes\"]]

[[contents.fields]]
id = 4
kind = \"TextLineSecret\"
title = \"Other\"
visible = true
value = [[1702851212, \"Some other notes\"]]"#,
		)
	}

	fn get_encrypted_vault() -> String {
		String::from("AAECAwQFBgcICQoLG4YLeolit5FN5dKEKZIpO6lblz/h6i0AmT7i0cJyUidis0rHpiBX0VWSheaZeN14qnamUefDSZ4rTi+Cl8x5B/V0YYqM4NpB4WItSlppRshaim7+xYa1rAqVoqYqUyacuST6DRK0p1eE15zZKjGEVAM8Y8zX2N6E93VbN7jPM1h0Ml9Y0np1PorLKTmyNjwOrOiQ/Y0YDbqhrq22kPhadW/uL8Mfo2Rvc2Opla87RVBXoZv3g8/D+kN/Sbwfw6YbxVQt+s7v2sAYkRBHUyOUQ0nIOvXuq2zwtKGXm1m6ejvRg/QfdYlnVHnVKy4k+RHaX1SDOlSSOJKSsQdazMxXoRpBRfHsfapkZhMKXMk/RuQcJZpAMrTNNx4IcsOyJyaYxG337gI7asx5PhtxIGG5GRsbHKdR7URmjlg9OtaQI4OYEdh92EeIE1/+FushLgpi7KI44BTh0Tt5GkQTRZ9A6oKb2D1EHLIHCVTbfamxScZRIv2itd/c/ZQ2zU3JZpMNukRSOpARemv9ZDcSGJpGlGxRaCTZ5ex6hZRuspaxLqrN+u6LSlHzWS8tgiUR8NyPcArUr80lKGuSN+E78FclN+eex+1zGtJCrY6KiAgRYNrLvro/mpcLe6Cvqc3tH4MHhL2TDWpDkbPHCrID4NDDJ9mEuOokAw8Gme52Wct54pVxc/2pOKReeDugl9QhN+uz8QnZWq69PmaZjPT3XuoUYo3TmYSzUJxwtc3kfBPLktY1oxguvnGHRpFOg6w+ujQpWVbNGrbW4T8V+mWBqxzO5kdIsMwXZl0e4vo0TFVIHdivdghMcWtvhg7/wrrn8/TX9BUWane6LPjIAvHy7ZaIRYXRXew3g/nrmb61HEgyjcWIA6cONsBVS/gt67su7ng7V+jVmV0KYGDPXC5I4h5DyjqUeAcqJskx4cqEPlZY6vDuDCgZgroOC1S8xmc")
	}

	fn get_password_hash() -> Zeroizing<[u8; 32]> {
		password_hash(b"TestPassword", "TestSalt").unwrap()
	}

	#[test]
	fn test_password_hash() {
		let expected: [u8; 32] = [
			137, 120, 26, 219, 42, 237, 139, 10, 46, 243, 159, 115, 172, 190, 29, 18,
			242, 81, 51, 110, 181, 61, 184, 22, 137, 47, 254, 97, 238, 23, 100, 185,
		];
		assert_eq!(*get_password_hash(), expected);
	}

	#[test]
	fn test_encrypt_vault() {
		let hash = get_password_hash();
		let expected = get_encrypted_vault();
		assert_eq!(
			encrypt_vault(get_decrypted_vault().as_bytes(), &hash).unwrap(),
			expected
		);
	}

	#[test]
	fn test_decrypt_vault() {
		let hash = get_password_hash();
		let expected = get_decrypted_vault();
		assert_eq!(
			*decrypt_vault(&get_encrypted_vault(), &hash).unwrap(),
			expected
		);
	}

	#[test]
	fn test_decrypt_truncated_vault() {
		let hash = get_password_hash();
		let too_short = general_purpose::STANDARD_NO_PAD.encode([1u8; 5]);
		assert!(matches!(
			decrypt_vault(&too_short, &hash),
			Err(CryptError::Truncated)
		));
	}

	#[test]
	fn test_decrypt_wrong_key() {
		let wrong = password_hash(b"WrongPassword", "TestSalt").unwrap();
		assert!(matches!(
			decrypt_vault(&get_encrypted_vault(), &wrong),
			Err(CryptError::Decryption(_))
		));
	}
}
