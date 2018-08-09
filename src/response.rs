// XXX: Figure out if we need this at all with actix

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: i16,
    pub error: String,
}

impl ErrorResponse {
    pub fn new<S>(code: i16, error: Option<S>) -> ErrorResponse
    where
        S: Into<String>,
    {
        ErrorResponse {
            code: code,
            error: if let Some(e) = error {
                e.into()
            } else {
                "".into()
            },
        }
    }

    pub fn bad_request<S>(error: S) -> ErrorResponse
    where
        S: Into<String>,
    {
        ErrorResponse::new(400, Some(error))
    }

    pub fn server_error() -> ErrorResponse {
        ErrorResponse::new(500, Some("Internal server error"))
    }

    pub fn not_implemented() -> ErrorResponse {
        ErrorResponse::new(500, Some("Not implemented yet"))
    }
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token: String,
}
