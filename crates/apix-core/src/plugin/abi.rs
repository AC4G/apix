use log::error;
use mlua::{FromLuaMulti, Function, IntoLuaMulti, Result as LuaResult};
use std::collections::HashSet;
use crate::loader::LuaPlugin;

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

    pub fn create(&self, project_name: String) -> LuaResult<i32> {
        self.call_fn("create", project_name)
    }

    pub fn extend(&self, args: Vec<String>) -> LuaResult<i32> {
        self.call_fn("extend", args)
    }

    pub fn migrate(&self, from_version: String) -> LuaResult<i32> {
        self.call_fn("migrate", from_version)
    }

    pub fn help(&self) -> LuaResult<i32> {
        self.call_fn("help", ())
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
