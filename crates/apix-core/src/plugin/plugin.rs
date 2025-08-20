use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatePaths {
    pub projects: String,
    pub packages: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub version: PluginVersion,
    pub monorepo_templates: Option<std::collections::HashMap<String, TemplatePaths>>,
    pub supports: Vec<String>,
}
