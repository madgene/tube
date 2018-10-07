use std::env;
use config::{ ConfigError, Config, File, Environment };


#[derive(Debug, Deserialize)]
pub struct Files {
    pub path: String
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub files: Files
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config/default"))?;

        // Add in the current environment file
        let env = env::var("RUN_MODE").unwrap_or("development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // Add in a local configuration file
        s.merge(File::with_name("config/local").required(false))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("app"))?;

        // Deserialize and freeze the entire configuration
        s.try_into()
    }
}

lazy_static! {
    static ref SETTINGS: Settings = {
        Settings::new().unwrap()
    };
}

pub fn get() -> &'static Settings {
    &SETTINGS
}

