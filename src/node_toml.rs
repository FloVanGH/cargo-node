use serde::Deserialize;
#[derive(Default, Debug, Deserialize)]
pub struct Window {
    pub name: Option<String>,
    pub width: f64,
    pub height: f64,
}

/// Package definition inside of a Node.toml file.
#[derive(Default, Debug, Deserialize)]
pub struct NodeToml {
    pub windows: Option<Vec<Window>>,
}
