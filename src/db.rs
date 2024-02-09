use anyhow::bail;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::{
	fs,
	io::Write,
	path::Path,
	sync::Arc,
	time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::{
	db::ChangeError::WrongPassword,
	encryption::{decrypt_vault, encrypt_vault, password_hash, CryptError},
};

type SecureField = (u64, String);

#[derive(thiserror::Error, Debug)]
pub enum ChangeError {
	#[error("Wrong password provided")]
	WrongPassword(),
	#[error("Crypt error")]
	CryptError(#[from] CryptError),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DynField {
	id: usize,
	kind: DynFieldKind,
	title: String,
	visible: bool,
	value: Vec<SecureField>,
}

#[derive(
	Debug, Deserialize, Serialize, Clone, PartialEq, Default, Eq, Hash,
)]
pub enum DynFieldKind {
	#[default]
	TextLine,
	SecretLine,
	Url,
}

impl std::fmt::Display for DynFieldKind {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			DynFieldKind::TextLine => write!(f, "TextLine"),
			DynFieldKind::SecretLine => write!(f, "SecretLine"),
			DynFieldKind::Url => write!(f, "Url"),
		}
	}
}

impl DynFieldKind {
	pub fn all_values() -> Vec<DynFieldKind> {
		vec![
			DynFieldKind::SecretLine,
			DynFieldKind::TextLine,
			DynFieldKind::Url,
		]
	}
}

