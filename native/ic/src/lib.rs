use std::{error, fmt, io};

/// A useless Error just for the Demo
#[derive(Copy, Clone, Debug)]
pub struct ICError;

impl fmt::Display for ICError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error While ICping this page.")
    }
}

impl error::Error for ICError {}

impl From<reqwest::Error> for ICError {
    fn from(_: reqwest::Error) -> Self {
        Self
    }
}

impl From<io::Error> for ICError {
    fn from(_: io::Error) -> Self {
        Self
    }
}

/// Load a page and return its HTML body as a `String`
pub async fn load_page(url: &str) -> Result<String, ICError> {
    Ok(reqwest::get(url).await?.text().await?)
}