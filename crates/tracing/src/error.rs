use anyhow::anyhow;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[cfg(feature = "uniffi")]
#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum AppbioticTracingError {
    #[error("Unknown: {message}")]
    Unknown { message: String },
}

#[cfg(feature = "uniffi")]
uniffi::custom_type!(Error, AppbioticTracingError, {
    lower: |value| match value {
        Error::Unknown(error) => AppbioticTracingError::Unknown {
            message: error.to_string(),
        },
    },
    try_lift: |value|  match value {
        AppbioticTracingError::Unknown { message } => Ok(Error::Unknown(anyhow!(message))),
    },
});

pub type Result<T> = std::result::Result<T, Error>;
