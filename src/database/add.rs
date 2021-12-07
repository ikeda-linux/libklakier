use crate::{base::structs::Package, database::initialise::DATABASE_PATH};
use rusqlite::{Connection, Error};
use std::path::Path;

pub fn add(pkg: Package) -> Result<(), Error> {
    let dbpath = Path::new(DATABASE_PATH);
    let mut conn = Connection::open(dbpath)?;
    let tx = conn.transaction()?;
    tx.execute("INSERT INTO packages (name, version, description, authors, license, tracked_files, dependencies, provides, conflicts, arch) VALUES (:name, :version, :description, :authors, :license, :tracked_files, :dependencies, :provides, :conflicts, :arch)", &[
        &pkg.name, 
        &pkg.version,
        &pkg.description.unwrap_or_else(|| "".to_string()),
        &pkg.authors.join(" || "),
        &pkg.license.unwrap_or_else(|| "".to_string()),
        &pkg.tracked_files.join(" || "),
        &pkg.dependencies.unwrap_or_default().join(" || "),
        &pkg.provides.unwrap_or_default().join(" || "),
        &pkg.conflicts.unwrap_or_default().join(" || "),
        &pkg.arch])?;
    tx.commit()?;
    Ok(())
}
