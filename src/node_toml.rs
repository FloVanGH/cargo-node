use serde::Deserialize;
#[derive(Default, Debug, Deserialize)]
pub struct App {
    pub name: String,
    pub width: f64,
    pub height: f64,
    pub assets: Option<String>,
}

/// Package definition inside of a Node.toml file.
#[derive(Default, Debug, Deserialize)]
pub struct NodeToml {
    pub apps: Option<Vec<App>>,
}

impl NodeToml {
    /// Gets the assets dir path of an app.
    pub fn assets(&self, name: &str) -> &Option<String> {
        if let Some(apps) = &self.apps {
            if let Some(app) = apps.iter().find(|a| a.name.eq(name)) {
                return &app.assets;
            }
        }

        &None
    }
}
