use std::env;
use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
	general: ConfigGeneral,
	db: ConfigDb,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigGeneral {
	something: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigDb {
	contents: String,
	timeout: u16,
}

impl Default for Config {
	fn default() -> Self {
		Config {
			general: ConfigGeneral { something: true },
			db: ConfigDb {
				contents: String::from("some long cypher text"),
				timeout: 60,
			},
		}
	}
}

impl Config {
	pub fn new() -> Self {
		let cwd = match env::current_dir() {
			Ok(path) => format!("{}", path.display()), //default to current working dir
			Err(_) => String::from(""),                //fallback to root dir
		};

		let path = format!("{}/vault_config.toml", cwd);
		match fs::read_to_string(&path) {
			Ok(content) => toml::from_str(&content).unwrap(),
			Err(_) => {
				println!("writing new config");
				let config = Config::default();
				match fs::write(&path, toml::to_string(&config).unwrap()) {
					Ok(_) => config,
					Err(_) => panic!("Can't write config file"),
				}
			}
		}
	}
}
