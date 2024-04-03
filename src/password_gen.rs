use rand::{rngs::OsRng, seq::SliceRandom, Rng};
use sha2::{Digest, Sha256};

use floem::reactive::use_context;

use crate::env::Environment;

pub fn generate_password(entropy: String) -> String {
	let env = use_context::<Environment>().expect("No env context provider");

	// Initialize RNG with system entropy
	let mut rng = OsRng;

	// Hash the byte array using SHA-256
	let hash_result = Sha256::digest(entropy.as_bytes());

	// Convert the resulting hash into a fixed-size byte array ([u8; 32])
	let mut hash_array = [0u8; 32];
	hash_array.copy_from_slice(&hash_result[..]);

	// Mix additional entropy into the RNG
	// We're doing nothing with the resulting array here
	// We're just adding entropy to the system by doing stuff
	// Not the greatest way to do this
	rng.fill(&mut hash_array);

	// Define the character set for the random string
	let charset: [char; 88] = [
		'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e',
		'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
		'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
		'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
		'Y', 'Z', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '-', '+',
		'=', '[', ']', '{', '}', '|', ';', ':', ',', '.', '<', '>', '/',
	];

	// Generate a string of n characters
	let length = env.config.general.read().pass_gen_letter_count;
	(0..length).map(|_| *charset.choose(&mut rng).unwrap()).collect()
}
