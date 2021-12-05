use crate::basic::structs::Package;
use rusqlite::Connection;
use std::path::Path;

pub fn query(str: &str) -> Package {
    let path = Path::new("/var/libdotpm/db.sqlite");
    let conn = Connection::open(path).unwrap();
    let mut stmt = conn
        .prepare("SELECT * FROM packages WHERE name = ?")
        .unwrap();
    let rows = stmt
        .query_map(&[&str], |row| {
            Ok(Package {
                name: row.get(0).unwrap(),
                version: row.get(1).unwrap(),
                description: row.get(2).unwrap(),
                license: row.get(4).unwrap(),
                authors: row
                    .get::<usize, String>(5)
                    .unwrap()
                    .split(" || ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
                tracked_files: row
                    .get::<usize, String>(6)
                    .unwrap()
                    .split(" || ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
                dependencies: Some(
                    row.get::<usize, String>(7)
                        .unwrap()
                        .split(" || ")
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>(),
                ),
                provides: Some(
                    row.get::<usize, String>(8)
                        .unwrap()
                        .split(" || ")
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>(),
                ),
                conflicts: Some(
                    row.get::<usize, String>(9)
                        .unwrap()
                        .split(" || ")
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>(),
                ),
                arch: row.get(10).unwrap(),
            })
        })
        .unwrap();
    let mut package = Package {
        name: "".to_string(),
        version: "".to_string(),
        description: Some("".to_string()),
        license: Some("".to_string()),
        authors: vec!["".to_string()],
        tracked_files: vec!["".to_string()],
        dependencies: Some(vec!["".to_string()]),
        provides: Some(vec!["".to_string()]),
        conflicts: Some(vec!["".to_string()]),
        arch: "".to_string(),
    };
    for row in rows {
        package = row.unwrap();
    }
    package
}

pub fn search(str: &str) -> Vec<Package> {
    let path = Path::new("/var/libdotpm/db.sqlite");
    let conn = Connection::open(path).unwrap();
    let mut stmt = conn
        .prepare("SELECT * FROM packages WHERE name LIKE ?")
        .unwrap();
    let rows = stmt
        .query_map(&[&str], |row| {
            Ok(Package {
                name: row.get(0).unwrap(),
                version: row.get(1).unwrap(),
                description: row.get(2).unwrap(),
                license: row.get(4).unwrap(),
                authors: row
                    .get::<usize, String>(5)
                    .unwrap()
                    .split(" || ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
                tracked_files: row
                    .get::<usize, String>(6)
                    .unwrap()
                    .split(" || ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
                dependencies: Some(
                    row.get::<usize, String>(7)
                        .unwrap()
                        .split(" || ")
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>(),
                ),
                provides: Some(
                    row.get::<usize, String>(8)
                        .unwrap()
                        .split(" || ")
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>(),
                ),
                conflicts: Some(
                    row.get::<usize, String>(9)
                        .unwrap()
                        .split(" || ")
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>(),
                ),
                arch: row.get(10).unwrap(),
            })
        })
        .unwrap();
    let mut packages = vec![];
    for row in rows {
        packages.push(row.unwrap());
    }
    packages
}
