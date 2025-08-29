use apix_core::plugin::{config::PluginConfig, instance::PluginInstance};
use log::{error, info};

pub fn call_plugin_info(plugin: String, plugin_config: PluginConfig, abi: PluginInstance) {
    match abi.info() {
        Ok(Some(info)) => {
            println!(
                "\n{}",
                info.format(&plugin, &plugin_config.version, &plugin_config.description)
            );
        }
        Ok(None) => info!("Plugin '{}' did not return any info", plugin),
        Err(e) => {
            error!("Plugin '{}' failed to provide info: {}", plugin, e);
            std::process::exit(1);
        }
    }
}
