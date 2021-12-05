pub struct Package {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub license: Option<String>,

    pub tracked_files: Vec<String>,
    pub dependencies: Option<Vec<Package>>,
    pub provides: Option<Vec<String>>,
    pub conflicts: Option<Vec<String>>,
    pub arch: String
}