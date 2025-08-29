use std::{cell::RefCell, rc::Rc};

use apix_core::plugin::{
    config::PluginConfig, instance::PluginInstance, plugin_ctx::ctx::PluginCtx,
};

use crate::cli::{
    PluginCommands,
    commands::plugin::{
        create::call_plugin_create, extend::call_plugin_extend, info::call_plugin_info,
        migrate::call_plugin_migrate,
    },
};

pub fn dispatch_plugin_command(
    plugin: String,
    plugin_config: PluginConfig,
    abi: PluginInstance,
    ctx: Rc<RefCell<PluginCtx>>,
    command: PluginCommands,
) {
    match command {
        PluginCommands::Create { name, flags } => call_plugin_create(flags, name, plugin, abi, ctx),
        PluginCommands::Extend { args, flags } => call_plugin_extend(flags, args, abi, ctx),
        PluginCommands::Migrate { flags } => call_plugin_migrate(flags, plugin_config, abi, ctx),
        PluginCommands::Info => call_plugin_info(plugin, plugin_config, abi),
    }
}
