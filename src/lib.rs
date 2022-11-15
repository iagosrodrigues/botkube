use config::config::Config;
use log::{debug, info};
pub struct Botkube {
    config: Config,
}

impl Botkube {
    pub fn load_config() -> Result<Self, std::io::Error> {
        let config = Config::load_from_file("Configuration.toml")?;

        Ok(Botkube { config })
    }

    pub fn run(&self) {
        debug!(
            "Bot token {:#?}",
            self.config.communications["main"]
                .telegram
                .token
                .as_ref()
                .unwrap()
        );
    }
}
