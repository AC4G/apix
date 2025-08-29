use clap::{Args, Parser, Subcommand, command};
use log::error;

use crate::{
    cli::{
        commands::{init::create_monorepo, install::install_plugin, update::update_plugin},
        prelude::pre_command_checks,
    },
    plugin::{dispatcher::dispatch_plugin_command, helpers::resolve_plugin},
};

#[derive(Parser)]
#[command(name = "apix")]
#[command(about = "A project scaffolding & automation CLI", long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init {
        name: String,

        #[arg(short, long)]
        template: Option<String>,
    },
    Install {
        plugin: Option<String>,
        version: Option<String>,
    },
    Update {
        plugin: Option<String>,
        version: Option<String>,
    },
    Plugin {
        plugin: String,
        #[command(subcommand)]
        command: PluginCommands,
    },
}

#[derive(Args)]
pub struct CommonFlags {
    #[arg(
        short = 'y',
        long = "yes",
        help = "Automatically accept all non-critical plugin prompts"
    )]
    pub accept_all: bool,

    #[arg(long = "allow-dirty", help = "Allow running with a dirty working tree")]
    pub allow_dirty: bool,
}

#[derive(Subcommand)]
pub enum PluginCommands {
    Create {
        name: String,
        #[command(flatten)]
        flags: CommonFlags,
    },
    Extend {
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
        #[command(flatten)]
        flags: CommonFlags,
    },
    Migrate {
        #[command(flatten)]
        flags: CommonFlags,
    },
    Info,
}

pub fn init_cli() {
    let cli = CLI::parse();

    if let Err(e) = pre_command_checks(&cli.command) {
        error!("{}", e);
        std::process::exit(1);
    }

    match cli.command {
        Commands::Init { name, template } => create_monorepo(name, template),
        Commands::Install { plugin, version } => install_plugin(plugin, version),
        Commands::Update { plugin, version } => update_plugin(plugin, version),
        Commands::Plugin { plugin, command } => {
            let (plugin_config, abi, ctx) = resolve_plugin(&plugin);
            dispatch_plugin_command(plugin, plugin_config, abi, ctx, command);
        }
    }
}
