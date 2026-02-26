/// Errors returned by the Forge SDK.
#[derive(Debug, thiserror::Error)]
pub enum ForgeError {
    /// HTTP or network-level error (connection refused, DNS failure, timeout).
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    /// The server returned a 4xx/5xx response with an error message.
    #[error("server error ({status}): {message}")]
    Server {
        status: u16,
        message: String,
    },
}
