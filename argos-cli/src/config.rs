use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use crate::error::{CliResult, CliError};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub default_format: String,
    pub default_interval_ms: u64,
    pub default_iterations: usize,
    pub auto_save: bool,
    pub database_url: Option<String>,
    pub log_level: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_format: "text".to_string(),
            default_interval_ms: 200,
            default_iterations: 10,
            auto_save: false,
            database_url: None,
            log_level: "info".to_string(),
        }
    }
}

impl Config {
    pub fn load() -> CliResult<Self> {
        let config_path = Self::config_path()?;
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .map_err(|e| CliError::io_error(format!("Error leyendo configuración: {}", e)))?;
            
            toml::from_str(&content)
                .map_err(|e| CliError::format_error(format!("Error parsing configuración: {}", e)))
        } else {
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> CliResult<()> {
        let config_path = Self::config_path()?;
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| CliError::io_error(format!("Error creando directorio config: {}", e)))?;
        }

        let content = toml::to_string_pretty(self)
            .map_err(|e| CliError::format_error(format!("Error serializando configuración: {}", e)))?;

        fs::write(&config_path, content)
            .map_err(|e| CliError::io_error(format!("Error escribiendo configuración: {}", e)))?;

        Ok(())
    }

    pub fn set_value(&mut self, key: &str, value: &str) -> CliResult<()> {
        match key {
            "default_format" => {
                if !["text", "json", "csv"].contains(&value) {
                    return Err(CliError::new(
                        crate::error::ErrorKind::ValidationError,
                        "Formato debe ser: text, json, o csv"
                    ));
                }
                self.default_format = value.to_string();
            }
            "default_interval_ms" => {
                self.default_interval_ms = value.parse()
                    .map_err(|_| CliError::new(
                        crate::error::ErrorKind::ValidationError,
                        "Intervalo debe ser un número"
                    ))?;
            }
            "default_iterations" => {
                self.default_iterations = value.parse()
                    .map_err(|_| CliError::new(
                        crate::error::ErrorKind::ValidationError,
                        "Iteraciones debe ser un número"
                    ))?;
            }
            "auto_save" => {
                self.auto_save = value.parse()
                    .map_err(|_| CliError::new(
                        crate::error::ErrorKind::ValidationError,
                        "auto_save debe ser true o false"
                    ))?;
            }
            "database_url" => {
                self.database_url = Some(value.to_string());
            }
            "log_level" => {
                if !["error", "warn", "info", "debug", "trace"].contains(&value) {
                    return Err(CliError::new(
                        crate::error::ErrorKind::ValidationError,
                        "Log level debe ser: error, warn, info, debug, o trace"
                    ));
                }
                self.log_level = value.to_string();
            }
            _ => {
                return Err(CliError::new(
                    crate::error::ErrorKind::ValidationError,
                    format!("Clave de configuración desconocida: {}", key)
                ));
            }
        }
        Ok(())
    }

    fn config_path() -> CliResult<PathBuf> {
        let mut path = dirs::config_dir()
            .ok_or_else(|| CliError::io_error("No se pudo encontrar directorio de configuración"))?;
        
        path.push("argos");
        path.push("config.toml");
        
        Ok(path)
    }

    pub fn display(&self) -> String {
        format!(
        "⚙️  Configuración de Argos\n\
         ┌───────────────────────────────────────────┐\n\
        {:8}Formato por defecto: {:<25}\n\
        {:8}Intervalo por defecto: {:<21}\n\
        {:8}Iteraciones por defecto: {:<20}\n\
        {:8}Auto-guardar: {:<28}\n\
        {:8}Log level: {:<31}\n\
         └───────────────────────────────────────────┘",
        "", self.default_format,
        "", format!("{} ms", self.default_interval_ms),
        "", self.default_iterations,
        "", if self.auto_save { "Sí" } else { "No" },
        "", self.log_level,
    )
    }
}
