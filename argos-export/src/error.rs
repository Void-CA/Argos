use std::fmt;

#[derive(Debug)]
pub enum ExportError {
    SerdeJson(serde_json::Error),
    Csv(csv::Error),
    Utf8(std::string::FromUtf8Error),
    Io(std::io::Error),
    UnsupportedFormat(String),
    // Otros errores...
}

impl From<serde_json::Error> for ExportError {
    fn from(e: serde_json::Error) -> Self { ExportError::SerdeJson(e) }
}
impl From<csv::Error> for ExportError {
    fn from(e: csv::Error) -> Self { ExportError::Csv(e) }
}
impl From<std::string::FromUtf8Error> for ExportError {
    fn from(e: std::string::FromUtf8Error) -> Self { ExportError::Utf8(e) }
}

impl fmt::Display for ExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for ExportError {}