impl Default for DynField {
	fn default() -> Self {
		Self {
			id: 0,
			kind: DynFieldKind::SecretLine,
			title: String::from("Notes"),
			visible: true,
			value: vec![(0, String::from("My notes"))],
		}
	}
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DbEntry {
	pub id: usize,
	pub title: String,
	pub fields: Vec<DynField>,
}

#[derive(Debug)]
pub struct NewDbEntry {
	pub title: String,
	pub fields: Vec<DynField>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DbEntryNonSecure {
	pub id: usize,
	pub title: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DbFields {
	Id,
	Title,
	Fields(usize),
}

impl std::fmt::Display for DbFields {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			DbFields::Id => write!(f, "Id"),
			DbFields::Title => write!(f, "Title"),
			DbFields::Fields(idx) => write!(f, "Fields-{}", idx),
		}
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DbFile {
	pub db: DbFileDb,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DbFileDb {
	pub encrypted: bool,
	pub salt: String,
	cypher: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct DbFileCypher {
	pub contents: Vec<DbEntry>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Db {
	#[serde(skip_serializing, with = "arc_rwlock_serde")]
	pub contents: Arc<RwLock<Vec<DbEntry>>>,
	#[serde(rename(serialize = "db"), with = "arc_rwlock_serde")]
	pub config_db: Arc<RwLock<DbFileDb>>,
	#[serde(skip)]
	pub vault_unlocked: Arc<RwLock<bool>>,
	#[serde(skip)]
	hash: Arc<RwLock<[u8; 32]>>,
	#[serde(skip)]
	db_path: Arc<RwLock<String>>,
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

impl Default for Db {
	fn default() -> Self {
		Db {
			contents: Arc::new(RwLock::new(vec![DbEntry {
				id: 1,
				title: String::from("My Bank Deets"),
				fields: vec![
					DynField {
						id: 0,
						kind: DynFieldKind::TextLine,
						title: String::from("Title"),
						visible: true,
						value: vec![(1702851212, String::from("Bank"))],
					},
					DynField {
						id: 1,
						kind: DynFieldKind::Url,
						title: String::from("Url"),
						visible: true,
						value: vec![(
							1702851212,
							String::from("https://bankofaustralia.com.au"),
						)],
					},
					DynField {
						id: 2,
						kind: DynFieldKind::SecretLine,
						title: String::from("Username"),
						visible: true,
						value: vec![(1702851212, String::from("Dom"))],
					},
					DynField {
						id: 3,
						kind: DynFieldKind::SecretLine,
						title: String::from("Password"),
						visible: true,
						value: vec![(
							1702851212,
							String::from("totally_secure_password!1"),
						)],
					},
					DynField {
						id: 4,
						kind: DynFieldKind::SecretLine,
						title: String::from("Notes"),
						visible: true,
						value: vec![(1702851212, String::from("These are my bank deets"))],
					},
				],
			}])),
			config_db: Arc::new(RwLock::new(DbFileDb {
				encrypted: false,
				salt: "".to_string(),
				cypher: "".to_string(),
			})),
			vault_unlocked: Arc::new(Default::default()),
			hash: Arc::new(Default::default()),
			db_path: Arc::new(RwLock::new(String::from(""))),
		}
	}
}

fn to_tuple(item: &DbEntry, idx: usize) -> (usize, &'static str, usize) {
	(item.id, Box::leak(item.title.clone().into_boxed_str()), idx)
}

impl From<DbFile> for Db {
	fn from(db_file: DbFile) -> Self {
		Db {
			contents: Arc::new(RwLock::new(Vec::<DbEntry>::new())),
			config_db: Arc::new(RwLock::new(DbFileDb {
				encrypted: db_file.db.encrypted,
				salt: db_file.db.salt,
				cypher: db_file.db.cypher,
			})),
			vault_unlocked: Arc::new(RwLock::new(false)),
			hash: Arc::new(RwLock::new(*b"00000000000000000000000000000000")),
			db_path: Arc::new(RwLock::new(String::from(""))),
		}
	}
}

impl Db {
	pub fn new(db_path: String) -> Self {
		let path = Path::new(db_path.as_str());

		match fs::read_to_string(path) {
			Ok(content) => {
				let file_contents: DbFile = toml::from_str(&content).unwrap();
				let db: Db = file_contents.into();
				*db.db_path.write() = db_path.clone();
				db
			},
			Err(_) => {
				println!("writing new config");
				// TODO: start onboarding flow (new password)
				let db = Db {
					db_path: Arc::new(RwLock::new(db_path.clone())),
					..Default::default()
				};
				match fs::write(path, toml::to_string_pretty(&db).unwrap()) {
					Ok(_) => db,
					Err(_) => panic!("Can't write config file"),
				}
			},
		}
	}

	pub fn decrypt_database(&self, password: String) -> anyhow::Result<()> {
		let mut hash = self.hash.write();
		*hash = password_hash(password, self.config_db.read().salt.clone())?;
		drop(hash);

		let contents = if self.config_db.read().encrypted {
			let decrypted =
				decrypt_vault(self.config_db.read().cypher.clone(), *self.hash.read())?;
			toml::from_str::<DbFileCypher>(decrypted.as_str())?
		} else {
			toml::from_str::<DbFileCypher>(&self.config_db.read().cypher.clone())?
		};

		*self.vault_unlocked.write() = true;
		*self.contents.write() = contents.contents;
		Ok(())
	}

	fn serialize_db(&self) -> anyhow::Result<()> {
		// self.db -> self.config_db.cypher as toml
		#[derive(Debug, Serialize, Deserialize)]
		struct DbStruct {
			contents: Vec<DbEntry>,
		}
		let db = DbStruct {
			contents: self.contents.read().clone(),
		};
		let mut cypher = toml::to_string(&db)?;
		if self.config_db.read().encrypted {
			cypher = encrypt_vault(cypher, *self.hash.read())?;
		}
		self.config_db.write().cypher = cypher;
		Ok(())
	}

	pub fn save(&self) -> anyhow::Result<()> {
		self.serialize_db()?;
		let config = toml::to_string_pretty(self)?;
		let mut config_file = fs::OpenOptions::new()
			.write(true)
			.truncate(true)
			.open(self.db_path.read().clone())?;
		config_file.write_all(config.as_bytes())?;
		config_file.flush()?;
		Ok(())
	}

	pub fn change_password(
		&self,
		old: String,
		new: String,
	) -> anyhow::Result<()> {
		let old_hash = password_hash(old, self.config_db.read().salt.clone())?;
		if old_hash != *self.hash.read() {
			bail!(WrongPassword())
		}
		let new_hash = password_hash(new, self.config_db.read().salt.clone())?;
		*self.hash.write() = new_hash;
		self.save()?;
		Ok(())
	}

	pub fn set_db_path(&self, path: String) {
		*self.db_path.write() = path;
	}

	pub fn clear_hash(&self) {
		// TODO: Eventually zeroize here?
		*self.hash.write() = *b"00000000000000000000000000000000";
	}

	// get the list of all entries for sidebar view
	pub fn get_list(&self) -> im::Vector<(usize, &'static str, usize)> {
		self
			.contents
			.read()
			.iter()
			.enumerate()
			.map(|(idx, item)| to_tuple(item, idx))
			.rev()
			.collect()
	}

	// get content of entry
	fn get_by_id_secure(&self, id: &usize) -> DbEntry {
		if let Some(found_entry) =
			self.contents.read().iter().find(|item| item.id == *id)
		{
			found_entry.clone()
		} else {
			DbEntry {
				id: *id,
				title: String::from("Not found"),
				fields: vec![DynField::default()],
			}
		}
	}

	// get name of dyn field
	pub fn get_name_of_dyn_field(&self, id: &usize, field: &DbFields) -> String {
		let entry = self.get_by_id_secure(id);
		let field_id = match field {
			DbFields::Fields(idx) => idx,
			_ => &0,
		};
		self.get_field_by_id(&entry, field_id).title
	}

	// get kind of dyn field
	pub fn get_dyn_field_kind(
		&self,
		id: &usize,
		field: &DbFields,
	) -> DynFieldKind {
		let entry = self.get_by_id_secure(id);
		match field {
			DbFields::Id | DbFields::Title => DynFieldKind::TextLine,
			DbFields::Fields(field_id) => self.get_field_by_id(&entry, field_id).kind,
		}
	}

	// get non secure content of entry
	pub fn get_by_id(&self, id: &usize) -> DbEntryNonSecure {
		let entry = self.get_by_id_secure(id);

		DbEntryNonSecure {
			id: *id,
			title: entry.title,
		}
	}

	// get content of dynamic field by id
	fn get_field_by_id(&self, entry: &DbEntry, field_id: &usize) -> DynField {
		entry
			.fields
			.clone()
			.into_iter()
			.find(|field| field.id == *field_id)
			.unwrap_or(DynField {
				id: *field_id,
				kind: DynFieldKind::SecretLine,
				title: String::from("Notes"),
				visible: true,
				value: vec![(0, String::from("My Notes"))],
			})
	}

	// get a list of all dynamic fields
	pub fn get_dyn_fields(&self, id: &usize) -> Vec<DbFields> {
		let entry = self.get_by_id_secure(id);

		entry
			.fields
			.iter()
			.filter(|field| field.visible)
			.map(|field| DbFields::Fields(field.id))
			.collect()
	}

	// get a list of all dynamic fields
	pub fn get_hidden_dyn_fields(&self, id: &usize) -> Vec<DbFields> {
		let entry = self.get_by_id_secure(id);

		entry
			.fields
			.iter()
			.filter(|field| !field.visible)
			.map(|field| DbFields::Fields(field.id))
			.collect()
	}

	// get the latest entry of a field
	pub fn get_last_by_field(&self, id: &usize, field: &DbFields) -> String {
		let entry = self.get_by_id_secure(id);

		match field {
			DbFields::Id => format!("{:?}", entry.id),
			DbFields::Title => entry.title,
			DbFields::Fields(field_id) => {
				self.get_field_by_id(&entry, field_id).value.last().unwrap().1.clone()
			},
		}
	}

	// get the entry n of a field (look into the history of a field)
	pub fn get_n_by_field(
		&self,
		id: &usize,
		field: &DbFields,
		n: usize,
	) -> String {
		let entry = self.get_by_id_secure(id);

		match field {
			DbFields::Id => format!("{:?}", entry.id),
			DbFields::Title => entry.title,
			DbFields::Fields(field_id) => self
				.get_field_by_id(&entry, field_id)
				.value
				.into_iter()
				.rev()
				.collect::<Vec<SecureField>>()[n]
				.1
				.clone(),
		}
	}

	// get the entire history of a field
	pub fn get_history(
		&self,
		id: &usize,
		field: &DbFields,
	) -> Option<im::Vector<SecureField>> {
		let entry = self.get_by_id_secure(id);

		match field {
			DbFields::Id => None,
			DbFields::Title => None,
			DbFields::Fields(field_id) => Some(
				self
					.get_field_by_id(&entry, field_id)
					.value
					.into_iter()
					.rev()
					.collect::<im::Vector<SecureField>>(),
			),
		}
	}

	// get the date and id of a field
	pub fn get_history_dates(
		&self,
		id: &usize,
		field: &DbFields,
	) -> Vec<(usize, u64)> {
		let entry = self.get_by_id_secure(id);

		match field {
			DbFields::Id => vec![(0, 0)],
			DbFields::Title => vec![(0, 0)],
			DbFields::Fields(field_id) => self
				.get_field_by_id(&entry, field_id)
				.value
				.iter()
				.map(|item| item.0)
				.enumerate()
				.collect(),
		}
	}

	// add a new entry
	pub fn add(&self, title: String) -> usize {
		let new_id = self
			.contents
			.read()
			.last()
			.unwrap_or(&DbEntry {
				id: 1,
				title: String::from("New Entry"),
				fields: vec![DynField::default()],
			})
			.id + 1;

		self.contents.write().push(DbEntry {
			id: new_id,
			title,
			fields: Vec::new(),
		});

		new_id
	}

	// add a new field to an entry
	pub fn add_dyn_field(
		&self,
		id: &usize,
		kind: DynFieldKind,
		title_value: String,
		field_value: String,
	) -> Vec<DbFields> {
		self.contents.write().iter_mut().for_each(|item| {
			if item.id == *id {
				let id = item.fields.last().unwrap_or(&DynField::default()).id + 1;
				item.fields.push(DynField {
					id,
					kind: kind.clone(),
					title: title_value.clone(),
					visible: true,
					value: vec![(0, field_value.clone())],
				});
			}
		});
		self.get_dyn_fields(id)
	}

	// change the title of a dyn field
	pub fn edit_dyn_field_title(
		&self,
		id: &usize,
		field: &DbFields,
		title: String,
	) {
		self.contents.write().iter_mut().for_each(|item| {
			if item.id == *id {
				if let DbFields::Fields(field_id) = field {
					item
						.fields
						.iter_mut()
						.find(|field| field.id == *field_id)
						.unwrap_or(&mut DynField {
							id: *field_id,
							kind: DynFieldKind::SecretLine,
							title: String::from("Notes"),
							visible: true,
							value: vec![(0, String::from("My Notes"))],
						})
						.title = title.clone();
				}
			}
		});
	}

	pub fn edit_dyn_field_visbility(
		&self,
		id: &usize,
		field: &DbFields,
		visible: bool,
	) -> Vec<DbFields> {
		self.contents.write().iter_mut().for_each(|item| {
			if item.id == *id {
				if let DbFields::Fields(field_id) = field {
					item
						.fields
						.iter_mut()
						.find(|field| field.id == *field_id)
						.unwrap_or(&mut DynField {
							id: *field_id,
							kind: DynFieldKind::SecretLine,
							title: String::from("Notes"),
							visible,
							value: vec![(0, String::from("My Notes"))],
						})
						.visible = visible;
				}
			}
		});

		self.get_hidden_dyn_fields(id)
	}

	// edit a field
	pub fn edit_field(&self, id: usize, field: &DbFields, new_content: String) {
		let mut index: usize = 0;
		self.contents.read().iter().enumerate().find(|(idx, item)| {
			if item.id == id {
				index = *idx;
				true
			} else {
				false
			}
		});

		if let Some(entry) = self.contents.write().get_mut(index) {
			let timestamp: u64 = SystemTime::now()
				.duration_since(UNIX_EPOCH)
				.unwrap_or(Duration::new(0, 0))
				.as_secs();

			match field {
				DbFields::Id => {
					panic!("Can't change the ID of an entry");
				},
				DbFields::Title => {
					entry.title = new_content;
				},
				DbFields::Fields(field_id) => {
					entry
						.fields
						.iter_mut()
						.find(|field| field.id == *field_id)
						.unwrap_or(&mut DynField {
							id: *field_id,
							kind: DynFieldKind::SecretLine,
							title: String::from("Notes"),
							visible: true,
							value: vec![(0, String::from("My Notes"))],
						})
						.value
						.push((timestamp, new_content));
				},
			}
		}
	}
}
