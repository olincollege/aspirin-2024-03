use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum AspirinEatsError {
    /// Error when trying to parse a JSON string
    #[error("Failed to parse request")]
    ParseError(#[from] serde_json::Error),

    /// Error when fetching or otherwise interacting with the database
    #[error("Failed to interact with database")]
    Database(#[from] rusqlite::Error),

    /// Error when reading/writing from Streams
    #[error("Failed to read/write from stream")]
    Io(#[from] std::io::Error),

    /// Error interpreting or parsing HTTP Request
    #[error("Invalid Request")]
    InvalidRequest,

    /// Error when receiving request for resource that does not exist
    #[error("Resource not found")]
    NotFound,

    /// Error when request is for an HTTP method not supported on that path
    #[error("Method not allowed")]
    MethodNotAllowed,
}
