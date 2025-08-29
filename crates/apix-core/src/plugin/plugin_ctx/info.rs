#[derive(Debug)]
pub struct PluginInfo {
    pub usage: Vec<String>,
    pub options: Vec<(String, String)>,
}

impl PluginInfo {
    pub fn format(&self, name: &str, version: &str, description: &str) -> String {
        let mut out = String::new();
        out.push_str(&format!("[{}: v{}]\n", name, version));
        out.push_str(&format!("{}\n\n", description));

        out.push_str("Usage:\n");
        for u in &self.usage {
            out.push_str(&format!("  {}\n", u));
        }
        out.push('\n');

        if !self.options.is_empty() {
            out.push_str("Options:\n");
            for (flag, desc) in &self.options {
                out.push_str(&format!("  {:<12} {}\n", flag, desc));
            }
        }

        out
    }
}
