use std::fs::File;
use std::io::Error;

use crate::models::Config;

pub fn parse_config(config_file: &str) -> Result<Config, Error> {
    match File::open(config_file) {
        Ok(content) => {
            let config: Config = serde_json::from_reader(content).unwrap();
            Ok(config)
        }
        Err(err) => Err(err),
    }
}
