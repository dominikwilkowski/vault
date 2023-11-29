use std::env;
use std::fs;

use serde::{Deserialize, Serialize};

use crate::db::{Db, DbEntry};

#[derive(Debug, Deserialize, Serialize)]
struct ConfigFile {
	general: ConfigGeneral,
	db: ConfigFileDb,
}

#[derive(Debug, Deserialize, Serialize)]
struct ConfigFileDb {
	cypher: String,
	timeout: u16,
}

#[derive(Debug, Deserialize, Serialize)]
struct ConfigFileCypher {
	contents: Vec<DbEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
	general: ConfigGeneral,
	db: Db,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigGeneral {
	something: bool,
}

impl Default for Config {
	fn default() -> Self {
		Config {
			general: ConfigGeneral { something: true },
			db: Db {
				contents: vec![DbEntry {
					id: 0,
					title: String::from("Hello world"),
					url: String::from("https://"),
					username: vec![String::from("")],
					password: vec![String::from("")],
					notes: String::from(""),
				}],
				timeout: 60,
			},
		}
	}
}

impl From<ConfigFile> for Config {
	fn from(config_file: ConfigFile) -> Self {
		Config {
			general: ConfigGeneral {
				something: config_file.general.something,
			},
			db: Db {
				timeout: config_file.db.timeout,
				contents: toml::from_str::<ConfigFileCypher>(&config_file.db.cypher)
					.unwrap()
					.contents,
			},
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