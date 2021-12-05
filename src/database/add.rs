pub fn add(pkg: Package) -> Result<()> {
    let dbpath = Path::new("/var/libdotpm/db.sqlite");
    let conn = Connection::open(dbpath)?;
    let tx = conn.transaction()?;
    let mut stmt = tx.prepare("INSERT INTO packages (name, version, description, authors, license, tracked_files, dependencies, provides, conflicts, arch) VALUES (:name, :version, :description, :authors, :license, :tracked_files, :dependencies, :provides, :conflicts, :arch)")?;
    stmt.execute(&[&pkg.name, &pkg.version, &pkg.description, &pkg.authors, &pkg.license, &pkg.tracked_files, &pkg.dependencies, &pkg.provides, &pkg.conflicts, &pkg.arch])?;
    tx.commit()?;
    Ok(())
}