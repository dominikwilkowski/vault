use anyhow::bail;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::{
	fs,
	io::Write,
	path::PathBuf,
	sync::Arc,
	time::{Duration, SystemTime, UNIX_EPOCH},
};
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

use floem::reactive::SignalUpdate;

use crate::{
	config::DB_FILE_NAME,
	db::ChangeError::WrongPassword,
	encryption::{decrypt_vault, encrypt_vault, password_hash, CryptError},
	env::Environment,
	password_gen::get_random_string,
	ui::app_view::SidebarList,
	ui::shared_context,
};

const SALT_LETTER_COUNT: usize = 32;

type SecureField = (u64, String);

#[derive(thiserror::Error, Debug)]
pub enum ChangeError {
	#[error("Wrong password provided")]
	WrongPassword(),
	#[error("Crypt error")]
	CryptError(#[from] CryptError),
}

#[derive(Debug, Deserialize, Serialize, Clone, Zeroize, ZeroizeOnDrop)]
pub struct DynField {
	id: usize,
	#[zeroize(skip)]
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
	TextLineSecret,
	TextLine,
	MultiLine,
	MultiLineSecret,
	Url,
	Heading,
}

impl std::fmt::Display for DynFieldKind {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			DynFieldKind::TextLineSecret => write!(f, "Secret Single Line"),
			DynFieldKind::TextLine => write!(f, "Single Line"),
			DynFieldKind::MultiLineSecret => write!(f, "Multi Line Secret"),
			DynFieldKind::MultiLine => write!(f, "Multi Line"),
			DynFieldKind::Url => write!(f, "Url"),
			DynFieldKind::Heading => write!(f, "Heading"),
		}
	}
}

impl DynFieldKind {
	pub fn all_values() -> Vec<DynFieldKind> {
		vec![
			DynFieldKind::TextLineSecret,
			DynFieldKind::TextLine,
			DynFieldKind::MultiLineSecret,
			DynFieldKind::MultiLine,
			DynFieldKind::Url,
			DynFieldKind::Heading,
		]
	}
}

impl Default for DynField {
	fn default() -> Self {
		Self {
			id: 0,
			kind: DynFieldKind::TextLineSecret,
			title: String::from("Notes"),
			visible: true,
			value: vec![(0, String::from("My notes"))],
		}
	}
}

#[derive(Debug, Deserialize, Serialize, Clone, Zeroize, ZeroizeOnDrop)]
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
	pub salt_letter_count: usize,
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
	hash: Arc<RwLock<Zeroizing<[u8; 32]>>>,
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
		let mut db_path = Environment::get_base_path();
		db_path.push(DB_FILE_NAME);

		Db {
			contents: Arc::new(RwLock::new(vec![DbEntry {
				id: 1,
				title: String::from("Vault tips and tricks"),
				fields: vec![DynField {
					id: 4,
					kind: DynFieldKind::MultiLine,
					title: String::from("Tips"),
					visible: true,
					value: vec![(
						1702851212,
						String::from(
							"Welcome to Vault\n\n\
							- To find an entry just use the search bar\n\
							- You can set your own presets for custom fields for faster editing\n\
							- You can adjust the auto-lock timer in the settings\n\
							",
						),
					)],
				}],
			}])),
			config_db: Arc::new(RwLock::new(DbFileDb {
				encrypted: true,
				salt_letter_count: SALT_LETTER_COUNT,
				salt: get_random_string(SALT_LETTER_COUNT),
				cypher: "".to_string(),
			})),
			vault_unlocked: Arc::new(Default::default()),
			hash: Arc::new(Default::default()),
			db_path: Arc::new(RwLock::new(
				db_path.into_os_string().to_string_lossy().to_string(),
			)),
		}
	}
}

fn to_tuple(item: &DbEntry, idx: usize) -> (usize, String, usize) {
	(item.id, item.title.clone(), idx)
}

impl From<DbFile> for Db {
	fn from(db_file: DbFile) -> Self {
		Db {
			contents: Arc::new(RwLock::new(Vec::<DbEntry>::new())),
			config_db: Arc::new(RwLock::new(DbFileDb {
				encrypted: db_file.db.encrypted,
				salt_letter_count: db_file.db.salt_letter_count,
				salt: db_file.db.salt,
				cypher: db_file.db.cypher,
			})),
			vault_unlocked: Arc::new(RwLock::new(false)),
			hash: Arc::new(Default::default()),
			db_path: Arc::new(RwLock::new(String::from(""))),
		}
	}
}

