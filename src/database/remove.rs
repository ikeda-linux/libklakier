use crate::base::structs::Package;
use rusqlite::{Connection, Error};
use std::path::Path;

pub fn remove(pkg: Package, dbpath: &Path) -> Result<(), Error> {
    let mut conn = Connection::open(dbpath)?;
    let tx = conn.transaction()?;
    tx.execute("DELETE FROM packages WHERE name = ?", &[&pkg.name])?;
    tx.commit()?;
    Ok(())
}
