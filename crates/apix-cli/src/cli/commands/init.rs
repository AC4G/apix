use apix_core::{db::Db, utils};
use log::{error, info};
use serde::Deserialize;
use std::{env, fs, path::Path};

use crate::utils::internal_dir::get_internal_dir;

#[derive(Debug, Deserialize)]
struct MetaYaml {
    templates: Vec<String>,
    default: String,
}

pub fn create_monorepo(name: String, provided_template: Option<String>) {
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
        error!(
            "Failed to create monorepo folder '{}': {}",
            dst_path.display(),
            e
        );
        return;
    }

    let provided_template = if let Some(template) = provided_template.clone() {
        template
    } else {
        "default".to_string()
    };

    let result = create_monorepo_from_template(&dst_path, &provided_template);

    let template = match result {
        Ok(tp) => tp,
        Err(e) => {
            error!(
                "Failed to create monorepo from template '{}': {}",
                provided_template, e
            );
            fs::remove_dir(dst_path).expect("Failed to remove monorepo folder");
            return;
        }
    };

    let apix_dir = dst_path.join(".apix");
    let db_path = apix_dir.join("state.db");

    if let Err(e) = Db::create_db_and_migrate(Path::new(&db_path)) {
        error!("Failed to create and migrate DB: {}", e);
        return;
    };

    info!(
        "Successfully created monorepo '{}' with {}template '{}'",
        name,
        if provided_template == "default" {
            "default "
        } else {
            ""
        },
        template
    );
}

fn create_monorepo_from_template(
    dst_path: &Path,
    template: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    let internal_dir = get_internal_dir();

    let meta_content = internal_dir
        .read_file_string("templates", "meta.yml")
        .expect("Failed to read meta.yml");

    let meta: MetaYaml = serde_yaml::from_str(&meta_content).expect("Failed to parse meta.yml");

    let template = if template == "default" {
        meta.default
    } else {
        template.to_string()
    };

    if !meta.templates.contains(&template) {
        return Err(format!("Template '{}' not found", template).into());
    }

    let template_dir = internal_dir.get_templates_dir().join(&template);

    if !template_dir.exists() {
        return Err(format!(
            "Selected template '{}' folder not found in templates directory",
            template
        )
        .into());
    }

    if let Err(e) = utils::fs::copy_dir_recursive(&template_dir, &dst_path) {
        return Err(Box::new(e));
    }

    Ok(template)
}
