use mlua::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::plugin::plugin_ctx::ctx::{PluginCtx, Proposal};

pub fn register_file_functions(
    lua: &Lua,
    ctx: Rc<RefCell<PluginCtx>>,
    table: &LuaTable,
) -> LuaResult<()> {
    let create_ctx = ctx.clone();
    let create_fn = lua.create_function(move |_, (path, content): (String, String)| {
        create_ctx
            .borrow_mut()
            .proposals
            .push(Proposal::CreateFile { path, content });
        Ok(())
    })?;
    table.set("create_file", create_fn)?;

    let modify_ctx = ctx.clone();
    let modify_fn = lua.create_function(move |_, (path, content): (String, String)| {
        modify_ctx
            .borrow_mut()
            .proposals
            .push(Proposal::ModifyFile { path, content });
        Ok(())
    })?;
    table.set("modify_file", modify_fn)?;

    let delete_ctx = ctx.clone();
    let delete_fn = lua.create_function(move |_, path: String| {
        delete_ctx
            .borrow_mut()
            .proposals
            .push(Proposal::DeleteFile { path });
        Ok(())
    })?;
    table.set("delete_file", delete_fn)?;

    Ok(())
}
