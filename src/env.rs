use crate::config::Config;
use crate::db::Db;
use anyhow::Result;
use std::sync::Arc;

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
	pub fn new(config: Config) -> Self {
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
