use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub license: Option<String>,
    pub tracked_files: Vec<String>,
    pub dependencies: Option<Vec<String>>,
    pub provides: Option<Vec<String>>,
    pub conflicts: Option<Vec<String>>,
    pub arch: String,
}
impl Package {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        version: String,
        description: Option<String>,
        authors: Vec<String>,
        license: Option<String>,
        tracked_files: Vec<String>,
        dependencies: Option<Vec<String>>,
        provides: Option<Vec<String>>,
        conflicts: Option<Vec<String>>,
        arch: String,
    ) -> Package {
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
    }
}