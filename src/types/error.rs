use edn_rs::EdnError;
use reqwest::Error;

/// Main error type for transistor crate
#[derive(Debug)]
pub enum CruxError {
    /// Error originated by `edn_rs` crate. The provided EDN did not match schema.
    ParseEdnError(String),
    /// Error originated by `reqwest` crate. Failed to make HTTP request.
    RequestError(Error),
    /// Query response error, most likely a Clojure stacktrace from Crux response.
    QueryError(String),
    /// Provided Query struct did not match schema.
    QueryFormatError(String),
}

impl std::error::Error for CruxError {
    fn description(&self) -> &str {
        match self {
            CruxError::ParseEdnError(s) => &s,
            CruxError::RequestError(_) => "HTTP request to Crux failed",
            CruxError::QueryError(s) => &s,
            CruxError::QueryFormatError(s) => &s,
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        Some(self)
    }
}

impl std::fmt::Display for CruxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CruxError::ParseEdnError(s) => write!(f, "{}", &s),
            CruxError::RequestError(e) => write!(f, "{:?}", &e),
            CruxError::QueryError(s) => write!(f, "{}", &s),
            CruxError::QueryFormatError(s) => write!(f, "{}", &s),
        }
    }
}

impl From<EdnError> for CruxError {
    fn from(err: EdnError) -> Self {
        match err {
            EdnError::ParseEdnError(s) => CruxError::ParseEdnError(s),
        }
    }
}

impl From<Error> for CruxError {
    fn from(err: Error) -> Self {
        CruxError::RequestError(err)
    }
}
