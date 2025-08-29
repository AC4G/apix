use mlua::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::plugin::plugin_ctx::logger::PluginLogger;
use crate::plugin::plugin_ctx::{ask, files, logger, system};

#[derive(Debug)]
pub enum Proposal {
    CreateFile { path: String, content: String },
    ModifyFile { path: String, content: String },
    DeleteFile { path: String },
    SystemCommand { command: String, args: Vec<String> },
}

#[derive(Debug)]
pub struct PluginCtx {
    pub logs: Vec<String>,
    pub proposals: Vec<Proposal>,
    pub logger: Rc<RefCell<PluginLogger>>,
}

impl PluginCtx {
    pub fn new(plugin_name: &str) -> Self {
        Self {
            logs: Vec::new(),
            proposals: Vec::new(),
            logger: Rc::new(RefCell::new(PluginLogger::new(plugin_name))),
        }
    }

    pub fn register(lua: &Lua, ctx: Rc<RefCell<Self>>) -> LuaResult<LuaTable> {
        let table = lua.create_table()?;

        logger::register_logger_functions(lua, ctx.borrow().logger.clone(), &table)?;
        ask::register_ask_function(lua, ctx.clone(), &table)?;
        files::register_file_functions(lua, ctx.clone(), &table)?;
        system::register_system_functions(lua, ctx.clone(), &table)?;

        Ok(table)
    }
}
