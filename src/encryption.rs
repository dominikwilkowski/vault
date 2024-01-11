use aes_gcm_siv::{
	aead,
	aead::{Aead, KeyInit},
	Aes256GcmSiv, Nonce,
};
use argon2::Argon2;
use base64::{engine::general_purpose, Engine as _};
use std::str::from_utf8;

#[derive(thiserror::Error, Debug)]
pub enum DecryptionError {
	#[error("failed to create hash")]
	Hash(#[from] argon2::Error),
	#[error("failed base64 decoding")]
	Base64Decode(#[from] base64::DecodeError),
	#[error("failed to create crypto cipher")]
	Cipher(#[from] crypto_common::InvalidLength),
	#[error("failed to decrypt")]
	Decryption(#[from] aead::Error),
	#[error("failed to utf8")]
	UTF8(#[from] std::str::Utf8Error),
}
pub fn decrypt_aes(
	string: String,
	password: String,
) -> Result<String, DecryptionError> {
	let salt = b"I'm making a note here: HUGE SUCCESS"; // Salt should be unique per password

	let mut okm = [0u8; 32]; // Can be any desired size
	Argon2::default().hash_password_into(password.as_bytes(), salt, &mut okm)?;

	let cipher = Aes256GcmSiv::new_from_slice(okm.as_slice())?;
	let nonce = Nonce::from_slice(b"not unique nonce");

	let cyphertext_from_string =
		general_purpose::STANDARD_NO_PAD.decode(string)?;

	let plaintext = cipher.decrypt(nonce, cyphertext_from_string.as_ref())?;
	Ok(from_utf8(plaintext.as_slice())?.to_string())
}

// pub fn encrypt_aes(string: String) -> String {
//     let password = b"password_is_bad!"; // Bad password; don't actually use!
//     let salt = b"randomly salty salt"; // Salt should be unique per password
//
//     let mut okm = [0u8; 32]; // Can be any desired size
//     Argon2::default().hash_password_into(password, salt, &mut okm).expect("Something went wrong!");
//
//     let cipher = Aes256GcmSiv::new_from_slice(okm.as_slice()).expect("creating new cypher blew up");
//     let nonce = Nonce::from_slice(b"unique nonce");
//
//     let ciphertext = cipher
//         .encrypt(nonce, string.as_bytes().as_ref())
//         .expect("encrypt should work");
//
//     general_purpose::STANDARD_NO_PAD.encode(ciphertext).to_string()
// }