impl Db {
	pub fn load(db_path: String) -> Self {
		let path = PathBuf::from(db_path.as_str());

		match fs::read_to_string(path.clone()) {
			Ok(content) => {
				// an unencrypted (debug) file holds the plaintext vault
				let content = Zeroizing::new(content);
				let file_contents: DbFile = toml::from_str(&content).unwrap();
				let db: Db = file_contents.into();
				db.db_path.write().clone_from(&db_path);
				db
			},
			Err(_) => Db {
				db_path: Arc::new(RwLock::new(db_path.clone())),
				..Default::default()
			},
		}
	}

	pub fn decrypt_database(&self, password: &str) -> anyhow::Result<()> {
		let hash = password_hash(password.as_bytes(), &self.config_db.read().salt)?;

		let contents = {
			let config_db = self.config_db.read();
			// the plaintext is wiped again once it has been parsed
			let decrypted = if config_db.encrypted {
				decrypt_vault(&config_db.cypher, &hash)?
			} else {
				Zeroizing::new(config_db.cypher.clone())
			};
			toml::from_str::<DbFileCypher>(&decrypted)?
		};

		*self.hash.write() = hash;
		*self.vault_unlocked.write() = true;
		*self.contents.write() = contents.contents;
		Ok(())
	}

	fn serialize_db(&self) -> anyhow::Result<()> {
		// self.contents -> self.config_db.cypher as toml, serialized by reference
		#[derive(Debug, Serialize)]
		struct DbStruct<'a> {
			contents: &'a Vec<DbEntry>,
		}
		let cypher = {
			let contents = self.contents.read();
			let plaintext = Zeroizing::new(toml::to_string(&DbStruct {
				contents: &contents,
			})?);
			if self.config_db.read().encrypted {
				encrypt_vault(plaintext.as_bytes(), &self.hash.read())?
			} else {
				plaintext.to_string()
			}
		};
		self.config_db.write().cypher = cypher;
		Ok(())
	}

	pub fn export(&self) -> anyhow::Result<String> {
		self.serialize_db()?;
		Ok(toml::to_string_pretty(self)?)
	}

	pub fn save(&self) -> anyhow::Result<()> {
		let config = self.export()?;
		let mut config_file = fs::OpenOptions::new()
			.write(true)
			.create(true)
			.truncate(true)
			.open(self.db_path.read().clone())?;
		config_file.write_all(config.as_bytes())?;
		config_file.flush()?;
		Ok(())
	}

	pub fn change_password(&self, old: &str, new: &str) -> anyhow::Result<()> {
		let old_hash = password_hash(old.as_bytes(), &self.config_db.read().salt)?;
		if old_hash != *self.hash.read() {
			bail!(WrongPassword())
		}
		self.set_password(new)
	}

	pub fn change_salt(&self) {
		let lenght = self.config_db.read().salt_letter_count;
		let salt = get_random_string(lenght);
		self.config_db.write().salt = salt;
	}

	pub fn set_password(&self, new: &str) -> anyhow::Result<()> {
		self.change_salt();

		let new_hash = password_hash(new.as_bytes(), &self.config_db.read().salt)?;
		*self.hash.write() = new_hash;
		self.save()?;
		Ok(())
	}

	pub fn set_db_path(&self, path: String) {
		*self.db_path.write() = path;
	}

	/// Forget the key and the decrypted entries; the entries wipe themselves
	/// when they are dropped.
	pub fn wipe(&self) {
		self.hash.write().zeroize();
		self.contents.write().clear();
	}

	pub fn lock(&self) {
		self.wipe();

		// cleaning context
		let list_sidebar_signal = shared_context::get::<SidebarList>()
			.expect("No list_sidebar_signal context provider");
		list_sidebar_signal.update(|sidebar| {
			// clear data
			for (id, title, idx) in sidebar.iter_mut() {
				id.zeroize();
				title.zeroize();
				idx.zeroize();
			}

			// remove meta data (number of entries)
			sidebar.clear();
		});
	}

	// PRIVATE: look at an entry without copying it
	fn with_entry<T>(
		&self,
		id: &usize,
		f: impl FnOnce(Option<&DbEntry>) -> T,
	) -> T {
		let contents = self.contents.read();
		f(contents.iter().find(|item| item.id == *id))
	}

	// PRIVATE: look at a field of an entry without copying it
	fn with_field<T>(
		entry: Option<&DbEntry>,
		field_id: &usize,
		f: impl FnOnce(&DynField) -> T,
	) -> T {
		match entry
			.and_then(|entry| entry.fields.iter().find(|field| field.id == *field_id))
		{
			Some(field) => f(field),
			None => f(&DynField {
				id: *field_id,
				kind: DynFieldKind::TextLineSecret,
				title: String::from("Notes"),
				visible: true,
				value: vec![(0, String::from("My Notes"))],
			}),
		}
	}

	// PRIVATE: the id a field request points at
	fn field_id(field: &DbFields) -> usize {
		match field {
			DbFields::Fields(idx) => *idx,
			_ => 0,
		}
	}

	// get the list of all entries for sidebar view
	pub fn get_sidebar_list(&self) -> imbl::Vector<(usize, String, usize)> {
		self
			.contents
			.read()
			.iter()
			.enumerate()
			.map(|(idx, item)| to_tuple(item, idx))
			.rev()
			.collect()
	}

	// search through db and return a list for sidebar view
	pub fn search(&self, needle: &str) -> imbl::Vector<(usize, String, usize)> {
		let lowercase_needle = Zeroizing::new(needle.to_lowercase());
		self
			.contents
			.read()
			.iter()
			.enumerate()
			.filter(|(_, entry)| {
				// look at entry title
				contains_ignore_case(&entry.title, &lowercase_needle)
					|| entry.fields.iter().any(|field| {
						// look at field title
						contains_ignore_case(&field.title, &lowercase_needle) ||
						// look at field value (important for notes) but we only look at the last value
						field
							.value
							.last()
							.is_some_and(|(_, value)| contains_ignore_case(value, &lowercase_needle))
					})
			})
			.map(|(idx, item)| to_tuple(item, idx))
			.rev()
			.collect()
	}

	// get non secure content of entry
	pub fn get_by_id(&self, id: &usize) -> DbEntryNonSecure {
		DbEntryNonSecure {
			id: *id,
			title: self.with_entry(id, |entry| {
				entry.map_or_else(
					|| String::from("Not found"),
					|entry| entry.title.clone(),
				)
			}),
		}
	}

	// get a list of all fields
	pub fn get_fields(&self, id: &usize) -> Vec<(DbFields, bool)> {
		self.with_entry(id, |entry| match entry {
			Some(entry) => entry
				.fields
				.iter()
				.map(|field| (DbFields::Fields(field.id), field.visible))
				.collect(),
			None => vec![(DbFields::Fields(0), true)],
		})
	}

	// get a list of all visible fields
	pub fn get_visible_fields(&self, id: &usize) -> Vec<DbFields> {
		self.with_entry(id, |entry| match entry {
			Some(entry) => entry
				.fields
				.iter()
				.filter(|field| field.visible)
				.map(|field| DbFields::Fields(field.id))
				.collect(),
			None => vec![DbFields::Fields(0)],
		})
	}

	// get a list of all hidden fields
	pub fn get_hidden_fields(&self, id: &usize) -> Vec<DbFields> {
		self.with_entry(id, |entry| match entry {
			Some(entry) => entry
				.fields
				.iter()
				.filter(|field| !field.visible)
				.map(|field| DbFields::Fields(field.id))
				.collect(),
			None => Vec::new(),
		})
	}

	// get name of field
	pub fn get_name_of_field(&self, id: &usize, field: &DbFields) -> String {
		self.with_entry(id, |entry| {
			Self::with_field(entry, &Self::field_id(field), |field| {
				field.title.clone()
			})
		})
	}

	// get kind of field
	pub fn get_field_kind(&self, id: &usize, field: &DbFields) -> DynFieldKind {
		match field {
			DbFields::Id | DbFields::Title => DynFieldKind::TextLine,
			DbFields::Fields(field_id) => self.with_entry(id, |entry| {
				Self::with_field(entry, field_id, |field| field.kind.clone())
			}),
		}
	}

	// get the latest entry of a field
	pub fn get_last_by_field(&self, id: &usize, field: &DbFields) -> String {
		self.with_entry(id, |entry| match field {
			DbFields::Id => format!("{:?}", id),
			DbFields::Title => entry
				.map_or_else(|| String::from("Not found"), |entry| entry.title.clone()),
			DbFields::Fields(field_id) => {
				Self::with_field(entry, field_id, |field| {
					field
						.value
						.last()
						.map_or_else(String::new, |(_, value)| value.clone())
				})
			},
		})
	}

	// get the entry n of a field (look into the history of a field)
	pub fn get_n_by_field(
		&self,
		id: &usize,
		field: &DbFields,
		n: usize,
	) -> String {
		self.with_entry(id, |entry| match field {
			DbFields::Id => format!("{:?}", id),
			DbFields::Title => entry
				.map_or_else(|| String::from("Not found"), |entry| entry.title.clone()),
			DbFields::Fields(field_id) => {
				Self::with_field(entry, field_id, |field| {
					field
						.value
						.iter()
						.rev()
						.nth(n)
						.map_or_else(String::new, |(_, value)| value.clone())
				})
			},
		})
	}

	// get the entire history of a field
	pub fn get_history(
		&self,
		id: &usize,
		field: &DbFields,
	) -> Option<imbl::Vector<SecureField>> {
		match field {
			DbFields::Id | DbFields::Title => None,
			DbFields::Fields(field_id) => Some(self.with_entry(id, |entry| {
				Self::with_field(entry, field_id, |field| {
					field.value.iter().rev().cloned().collect()
				})
			})),
		}
	}

	// get the date and id of a field
	pub fn get_history_dates(
		&self,
		id: &usize,
		field: &DbFields,
	) -> Vec<(usize, u64)> {
		match field {
			DbFields::Id | DbFields::Title => vec![(0, 0)],
			DbFields::Fields(field_id) => self.with_entry(id, |entry| {
				Self::with_field(entry, field_id, |field| {
					field.value.iter().rev().map(|item| item.0).enumerate().collect()
				})
			}),
		}
	}

	// add a new entry
	pub fn add(&self, title: String) -> usize {
		let new_id =
			self.contents.read().iter().map(|item| item.id).max().unwrap_or(1) + 1;

		self.contents.write().push(DbEntry {
			id: new_id,
			title,
			fields: Vec::new(),
		});

		new_id
	}

	// add a new field to an entry
	pub fn add_field(
		&self,
		id: &usize,
		kind: DynFieldKind,
		title_value: String,
		field_value: String,
	) -> DbFields {
		let mut field = DbFields::Id;
		if let Some(item) =
			self.contents.write().iter_mut().find(|item| item.id == *id)
		{
			let field_id =
				item.fields.iter().map(|field| field.id).max().unwrap_or(0) + 1;
			let timestamp: u64 = SystemTime::now()
				.duration_since(UNIX_EPOCH)
				.unwrap_or(Duration::new(0, 0))
				.as_secs();
			item.fields.push(DynField {
				id: field_id,
				kind,
				title: title_value,
				visible: true,
				value: vec![(timestamp, field_value)],
			});
			field = DbFields::Fields(field_id);
		}

		field
	}

	// change the title of a field
	pub fn edit_field_title(&self, id: &usize, field: &DbFields, title: String) {
		if let DbFields::Fields(field_id) = field {
			if let Some(field) =
				self.contents.write().iter_mut().find(|item| item.id == *id).and_then(
					|item| item.fields.iter_mut().find(|field| field.id == *field_id),
				) {
				field.title.zeroize();
				field.title = title;
			}
		}
	}

	pub fn edit_field_visbility(
		&self,
		id: &usize,
		field: &DbFields,
		visible: bool,
	) -> Vec<DbFields> {
		if let DbFields::Fields(field_id) = field {
			if let Some(field) =
				self.contents.write().iter_mut().find(|item| item.id == *id).and_then(
					|item| item.fields.iter_mut().find(|field| field.id == *field_id),
				) {
				field.visible = visible;
			}
		}

		self.get_hidden_fields(id)
	}

	// edit a field
	pub fn edit_field(&self, id: usize, field: &DbFields, new_content: String) {
		if let Some(entry) =
			self.contents.write().iter_mut().find(|item| item.id == id)
		{
			let timestamp: u64 = SystemTime::now()
				.duration_since(UNIX_EPOCH)
				.unwrap_or(Duration::new(0, 0))
				.as_secs();

			match field {
				DbFields::Id => {
					panic!("Can't change the ID of an entry");
				},
				DbFields::Title => {
					entry.title.zeroize();
					entry.title = new_content;
				},
				DbFields::Fields(field_id) => {
					if let Some(field) =
						entry.fields.iter_mut().find(|field| field.id == *field_id)
					{
						field.value.push((timestamp, new_content));
					}
				},
			}
		}
	}

	// save order of dyn fields
	pub fn save_order(&self, id: &usize, order: Vec<usize>) {
		if let Some(item) =
			self.contents.write().iter_mut().find(|item| item.id == *id)
		{
			let visible_count =
				item.fields.iter().filter(|field| field.visible).count();
			let mut reordered: Vec<DynField> = order
				.iter()
				.filter_map(|field_id| {
					item
						.fields
						.iter()
						.find(|field| field.visible && field.id == *field_id)
						.cloned()
				})
				.collect();
			if reordered.len() != visible_count {
				// the order does not cover every visible field; keep what we have
				return;
			}
			reordered
				.extend(item.fields.iter().filter(|field| !field.visible).cloned());
			item.fields = reordered;
		}
	}
}

