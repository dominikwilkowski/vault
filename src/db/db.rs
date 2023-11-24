fn get_db() -> Vec<(usize, &'static str, &'static str)> {
	// the db with all the things
	vec![
		(1, "password 1", "Body of this item 1"),
		(2, "password 2", "Body of this item 2"),
		(3, "password 3", "Body of this item 3"),
		(4, "password 4", "Body of this item 4"),
		(7, "password 7 with really long text and stuff", "Body of this item 5\ntest new line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nnew line\nlast line"),
		(8, "password 8", "Body of this item 8"),
		(9, "password 9", "Body of this item 9"),
		(10, "password 10", "Body of this item 10"),
		(11, "password 11", "Body of this item 11"),
		(12, "password 12 with also some", "Body of this item 12"),
		(13, "password 13", "Body of this item 13"),
		(14, "password 14", "Body of this item 14"),
		(15, "password 15", "Body of this item 15"),
		(16, "password 16", "Body of this item 16"),
		(17, "password 17", "Body of this item 17"),
		(18, "password 18", "Body of this item 18"),
		(19, "password 19", "Body of this item 19"),
		(20, "password 20", "Body of this item 20"),
		(21, "password 21", "Body of this item 21"),
		(22, "password 22", "Body of this item 22"),
		(23, "password 23", "Body of this item 23"),
		(34, "password 34", "Body of this item 34"),
		(35, "password 35", "Body of this item 35"),
		(36, "password 36", "Body of this item 36"),
		(37, "password 37", "Body of this item 37"),
		(38, "password 38", "Body of this item 38"),
		(39, "last password", "Body of last item"),
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
