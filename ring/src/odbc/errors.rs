use std::error::Error;


#[derive(Debug)]
pub struct ErrorIn {
    locale: String,
    cause: Box<dyn Error + Send + Sync>
}

impl ErrorIn {
    pub fn new<L: Into<String>>(
        Locale: L,
        cause: impl Error + Send + Sync + 'static
    ) -> Self {
        Self { 
            locale: Locale.into(), 
            cause: Box::new(cause), 
        }
    }
}