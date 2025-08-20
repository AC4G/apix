use libsql::{Builder, Connection, Value};
use smol::{fs, stream::StreamExt};
use std::{error::Error, path::{Path, PathBuf}};

use crate::{plugin::PluginInfo, TemplatePaths};

pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn new(db_path: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        smol::block_on(async {
            if fs::metadata(db_path).await.is_err() {
                return Err("Database does not exist".into());
            }

            let db = Builder::new_local(db_path).build().await.expect("Failed to connect to DB");
            let conn = db.connect()?;

            Ok(Self { conn })
        })
    }

    pub fn create_db_and_migrate(
        db_path: &Path,
        monorepo_name: &str,
        plugin: &Option<String>,
        plugin_info: &Option<PluginInfo>,
        template_paths: &Option<TemplatePaths>,
        template: &String
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        smol::block_on(async {
            let apix_dir = db_path.parent().ok_or("Invalid DB path: no parent directory").unwrap();
            if fs::metadata(&apix_dir).await.is_err() {
                fs::create_dir_all(&apix_dir).await.unwrap();
            }

            let db = Builder::new_local(db_path).build().await.expect("Failed to connect to DB");
            let conn = db.connect().unwrap();

            let mut migrations_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            migrations_path.push("migrations");

            let mut entries = fs::read_dir(&migrations_path).await.unwrap();
            let mut files = Vec::new();

            while let Some(entry) = entries.next().await {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "sql") {
                    files.push(path);
                }
            }

            files.sort_by_key(|p| p.file_name().map(|n| n.to_os_string()));

            for file_path in files {
                let sql = fs::read_to_string(&file_path).await.unwrap();
                conn.execute_batch(&sql).await.unwrap();
            }

            let db = Self { conn: conn.clone() };

            let plugin_id = if let Some(plugin_name) = plugin {
                let plugin_info = plugin_info.clone().unwrap();
                let version = plugin_info.version;

                let plugin_id = Self::insert_plugin(&db, &plugin_name, &version.major, &version.minor, &version.patch).unwrap();

                Some(plugin_id)
            } else {
                None
            };

            let plugin_id_value = plugin_id.map_or(libsql::Value::Null, |id| libsql::Value::Integer(id as i64));

            let (projects, packages) = if let Some(template_paths) = template_paths {
                (template_paths.projects.clone(), template_paths.packages.clone())
            } else {
                let plugin_info = plugin_info.as_ref().unwrap();
                let templates = plugin_info.monorepo_templates.as_ref().unwrap();
                let template_paths = templates.get(&template.clone()).unwrap();

                (template_paths.projects.clone(), template_paths.packages.clone())
            };

            conn.execute(
                "INSERT INTO monorepo (name, plugin, projects, packages) VALUES (?1, ?2, ?3, ?4);",
                libsql::params![
                    monorepo_name,
                    plugin_id_value.clone(),
                    projects,
                    packages
                ],
            ).await.expect("Failed to store monorepo metadata");

            let _ = Self::insert_event(&db, plugin_id_value, None, None, "create_monorepo", &format!("--template {}", template));
        });

        Ok(())
    }

    pub fn insert_plugin(
        &self,
        name: &str,
        major: &u32,
        minor: &u32,
        patch: &u32
    ) -> Result<i64, Box<dyn Error + Send + Sync>> {
        smol::block_on(async {
            let mut rows = self.conn.query(
                "INSERT INTO plugins (name, major, minor, patch) VALUES (?1, ?2, ?3, ?4) RETURNING id;",
                libsql::params![name, major, minor, patch],
            ).await?;

            if let Some(row) = rows.next().await? {
                let id: i64 = row.get(0)?;
                Ok(id)
            } else {
                panic!("No id returned from insert");
            }
        })
    }

    pub fn event_exists(
        &self,
        plugin: &u64,
        project: Option<String>,
        package: Option<String>,
        action: &str,
        args: &str
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        smol::block_on(async {
            let mut rows = self.conn.query(
                "SELECT COUNT(*) as cnt FROM events WHERE plugin=?1 AND project=?2 AND package=?3 AND action=?4 AND args=?5;",
                libsql::params![plugin, project, package, action, args],
            ).await?;

            if let Some(row) = rows.next().await? {
                let count: i64 = row.get(0)?;
                Ok(count > 0)
            } else {
                Ok(false)
            }
        })
    }

    pub fn insert_event(
        &self,
        plugin: Value,
        project: Option<String>,
        package: Option<String>,
        action: &str,
        args: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        smol::block_on(async {
            self.conn.execute(
                "INSERT INTO events (plugin, project, package, action, args) VALUES (?1, ?2, ?3, ?4, ?5);",
                libsql::params![
                    plugin,
                    project,
                    package,
                    action,
                    args
                ],
            ).await?;

            Ok(())
        })
    }
}
