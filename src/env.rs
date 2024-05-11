use anyhow::Result;
use std::{
	path::PathBuf,
	{fs, sync::Arc},
};

use crate::{
	config::{Config, CONFIG_FILE_NAME},
	db::Db,
};

#[derive(Debug, Clone)]
pub struct Environment {
	pub config: Arc<Config>,
	pub db: Arc<Db>,
}

impl Default for Environment {
	fn default() -> Self {
		Environment {
			config: Arc::new(Config::default()),
			db: Arc::new(Db::default()),
		}
	}
}

impl Environment {
	pub fn get_base_path() -> PathBuf {
		if std::env::var("DEBUG").is_ok() {
			return PathBuf::from(".");
		}

		let config_path = match dirs::config_dir() {
			Some(path) => format!("{}{}", path.display(), "/rusty_vault/"),
			None => String::from("."), // fallback to current dir
		};

		if fs::create_dir_all(config_path.clone()).is_err() {
			PathBuf::from(".")
		} else {
			PathBuf::from(config_path)
		}
	}

	pub fn has_config() -> Result<String> {
		let mut config_path = Environment::get_base_path();
		config_path.push(CONFIG_FILE_NAME);
		Ok(fs::read_to_string(config_path)?)
	}

	pub fn load() -> Self {
		let config = Config::load();
		let db = Db::load(config.general.read().db_path.clone());
		Environment {
			config: Arc::new(config),
			db: Arc::new(db),
		}
	}

	pub fn save(&self) -> Result<()> {
		self.config.save()?;
		self.db.save()?;
		Ok(())
	}
}
