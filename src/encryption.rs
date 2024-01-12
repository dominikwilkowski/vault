use aes_gcm_siv::{
	aead,
	aead::{Aead, KeyInit},
	Aes256GcmSiv, Nonce,
};
use argon2::Argon2;
use base64::{engine::general_purpose, Engine as _};
use std::str::from_utf8;

#[derive(thiserror::Error, Debug)]
pub enum CryptError {
	#[error("failed to create hash")]
	Hash(#[from] argon2::Error),
	#[error("failed base64 decoding")]
	Base64Decode(#[from] base64::DecodeError),
	#[error("failed to create crypto cipher, invalid length nonce")]
	Cipher(#[from] crypto_common::InvalidLength),
	#[error("failed to decrypt or encrypt vault")]
	Decryption(#[from] aead::Error),
	#[error("failed to decode utf8")]
	UTF8(#[from] std::str::Utf8Error),
}
pub fn decrypt_vault(
	payload: String,
	password: String,
	nonce: String,
	salt: String,
) -> Result<String, CryptError> {
	let mut okm = [0u8; 32]; // Can be any desired size
	Argon2::default().hash_password_into(
		password.as_bytes(),
		salt.as_bytes(),
		&mut okm,
	)?;

	let cipher = Aes256GcmSiv::new_from_slice(okm.as_slice())?;
	let nonce = Nonce::from_slice(nonce.as_bytes());

	let cyphertext_from_string =
		general_purpose::STANDARD_NO_PAD.decode(payload)?;

	let plaintext = cipher.decrypt(nonce, cyphertext_from_string.as_ref())?;
	Ok(from_utf8(plaintext.as_slice())?.to_string())
}

pub fn encrypt_vault(
	payload: String,
	password: String,
	nonce: String,
	salt: String,
) -> Result<String, CryptError> {
	let mut okm = [0u8; 32]; // Can be any desired size
	Argon2::default().hash_password_into(
		password.as_bytes(),
		salt.as_bytes(),
		&mut okm,
	)?;

	let cipher = Aes256GcmSiv::new_from_slice(okm.as_slice())?;
	let nonce = Nonce::from_slice(nonce.as_bytes());

	let ciphertext = cipher.encrypt(nonce, payload.as_bytes().as_ref())?;

	Ok(general_purpose::STANDARD_NO_PAD.encode(ciphertext).to_string())
}
