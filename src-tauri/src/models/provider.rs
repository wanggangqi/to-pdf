use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    pub id: String,
    pub name: String,
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub enabled: bool,
}

impl Default for Provider {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            api_key: String::new(),
            base_url: String::new(),
            model: String::new(),
            enabled: false,
        }
    }
}