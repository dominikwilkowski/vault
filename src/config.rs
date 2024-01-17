use serde::{Deserialize, Serialize};
use std::{
	env, fs,
	sync::{Arc, RwLock},
};

use crate::{
	db::{Db, DbEntry},
	encryption::decrypt_vault,
};

#[derive(Debug, Deserialize, Serialize)]
struct ConfigFile {
	pub general: ConfigGeneral,
	pub db: ConfigFileDb,
}

#[derive(Debug, Deserialize, Serialize)]
struct ConfigFileDb {
	pub cypher: String,
	pub salt: String,
	pub encrypted: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct ConfigFileCypher {
	pub contents: Vec<DbEntry>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Config {
	#[serde(with = "arc_rwlock_serde")]
	pub general: Arc<RwLock<ConfigGeneral>>,
	#[serde(with = "arc_rwlock_serde")]
	pub db: Arc<RwLock<Db>>,
	#[serde(with = "arc_rwlock_serde")]
	config_db: Arc<RwLock<ConfigFileDb>>,
	pub vault_unlocked: bool,
}

mod arc_rwlock_serde {
	use serde::ser::Serializer;
	use serde::Serialize;
	use std::sync::{Arc, RwLock};

	pub fn serialize<S, T>(val: &Arc<RwLock<T>>, s: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
		T: Serialize,
	{
		T::serialize(&*val.read().unwrap(), s)
	}

	// pub fn deserialize<'de, D, T>(d: D) -> Result<Arc<RwLock<T>>, D::Error>
	// where
	// 	D: Deserializer<'de>,
	// 	T: Deserialize<'de>,
	// {
	// 	Ok(Arc::new(RwLock::new(T::deserialize(d)?)))
	// }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigGeneral {
	pub something: bool,
	pub db_timeout: f64,
}

impl Default for Config {
	fn default() -> Self {
		Config {
			general: Arc::new(RwLock::new(ConfigGeneral {
				something: true,
				db_timeout: 900.0,
			})),
			db: Arc::new(RwLock::new(Db::default())),
			vault_unlocked: false,
			config_db: Arc::new(RwLock::new(ConfigFileDb {
				cypher: "".to_string(),
				salt: "".to_string(),
				encrypted: false,
			})),
		}
	}
}

impl From<ConfigFile> for Config {
	fn from(config_file: ConfigFile) -> Self {
		Config {
			general: Arc::new(RwLock::new(ConfigGeneral {
				something: config_file.general.something,
				db_timeout: config_file.general.db_timeout,
			})),
			vault_unlocked: false,
			db: Arc::new(RwLock::new(Db::default())),
			config_db: Arc::new(RwLock::new(ConfigFileDb {
				cypher: config_file.db.cypher.clone(),
				encrypted: config_file.db.encrypted,
				salt: config_file.db.salt,
			})),
		}
	}
}

impl Config {
	pub fn new() -> Self {
		let cwd = match env::current_dir() {
			Ok(path) => format!("{}", path.display()), // default to current working dir
			Err(_) => String::from(""),                // fallback to root dir
		};

		let path = format!("{}/vault_config.toml", cwd);
		match fs::read_to_string(&path) {
			Ok(content) => {
				let file_contents: ConfigFile = toml::from_str(&content).unwrap();
				file_contents.into()
			}
			Err(_) => {
				println!("writing new config");
				let config = Config::default();
				match fs::write(&path, toml::to_string_pretty(&config).unwrap()) {
					Ok(_) => config,
					Err(_) => panic!("Can't write config file"),
				}
			}
		}
	}

	pub fn decrypt_database(&mut self, password: String) -> bool {
		let contents = if self.config_db.read().unwrap().encrypted {
			let decrypted = decrypt_vault(
				self.config_db.read().unwrap().cypher.clone(),
				password,
				self.config_db.read().unwrap().salt.clone(),
			);
			match decrypted {
				Ok(data) => {
					self.vault_unlocked = true;
					toml::from_str::<ConfigFileCypher>(data.as_str())
				}
				Err(err) => {
					eprintln!("Failed: {err}");
					return false;
				}
			}
		} else {
			self.vault_unlocked = true;
			toml::from_str::<ConfigFileCypher>(
				&self.config_db.read().unwrap().cypher.clone(),
			)
		};
		return match contents {
			Ok(contents) => {
				self.db.write().unwrap().contents = contents.contents;
				true
			}
			Err(_) => false,
		};
	}

	// pub fn encrypt_database(&mut self, password: String) -> bool {
	// 	let contents = if self.config_db.read().unwrap().encrypted {
	//
	//
	// 	} else {
	// 		toml::from_str::<ConfigFileCypher>(
	// 			&self.config_db.read().unwrap().cypher.clone(),
	// 		)
	// 	};
	// 	return match contents {
	// 		Ok(contents) => {
	// 			self.db.write().unwrap().contents = contents.contents;
	// 			true
	// 		}
	// 		Err(_) => false,
	// 	};
	// 	false
	// }
}
