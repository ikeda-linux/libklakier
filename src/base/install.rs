use crate::database;
use blake2::{Blake2b, Digest};
use std::{
    error::Error,
    fs::{self, File},
    path::Path,
    process::Command,
};
use tar::Archive;
use zstd_util::ZstdContext;

use super::structs::Package;

#[allow(dead_code)]
pub fn install(pkg: &Path) -> Result<(), Box<dyn Error>> {
    // decompress the .tar.zst packagefile
    let mut zstd = ZstdContext::new(11, Some(&[]));
    let inflated = zstd
        .decompress(&fs::read(pkg).unwrap())
        .unwrap_or_else(|err| {
            panic!("Failed to decompress {}: {}", pkg.display(), err);
        });

    // hash the file to create a unique directory to unpack it to
    let mut hasher = Blake2b::new();
    hasher.update(fs::read(pkg).unwrap());
    let hash = hasher.finalize();
    let hash = String::from_utf8_lossy(hash.as_slice()).replace(|c: char| !c.is_alphanumeric(), "");

    // create directory strings to use later
    let dir = format!("/tmp/libdlta/{}", hash);
    let infl_path = format!("/tmp/libdlta/{}/package.tar", hash);

    // create the actual directories to unpack to
    fs::create_dir_all(&dir).unwrap();
    fs::write(&infl_path, inflated).unwrap_or_else(|err| {
        panic!("Failed to write {}: {}", infl_path, err);
    });

    // untar the now decompressed tarball
    let mut tar = Archive::new(File::open(infl_path).unwrap());
    tar.unpack(&dir).unwrap();

    // initialise the database if not already found
    if !Path::new("/var/libdlta/db.sqlite").exists() {
        database::initialise::initialise(Path::new(database::initialise::DATABASE_PATH), true)
            .unwrap_or_else(|err| {
                panic!("Failed to initialise database: {}", err);
            });
    }

    // copies everything from the overlay/ directory directly into /
    let overlay = format!("{}/overlay/", dir);
    Command::new("rsync")
        .args(&["-rK", &overlay, "/"])
        .status()
        .unwrap_or_else(|err| {
            panic!("Failed to rsync from overlay to rootfs: {}", err);
        });

    // adds the package to the database
    let pkginfo: Package =
        toml::from_str(&fs::read_to_string(format!("{}/md.toml", &dir)).unwrap()).unwrap();
    database::add::add(
        pkginfo,
        Path::new(crate::database::initialise::DATABASE_PATH),
    )
    .unwrap_or_else(|err| {
        panic!("Failed to add package to database: {}", err);
    });

    // removes the temporary directory
    fs::remove_dir_all(&dir).unwrap_or_else(|err| {
        panic!("Failed to remove {}: {}", dir, err);
    });

    Ok(())
}
