use mlua::{FromLua, Lua, Table, Value, Result as LuaResult};
use crate::plugin::{PluginInfo, PluginVersion, TemplatePaths};
use std::collections::HashMap;

impl FromLua for TemplatePaths {
    fn from_lua(value: Value, _lua: &Lua) -> LuaResult<Self> {
        let table = match value {
            Value::Table(t) => t,
            _ => return Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "TemplatePaths".to_string(),
                message: Some("expected a table".to_string()),
            }),
        };

        let projects: String = table.get("projects")?;
        let packages: String = table.get("packages")?;

        Ok(TemplatePaths { projects, packages })
    }
}

impl FromLua for PluginVersion {
    fn from_lua(value: Value, _lua: &Lua) -> LuaResult<Self> {
        let table = match value {
            Value::Table(t) => t,
            _ => return Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "PluginVersion".to_string(),
                message: Some("expected a table".to_string()),
            }),
        };

        let major: u32 = table.get("major")?;
        let minor: u32 = table.get("minor")?;
        let patch: u32 = table.get("patch")?;

        Ok(PluginVersion { major, minor, patch })
    }
}

impl FromLua for PluginInfo {
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        let table = match value {
            Value::Table(t) => t,
            _ => return Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "PluginInfo".to_string(),
                message: Some("expected a table".to_string()),
            }),
        };

        let name: String = table.get("name")?;
        let version_table: Table = table.get("version")?;
        let version = PluginVersion::from_lua(Value::Table(version_table), lua)?;

        let monorepo_templates: Option<HashMap<String, TemplatePaths>> =
            table.get("monorepo_templates").ok();

        let supports: Vec<String> = table.get("supports")?;

        Ok(PluginInfo {
            name,
            version,
            monorepo_templates,
            supports,
        })
    }
}
