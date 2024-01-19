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
	hash: [u8; 32]
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

pub fn encrypt_vault(
	payload: String,
	hash: [u8; 32]
) -> Result<String, CryptError> {
	let cipher = Aes256GcmSiv::new_from_slice(hash.as_slice())?;
	let nonce = Aes256GcmSiv::generate_nonce(&mut OsRng);

	let ciphertext = cipher.encrypt(&nonce, payload.as_bytes().as_ref())?;
	let payload = [&nonce, ciphertext.as_slice()].concat();
	let b64_payload =
		general_purpose::STANDARD_NO_PAD.encode(payload).to_string();
	Ok(b64_payload)
}

pub fn password_hash(password: String, salt: String) -> Result<[u8; 32], CryptError> {
	let mut okm = [0u8; 32];
	Argon2::default().hash_password_into(
		password.as_bytes(),
		salt.as_bytes(),
		&mut okm,
	)?;
	Ok(okm)
}