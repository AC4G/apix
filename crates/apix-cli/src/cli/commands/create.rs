use std::{collections::HashMap, env, fs, path::Path};
use log::{error, info};
use serde::Deserialize;
use apix_core::{abi::PluginAbi, loader::LuaPlugin, utils::fs_utils, Db, PluginInfo, TemplatePaths};

use crate::internal_dir::get_internal_dir;

#[derive(Debug, Deserialize)]
struct MetaYaml {
    templates: HashMap<String, TemplatePaths>,
    default: String,
}

pub fn create_monorepo(
    name: String,
    template: Option<String>,
    plugin: Option<String>,
) {
    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            error!("Failed to get current folder: {}", e);
            return;
        }
    };
    let dst_path = current_dir.join(&name);

    if dst_path.exists() {
        error!("Folder '{}' already exists", name);
        return;
    }

    if let Err(e) = fs::create_dir_all(&dst_path) {
        error!("Failed to create monorepo folder '{}': {}", dst_path.display(), e);
        return;
    }

    let template = if let Some(template) = template.clone() {
        template
    } else {
        "default".to_string()
    };

    let plugin_info = if let Some(plugin) = plugin.clone() {
        let result = create_monorepo_from_plugin(&dst_path, &plugin, &template);

        match result {
            Ok(plugin_info) => Some(plugin_info),
            Err(e) => {
                error!("Failed to create monorepo from plugin: {}", e);
                fs::remove_dir(dst_path).expect("Failed to remove monorepo folder");
                return;
            }
        }
    } else {
        None
    };

    let template_paths = if plugin.is_none() {
        let result = create_monorepo_from_template(&dst_path, &template);

        match result {
            Ok(tp) => Some(tp),
            Err(e) => { 
                error!("Failed to create monorepo from template: {}", e);
                fs::remove_dir(dst_path).expect("Failed to remove monorepo folder");
                return;
            }
        }
    } else {
        None
    };

    let apix_dir = dst_path.join(".apix");
    let db_path = apix_dir.join("state.db");

    if let Err(e) = Db::create_db_and_migrate(
        Path::new(&db_path),
        &name,
        &plugin.clone(),
        &plugin_info,
        &template_paths,
        &template
    ) {
        error!("Failed to create and migrate DB: {}", e);
        return;
    };

    info!(
        "Successfully created monorepo '{}' with template '{}'{}",
        name, template, if plugin.is_some() {
            format!(" using plugin '{}'", plugin.unwrap())
        } else {
            String::new()
        }
    );
}

fn create_monorepo_from_plugin(
    dst_path: &Path,
    plugin_name: &String,
    template: &String
) -> Result<PluginInfo, Box<dyn std::error::Error>> {
    let plugins_dir = get_internal_dir()
        .get_plugins_dir()
        .clone();

    let plugin = LuaPlugin::load(plugin_name, &plugins_dir, dst_path.to_path_buf())?;
    let plugin_abi = PluginAbi::new(&plugin);

    let plugin_info = match plugin_abi.get_plugin_info() {
        Ok(info) => info,
        Err(e) => {
            return Err(Box::new(e));
        }
    };

    if !plugin_info.supports.contains(&"create_monorepo".to_string()) || plugin_info.monorepo_templates.is_none() {
        return Err("Plugin does not support 'create_monorepo' command".into());
    }

    let supported_templates = plugin_info.monorepo_templates.clone().unwrap();

    if supported_templates.get(template).is_none() {
        return Err(format!("Plugin '{}' does not support template '{}'", plugin_info.name, template).into());
    }

    let create_monorepo_result = plugin_abi.create_monorepo(
        dst_path.to_string_lossy().to_string(),
        template.clone()
    );

    if let Err(e) = create_monorepo_result {
        return Err(Box::new(e));
    }

    /*// get plugin info first and check if the function exists, if not, return error, if yes and info contains folder paths for packages and projects, execute

    let create_monorepo_func = match instance.exports.get_function("create_monorepo") {
        Ok(func) => func,
        Err(_) => {
            return Err("Plugin does not support 'create_monorepo' command".into());
        }
    };

    let result = create_monorepo_func.call(&mut store, &[Value::String(template.clone().unwrap_or(String::from("default")))]);
    
    if let Err(e) = result {
        return Err(Box::new(e));
    }*/

    Ok(plugin_info)
}

fn create_monorepo_from_template(
    dst_path: &Path,
    template: &String
) -> Result<TemplatePaths, Box<dyn std::error::Error>> {
    let internal_dir = get_internal_dir();

    let meta_content = internal_dir.read_file_string("templates", "meta.yml")
        .expect("Failed to read meta.yml");

    let meta: MetaYaml = serde_yaml::from_str(&meta_content).expect("Failed to parse meta.yml");

    if !meta.templates.contains_key(template) {
        return Err(format!("Template '{}' not found", template).into());
    }
    
    let template = if template == "default" {
        meta.default
    } else {
        template.to_string()
    };

    let template_paths = meta.templates
        .get(&template)
        .unwrap();

    let template_dir = internal_dir.get_templates_dir().join(&template);

    if !template_dir.exists() {
        return Err(format!(
            "Selected template '{}' folder not found in templates directory",
            template
        ).into());
    }

    if let Err(e) = fs_utils::copy_dir_recursive(&template_dir, &dst_path) {
        return Err(Box::new(e));
    }

    Ok(template_paths.clone())
}
