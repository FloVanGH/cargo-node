use serde_derive::Deserialize;

/// Package definition inside of a Cargo.toml file.
#[derive(Debug, Deserialize)]
pub struct CargoPackage {
    pub name: Option<String>,
    pub description: Option<String>,
    pub license: Option<String>,
    // todo: authors, keywords, repository
}

/// A representation of a Cargo.toml file.
#[derive(Debug, Deserialize)]
pub struct CargoToml {
    pub package: Option<CargoPackage>,
}
