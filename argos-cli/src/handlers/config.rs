use crate::config::Config;
use crate::error::{CliResult};
use crate::cli::ConfigAction;

pub fn handle_config(config: &mut Config, action: ConfigAction) -> CliResult<()> {
    match action {
        ConfigAction::Show => {
            // Mostrar la configuración actual
            println!("{}", config.display());
        }
        ConfigAction::Set { key, value } => {
            // Modificar el valor en memoria
            config.set_value(&key, &value)?;

            // Guardar cambios en el TOML
            config.save()?;

            println!("⚙️  Configuración actualizada: {} = {}", key, value);
        }
        ConfigAction::Reset => {
            // Resetear a valores por defecto
            *config = Config::default();
            config.save()?;

            println!("⚙️  Configuración reseteada a valores por defecto");
        }
    }
    Ok(())
}
