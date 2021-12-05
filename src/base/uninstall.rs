use crate::database::{query::query, remove::remove};

#[allow(dead_code)]
pub fn uninstall(pkg: &str) {
    // queries the database for the package that exactly matches pkg
    let res = query(pkg);
    let files = &res.tracked_files;

    // deletes the files
    for file in files {
        std::fs::remove_file(&file).unwrap_or_else(|_| {
            panic!("Could not delete file: {}", file);
        });
    }

    // removes the package from the database
    remove(res).unwrap_or_else(|_| {
        panic!("Could not delete package from database");
    });
}
