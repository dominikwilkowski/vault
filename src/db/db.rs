fn db() -> Vec<(usize, &'static str, &'static str)> {
	// the db with all the things
	vec![
		(1, "password 1", "Body of this item 1"),
		(2, "password 2", "Body of this item 2"),
		(3, "password 3", "Body of this item 3"),
		(4, "password 4", "Body of this item 4"),
		(5, "password 5 with really long text and stuff", "Body of this item 5"),
		(6, "password 6", "Body of this item 6"),
		(7, "password 7", "Body of this item 7"),
		(8, "password 8", "Body of this item 8"),
		(9, "password 9", "Body of this item 9"),
		(10, "password 10", "Body of this item 10"),
		(11, "password 11", "Body of this item 11"),
	]
}

pub fn get_list() -> im::Vector<(usize, &'static str)> {
	db().iter().map(|item| (item.0, item.1)).collect()
}

pub fn get_by_id(id: usize) -> (usize, &'static str, &'static str) {
	let entry = db().into_iter().find(|item| item.0 == id).unwrap_or((id, "Not found", "Not found"));

	(id, entry.1, entry.2)
}
