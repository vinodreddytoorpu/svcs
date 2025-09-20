use crate::traits::TokenConfig;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LexerConfig {
    pub token_config: TokenConfig,
    pub enabled_plugins: Vec<String>,
    pub disabled_plugins: Vec<String>,
    pub debug_mode: bool,
}

impl Default for LexerConfig {
    fn default() -> Self {
        Self {
            token_config: TokenConfig::default(),
            enabled_plugins: vec![
                "module".to_string(),
                "interface".to_string(), 
                "datatype".to_string(),
                "control".to_string(),
                "operator".to_string(),
                "general".to_string(),
            ],
            disabled_plugins: vec![],
            debug_mode: false,
        }
    }
}

impl LexerConfig {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: LexerConfig = toml::from_str(&content)?;
        Ok(config)
    }
    
    pub fn to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
