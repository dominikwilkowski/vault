use aes_gcm_siv::{
	aead::{Aead, KeyInit},
	Aes256GcmSiv, Nonce,
};
use argon2::Argon2;
use base64::{engine::general_purpose, Engine as _};
use std::str::from_utf8;

pub fn decrypt_aes(string: String) -> String {
	let password = b"password_is_bad!"; // Bad password; don't actually use!
	let salt = b"randomly salty salt"; // Salt should be unique per password

	let mut okm = [0u8; 32]; // Can be any desired size
	Argon2::default()
		.hash_password_into(password, salt, &mut okm)
		.expect("Something went wrong!");

	let cipher = Aes256GcmSiv::new_from_slice(okm.as_slice())
		.expect("creating new cypher blew up");
	let nonce = Nonce::from_slice(b"unique nonce");

	let cyphertext_from_string =
		general_purpose::STANDARD_NO_PAD.decode(string).unwrap();

	let plaintext = cipher
		.decrypt(nonce, cyphertext_from_string.as_ref())
		.expect("decrypt should work");
	from_utf8(plaintext.as_slice()).unwrap().to_string()
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
