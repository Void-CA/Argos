use std::fmt;

pub type CliResult<T> = Result<T, CliError>;

#[derive(Debug)]
pub struct CliError {
    pub message: String,
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    ProcessNotFound,
    DatabaseError,
    FormatError,
    ConfigError,
    IoError,
    ValidationError,
    CoreError,
}

impl CliError {
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            kind,
        }
    }

    pub fn process_not_found(pid: u32) -> Self {
        Self::new(
            ErrorKind::ProcessNotFound,
            format!("No se encontró el proceso con PID {}", pid),
        )
    }

    pub fn database_error(msg: impl Into<String>) -> Self {
        Self::new(ErrorKind::DatabaseError, msg)
    }

    pub fn format_error(msg: impl Into<String>) -> Self {
        Self::new(ErrorKind::FormatError, msg)
    }

    pub fn io_error(msg: impl Into<String>) -> Self {
        Self::new(ErrorKind::IoError, msg)
    }

    pub fn core_error<E: std::fmt::Display>(err: E) -> Self {
        Self::new(ErrorKind::CoreError, err.to_string())
    }
}


impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::ProcessNotFound => write!(f, "❌ Proceso no encontrado: {}", self.message),
            ErrorKind::DatabaseError => write!(f, "💾 Error de base de datos: {}", self.message),
            ErrorKind::FormatError => write!(f, "📄 Error de formato: {}", self.message),
            ErrorKind::ConfigError => write!(f, "⚙️  Error de configuración: {}", self.message),
            ErrorKind::IoError => write!(f, "📁 Error de E/S: {}", self.message),
            ErrorKind::ValidationError => write!(f, "⚠️  Error de validación: {}", self.message),
            ErrorKind::CoreError => write!(f, "🛠️  Error interno: {}", self.message),
        }
    }
}

// Implementaciones de From para conversión automática de errores
impl From<String> for CliError {
    fn from(msg: String) -> Self {
        Self::new(ErrorKind::ValidationError, msg)
    }
}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        Self::io_error(err.to_string())
    }
}
