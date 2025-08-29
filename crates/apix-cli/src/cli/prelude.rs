use crate::{cli::cli::Commands, config::get_config, db::connect_db};

pub fn pre_command_checks(command: &Commands) -> Result<(), String> {
    match command {
        Commands::Init { .. } => Ok(()),
        _ => {
            let current_dir = std::env::current_dir()
                .map_err(|_| "Failed to get current directory".to_string())?;

            if !current_dir.join("monorepo.toml").exists() {
                return Err(
                    "This command must be run inside a managed directory with 'monorepo.toml'"
                        .into(),
                );
            }

            connect_db(&get_config().db_path)
                .map_err(|_| "Failed to connect to the DB".to_string())?;

            Ok(())
        }
    }
}
