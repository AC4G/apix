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
    Init {
        name: String,

        #[arg(short, long)]
        template: Option<String>,
    },
    Install {
        plugin_name: String,
        version: Option<String>,
    },
    Update {
        plugin_name: Option<String>,
        version: Option<String>,
    },
    Plugin {
        plugin: String,
        #[command(subcommand)]
        command: PluginCommands,
    },
}

#[derive(Subcommand)]
enum PluginCommands {
    Create {
        name: String,
    },
    Extend {
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    Migrate,
    Help
}

pub fn init_cli() {
    let cli = CLI::parse();

    if !matches!(cli.command, Commands::Init { .. }) {
        if let Err(_e) = connect_db(&get_config().db_path) {
            error!("Failed to connect to the DB. It may not be initialized in the current directory.");
            std::process::exit(1);
        }
    }

    match cli.command {
    Commands::Init { name, template } => {
        commands::init::create_monorepo(name, template)
    }

    Commands::Install { plugin_name, version } => {
        // install plugin if not already installed
    }

    Commands::Update { plugin_name, version } => {
        // update installed plugins to the latest version or one to the latest or specified version
    }

    Commands::Plugin { plugin, command } => match command {
        PluginCommands::Create { name } => {
            // 1. Load monorepo.toml
            // 2. Check plugin version & permissions
            // 3. Call Lua plugin `create` with project name
        }

        PluginCommands::Extend { args } => {
            // 1. Load monorepo.toml
            // 2. Check plugin permissions
            // 3. Call Lua plugin `extend` with args
        }

        PluginCommands::Migrate => {
            // 1. Load monorepo.toml
            // 2. Check plugin permissions
            // 3. Call Lua plugin `migrate`
        }

        PluginCommands::Help => {
            // 1. Load plugin
            // 2. Call Lua plugin’s `help` function
            // 3. Print output
        }
    },
}
}
