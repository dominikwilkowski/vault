use aes_gcm_siv::{
	aead,
	aead::{Aead, AeadCore, KeyInit, OsRng},
	Aes256GcmSiv, Nonce,
};
use argon2::Argon2;
use base64::{engine::general_purpose, Engine as _};
use std::str::from_utf8;

#[derive(thiserror::Error, Debug)]
pub enum CryptError {
	#[error("Failed to create hash from password.")]
	Hash(#[from] argon2::Error),
	#[error("Failed to decode base64")]
	Base64Decode(#[from] base64::DecodeError),
	#[error("Failed to create crypto cipher, likely invalid length nonce")]
	Cipher(#[from] crypto_common::InvalidLength),
	#[error("Incorrect password.")]
	Decryption(#[from] aead::Error),
	#[error("Failed to decode utf8.")]
	UTF8(#[from] std::str::Utf8Error),
}

pub fn decrypt_vault(
	payload: String,
	hash: [u8; 32],
) -> Result<String, CryptError> {
	let cipher = Aes256GcmSiv::new_from_slice(hash.as_slice())?;

	let cyphertext_from_string =
		general_purpose::STANDARD_NO_PAD.decode(payload)?;
	let (nonce_bytes, cyphertext) = cyphertext_from_string.split_at(12);
	let nonce = Nonce::from_slice(nonce_bytes);

	let plaintext = cipher.decrypt(nonce, cyphertext)?;
	let utf8_string = from_utf8(plaintext.as_slice())?.to_string();
	Ok(utf8_string)
}

#[cfg(not(test))]
fn generate_nonce() -> Nonce {
	Aes256GcmSiv::generate_nonce(&mut OsRng)
}

#[cfg(test)]
fn generate_nonce() -> Nonce {
	Nonce::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])
}

pub fn encrypt_vault(
	payload: String,
	hash: [u8; 32],
) -> Result<String, CryptError> {
	let cipher = Aes256GcmSiv::new_from_slice(hash.as_slice())?;
	let nonce = generate_nonce();

	let ciphertext = cipher.encrypt(&nonce, payload.as_bytes().as_ref())?;
	let payload = [&nonce, ciphertext.as_slice()].concat();
	let b64_payload =
		general_purpose::STANDARD_NO_PAD.encode(payload).to_string();
	Ok(b64_payload)
}

pub fn password_hash(
	password: String,
	salt: String,
) -> Result<[u8; 32], CryptError> {
	let mut okm = [0u8; 32];
	Argon2::default().hash_password_into(
		password.as_bytes(),
		salt.as_bytes(),
		&mut okm,
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

	fn get_password_hash() -> [u8; 32] {
		let password = String::from("TestPassword");
		let salt = String::from("TestSalt");
		password_hash(password, salt).unwrap()
	}

	#[test]
	fn test_password_hash() {
		let expected: [u8; 32] = [
			137, 120, 26, 219, 42, 237, 139, 10, 46, 243, 159, 115, 172, 190, 29, 18,
			242, 81, 51, 110, 181, 61, 184, 22, 137, 47, 254, 97, 238, 23, 100, 185,
		];
		assert_eq!(get_password_hash(), expected);
	}

	#[test]
	fn test_encrypt_vault() {
		let hash = get_password_hash();
		let expected = get_encrypted_vault();
		assert_eq!(encrypt_vault(get_decrypted_vault(), hash).unwrap(), expected);
	}

	#[test]
	fn test_decrypt_vault() {
		let hash = get_password_hash();
		let expected = get_decrypted_vault();
		assert_eq!(decrypt_vault(get_encrypted_vault(), hash).unwrap(), expected);
	}
}
