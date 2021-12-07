use std::{path::Path, fs::{File, self}, error::Error};
use blake2::{Blake2b, Digest};
use zstd_util::ZstdContext;
use tar::Archive;
use crate::database;

use super::structs::Package;

#[allow(dead_code)]
pub fn install(pkg: &Path) -> Result<(), Box<dyn Error>> {
    // decompress the .tar.zst packagefile
    let mut zstd = ZstdContext::new(11, Some(&[]));
    let inflated = zstd.decompress(&fs::read(pkg).unwrap()).unwrap_or_else(|_| {
        panic!("Failed to decompress {}", pkg.display());
    });

    // hash the file to create a unique directory to unpack it to
    let mut hasher = Blake2b::new();
    hasher.update(fs::read(pkg).unwrap());
    let hash = hasher.finalize();
    let hash = String::from_utf8_lossy(hash.as_slice());

    // create directory strings to use later
    let dir = format!("/tmp/libdlta/{}", hash);
    let infl_path = format!("/tmp/libdlta/{}/package.tar", hash);
    let outpath = format!("/tmp/libdlta/{}/out", hash);

    // create the actual directories to unpack to
    fs::create_dir_all(&dir).unwrap();
    fs::write(&infl_path, inflated).unwrap_or_else(|_| {
        panic!("Failed to write {}", infl_path);
    });

    // untar the now decompressed tarball
    let mut tar = Archive::new(File::open(infl_path).unwrap());
    tar.unpack(&dir).unwrap();

    // initialise the database if not already found
    if !Path::new("/var/libdlta/db.sqlite").exists() {
        database::initialise::initialise().unwrap_or_else(|_| {
            panic!("Failed to initialise database");
        });
    }

    // copies the files from the package's overlay/ directory directly into the rootfs
    for entry in fs::read_dir(format!("{}/overlay", outpath)).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            fs::create_dir_all(format!("/{}", path.to_str().unwrap())).unwrap();
        } else {
            fs::copy(&path, format!("/{}", path.to_str().unwrap())).unwrap_or_else(|_| {
                panic!("Failed to copy {}", path.to_str().unwrap());
            });
        }
    }

    // adds the package to the database
    let pkginfo: Package = toml::from_str(&fs::read_to_string(format!("{}/md.toml", outpath)).unwrap()).unwrap();
    database::add::add(pkginfo).unwrap_or_else(|_| {
        panic!("Failed to add package to database");
    });

    // removes the temporary directory
    fs::remove_dir_all(&dir).unwrap_or_else(|_| {
        panic!("Failed to remove {}", dir);
    });

    Ok(())
}
