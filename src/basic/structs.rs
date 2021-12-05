pub struct Package {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub authors: Vec<String>,

    pub tracked_files: Vec<String>,
    pub dependencies: Option<Vec<Package>>,
    pub provides: Option<Vec<String>>,
    pub conflicts: Option<Vec<String>>,

    pub install_instructions: Vec<String>,
    pub uninstall_instructions: Vec<String>,
}