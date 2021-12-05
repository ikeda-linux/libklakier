use crate::basic::structs::Package;
use rusqlite::{Connection, Error};
use std::path::Path;

pub fn remove(pkg: Package) -> Result<(), Error> {
    let dbpath = Path::new("/var/libdotpm/db.sqlite");
    let mut conn = Connection::open(dbpath)?;
    let tx = conn.transaction()?;
    tx.execute("DELETE FROM packages WHERE name = ?", &[&pkg.name])?;
    tx.commit()?;
    Ok(())
}
