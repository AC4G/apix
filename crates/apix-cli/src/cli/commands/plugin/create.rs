use std::{cell::RefCell, rc::Rc};

use apix_core::plugin::{instance::PluginInstance, plugin_ctx::ctx::PluginCtx};

use crate::{cli::cli::CommonFlags, utils::git::ensure_clean_tree};

pub fn call_plugin_create(
    CommonFlags {
        allow_dirty,
        accept_all,
    }: CommonFlags,
    name: String,
    plugin: String,
    abi: PluginInstance,
    ctx: Rc<RefCell<PluginCtx>>,
) {
    ensure_clean_tree(allow_dirty);

    let res = abi.create(name).unwrap();

    let proposals = &ctx.borrow().proposals;

    // validate proposals

    // write to tmp folder

    // if error happens, rollback by deleting tmp folder

    // if no errors happen, write  the same

    // create project changes in the same visuals like git with git status

    // if accept_changes is true, execute those proposals
    // else ask first

    // on decline, end here
}
