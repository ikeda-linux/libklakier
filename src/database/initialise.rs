use rusqlite::{Connection, Result};
use std::path::Path;

pub fn initialise() -> Result<()> {
    let dbpath = Path::new("/var/libdotpm/db.sqlite");
    if !dbpath.exists() {
        let conn = Connection::open(dbpath)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS PACKAGES (
                NAME TEXT PRIMARY KEY,
                VERSION TEXT NOT NULL,
                DESCRIPTION TEXT,
                AUTHORS TEXT NOT NULL,
                LICENSE TEXT,
                TRACKED_FILES TEXT NOT NULL,
                DEPENDENCIES TEXT,
                PROVIDES TEXT,
                CONFLICTS TEXT,
                ARCH TEXT
            )",
            [],
        )?;
    }
    Ok(())
}
