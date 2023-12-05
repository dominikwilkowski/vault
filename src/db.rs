use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DbEntry {
	pub id: usize,
	pub title: String,
	pub url: String,
	pub username: Vec<String>,
	pub password: Vec<String>,
	pub notes: String,
}

#[derive(Debug)]
pub struct NewDbEntry {
	pub title: String,
	pub url: String,
	pub username: Vec<String>,
	pub password: Vec<String>,
	pub notes: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DbEntryNonSecure {
	pub id: usize,
	pub title: String,
	pub url: String,
	pub notes: String,
}

#[derive(Debug, Copy, Clone)]
pub enum DbFields {
	Id,
	Title,
	Url,
	Username,
	Password,
	Notes,
}

impl std::fmt::Display for DbFields {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			DbFields::Id => write!(f, "Id"),
			DbFields::Title => write!(f, "Title"),
			DbFields::Url => write!(f, "Url"),
			DbFields::Username => write!(f, "Username"),
			DbFields::Password => write!(f, "Password"),
			DbFields::Notes => write!(f, "Notes"),
		}
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Db {
	pub contents: Vec<DbEntry>,
	pub timeout: u16,
}

impl Default for Db {
	fn default() -> Self {
		Db {
			timeout: 60,
			contents: get(),
		}
	}
}

fn to_tuple(item: &DbEntry, idx: usize) -> (usize, &'static str, usize) {
	(item.id, Box::leak(item.title.clone().into_boxed_str()), idx)
}

impl Db {
	pub fn get_list(&self) -> im::Vector<(usize, &'static str, usize)> {
		self
			.contents
			.iter()
			.enumerate()
			.map(|(idx, item)| to_tuple(item, idx))
			.rev()
			.collect()
	}

	fn get_by_id_secure(&self, id: &usize) -> DbEntry {
		// TODO: don't consume the data... find a better way to find
		let db = self.contents.clone();
		db.into_iter().find(|item| item.id == *id).unwrap_or(DbEntry {
			id: *id,
			title: String::from("Not found"),
			url: String::from(""),
			username: vec![String::from("")],
			password: vec![String::from("!1")],
			notes: String::from(""),
		})
	}

	pub fn get_by_id(&self, id: &usize) -> DbEntryNonSecure {
		let entry = self.get_by_id_secure(id);

		DbEntryNonSecure {
			id: *id,
			title: entry.title,
			url: entry.url,
			notes: entry.notes,
		}
	}

	pub fn add(&mut self, data: NewDbEntry) {
		let last_id = self
			.contents
			.last()
			.unwrap_or(&DbEntry {
				id: 1,
				title: String::from(""),
				url: String::from(""),
				username: vec![String::from("")],
				password: vec![String::from("")],
				notes: String::from(""),
			})
			.id;
		self.contents.push(DbEntry {
			id: last_id + 1,
			title: data.title,
			url: data.url,
			username: vec![String::from("")],
			password: vec![String::from("")],
			notes: data.notes,
		});
	}

	pub fn get_db_by_field(&self, id: &usize, field: &DbFields) -> String {
		let entry = self.get_by_id_secure(id);

		match field {
			DbFields::Id => format!("{:?}", entry.id),
			DbFields::Title => entry.title,
			DbFields::Url => entry.url,
			DbFields::Username => entry.username.last().unwrap().clone(),
			DbFields::Password => entry.password.last().unwrap().clone(),
			DbFields::Notes => entry.notes,
		}
	}
}

pub fn get() -> Vec<DbEntry> {
	vec![DbEntry {
		id: 1,
		title: String::from("Bank"),
		url: String::from("https://bankofaustralia.com.au"),
		username: vec![String::from("Dom")],
		password: vec![String::from("totally_secure_password!1")],
		notes: String::from(""),
	}]
}
