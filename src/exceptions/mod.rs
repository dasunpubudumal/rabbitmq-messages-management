use core::fmt;

/// Common struct to denote server errors
#[derive(Debug, Clone)]
pub struct ServerError {
    /// Custom message returned with the error
    pub message: String,
}

impl ServerError {
    pub fn new(message: String) -> ServerError {
        ServerError { message }
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
