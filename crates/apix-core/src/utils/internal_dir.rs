use std::{env, fs, path::PathBuf};

use crate::fs_utils::ensure_dir_and_copy_files;

#[derive(Debug)]
pub struct InternalDir {
    templates_dir: PathBuf,
    migrations_dir: PathBuf,
    plugins_dir: PathBuf
}

impl InternalDir {
    pub fn init_internal_dir() -> Result<Self, Box<dyn std::error::Error>> {
        let home_dir = PathBuf::from(env::var("HOME")?);
        let apix_dir = home_dir.join(".apix");

        if !apix_dir.exists() {
            fs::create_dir_all(&apix_dir).expect("Failed to create .apix directory");
        }

        let templates_dir = apix_dir.join("templates");
        let migrations_dir = apix_dir.join("migrations");
        let plugins_dir = apix_dir.join("plugins");

        let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let templates_src = crate_root.join("templates");
        let migrations_src = crate_root.join("migrations");

        ensure_dir_and_copy_files(&templates_dir, &templates_src).expect("Failed to copy templates");
        ensure_dir_and_copy_files(&migrations_dir, &migrations_src).expect("Failed to copy migrations");

        if !plugins_dir.exists() {
            fs::create_dir_all(&plugins_dir).expect("Failed to create plugins directory");
        }

        Ok(Self {
            templates_dir,
            migrations_dir,
            plugins_dir,
        })
    }

    pub fn read_file_string(
        &self,
        folder: &str,
        file_name: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let base_path = match folder {
            "templates" => &self.templates_dir,
            "migrations" => &self.migrations_dir,
            "plugins" => &self.plugins_dir,
            _ => return Err(format!("Unknown folder: {}", folder).into()),
        };

        let file_path = base_path.join(file_name);

        if !file_path.exists() {
            return Err(format!("File {:?} does not exist", file_path).into());
        }

        Ok(fs::read_to_string(file_path)?)
    }

    pub fn read_file_bytes(
        &self,
        folder: &str,
        file_name: &str,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let base_path = match folder {
            "templates" => &self.templates_dir,
            "migrations" => &self.migrations_dir,
            "plugins" => &self.plugins_dir,
            _ => return Err(format!("Unknown folder: {}", folder).into()),
        };

        let file_path = base_path.join(file_name);

        if !file_path.exists() {
            return Err(format!("File {:?} does not exist", file_path).into());
        }

        Ok(fs::read(file_path)?)
    }

    pub fn get_migrations_dir(&self) -> &PathBuf {
        &self.migrations_dir
    }

    pub fn get_templates_dir(&self) -> &PathBuf {
        &self.templates_dir
    }

    pub fn get_plugins_dir(&self) -> &PathBuf {
        &self.plugins_dir
    }
}
