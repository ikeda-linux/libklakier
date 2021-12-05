
pub fn query(str: &str) -> Vec<Package> {
    let dbpath = Path::new("/var/libdotpm/db.sqlite");
    let conn = Connection::open(dbpath).unwrap();
    let mut stmt = conn.prepare(str).unwrap();
    let rows = stmt.query_map(&[], |row| {
        let name: String = row.get(0);
        let version: String = row.get(1);
        let description: String = row.get(2);
        let authors: String = row.get(3);
        let license: String = row.get(4);
        let tracked_files: String = row.get(5);
        let dependencies: String = row.get(6);
        let provides: String = row.get(7);
        let conflicts: String = row.get(8);
        let arch: String = row.get(9);
        Package {
            name,
            version,
            description,
            authors,
            license,
            tracked_files,
            dependencies,
            provides,
            conflicts,
            arch,
        }
    }).unwrap();
    let mut packages: Vec<Package> = Vec::new();
    for row in rows {
        let pkg = row.unwrap();
        packages.push(pkg);
    }
    packages
}
