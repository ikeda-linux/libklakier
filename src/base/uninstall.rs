use crate::database::{query::query, remove::remove};
use std::path::Path;

#[allow(dead_code)]
pub fn uninstall(pkg: &str) {
    // queries the database for the package that exactly matches pkg
    let res = query(pkg, Path::new(crate::database::initialise::DATABASE_PATH));
    let files = &res.tracked_files;

    // deletes the files
    for file in files {
        std::fs::remove_file(&file).unwrap_or_else(|_| {
            panic!("Could not delete file: {}", file);
        });
    }

    // removes the package from the database
    remove(res, Path::new(crate::database::initialise::DATABASE_PATH)).unwrap_or_else(|_| {
        panic!("Could not delete package from database");
    });
}
