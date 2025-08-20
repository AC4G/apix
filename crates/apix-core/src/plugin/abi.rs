use log::error;
use mlua::{FromLuaMulti, Function, IntoLuaMulti, Result as LuaResult, Table};
use std::collections::HashSet;
use crate::{loader::LuaPlugin, PluginInfo};

pub struct PluginAbi<'lua> {
    lua_plugin: &'lua LuaPlugin,
}

impl<'lua> PluginAbi<'lua> {
    pub fn new(lua_plugin: &'lua LuaPlugin) -> Self {
        Self { lua_plugin }
    }

    fn call_fn<T>(&self, fn_name: &str, args: impl IntoLuaMulti) -> LuaResult<T>
    where
        T: FromLuaMulti,
    {
        let globals = self.lua_plugin.lua.globals();
        
        if !globals.contains_key(fn_name)? {
            error!("Plugin does not implement '{}'", fn_name);
            return Err(mlua::Error::RuntimeError(format!(
                "Function '{}' not found in plugin", fn_name
            )));
        }

        let func: Function = globals.get(fn_name)?;
        func.call::<T>(args)
    }

    pub fn get_plugin_info(&self) -> LuaResult<PluginInfo> {
        self.call_fn("get_plugin_info", ())
    }

    pub fn migrate(&self, from_major: u32, from_minor: u32, from_patch: u32) -> LuaResult<i32> {
        self.call_fn("migrate", (from_major, from_minor, from_patch))
    }

    pub fn help(&self) -> LuaResult<String> {
        self.call_fn("help", ())
    }

    pub fn create_project(&self, name: String, temp_path: String) -> LuaResult<i32> {
        self.call_fn("create_project", (name, temp_path))
    }

    pub fn create_package(&self, name: String, temp_path: String) -> LuaResult<i32> {
        self.call_fn("create_package", (name, temp_path))
    }

    pub fn create_monorepo(&self, dst_path: String, template: String) -> LuaResult<i32> {
        self.call_fn("create_monorepo", (dst_path, template))
    }

    pub fn custom_command(&self, command: String, args: Vec<String>) -> LuaResult<i32> {
        self.call_fn("custom_command", (command, args))
    }
}

pub fn has_command(available_exports: &HashSet<String>, command_name: &str) -> bool {
    available_exports.contains(command_name)
}

pub fn validate_required_exports(available_exports: &HashSet<String>, plugin_name: &str) -> Result<(), String> {
    let required = ["get_plugin_info", "migrate", "help"];
    for &cmd in &required {
        if !has_command(available_exports, cmd) {
            return Err(format!("Plugin '{}' is missing required command '{}'", plugin_name, cmd));
        }
    }
    Ok(())
}
