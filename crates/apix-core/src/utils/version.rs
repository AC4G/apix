use semver::Version;

pub enum VersionCheck {
    UpToDate,
    PluginNewer,
    PluginOutdated,
}

pub fn check_plugin_version(
    plugin_version: &str,
    monorepo_plugin_version: &str,
) -> Result<VersionCheck, String> {
    if plugin_version == "*" || monorepo_plugin_version == "*" {
        return Ok(VersionCheck::UpToDate);
    }

    let plugin_ver =
        Version::parse(plugin_version).map_err(|e| format!("Invalid plugin version: {e}"))?;
    let monorepo_ver = Version::parse(monorepo_plugin_version)
        .map_err(|e| format!("Invalid monorepo plugin version: {e}"))?;

    if plugin_ver < monorepo_ver {
        Ok(VersionCheck::PluginOutdated)
    } else if plugin_ver > monorepo_ver {
        Ok(VersionCheck::PluginNewer)
    } else {
        Ok(VersionCheck::UpToDate)
    }
}
