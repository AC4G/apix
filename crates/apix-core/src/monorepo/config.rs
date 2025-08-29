use serde::Deserialize;
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct MonorepoConfig {
    pub repo: RepoConfig,
    pub projects: HashMap<String, ProjectConfig>,
    pub packages: HashMap<String, PackageConfig>,
    pub plugins: HashMap<String, PluginMeta>,
}

#[derive(Debug, Deserialize)]
pub struct RepoConfig {
    pub name: String,
    pub version: String,
    pub template: String,
}

#[derive(Debug, Deserialize)]
pub struct ProjectConfig {
    pub path: String,
    pub language: String,
    pub permissions: Vec<(String, Vec<String>)>,
}

#[derive(Debug, Deserialize)]
pub struct PackageConfig {
    pub path: String,
    pub language: String,
    pub permissions: Vec<(String, Vec<String>)>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PluginMeta {
    Simple(String),
    Detailed { version: String },
}

impl PluginMeta {
    pub fn version(&self) -> &str {
        match self {
            PluginMeta::Simple(v) => v,
            PluginMeta::Detailed { version } => version,
        }
    }
}

pub fn get_monorepo_config(
    monorepo_root: &PathBuf,
) -> Result<MonorepoConfig, Box<dyn std::error::Error>> {
    let monorepo_config_path = monorepo_root.join("monorepo.toml");

    let monorepo_config_str = fs::read_to_string(&monorepo_config_path)?;
    let monorepo_config: MonorepoConfig = toml::from_str(&monorepo_config_str)?;

    Ok(monorepo_config)
}
