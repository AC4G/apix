use semver::Version;
use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct PluginConfig {
    pub name: String,
    pub version: String,
    pub description: String,
    pub supported: Supported,
}

#[derive(Debug, Deserialize)]
pub struct Supported {
    pub actions: Vec<String>,
    pub languages: Vec<String>,
    pub features: Vec<String>,
}

/// Returns (PluginConfig, resolved_version)
pub fn get_plugin_config(
    plugins_dir: &PathBuf,
    plugin_name: &str,
    requested_version: &str,
) -> Result<(PluginConfig, String), Box<dyn std::error::Error>> {
    let plugin_dir = plugins_dir.join(plugin_name);

    let mut installed_versions: Vec<Version> = fs::read_dir(&plugin_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.path().is_dir() {
                entry.file_name().to_str()?.parse::<Version>().ok()
            } else {
                None
            }
        })
        .collect();

    if installed_versions.is_empty() {
        return Err(format!("No installed versions found for plugin '{}'", plugin_name).into());
    }

    installed_versions.sort();

    let resolved_version = if requested_version == "*" {
        installed_versions.last().unwrap().to_string()
    } else {
        let req_ver = Version::parse(requested_version)
            .map_err(|_| format!("Invalid version string '{}'", requested_version))?;

        if installed_versions.contains(&req_ver) {
            req_ver.to_string()
        } else {
            // pick next higher, if none, pick next lower
            if let Some(next_highest) = installed_versions.iter().find(|v| **v > req_ver) {
                next_highest.to_string()
            } else {
                installed_versions
                    .iter()
                    .rev()
                    .find(|v| **v < req_ver)
                    .unwrap()
                    .to_string()
            }
        }
    };

    let plugin_config_path = plugin_dir.join(&resolved_version).join("plugin.toml");
    if !plugin_config_path.exists() {
        return Err(format!(
            "Plugin '{}' version '{}' not found in {:?}",
            plugin_name, resolved_version, plugin_config_path
        )
        .into());
    }

    let plugin_config_str = fs::read_to_string(&plugin_config_path)?;
    let plugin_config: PluginConfig = toml::from_str(&plugin_config_str)?;

    Ok((plugin_config, resolved_version))
}
