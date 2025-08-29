use mlua::Result as LuaResult;
use std::{cell::RefCell, path::PathBuf, rc::Rc};

use crate::plugin::{instance::PluginInstance, loader::LuaPlugin, plugin_ctx::ctx::PluginCtx};

pub fn load_plugin(
    name: &str,
    plugin_version: &str,
    monorepo_root: &PathBuf,
    plugins_dir: &PathBuf,
) -> LuaResult<(PluginInstance, Rc<RefCell<PluginCtx>>)> {
    let ctx = Rc::new(RefCell::new(PluginCtx::new(name)));

    let lua_plugin = LuaPlugin::load(
        name,
        &plugins_dir,
        plugin_version,
        monorepo_root,
        ctx.clone(),
    )?;
    let abi = PluginInstance::new(lua_plugin);

    Ok((abi, ctx))
}
