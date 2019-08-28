use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Window {
    width: f64,
    height: f64
}

/// Package definition inside of a Node.toml file.
#[derive(Debug, Deserialize)]
pub struct NodeToml {
    windows: Vec<Window>
}