// case-insensitive `contains` whose lowercase copy of the haystack is wiped again
fn contains_ignore_case(haystack: &str, lowercase_needle: &str) -> bool {
	Zeroizing::new(haystack.to_lowercase()).contains(lowercase_needle)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		encryption::{encrypt_vault, password_hash},
		leakcheck,
		ui::app_view::SidebarList,
	};
	use floem::reactive::RwSignal;
	use zeroize::Zeroizing;

	// Unique strings so that no other test can free a block containing them.
	const MASTER: &str = "Leak-Check-Master-Pw-9c1e";
	const NEW_MASTER: &str = "Leak-Check-New-Pw-77aa";
	const SECRET: &str = "leakcheck_vault_secret_4b7d";
	const EDITED_SECRET: &str = "leakcheck_vault_secret_edit_51f0";
	const SALT: &str = "LeakCheckSalt";

	fn leak_check_db(db_path: &str) -> Db {
		let plaintext = Zeroizing::new(format!(
			"[[contents]]\nid = 1\ntitle = \"Leak\"\n\n\
			[[contents.fields]]\nid = 1\nkind = \"TextLineSecret\"\ntitle = \"Password\"\n\
			visible = true\nvalue = [[1702851212, \"{SECRET}\"]]\n"
		));
		let hash = password_hash(MASTER.as_bytes(), SALT).unwrap();
		let cypher = encrypt_vault(plaintext.as_bytes(), &hash).unwrap();
		let file: DbFile = toml::from_str(&format!(
			"[db]\nencrypted = true\nsalt = \"{SALT}\"\nsalt_letter_count = 32\n\
			cypher = \"{cypher}\"\n"
		))
		.unwrap();
		let db: Db = file.into();
		db.set_db_path(String::from(db_path));
		db
	}

	/// GHSA-vjvh-m6q3-jrgc: nothing the database handed out or kept may be
	/// freed while it still contains a secret, otherwise it stays readable in
	/// a core dump after the vault was locked.
	#[test]
	fn lock_leaves_no_secrets_in_freed_memory() {
		let db_path = std::env::temp_dir()
			.join(format!("vault-leakcheck-{}.toml", std::process::id()));
		let db = leak_check_db(db_path.to_str().unwrap());
		shared_context::provide::<SidebarList>(RwSignal::new(imbl::Vector::new()));

		let watched = [
			("master password", leakcheck::watch(MASTER.as_bytes())),
			("new master password", leakcheck::watch(NEW_MASTER.as_bytes())),
			("vault secret", leakcheck::watch(SECRET.as_bytes())),
			("edited vault secret", leakcheck::watch(EDITED_SECRET.as_bytes())),
		];

		// what the app does between unlocking and locking
		db.decrypt_database(MASTER).unwrap();
		let field = DbFields::Fields(1);
		assert_eq!(db.get_sidebar_list().len(), 1);
		assert_eq!(*Zeroizing::new(db.get_last_by_field(&1, &field)), SECRET);
		assert_eq!(*Zeroizing::new(db.get_n_by_field(&1, &field, 0)), SECRET);
		assert_eq!(db.get_name_of_field(&1, &field), "Password");
		assert_eq!(db.get_fields(&1).len(), 1);
		assert_eq!(db.get_history_dates(&1, &field).len(), 1);
		let mut history = db.get_history(&1, &field).unwrap();
		assert_eq!(history.len(), 1);
		for (_, value) in history.iter_mut() {
			value.zeroize();
		}
		assert_eq!(db.search("leak").len(), 1);
		assert_eq!(db.search("4B7D").len(), 1);
		db.edit_field(1, &field, String::from(EDITED_SECRET));
		db.save().unwrap();
		db.change_password(MASTER, NEW_MASTER).unwrap();
		db.lock();
		drop(db);
		let _ = std::fs::remove_file(&db_path);

		leakcheck::report_summary();
		let found = watched
			.iter()
			.filter(|(_, index)| leakcheck::hits_for(*index) > 0)
			.map(|(name, index)| {
				format!("{name} ({} blocks)", leakcheck::hits_for(*index))
			})
			.collect::<Vec<_>>();
		assert!(
			found.is_empty(),
			"secrets found in freed memory: {}",
			found.join(", ")
		);
		assert_eq!(leakcheck::hits(), 0);
	}
}
