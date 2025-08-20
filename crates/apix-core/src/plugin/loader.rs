use log::info;
use mlua::{Lua, Result, UserData, Value, Variadic};
use std::{fs, path::{Path, PathBuf}};

use crate::bindings::LuaFile;

pub struct LuaPlugin {
    pub name: String,
    pub lua: Lua,
    pub allowed_dirs: Vec<PathBuf>,
}

pub struct LuaDir {
    pub path: PathBuf,
}

impl UserData for LuaDir {
    fn add_methods<'lua, M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("list", |lua, this, ()| {
            let table = lua.create_table()?;
            for (i, entry) in fs::read_dir(&this.path)?.enumerate() {
                let entry = entry?;
                let name = entry.file_name().into_string().unwrap_or_default();
                table.set(i + 1, name)?;
            }
            Ok(table)
        });

        methods.add_method("join", |_, this, name: String| {
            Ok(this.path.join(name).to_string_lossy().to_string())
        });

        methods.add_method("exists", |_, this, ()| {
            Ok(this.path.exists())
        });

        methods.add_method("ensure_exists", |_, this, ()| {
            if !this.path.exists() {
                fs::create_dir_all(&this.path)?;
            }
            Ok(())
        });
    }
}

impl LuaPlugin {
    pub fn load(name: &str, plugin_dir: &Path, target_dir: PathBuf) -> Result<Self> {
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
            .expect("Failed to canonicalize target dir");

        let canon_target_dir = fs::canonicalize(&target_dir).expect("Failed to canonicalize target dir");

        let plugin_io = lua.create_table()?;

        let print = lua.create_function(|_, args: Variadic<Value>| {
            let mut output = String::new();
            for arg in args {
                output.push_str(&format!("{:?}\t", arg));
            }
            info!("[plugin] {}", output.trim_end());
            Ok(())
        })?;

        let io_open = {
            let canon_target_dir = canon_target_dir.clone();
            let canon_plugin_data_dir = canon_plugin_data_dir.clone();
            lua.create_function(move |lua_ctx, path: String| {
                let path_obj = fs::canonicalize(&path).map_err(|e| mlua::Error::external(e))?;

                if path_obj.starts_with(&canon_target_dir) || path_obj.starts_with(&canon_plugin_data_dir) {
                    info!("[plugin_io] Opening file: {}", path_obj.display());

                    let file = std::fs::OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(&path_obj)
                        .map_err(|e| mlua::Error::external(e))?;

                    Ok(Value::UserData(lua_ctx.create_userdata(LuaFile(file))?))
                } else {
                    Err(mlua::Error::RuntimeError(format!("Access to {} denied", path)))
                }
            })?
        };

        plugin_io.set("open", io_open)?;

        let data_dir_userdata = lua.create_userdata(LuaDir { path: canon_plugin_data_dir.clone() })?;
        let target_dir_userdata = lua.create_userdata(LuaDir { path: canon_target_dir.clone() })?;

        plugin_io.set("data_dir", data_dir_userdata)?;
        plugin_io.set("target_dir_obj", target_dir_userdata)?;
        plugin_io.set("target_dir", canon_target_dir.to_string_lossy().to_string())?;
        plugin_io.set("plugin_data_dir", canon_plugin_data_dir.to_string_lossy().to_string())?;

        lua.globals().set("plugin_io", plugin_io)?;

        lua.globals().set("print", print)?;

        let plugin_code = fs::read_to_string(&plugin_path)?;
        lua.load(&plugin_code).exec()?;

        Ok(Self {
            name: name.to_string(),
            lua,
            allowed_dirs: vec![
                canon_plugin_data_dir,
                canon_target_dir
            ],
        })
    }
}
