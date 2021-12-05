use crate::{basic::structs::Package, database::initialise::DATABASE_PATH};
use rusqlite::{Connection, Error};
use std::path::Path;

pub fn remove(pkg: Package) -> Result<(), Error> {
    let dbpath = Path::new(DATABASE_PATH);
    let mut conn = Connection::open(dbpath)?;
    let tx = conn.transaction()?;
    tx.execute("DELETE FROM packages WHERE name = ?", &[&pkg.name])?;
    tx.commit()?;
    Ok(())
}
