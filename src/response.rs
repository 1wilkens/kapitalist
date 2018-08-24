#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error: String,
}

impl ErrorResponse {
    pub fn new<S>(err: S) -> ErrorResponse
    where S: Into<String> {
        ErrorResponse { error: err.into() }
    }

    pub fn server_error() -> ErrorResponse {
        ErrorResponse::new("Internal server error")
    }

    pub fn not_implemented() -> ErrorResponse {
        ErrorResponse::new("Not implemented yet")
    }
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token: String,
}
