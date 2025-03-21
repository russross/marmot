use std::error::Error;
use std::fmt;

/// Custom error type for Marmot that can be created from a String or &str
/// and can automatically convert from other error types.
#[derive(Debug)]
pub struct MarmotError {
    message: String,
}

impl MarmotError {
    /// Create a new MarmotError from a string
    pub fn new<S: Into<String>>(message: S) -> Self {
        MarmotError { message: message.into() }
    }
}

impl fmt::Display for MarmotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for MarmotError {}

// Implement From for String
impl From<String> for MarmotError {
    fn from(message: String) -> Self {
        MarmotError::new(message)
    }
}

// Implement From for &str
impl From<&str> for MarmotError {
    fn from(message: &str) -> Self {
        MarmotError::new(message)
    }
}

// Implement From for sqlite::Error
impl From<sqlite::Error> for MarmotError {
    fn from(error: sqlite::Error) -> Self {
        MarmotError::new(error.to_string())
    }
}

// Type alias for Result with Box<dyn Error>
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

// Helper function to convert any error to Box<dyn Error>
pub fn to_box_err<E: Error + 'static>(err: E) -> Box<dyn Error> {
    Box::new(MarmotError::new(err.to_string()))
}

// Helper function for easily creating boxed errors from strings
pub fn err<T, S: Into<String>>(message: S) -> Result<T> {
    Err(Box::new(MarmotError::new(message.into())))
}
