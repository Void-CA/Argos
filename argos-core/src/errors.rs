use serde_json;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Error de IO: {0}")]
    Io(#[from] io::Error),

    #[error("Error de parseo: {0}")]
    Parse(#[from] serde_json::Error),

    #[error("Error de comparación")]
    CompareError,

    #[error("Formato no soportado: {0}")]
    UnsupportedFormat(String),

    #[error("Error desconocido: {0}")]
    Other(String),

    #[error("Error de comparación: {0}")]
    ComparisonError(String),

    #[error("Error de validación: {0}")]
    ValidationError(String),

    #[error("Error de proceso no encontrado: {0:?}")]
    ProcessNotFoundList(Vec<u32>),

    #[error("Error de proceso no encontrado: {0}")]
    ProcessNotFound(u32),

    #[error("Error de watchdog: {0}")]
    WatchdogError(String),
}

pub type CoreResult<T> = Result<T, CoreError>;