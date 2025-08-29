use std::{cell::RefCell, rc::Rc};

use apix_core::plugin::{
    config::PluginConfig, instance::PluginInstance, plugin_ctx::ctx::PluginCtx,
};

use crate::{cli::cli::CommonFlags, utils::git::ensure_clean_tree};

pub fn call_plugin_migrate(
    CommonFlags {
        allow_dirty,
        accept_all,
    }: CommonFlags,
    plugin_config: PluginConfig,
    abi: PluginInstance,
    ctx: Rc<RefCell<PluginCtx>>,
) {
    ensure_clean_tree(allow_dirty);

    // get version of plugin from plugin.toml
    // and used version from monorepo.toml
    // check the difference
    // if they are different and the monorepo.toml version is smaller than the plugin
    // run it

    let res = abi.migrate(plugin_config.version).unwrap();

    let proposals = &ctx.borrow().proposals;

    // create project changes in the same visuals like git with git status

    // if accept_changes is true, execute those proposals
    // else ask first

    // on decline, end here

    // update version in monorepo.toml
}
