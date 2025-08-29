use std::{cell::RefCell, rc::Rc};

use apix_core::{
    monorepo::config::get_monorepo_config,
    plugin::{
        config::{PluginConfig, get_plugin_config},
        instance::PluginInstance,
        plugin_ctx::ctx::PluginCtx,
        utils::load_plugin,
    },
    utils::version::{VersionCheck, check_plugin_version},
};
use log::error;

use crate::utils::internal_dir::get_internal_dir;

pub fn resolve_plugin(plugin: &str) -> (PluginConfig, PluginInstance, Rc<RefCell<PluginCtx>>) {
    let monorepo_root = std::env::current_dir().unwrap();
    let monorepo_config = get_monorepo_config(&monorepo_root).unwrap_or_else(|e| {
        error!("Error reading monorepo config: {}", e);
        std::process::exit(1);
    });

    let required_version = monorepo_config
        .plugins
        .get(plugin)
        .map(|v| v.version())
        .unwrap_or_else(|| {
            error!("Plugin '{}' not registered in monorepo.toml", plugin);
            std::process::exit(1);
        });

    let binding = get_internal_dir();
    let plugins_dir = binding.get_plugins_dir();
    let (plugin_config, resolved_version) =
        get_plugin_config(&plugins_dir, plugin, &required_version).unwrap_or_else(|e| {
            error!("Error reading plugin config: {}", e);
            std::process::exit(1);
        });

    validate_plugin_versions(
        plugin,
        &plugin_config.version,
        &resolved_version,
        &required_version,
    );

    let (abi, ctx) = load_plugin(plugin, &plugin_config.version, &monorepo_root, &plugins_dir)
        .unwrap_or_else(|e| {
            error!("Failed to load plugin '{}': {}", plugin, e);
            std::process::exit(1);
        });

    (plugin_config, abi, ctx)
}

pub fn validate_plugin_versions(
    plugin: &str,
    config_version: &str,
    resolved_version: &str,
    required_version: &str,
) {
    match check_plugin_version(resolved_version, required_version) {
        Ok(VersionCheck::UpToDate) => {}
        Ok(VersionCheck::PluginNewer) => {
            error!(
                "Installed plugin '{}' (v{}) is newer than required '{}'",
                plugin, resolved_version, required_version
            );
            std::process::exit(1);
        }
        Ok(VersionCheck::PluginOutdated) => {
            error!(
                "Installed plugin '{}' (v{}) is older than required '{}'",
                plugin, resolved_version, required_version
            );
            std::process::exit(1);
        }
        Err(e) => {
            error!("Failed to compare versions for plugin '{}': {}", plugin, e);
            std::process::exit(1);
        }
    }

    if config_version != resolved_version {
        error!(
            "Plugin '{}' folder name version '{}' does not match plugin.toml '{}'",
            plugin, resolved_version, config_version
        );
        std::process::exit(1);
    }
}
