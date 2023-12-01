use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Db {
	pub contents: Vec<DbEntry>,
	pub timeout: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DbEntry {
	pub id: u64,
	pub title: String,
	pub url: String,
	pub username: Vec<String>,
	pub password: Vec<String>,
	pub notes: String,
}

// below methods will be implemented on the Db struct and passed into the application as a global

fn get_db() -> Vec<(
	usize,
	&'static str,
	&'static str,
	&'static str,
	&'static str,
	&'static str,
	&'static str,
)> {
	// the db with all the things
	vec![
		(
			1,
			"Password 1",
			"Body of this item 1",
			"https://",
			"user 1",
			"pass1",
			"Notes 1",
		),
		(
			2,
			"Password 2",
			"Body of this item 2",
			"https://",
			"user 2",
			"pass2",
			"Notes 2",
		),
		(
			3,
			"Password 3",
			"Body of this item 3",
			"https://",
			"user 3",
			"pass3",
			"Notes 3",
		),
		(
			4,
			"Password 4",
			"Body of this item 4",
			"https://",
			"user 4",
			"pass4",
			"Notes 4",
		),
		(
			7,
			"Password 7 with really long text and stuff",
			"Body of this item 5",
			"https://",
			"user 7",
			"pass7",
			"Notes 7",
		),
		(
			8,
			"Password 8",
			"Body of this item 8",
			"https://",
			"user 8",
			"pass8",
			"Notes 8",
		),
		(
			9,
			"Password 9",
			"Body of this item 9",
			"https://",
			"user 9",
			"pass9",
			"Notes 9",
		),
		(
			10,
			"Password 10",
			"Body of this item 10",
			"https://",
			"user 10",
			"pass10",
			"Notes 10",
		),
		(
			11,
			"Password 11",
			"Body of this item 11",
			"https://",
			"user 11",
			"pass11",
			"Notes 11",
		),
		(
			12,
			"Password 12 with also some",
			"Body of this item 12",
			"https://",
			"user 12",
			"pass12",
			"Notes 12",
		),
		(
			13,
			"Password 13",
			"Body of this item 13",
			"https://",
			"user 13",
			"pass13",
			"Notes 13",
		),
		(
			14,
			"Password 14",
			"Body of this item 14",
			"https://",
			"user 14",
			"pass14",
			"Notes 14",
		),
		(
			15,
			"Password 15",
			"Body of this item 15",
			"https://",
			"user 15",
			"pass15",
			"Notes 15",
		),
		(
			16,
			"Password 16",
			"Body of this item 16",
			"https://",
			"user 16",
			"pass16",
			"Notes 16",
		),
		(
			17,
			"Password 17",
			"Body of this item 17",
			"https://",
			"user 17",
			"pass17",
			"Notes 17",
		),
		(
			18,
			"Password 18",
			"Body of this item 18",
			"https://",
			"user 18",
			"pass18",
			"Notes 18",
		),
		(
			19,
			"Password 19",
			"Body of this item 19",
			"https://",
			"user 19",
			"pass19",
			"Notes 19",
		),
		(
			20,
			"Password 20",
			"Body of this item 20",
			"https://",
			"user 20",
			"pass20",
			"Notes 20",
		),
		(
			21,
			"Password 21",
			"Body of this item 21",
			"https://",
			"user 21",
			"pass21",
			"Notes 21",
		),
		(
			22,
			"Password 22",
			"Body of this item 22",
			"https://",
			"user 22",
			"pass22",
			"Notes 22",
		),
		(
			23,
			"Password 23",
			"Body of this item 23",
			"https://",
			"user 23",
			"pass23",
			"Notes 23",
		),
		(
			34,
			"Password 34",
			"Body of this item 34",
			"https://",
			"user 34",
			"pass34",
			"Notes 34",
		),
		(
			35,
			"Password 35",
			"Body of this item 35",
			"https://",
			"user 35",
			"pass35",
			"Notes 35",
		),
		(
			36,
			"Password 36",
			"Body of this item 36",
			"https://",
			"user 36",
			"pass36",
			"Notes 36",
		),
		(
			37,
			"Password 37",
			"Body of this item 37",
			"https://",
			"user 37",
			"pass37",
			"Notes 37",
		),
		(
			38,
			"Password 38",
			"Body of this item 38",
			"https://",
			"user 38",
			"pass38",
			"Notes 38",
		),
		(
			39,
			"Last password",
			"Body of last item",
			"https://",
			"user 39",
			"pass39",
			"Notes 39",
		),
	]
}

pub fn get_db_list() -> im::Vector<(usize, &'static str, usize)> {
	get_db().iter().enumerate().map(|(idx, item)| (item.0, item.1, idx)).collect()
}

pub fn get_db_by_id(
	id: usize,
) -> (usize, &'static str, &'static str, &'static str) {
	let entry = get_db().into_iter().find(|item| item.0 == id).unwrap_or((
		id,
		"##Not found##",
		"Not found",
		"Not found",
		"Not found",
		"Not found",
		"Not found",
	));

	(id, entry.1, entry.2, entry.5)
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

pub fn get_db_by_field(id: &usize, field: &DbFields) -> String {
	let entry = get_db().into_iter().find(|item| item.0 == *id).unwrap_or((
		*id,
		"##Not found##",
		"Not found",
		"Not found",
		"Not found",
		"Not found",
		"Not found",
	));

	match field {
		DbFields::Id => format!("{:?}", entry.0),
		DbFields::Title => String::from(entry.1),
		DbFields::Url => String::from(entry.2),
		DbFields::Username => String::from(entry.3),
		DbFields::Password => String::from(entry.4),
		DbFields::Notes => String::from(entry.5),
	}
}
