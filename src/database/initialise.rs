use rusqlite::{Connection, Result};
use std::path::Path;

pub static DATABASE_PATH: &str = "/var/libdlta/db.sqlite";

pub fn initialise(dbpath: &Path, os: bool) -> Result<()> {
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
                RECCOMENDATIONS TEXT,
                ARCH TEXT
            )",
            [],
        )?;
        if os {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS SETTINGS (
                    KEY TEXT PRIMARY KEY,                
                )",
                [],
            )?;
        }
    }
    Ok(())
}
