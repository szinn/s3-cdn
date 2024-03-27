use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct S3CdnConfig {
    /// (optional) Port number to run S3-CDN on. Defaults to 8080.
    pub port: Option<u16>,

    /// S3 host.
    pub host: String,

    /// S3 region.
    pub region: String,

    /// S3 bucket.
    pub bucket: String,

    /// S3 access key id.
    pub access_key_id: String,

    /// S3 secret access key.
    pub secret_access_key: String,
}

impl S3CdnConfig {
    pub fn load() -> Result<S3CdnConfig, ConfigError> {
        let config = Config::builder()
            .add_source(Environment::with_prefix("S3CDN").try_parsing(true).separator("__"))
            .build()?;

        let config: S3CdnConfig = config.try_deserialize()?;

        Ok(config)
    }
}
