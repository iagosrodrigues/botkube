use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub communications: HashMap<String, Communication>,
}

#[derive(Deserialize, Serialize)]
pub struct Communication {
    pub telegram: Telegram,
}

#[derive(Deserialize, Serialize)]
pub struct Telegram {
    pub enabled: bool,
    pub token: Option<String>,
    pub chat_id: Option<String>,
    pub notification: Option<Notification>,
    pub bindings: Option<Binding>,
}

#[derive(Deserialize, Serialize)]
pub struct Notification {
    pub enabled: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Binding {
    pub sources: Vec<String>,
}

impl Config {
    pub fn load_from_file(str: &str) -> Result<Config, std::io::Error> {
        let content = fs::read_to_string(str)?;
        let config: Config = toml::from_str(content.as_str())?;

        Ok(config)
    }
}
