use std::env;
use std::fs;
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

use crate::db::{Db, DbEntry};

#[derive(Debug, Deserialize, Serialize)]
struct ConfigFile {
	pub general: ConfigGeneral,
	pub db: ConfigFileDb,
}

#[derive(Debug, Deserialize, Serialize)]
struct ConfigFileDb {
	pub cypher: String,
	pub nonce: String,
	pub encrypted: bool,
	pub timeout: u16,
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
	// 	where D: Deserializer<'de>,
	// 		  T: Deserialize<'de>,
	// {
	// 	Ok(Arc::new(RwLock::new(T::deserialize(d)?)))
	// }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigGeneral {
	pub something: bool,
}

impl Default for Config {
	fn default() -> Self {
		Config {
			general: Arc::new(RwLock::new(ConfigGeneral { something: true })),
			db: Arc::new(RwLock::new(Db::default())),
		}
	}
}

impl From<ConfigFile> for Config {
	fn from(config_file: ConfigFile) -> Self {
		let contents;
		if config_file.db.encrypted {
			contents = toml::from_str::<ConfigFileCypher>(
				crate::encryption::decrypt_aes(config_file.db.cypher).as_str(),
			)
			.unwrap()
			.contents;
		} else {
			contents = toml::from_str::<ConfigFileCypher>(&config_file.db.cypher)
				.unwrap()
				.contents;
		}
		Config {
			general: Arc::new(RwLock::new(ConfigGeneral {
				something: config_file.general.something,
			})),
			db: Arc::new(RwLock::new(Db {
				timeout: config_file.db.timeout,
				contents: contents,
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
}
