use config::{Config, Environment};
use serde::Deserialize;

use crate::error::Error;

#[derive(Debug, Deserialize)]
pub struct S3CdnConfig {
    /// (optional) Port number to run S3-CDN on . Defaults to 8080.
    pub port: Option<u16>,
}

impl S3CdnConfig {
    pub fn load() -> Result<S3CdnConfig, Error> {
        let config = Config::builder()
            .add_source(Environment::with_prefix("S3CDN").try_parsing(true).separator("__"))
            .build()?;

        let config: S3CdnConfig = config.try_deserialize()?;

        Ok(config)
    }
}
