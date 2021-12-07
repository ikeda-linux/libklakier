use crate::base::structs::Package;
use rusqlite::Connection;
use std::path::Path;

pub fn search(str: &str, path: &Path) -> Vec<Package> {
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
                license: row.get(3).unwrap(),
                authors: row
                    .get::<usize, String>(4)
                    .unwrap()
                    .split(" || ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
                tracked_files: row
                    .get::<usize, String>(5)
                    .unwrap()
                    .split(" || ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
                dependencies: Some(
                    row.get::<usize, String>(6)
                        .unwrap()
                        .split(" || ")
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>(),
                ),
                provides: Some(
                    row.get::<usize, String>(7)
                        .unwrap()
                        .split(" || ")
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>(),
                ),
                conflicts: Some(
                    row.get::<usize, String>(8)
                        .unwrap()
                        .split(" || ")
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>(),
                ),
                arch: row.get(9).unwrap(),
            })
        })
        .unwrap();
    let mut packages = vec![];
    for row in rows {
        packages.push(row.unwrap());
    }
    packages
}