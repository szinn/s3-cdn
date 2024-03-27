use config::ConfigError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Bad port - {}", _0)]
    BadPort(u16),

    #[error("Bucket missing - {}", _0)]
    BucketNotFound(String),

    #[error("Can't retrieve object - {}", _0)]
    CantRetrieveObject(String),

    #[error(transparent)]
    ConfigError(#[from] ConfigError),

    #[error(transparent)]
    MinioError(#[from] minio::s3::error::Error),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}
