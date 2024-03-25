use config::ConfigError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Bad port - {}", _0)]
    BadPort(u16),

    #[error(transparent)]
    ConfigError(#[from] ConfigError),
}
