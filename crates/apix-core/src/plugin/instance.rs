use mlua::prelude::LuaTable;
use mlua::{
    Error as LuaError, FromLuaMulti, Function, IntoLuaMulti, Result as LuaResult, Value as LuaValue,
};

use crate::plugin::loader::LuaPlugin;
use crate::plugin::plugin_ctx::info::PluginInfo;

pub struct PluginInstance {
    lua_plugin: LuaPlugin,
}

impl PluginInstance {
    pub fn new(lua_plugin: LuaPlugin) -> Self {
        Self { lua_plugin }
    }

    fn call_fn<T>(&self, fn_name: &str, args: impl IntoLuaMulti) -> LuaResult<T>
    where
        T: FromLuaMulti,
    {
        let globals = self.lua_plugin.lua.globals();

        if !globals.contains_key(fn_name)? {
            return Err(mlua::Error::RuntimeError(format!(
                "Function '{}' not found in plugin",
                fn_name
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

    pub fn info(&self) -> Result<Option<PluginInfo>, LuaError> {
        let table: Option<LuaTable> = match self.call_fn("info", ())? {
            LuaValue::Table(t) => Some(t),
            LuaValue::Nil => None,
            other => {
                return Err(LuaError::FromLuaConversionError {
                    from: other.type_name(),
                    to: "table".to_string(),
                    message: Some("Expected a table or nil from info".to_string()),
                });
            }
        };

        match table {
            Some(tbl) => {
                let usage: Vec<String> = tbl.get("usage")?;
                let options_table: LuaTable = tbl.get("options")?;
                let mut options = Vec::new();
                for pair in options_table.sequence_values::<LuaTable>() {
                    let pair_table = pair?;
                    let opt_name: String = pair_table.get(1)?;
                    let opt_desc: String = pair_table.get(2)?;
                    options.push((opt_name, opt_desc));
                }

                Ok(Some(PluginInfo { usage, options }))
            }
            None => Ok(None),
        }
    }
}
