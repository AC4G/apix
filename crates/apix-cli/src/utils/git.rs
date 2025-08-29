use std::process::Command as ProcCommand;

use log::error;

pub fn ensure_clean_tree(allow_dirty: bool) {
    if allow_dirty {
        return;
    }

    let is_clean = ProcCommand::new("git")
        .args(["diff", "--quiet", "--exit-code"])
        .status()
        .map(|s| s.success())
        .unwrap_or(true)
        && ProcCommand::new("git")
            .args(["diff", "--cached", "--quiet", "--exit-code"])
            .status()
            .map(|s| s.success())
            .unwrap_or(true);

    if !is_clean {
        error!(
            "[apix] Error: working tree not clean. Commit or stash changes first, or use --allow-dirty."
        );
        std::process::exit(1);
    }
}
