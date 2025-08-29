use mlua::prelude::*;
use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;

use crate::plugin::plugin_ctx::ctx::PluginCtx;

pub fn register_ask_function(
    lua: &Lua,
    ctx: Rc<RefCell<PluginCtx>>,
    table: &LuaTable,
) -> LuaResult<()> {
    let ask_table = table.clone();
    let ask_ctx = ctx.clone();

    let ask_fn = lua.create_function(move |_, question: String| {
        let log_fn: LuaFunction = ask_table.get("info")?;
        log_fn.call::<()>(format!("> {}", question))?;

        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let answer = input.trim().to_string();
        ask_ctx.borrow_mut().logs.push(format!("[ask] {}", answer));

        Ok(answer)
    })?;

    table.set("ask", ask_fn)?;
    Ok(())
}
