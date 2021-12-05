use crate::basic::structs::Package;
use std::path::Path;
use rusqlite::{Connection, Error};

pub fn remove(pkg: Package) -> Result<(), Error> {
    let dbpath = Path::new("/var/libdotpm/db.sqlite");
    let mut conn = Connection::open(dbpath)?;
    let tx = conn.transaction()?;
    tx.execute("DELETE FROM packages WHERE name = ?", &[&pkg.name])?;
    tx.commit()?;
    Ok(())
}