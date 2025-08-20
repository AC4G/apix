use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Config {
    pub db_path: String,
}

impl Config {
    fn new() -> Self {
        Config {
            db_path: ".apix/state.db".into(),
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

pub fn get_config() -> &'static Config {
    &CONFIG
}
