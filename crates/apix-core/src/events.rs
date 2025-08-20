use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub plugin: String,
    pub project: String,
    pub action: String,
    pub args: String,
}

impl Event {
    pub fn new(plugin: &str, project: &str, action: &str, args: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            plugin: plugin.to_string(),
            project: project.to_string(),
            action: action.to_string(),
            args: args.to_string(),
        }
    }
}
