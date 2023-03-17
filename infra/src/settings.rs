use std::env;

use config::{Config, ConfigError, File};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub web: Web,
}

#[derive(Debug, Deserialize)]
pub struct Web {
    pub port: u16,
}

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        let settings = Config::builder()
            .add_source(File::with_name("config/default.yaml"))
            .add_source(File::with_name(&format!("config/{}", env)).required(true))
            .build()
            .unwrap();

        println!("debug: {:?}", settings.get_bool("debug"));
        println!("web: {:?}", settings.get::<Web>("web"));
        settings.try_deserialize()
    }
}
