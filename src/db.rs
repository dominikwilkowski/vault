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

fn get_db() -> Vec<(usize, &'static str, &'static str)> {
	// the db with all the things
	vec![
		(1, "Password 1", "Body of this item 1"),
		(2, "Password 2", "Body of this item 2"),
		(3, "Password 3", "Body of this item 3"),
		(4, "Password 4", "Body of this item 4"),
		(7, "Password 7 with really long text and stuff", "Body of this item 5"),
		(8, "Password 8", "Body of this item 8"),
		(9, "Password 9", "Body of this item 9"),
		(10, "Password 10", "Body of this item 10"),
		(11, "Password 11", "Body of this item 11"),
		(12, "Password 12 with also some", "Body of this item 12"),
		(13, "Password 13", "Body of this item 13"),
		(14, "Password 14", "Body of this item 14"),
		(15, "Password 15", "Body of this item 15"),
		(16, "Password 16", "Body of this item 16"),
		(17, "Password 17", "Body of this item 17"),
		(18, "Password 18", "Body of this item 18"),
		(19, "Password 19", "Body of this item 19"),
		(20, "Password 20", "Body of this item 20"),
		(21, "Password 21", "Body of this item 21"),
		(22, "Password 22", "Body of this item 22"),
		(23, "Password 23", "Body of this item 23"),
		(34, "Password 34", "Body of this item 34"),
		(35, "Password 35", "Body of this item 35"),
		(36, "Password 36", "Body of this item 36"),
		(37, "Password 37", "Body of this item 37"),
		(38, "Password 38", "Body of this item 38"),
		(39, "Last password", "Body of last item"),
	]
}

pub fn get_db_list() -> im::Vector<(usize, &'static str, usize)> {
	get_db().iter().enumerate().map(|(idx, item)| (item.0, item.1, idx)).collect()
}

pub fn get_db_by_id(id: usize) -> (usize, &'static str, &'static str) {
	let entry = get_db().into_iter().find(|item| item.0 == id).unwrap_or((
		id,
		"##Not found##",
		"Not found",
	));

	(id, entry.1, entry.2)
}
