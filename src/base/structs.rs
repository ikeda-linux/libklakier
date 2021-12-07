use serde::Deserialize;

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct SrcPackage {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub license: Option<String>,
    pub tracked_files: Vec<String>,
    pub dependencies: Option<Vec<String>>,
    pub provides: Option<Vec<String>>,
    pub conflicts: Option<Vec<String>>,
    pub arch: Option<Vec<String>>,
    pub remote: bool,
    pub vcs: Option<String>,
    pub repo: Option<String>
}
impl SrcPackage {
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
        arch: Option<Vec<String>>,
        remote: bool,
        vcs: Option<String>,
        repo: Option<String>
    ) -> SrcPackage {
        SrcPackage {
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
            remote,
            vcs,
            repo
        }
    }
    pub fn create(str: &str) -> Self {
        SrcPackage {
            name: str.to_string(),
            version: "0.1.0".to_string(),
            description: None,
            authors: Vec::new(),
            license: None,
            tracked_files: Vec::new(),
            dependencies: None,
            provides: None,
            conflicts: None,
            arch: None,
            remote: false,
            vcs: None,
            repo: None
        }
    }
}