mod cli;
mod db;
mod config;
mod internal_dir;

use env_logger::Env;
use crate::{cli::init_cli, internal_dir::init_internal_dir};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    init_internal_dir().expect("Failed to initialize internal directory");
    init_cli();
    
    Ok(())
}
