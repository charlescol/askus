use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    MissingRequiredEnvironmentVariable(String),

    #[error("Configuration file not found: {0}")]
    ConfigFileNotFound(String),

    #[error("Failed to parse configuration: {0}")]
    ParseError(String),

    #[error("Invalid value: {0}")]
    InvalidValue(String),
}
