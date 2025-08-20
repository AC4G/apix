use mlua::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::{self, Write};

#[derive(Debug)]
pub enum Proposal {
    CreateFile { path: String, content: String },
    ModifyFile { path: String, content: String },
    DeleteFile { path: String },
}

#[derive(Debug)]
pub struct PluginCtx {
    pub logs: Vec<String>,
    pub proposals: Vec<Proposal>,
}

impl PluginCtx {
    pub fn new() -> Self {
        Self {
            logs: Vec::new(),
            proposals: Vec::new(),
        }
    }

    pub fn register(lua: &Lua, ctx: Rc<RefCell<Self>>) -> LuaResult<LuaTable> {
        let table = lua.create_table()?;

        let log_ctx = ctx.clone();
        let log_fn = lua.create_function(move |_, msg: LuaValue| {
            let msg_str = match msg {
                LuaValue::String(s) => s.to_str()?.to_string(),
                _ => return Err(LuaError::FromLuaConversionError {
                    from: msg.type_name(),
                    to: "String".into(),
                    message: Some("Expected a string for logging".to_string()),
                }),
            };
            println!("[plugin] {}", msg_str);
            log_ctx.borrow_mut().logs.push(msg_str);
            Ok(())
        })?;
        table.set("log", log_fn)?;

        let ask_ctx = ctx.clone();
        let ask_fn = lua.create_function(move |_, question: String| {
            print!("[plugin] {}: ", question);
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let answer = input.trim().to_string();
            ask_ctx.borrow_mut().logs.push(format!("[ask] {}", answer));
            Ok(answer)
        })?;
        table.set("ask", ask_fn)?;

        let create_ctx = ctx.clone();
        let create_fn = lua.create_function(move |_, (path, content): (String, String)| {
            create_ctx.borrow_mut().proposals.push(Proposal::CreateFile { path, content });
            Ok(())
        })?;
        table.set("create_file", create_fn)?;

        let modify_ctx = ctx.clone();
        let modify_fn = lua.create_function(move |_, (path, content): (String, String)| {
            modify_ctx.borrow_mut().proposals.push(Proposal::ModifyFile { path, content });
            Ok(())
        })?;
        table.set("modify_file", modify_fn)?;

        let delete_ctx = ctx.clone();
        let delete_fn = lua.create_function(move |_, path: String| {
            delete_ctx.borrow_mut().proposals.push(Proposal::DeleteFile { path });
            Ok(())
        })?;
        table.set("delete_file", delete_fn)?;

        Ok(table)
    }
}
