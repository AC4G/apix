use mlua::{Lua, Result};
use std::{fs, path::{Path, PathBuf}, rc::Rc, cell::RefCell};

use crate::ctx::PluginCtx;
use crate::file_tree::LuaDir;

pub struct LuaPlugin {
    pub name: String,
    pub lua: Lua,
}

impl LuaPlugin {
    pub fn load(
        name: &str,
        plugin_dir: &Path,
        monorepo_root: PathBuf,
        ctx: Rc<RefCell<PluginCtx>>,
    ) -> Result<Self> {
        let lua_path = plugin_dir.join(format!("{}/{}.lua", name, name));
        let luau_path = plugin_dir.join(format!("{}/{}.luau", name, name));

        let plugin_path = if lua_path.exists() {
            lua_path
        } else if luau_path.exists() {
            luau_path
        } else {
            return Err(mlua::Error::RuntimeError(format!(
                "Plugin {} does not exist at either {:?} or {:?}",
                name, lua_path, luau_path
            )));
        };

        let lua = Lua::new();

        let data_dir = plugin_dir.join(name).join("data");
        if !data_dir.exists() {
            fs::create_dir_all(&data_dir)?;
        }

        let canon_plugin_data_dir = fs::canonicalize(&data_dir)
            .expect("Failed to canonicalize plugin data dir");
        let canon_monorepo_root = fs::canonicalize(&monorepo_root)
            .expect("Failed to canonicalize monorepo root");

        let root_dir = LuaDir::scan(&canon_monorepo_root)?;
        let root_dir_userdata = lua.create_userdata(root_dir)?;
        let data_dir_userdata = lua.create_userdata(LuaDir::scan(&canon_plugin_data_dir)?)?;

        let lua_ctx_table = PluginCtx::register(&lua, ctx.clone())?;

        let globals = lua.globals();
        globals.set("plugin_ctx", lua_ctx_table)?;
        globals.set("root_dir", root_dir_userdata)?;
        globals.set("plugin_data_dir", data_dir_userdata)?;

        let plugin_code = fs::read_to_string(&plugin_path)?;
        lua.load(&plugin_code).exec()?;

        Ok(Self {
            name: name.to_string(),
            lua,
        })
    }
}
