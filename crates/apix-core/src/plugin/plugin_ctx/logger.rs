use ansi_term::Colour::{Blue, Green, Purple, Red, Yellow};
use chrono::Local;
use log::Level;
use mlua::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct PluginLogger {
    pub name: String,
    pub logs: Vec<String>,
}

impl PluginLogger {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            logs: Vec::new(),
        }
    }
}

pub fn register_logger_functions(
    lua: &Lua,
    logger: Rc<RefCell<PluginLogger>>,
    table: &LuaTable,
) -> LuaResult<()> {
    let log_ctx = logger.clone();
    let log_fn = lua.create_function(move |_, (level, msg): (String, LuaValue)| {
        let msg_str = match msg {
            LuaValue::String(s) => s.to_str()?.to_string(),
            _ => {
                return Err(LuaError::FromLuaConversionError {
                    from: msg.type_name(),
                    to: "String".into(),
                    message: Some("Expected a string for logging".to_string()),
                });
            }
        };

        let level_enum = match level.to_lowercase().as_str() {
            "error" => Level::Error,
            "warn" | "warning" => Level::Warn,
            "info" => Level::Info,
            "debug" => Level::Debug,
            "trace" => Level::Trace,
            _ => Level::Info,
        };

        let level_colored = match level_enum {
            Level::Error => Red.bold().paint(format!("{}", level_enum)),
            Level::Warn => Yellow.bold().paint(format!("{}", level_enum)),
            Level::Info => Green.paint(format!("{}", level_enum)),
            Level::Debug => Blue.paint(format!("{}", level_enum)),
            Level::Trace => Purple.paint(format!("{}", level_enum)),
        };

        let mut log_ctx = log_ctx.borrow_mut();

        let ts = Local::now().format("%Y-%m-%dT%H:%M:%S");
        let prefix = format!("[{}]", log_ctx.name);
        let formatted = format!("{} [{}] [{}] {}", prefix, ts, level_colored, msg_str);

        println!("{}", formatted);
        log_ctx.logs.push(formatted);

        Ok(())
    })?;
    table.set("log", log_fn)?;

    let levels = vec!["error", "warn", "info", "debug", "trace"];
    for &lvl in &levels {
        let table_clone = table.clone();
        let lvl_string = lvl.to_string();
        let func = lua.create_function(move |_, msg: LuaValue| {
            let log_fn: LuaFunction = table_clone.get("log")?;
            log_fn.call::<()>((lvl_string.clone(), msg))?;
            Ok(())
        })?;
        table.set(lvl, func)?;
    }

    Ok(())
}
