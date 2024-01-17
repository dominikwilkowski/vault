use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

type SecureField = (u64, String);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DynamicField {
	id: usize,
	title: String,
	visible: bool,
	value: Vec<SecureField>,
}

impl Default for DynamicField {
	fn default() -> Self {
		Self {
			id: 0,
			title: String::from(""),
			visible: true,
			value: vec![(0, String::from(""))],
		}
	}
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DbEntry {
	pub id: usize,
	pub title: String,
	pub url: String,
	pub username: Vec<SecureField>,
	pub password: Vec<SecureField>,
	pub fields: Vec<DynamicField>,
}

#[derive(Debug)]
pub struct NewDbEntry {
	pub title: String,
	pub url: String,
	pub username: Vec<SecureField>,
	pub password: Vec<SecureField>,
	pub fields: Vec<DynamicField>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DbEntryNonSecure {
	pub id: usize,
	pub title: String,
	pub url: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DbFields {
	Id,
	Title,
	Url,
	Username,
	Password,
	Fields(usize),
}

impl std::fmt::Display for DbFields {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			DbFields::Id => write!(f, "Id"),
			DbFields::Title => write!(f, "Title"),
			DbFields::Url => write!(f, "URL"),
			DbFields::Username => write!(f, "Username"),
			DbFields::Password => write!(f, "Password"),
			DbFields::Fields(idx) => write!(f, "Fields-{}", idx),
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
			contents: vec![DbEntry {
				id: 1,
				title: String::from("Bank"),
				url: String::from("https://bankofaustralia.com.au"),
				username: vec![(1702851212, String::from("Dom"))],
				password: vec![(1702851212, String::from("totally_secure_password!1"))],
				fields: vec![DynamicField {
					id: 0,
					title: String::from("Notes"),
					visible: true,
					value: vec![(1702851212, String::from("These are my bank deets"))],
				}],
			}],
		}
	}
}

fn to_tuple(item: &DbEntry, idx: usize) -> (usize, &'static str, usize) {
	(item.id, Box::leak(item.title.clone().into_boxed_str()), idx)
}

impl Db {
	// get the list of all entries for sidebar view
	pub fn get_list(&self) -> im::Vector<(usize, &'static str, usize)> {
		self
			.contents
			.iter()
			.enumerate()
			.map(|(idx, item)| to_tuple(item, idx))
			.rev()
			.collect()
	}

