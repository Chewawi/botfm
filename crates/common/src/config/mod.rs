// stealed from assyst owo

#[allow(clippy::module_inception)]
pub mod config;

pub static CONFIG_LOCATION: &str = "./config.toml";

use lazy_static::lazy_static;
use toml::from_str;
use tracing::info;

use config::Config;

lazy_static! {
    pub static ref CONFIG: Config = {
        let toml = std::fs::read_to_string(CONFIG_LOCATION).unwrap();
        let config = from_str::<Config>(&toml).unwrap();
        info!("Loaded config file {}", CONFIG_LOCATION);
        config
    };
}
