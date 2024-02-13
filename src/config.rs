use anyhow::Result;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::{
	io::Write,
	{fs, sync::Arc},
};

use crate::{db::DynFieldKind, env::Environment};

#[derive(Debug, Deserialize, Serialize)]
struct ConfigFile {
	pub general: ConfigGeneral,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
	#[serde(with = "arc_rwlock_serde")]
	pub general: Arc<RwLock<ConfigGeneral>>,
	#[serde(skip)]
	config_path: Arc<RwLock<String>>,
}

mod arc_rwlock_serde {
	use parking_lot::RwLock;
	use serde::de::Deserializer;
	use serde::ser::Serializer;
	use serde::{Deserialize, Serialize};
	use std::sync::Arc;

	pub fn serialize<S, T>(val: &Arc<RwLock<T>>, s: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
		T: Serialize,
	{
		T::serialize(&*val.read(), s)
	}

	pub fn deserialize<'de, D, T>(d: D) -> Result<Arc<RwLock<T>>, D::Error>
	where
		D: Deserializer<'de>,
		T: Deserialize<'de>,
	{
		Ok(Arc::new(RwLock::new(T::deserialize(d)?)))
	}
}

pub type PresetFields = Vec<(usize, String, String, DynFieldKind)>;

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigGeneral {
	pub db_timeout: f32,
	pub preset_fields: PresetFields,
	pub window_settings: WindowSettings,
	pub db_path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WindowSettings {
	pub sidebar_width: f64,
	pub window_size: (f64, f64),
}

impl Default for WindowSettings {
	fn default() -> Self {
		WindowSettings {
			sidebar_width: 140.0,
			window_size: (800.0, 350.0),
		}
	}
}

impl Default for Config {
	fn default() -> Self {
		let mut config_path = Environment::get_base_path();
		config_path.push("vault_config.toml");

		let mut db_path = Environment::get_base_path();
		db_path.push("vault_db.toml");

		Config {
			general: Arc::new(RwLock::new(ConfigGeneral {
				db_timeout: 900.0,
				window_settings: WindowSettings::default(),
				preset_fields: vec![
					(
						0,
						String::from("Custom"),
						String::from(""),
						DynFieldKind::SecretLine,
					),
					(
						1,
						String::from("Username"),
						String::from("Username"),
						DynFieldKind::SecretLine,
					),
					(
						2,
						String::from("Password"),
						String::from("Password"),
						DynFieldKind::SecretLine,
					),
					(3, String::from("Website"), String::from("URL"), DynFieldKind::Url),
					(
						4,
						String::from("Notes"),
						String::from("Notes"),
						DynFieldKind::TextLine,
					),
				],
				db_path: db_path.into_os_string().to_string_lossy().to_string(),
			})),
			config_path: Arc::new(RwLock::new(
				config_path.into_os_string().to_string_lossy().to_string(),
			)),
		}
	}
}

impl From<ConfigFile> for Config {
	fn from(config_file: ConfigFile) -> Self {
		Config {
			general: Arc::new(RwLock::new(ConfigGeneral {
				db_timeout: config_file.general.db_timeout,
				preset_fields: config_file.general.preset_fields,
				window_settings: WindowSettings {
					sidebar_width: config_file.general.window_settings.sidebar_width,
					window_size: config_file.general.window_settings.window_size,
				},
				db_path: config_file.general.db_path,
			})),
			config_path: Arc::new(RwLock::new(String::from(""))),
		}
	}
}

impl Config {
	pub fn new() -> Self {
		let mut path = Environment::get_base_path();
		path.push("vault_config.toml");
		let config_path = path.into_os_string().to_string_lossy().to_string();

		match Environment::has_config() {
			Ok(content) => {
				let file_contents: ConfigFile = toml::from_str(&content).unwrap();
				let config: Config = file_contents.into();
				*config.config_path.write() = config_path;
				config
			},
			Err(_) => {
				let mut path = Environment::get_base_path();
				path.push("vault_db.toml");
				let db_path = path.into_os_string().to_string_lossy().to_string();

				let config = Config {
					config_path: Arc::new(RwLock::new(config_path.clone())),
					..Default::default()
				};

				// Set the path to the same place the default config goes
				{
					config.general.write().db_path = db_path;
				}

				match fs::write(&config_path, toml::to_string_pretty(&config).unwrap())
				{
					Ok(_) => config,
					Err(_) => panic!("Can't write config file"),
				}
			},
		}
	}

	pub fn save(&self) -> Result<()> {
		let config = toml::to_string_pretty(self)?;
		let mut config_file = fs::OpenOptions::new()
			.write(true)
			.create(true)
			.truncate(true)
			.open(self.config_path.read().clone())?;
		config_file.write_all(config.as_bytes())?;
		config_file.flush()?;
		Ok(())
	}

	pub fn get_field_presets(&self) -> PresetFields {
		self.general.read().preset_fields.clone()
	}

	pub fn add_field_preset(
		&self,
		title: String,
		kind: DynFieldKind,
	) -> PresetFields {
		{
			let id = self
				.general
				.read()
				.preset_fields
				.last()
				.unwrap_or(&(
					0,
					String::from(""),
					String::from(""),
					DynFieldKind::default(),
				))
				.0 + 1;
			self.general.write().preset_fields.push((id, title.clone(), title, kind));
		}

		self.get_field_presets()
	}

	pub fn edit_field_preset(
		&self,
		id: usize,
		title: String,
		kind: DynFieldKind,
	) -> PresetFields {
		let index = self
			.general
			.read()
			.preset_fields
			.iter()
			.position(|item| item.0 == id)
			.unwrap_or(0);
		self.general.write().preset_fields[index] =
			(id, title.clone(), title, kind);

		self.get_field_presets()
	}

	pub fn delete_field_preset(&self, id: usize) -> PresetFields {
		self.general.write().preset_fields.retain(|item| item.0 != id);

		self.get_field_presets()
	}

	pub fn set_sidebar_width(&self, width: f64) {
		self.general.write().window_settings.sidebar_width = width;
		let _ = self.save();
	}

	pub fn set_window_size(&self, size: (f64, f64)) {
		self.general.write().window_settings.window_size = size;
		let _ = self.save();
	}
}
