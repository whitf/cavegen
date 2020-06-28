use rusqlite::{Connection, NO_PARAMS, params};
use std::path::Path;
use uuid::Uuid;

pub fn dbinit(id: Uuid) -> bool {

	println!("running db init (id = {})", id);

	let meta_conn = Connection::open("./data/cavegen_meta.db")
		.expect("[dbinit] Failed to open meta db file.");
	meta_conn.execute(
		"CREATE TABLE IF NOT EXISTS game (
			id TEXT NOT NULL PRIMARY KEY
		);",
		NO_PARAMS,
	).expect("[dbinit] Failed create_if for table = game, db = cavegen_meta.db");

	meta_conn.execute(
		"INSERT OR IGNORE INTO game(id) VALUES(?1);",
		params![id.to_string()],
	).expect("[dbinit] Failed to insert id into meta db.");

	let dbfile: &Path = Path::new("./data");
	let mut dbfile = dbfile.join(id.to_string());
	dbfile.set_extension("game.db");

	let conn = Connection::open(dbfile)
		.expect("[dbinit] Failed to open dbfile.");

	conn.execute(
		"CREATE TABLE IF NOT EXISTS level (
			id INT DEFAULT 0 PRIMARY KEY,
			map_size_x INT DEFAULT 0,
			map_size_y INT DEFAULT 0,
			grid BLOB,
			portal BLOB
		);",
		NO_PARAMS,
	).expect("[dbinit] Failed create_if for table = level.");

	true
}
