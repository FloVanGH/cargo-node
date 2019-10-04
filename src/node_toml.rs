use serde_derive::Deserialize;

#[derive(Default, Debug, Deserialize)]
pub struct Font {
    pub font_family: String,
    pub src: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct App {
    pub name: String,
    pub width: f64,
    pub height: f64,
    pub assets: Option<String>,
    pub fonts: Option<Vec<Font>>,
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

    /// Gets the fonts of an app.
    pub fn fonts(&self, name: &str) -> &Option<Vec<Font>> {
        if let Some(apps) = &self.apps {
            if let Some(app) = apps.iter().find(|a| a.name.eq(name)) {
                return &app.fonts;
            }
        }

        &None
    }
}
