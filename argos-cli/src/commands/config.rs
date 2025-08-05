use crate::{cli::ConfigAction, config::Config, error::CliResult};


pub fn handle_config(config: &mut Config, action: ConfigAction) -> CliResult<()> {
    match action {
        ConfigAction::Show => {
            println!("{}", config.display());
        }
        ConfigAction::Set { key, value } => {
            config.set_value(&key, &value)?;
            config.save()?;
            println!("⚙️  Configuración actualizada: {} = {}", key, value);
        }
        ConfigAction::Reset => {
            *config = Config::default();
            config.save()?;
            println!("⚙️  Configuración reseteada a valores por defecto");
        }
    }
    Ok(())
}