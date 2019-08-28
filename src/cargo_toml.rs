use serde::Deserialize;

/// Package definition inside of a Cargo.toml file.
#[derive(Debug, Deserialize)]
pub struct CargoPackage {
    name: Option<String>,
    description: Option<String>,
}

/// A representation of a Cargo.toml file.
#[derive(Debug, Deserialize)]
pub struct CargoToml {
    package: Option<CargoPackage>,
}