	// get content of entry
	fn get_by_id_secure(&self, id: &usize) -> DbEntry {
		if let Some(found_entry) = self.contents.iter().find(|item| item.id == *id)
		{
			found_entry.clone()
		} else {
			DbEntry {
				id: *id,
				title: String::from("Not found"),
				url: String::from(""),
				username: vec![(0, String::from(""))],
				password: vec![(0, String::from(""))],
				fields: vec![DynamicField::default()],
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

	// get non secure content of entry
	pub fn get_by_id(&self, id: &usize) -> DbEntryNonSecure {
		let entry = self.get_by_id_secure(id);

		DbEntryNonSecure {
			id: *id,
			title: entry.title,
			url: entry.url,
		}
	}

	// get content of dynamic field by id
	fn get_field_by_id(&self, entry: &DbEntry, field_id: &usize) -> DynamicField {
		entry
			.fields
			.clone()
			.into_iter()
			.find(|field| field.id == *field_id)
			.unwrap_or(DynamicField {
				id: *field_id,
				title: String::from(""),
				visible: true,
				value: vec![(0, String::from(""))],
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
			DbFields::Url => entry.url,
			DbFields::Username => entry.username.last().unwrap().1.clone(),
			DbFields::Password => entry.password.last().unwrap().1.clone(),
			DbFields::Fields(field_id) => {
				self.get_field_by_id(&entry, field_id).value.last().unwrap().1.clone()
			}
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
			DbFields::Url => entry.url,
			DbFields::Username => {
				entry.username.into_iter().rev().collect::<Vec<SecureField>>()[n]
					.1
					.clone()
			}
			DbFields::Password => {
				entry.password.into_iter().rev().collect::<Vec<SecureField>>()[n]
					.1
					.clone()
			}
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
			DbFields::Url => None,
			DbFields::Username => Some(
				entry.username.into_iter().rev().collect::<im::Vector<SecureField>>(),
			),
			DbFields::Password => Some(
				entry.password.into_iter().rev().collect::<im::Vector<SecureField>>(),
			),
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
			DbFields::Url => vec![(0, 0)],
			DbFields::Username => {
				entry.username.iter().map(|item| item.0).enumerate().collect()
			}
			DbFields::Password => {
				entry.password.iter().map(|item| item.0).enumerate().collect()
			}
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
	pub fn add(&mut self, title: String) -> usize {
		let timestamp: u64 = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap_or(Duration::new(0, 0))
			.as_secs();

		let new_id = self
			.contents
			.last()
			.unwrap_or(&DbEntry {
				id: 1,
				title: String::from(""),
				url: String::from(""),
				username: vec![(0, String::from(""))],
				password: vec![(0, String::from(""))],
				fields: vec![DynamicField::default()],
			})
			.id + 1;

		self.contents.push(DbEntry {
			id: new_id,
			title,
			url: String::from(""),
			username: vec![(timestamp, String::from(""))],
			password: vec![(timestamp, String::from(""))],
			fields: vec![DynamicField {
				id: 0,
				title: String::from("Note"),
				visible: true,
				value: vec![(0, String::from(""))],
			}],
		});

		new_id
	}

	// add a new field to an entry
	pub fn add_dyn_field(
		&mut self,
		id: &usize,
		title_value: String,
		field_value: String,
	) -> Vec<DbFields> {
		self.contents.iter_mut().for_each(|item| {
			if item.id == *id {
				let id = item.fields.last().unwrap_or(&DynamicField::default()).id + 1;
				item.fields.push(DynamicField {
					id,
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
		&mut self,
		id: &usize,
		field: &DbFields,
		title: String,
	) {
		self.contents.iter_mut().for_each(|item| {
			if item.id == *id {
				if let DbFields::Fields(field_id) = field {
					item
						.fields
						.iter_mut()
						.find(|field| field.id == *field_id)
						.unwrap_or(&mut DynamicField {
							id: *field_id,
							title: String::from(""),
							visible: true,
							value: vec![(0, String::from(""))],
						})
						.title = title.clone();
				}
			}
		});
	}

	pub fn edit_dyn_field_visbility(
		&mut self,
		id: &usize,
		field: &DbFields,
		visible: bool,
	) -> Vec<DbFields> {
		self.contents.iter_mut().for_each(|item| {
			if item.id == *id {
				if let DbFields::Fields(field_id) = field {
					item
						.fields
						.iter_mut()
						.find(|field| field.id == *field_id)
						.unwrap_or(&mut DynamicField {
							id: *field_id,
							title: String::from(""),
							visible,
							value: vec![(0, String::from(""))],
						})
						.visible = visible;
				}
			}
		});

		self.get_hidden_dyn_fields(id)
	}

	// edit a field
	pub fn edit_field(
		&mut self,
		id: usize,
		field: &DbFields,
		new_content: String,
	) {
		let mut index: usize = 0;
		self.contents.iter().enumerate().find(|(idx, item)| {
			if item.id == id {
				index = *idx;
				true
			} else {
				false
			}
		});

		if let Some(entry) = self.contents.get_mut(index) {
			let timestamp: u64 = SystemTime::now()
				.duration_since(UNIX_EPOCH)
				.unwrap_or(Duration::new(0, 0))
				.as_secs();

			match field {
				DbFields::Id => {
					panic!("Can't change the ID of an entry");
				}
				DbFields::Title => {
					entry.title = new_content;
				}
				DbFields::Url => {
					entry.url = new_content;
				}
				DbFields::Username => {
					entry.username.push((timestamp, new_content));
				}
				DbFields::Password => {
					entry.password.push((timestamp, new_content));
				}
				DbFields::Fields(field_id) => {
					entry
						.fields
						.iter_mut()
						.find(|field| field.id == *field_id)
						.unwrap_or(&mut DynamicField {
							id: *field_id,
							title: String::from(""),
							visible: true,
							value: vec![(0, String::from(""))],
						})
						.value
						.push((timestamp, new_content));
				}
			}
		}
	}
}
