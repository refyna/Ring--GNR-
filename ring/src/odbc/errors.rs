use std::error::Error;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{locale}: {cause}")]
pub struct ErrorIn {
    locale: String,

    #[source]
    cause: Box<dyn Error + Send + Sync>
}

impl ErrorIn {
    pub fn new<L: Into<String>>(
        locale: L,
        cause: impl Error + Send + Sync + 'static
    ) -> Self {
        Self { 
            locale: locale.into(), 
            cause: Box::new(cause), 
        }
    }
}