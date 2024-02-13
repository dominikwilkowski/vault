use anyhow::Result;
use std::{
	path::PathBuf,
	{env, fs, sync::Arc},
};

use crate::{config::Config, db::Db};

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
		// TODO: change this to use the systems config folder
		let cwd = match env::current_dir() {
			Ok(path) => format!("{}", path.display()), // default to current working dir
			Err(_) => String::from(""),                // fallback to root dir
		};

		PathBuf::from(cwd)
	}

	pub fn has_config() -> Result<String> {
		let mut config_path = Environment::get_base_path();
		config_path.push("vault_config.toml");
		Ok(fs::read_to_string(config_path)?)
	}

	pub fn new() -> Self {
		let config = Config::new();
		let db = Db::new(config.general.read().db_path.clone());
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
