mod commands;

use clap::{command, Parser, Subcommand};
use log::error;

use crate::{config::get_config, db::connect_db};

#[derive(Parser)]
#[command(name = "apix")]
#[command(about = "A project scaffolding & automation CLI", long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create {
        name: String,

        #[arg(short, long)]
        template: Option<String>,

        #[arg(short, long)]
        plugin: Option<String>,
    },
    Rename {
        old: String,
        new: String
    },
    ListProjects,
    ListPlugins,
    Plugin {
        #[command(subcommand)]
        command: PluginCommands,
    },
}

#[derive(Subcommand)]
enum PluginCommands {
    Create {
        name: String
    },
    Migrate,
    #[command(external_subcommand)]
    Custom(Vec<String>),
}

pub fn init_cli() {
    let cli = CLI::parse();

    if !matches!(cli.command, Commands::Create { .. }) {
        if let Err(_e) = connect_db(&get_config().db_path) {
            error!("Failed to connect to the DB. It may not be initialized in the current directory.");
            std::process::exit(1);
        }
    }

    match cli.command {
        Commands::Create { name, plugin, template } => commands::create::create_monorepo(name, template, plugin),
        Commands::Rename { old, new } => commands::rename::rename_project(old, new),
        Commands::ListProjects => commands::list_projects::list_projects(),
        Commands::ListPlugins => commands::list_plugins::list_plugins(),
        Commands::Plugin { command } => match command {
            PluginCommands::Create { name } => todo!(),
            PluginCommands::Migrate => todo!(),
            PluginCommands::Custom(args) => todo!(),
        },
    }
}
