use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::env;


#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub db_url : String,
    pub redis_url: String,
    pub debug: bool,
    pub secret: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("config/default"))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(
                File::with_name(&format!("config/{}", run_mode))
                    .required(false),
            )
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name("config/local").required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("APP"))
            // You may also programmatically change settings
            // .set_override("database.url", "postgres://")?
            .build()?;

        // Now that we're done, let's access our configuration
        println!("debug: {:?}", s.get_bool("debug"));
        println!("database: {:?}", s.get::<String>("db_url"));
        println!("database: {:?}", s.get::<String>("redis_url"));
        println!("database: {:?}", s.get::<String>("secret"));

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}
