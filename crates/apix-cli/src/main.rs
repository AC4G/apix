mod cli;
mod config;
mod db;
mod logger;
mod plugin;
mod utils;

use crate::{cli::init_cli, logger::init_logger, utils::internal_dir::init_internal_dir};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();
    init_internal_dir().expect("Failed to initialize internal directory");
    init_cli();

    Ok(())
}
