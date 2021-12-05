use crate::basic::structs::Package;
use std::path::Path;
use rusqlite::{Connection, Error};

pub fn add(pkg: Package) -> Result<(), Error> {
    let dbpath = Path::new("/var/libdotpm/db.sqlite");
    let mut conn = Connection::open(dbpath)?;
    let tx = conn.transaction()?;
    tx.execute("INSERT INTO packages (name, version, description, authors, license, tracked_files, dependencies, provides, conflicts, arch) VALUES (:name, :version, :description, :authors, :license, :tracked_files, :dependencies, :provides, :conflicts, :arch)", &[&pkg.name, &pkg.version, &pkg.description.unwrap_or( "".to_string()), &pkg.authors.join(" || "), &pkg.license.unwrap_or("".to_string()), &pkg.tracked_files.join(" || "), &pkg.dependencies.unwrap_or(vec![]).join(" || "), &pkg.provides.unwrap_or(vec![]).join(" || "), &pkg.conflicts.unwrap_or(vec![]).join(" || "), &pkg.arch])?;
    tx.commit()?;
    Ok(())
}