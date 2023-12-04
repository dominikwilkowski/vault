use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DbEntry {
	pub id: usize,
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
	pub contents: Vec<DbEntryNonSecure>,
	pub timeout: u16,
}

impl Default for Db {
	fn default() -> Self {
		Db {
			timeout: 60,
			contents: get_secure(),
		}
	}
}

fn to_tuple(item: &DbEntry, idx: usize) -> (usize, &'static str, usize) {
	(item.id, Box::leak(item.title.clone().into_boxed_str()), idx)
}

impl Db {
	pub fn get_list(&self) -> im::Vector<(usize, &'static str, usize)> {
		get().iter().enumerate().map(|(idx, item)| to_tuple(item, idx)).collect()
	}

	fn get_by_id_secure(&self, id: &usize) -> DbEntry {
		get().into_iter().find(|item| item.id == *id).unwrap_or(DbEntry {
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
	vec![
		DbEntry {
			id: 1,
			title: String::from("Bank"),
			url: String::from("https://bankofaustralia.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from(""),
		},
		DbEntry {
			id: 2,
			title: String::from("Google Account"),
			url: String::from("https://Google Account.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("Google Account"),
		},
		DbEntry {
			id: 5,
			title: String::from("Facebook Login"),
			url: String::from("https://Facebook Login.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("Facebook Login"),
		},
		DbEntry {
			id: 6,
			title: String::from("Amazon Password"),
			url: String::from("https://Amazon Password.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("Amazon Password"),
		},
		DbEntry {
			id: 7,
			title: String::from("Twitter Access"),
			url: String::from("https://Twitter Access.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("Twitter Access"),
		},
		DbEntry {
			id: 8,
			title: String::from("LinkedIn Credentials"),
			url: String::from("https://LinkedIn Credentials.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("LinkedIn Credentials"),
		},
		DbEntry {
			id: 10,
			title: String::from("Microsoft Account"),
			url: String::from("https://Microsoft Account.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("Microsoft Account"),
		},
		DbEntry {
			id: 11,
			title: String::from("Instagram Secure Key"),
			url: String::from("https://Instagram Secure Key.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("Instagram Secure Key"),
		},
		DbEntry {
			id: 12,
			title: String::from("Dropbox Passcode"),
			url: String::from("https://Dropbox Passcode.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("Dropbox Passcode"),
		},
		DbEntry {
			id: 13,
			title: String::from("GitHub Authentication"),
			url: String::from("https://GitHub Authentication.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("GitHub Authentication"),
		},
		DbEntry {
			id: 14,
			title: String::from("Netflix Login"),
			url: String::from("https://Netflix Login.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("Netflix Login"),
		},
		DbEntry {
			id: 15,
			title: String::from("Apple ID Password"),
			url: String::from("https://Apple ID Password.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("Apple ID Password"),
		},
		DbEntry {
			id: 16,
			title: String::from("Spotify Access Code"),
			url: String::from("https://Spotify Access Code.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("Spotify Access Code"),
		},
		DbEntry {
			id: 17,
			title: String::from("PayPal Secure Key"),
			url: String::from("https://PayPal Secure Key.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("PayPal Secure Key"),
		},
		DbEntry {
			id: 18,
			title: String::from("Reddit Credentials"),
			url: String::from("https://Reddit Credentials.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("Reddit Credentials"),
		},
		DbEntry {
			id: 20,
			title: String::from("Airbnb Login"),
			url: String::from("https://Airbnb Login.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("Airbnb Login"),
		},
		DbEntry {
			id: 21,
			title: String::from("Office 365 Password"),
			url: String::from("https://Office 365 Password.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("Office 365 Password"),
		},
		DbEntry {
			id: 22,
			title: String::from("Evernote Secure Code"),
			url: String::from("https://Evernote Secure Code.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("Evernote Secure Code"),
		},
		DbEntry {
			id: 23,
			title: String::from("Tumblr Access"),
			url: String::from("https://Tumblr Access.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("Tumblr Access"),
		},
		DbEntry {
			id: 24,
			title: String::from("Pinterest Passphrase"),
			url: String::from("https://Pinterest Passphrase.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("Pinterest Passphrase"),
		},
		DbEntry {
			id: 25,
			title: String::from("Skype Authentication"),
			url: String::from("https://Skype Authentication.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("Skype Authentication"),
		},
		DbEntry {
			id: 26,
			title: String::from("WhatsApp Secure Key"),
			url: String::from("https://WhatsApp Secure Key.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("WhatsApp Secure Key"),
		},
		DbEntry {
			id: 27,
			title: String::from("Snapchat Password"),
			url: String::from("https://Snapchat Password.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("Snapchat Password"),
		},
		DbEntry {
			id: 28,
			title: String::from("Zoom Login"),
			url: String::from("https://Zoom Login.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("Zoom Login"),
		},
		DbEntry {
			id: 29,
			title: String::from("Slack Access Code"),
			url: String::from("https://Slack Access Code.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("Slack Access Code"),
		},
		DbEntry {
			id: 30,
			title: String::from("Uber Secure Key"),
			url: String::from("https://Uber Secure Key.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("Uber Secure Key"),
		},
		DbEntry {
			id: 32,
			title: String::from("LastPass Master Password"),
			url: String::from("https://LastPass Master Password.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("LastPass Master Password"),
		},
		DbEntry {
			id: 33,
			title: String::from("1Password Master Key"),
			url: String::from("https://1Password Master Key.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("1Password Master Key"),
		},
		DbEntry {
			id: 34,
			title: String::from("Dashlane Access"),
			url: String::from("https://Dashlane Access.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("Dashlane Access"),
		},
		DbEntry {
			id: 35,
			title: String::from("Keeper Security Code"),
			url: String::from("https://Keeper Security Code.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("Keeper Security Code"),
		},
		DbEntry {
			id: 36,
			title: String::from("Bitwarden Password"),
			url: String::from("https://Bitwarden Password.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("Bitwarden Password"),
		},
		DbEntry {
			id: 37,
			title: String::from("RoboForm Master Key"),
			url: String::from("https://RoboForm Master Key.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("RoboForm Master Key"),
		},
		DbEntry {
			id: 38,
			title: String::from("Password Manager Master Password"),
			url: String::from("https://Password Manager Master Password.com.au"),
			username: vec![String::from("Dom")],
			password: vec![String::from("totally_secure_password!1")],
			notes: String::from("Password Manager Master Password"),
		},
	]
}

fn get_secure() -> Vec<DbEntryNonSecure> {
	get()
		.iter()
		.map(|entry| DbEntryNonSecure {
			id: entry.id,
			title: entry.title.clone(),
			url: entry.url.clone(),
			notes: entry.notes.clone(),
		})
		.collect()
}

// below methods will be removed

fn get_db() -> Vec<(
	usize,
	&'static str,
	&'static str,
	&'static str,
	&'static str,
	&'static str,
)> {
	// the db with all the things
	vec![
		(1, "Password 1", "https://", "user 1", "pass1", "Notes 1"),
		(2, "Password 2", "https://", "user 2", "pass2", "Notes 2"),
		(3, "Password 3", "https://", "user 3", "pass3", "Notes 3"),
		(4, "Password 4", "https://", "user 4", "pass4", "Notes 4"),
		(
			7,
			"Password 7 with really long text and stuff",
			"https://",
			"user 7",
			"pass7",
			"Notes 7",
		),
		(8, "Password 8", "https://", "user 8", "pass8", "Notes 8"),
		(9, "Password 9", "https://", "user 9", "pass9", "Notes 9"),
		(10, "Password 10", "https://", "user 10", "pass10", "Notes 10"),
		(11, "Password 11", "https://", "user 11", "pass11", "Notes 11"),
		(
			12,
			"Password 12 with also some",
			"https://",
			"user 12",
			"pass12",
			"Notes 12",
		),
		(13, "Password 13", "https://", "user 13", "pass13", "Notes 13"),
		(14, "Password 14", "https://", "user 14", "pass14", "Notes 14"),
		(15, "Password 15", "https://", "user 15", "pass15", "Notes 15"),
		(16, "Password 16", "https://", "user 16", "pass16", "Notes 16"),
		(17, "Password 17", "https://", "user 17", "pass17", "Notes 17"),
		(18, "Password 18", "https://", "user 18", "pass18", "Notes 18"),
		(19, "Password 19", "https://", "user 19", "pass19", "Notes 19"),
		(20, "Password 20", "https://", "user 20", "pass20", "Notes 20"),
		(21, "Password 21", "https://", "user 21", "pass21", "Notes 21"),
		(22, "Password 22", "https://", "user 22", "pass22", "Notes 22"),
		(23, "Password 23", "https://", "user 23", "pass23", "Notes 23"),
		(34, "Password 34", "https://", "user 34", "pass34", "Notes 34"),
		(35, "Password 35", "https://", "user 35", "pass35", "Notes 35"),
		(36, "Password 36", "https://", "user 36", "pass36", "Notes 36"),
		(37, "Password 37", "https://", "user 37", "pass37", "Notes 37"),
		(38, "Password 38", "https://", "user 38", "pass38", "Notes 38"),
		(39, "Last password", "https://", "user 39", "pass39", "Notes 39"),
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
	));

	(id, entry.1, entry.2, entry.5)
}

pub fn get_db_by_field(id: &usize, field: &DbFields) -> String {
	let entry = get_db().into_iter().find(|item| item.0 == *id).unwrap_or((
		*id,
		"##Not found##",
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
