use mlua::{Function as LuaFunction, Lua, Result as LuaResult, Table as LuaTable};
#[cfg(unix)]
use std::process::Command;
use std::{cell::RefCell, rc::Rc};

use crate::plugin::plugin_ctx::ctx::{PluginCtx, Proposal};

pub fn register_system_functions(
    lua: &Lua,
    ctx: Rc<RefCell<PluginCtx>>,
    table: &LuaTable,
) -> LuaResult<()> {
    let sys_ctx = ctx.clone();
    let table_clone = table.clone();

    let sys_fn =
        lua.create_function(move |_, (command, args): (String, Option<Vec<String>>)| {
            let args = args.unwrap_or_default();

            sys_ctx
                .borrow_mut()
                .proposals
                .push(Proposal::SystemCommand {
                    command: command.clone(),
                    args: args.clone(),
                });

            let log_fn: LuaFunction = table_clone.get("info")?;
            log_fn.call::<()>(format!("Proposed system command: {} {:?}", command, args))?;

            Ok(())
        })?;

    table.set("system", sys_fn)?;

    let exists_fn = lua.create_function(|_, command: String| {
        #[cfg(unix)]
        let status = Command::new("which").arg(&command).status();

        #[cfg(windows)]
        let status = Command::new("where").arg(&command).status();

        let exists = status.map(|s| s.success()).unwrap_or(false);

        Ok(exists)
    })?;
    table.set("system_exists", exists_fn)?;

    let version_fn = lua.create_function(|_, (command, flag): (String, Option<String>)| {
        let flag = flag.unwrap_or_else(|| "--version".to_string());

        let output = Command::new(&command).arg(&flag).output();

        match output {
            Ok(out) if out.status.success() => {
                let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
                Ok(stdout)
            }
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
                Ok(format!("(error) {}", stderr))
            }
            Err(e) => Ok(format!("(failed to run '{} {}': {})", command, flag, e)),
        }
    })?;
    table.set("system_version", version_fn)?;

    Ok(())
}
