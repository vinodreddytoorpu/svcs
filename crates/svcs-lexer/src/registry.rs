use crate::traits::{LexerPlugin, TokenRecognizer};
use crate::Token;
use std::collections::{HashMap, HashSet};
use tracing::{debug, info, warn};

pub struct LexerRegistry {
    plugins: HashMap<String, Box<dyn LexerPlugin>>,
    recognizers: HashMap<String, Box<dyn TokenRecognizer>>,
    enabled_categories: HashSet<String>,
}

impl LexerRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            recognizers: HashMap::new(),
            enabled_categories: HashSet::new(),
        }
    }
    
    /// Register a plugin with dependency checking
    pub fn register_plugin(&mut self, mut plugin: Box<dyn LexerPlugin>) -> Result<(), String> {
        let name = plugin.name().to_string();
        
        if self.plugins.contains_key(&name) {
            return Err(format!("Plugin '{}' already registered", name));
        }
        
        // Check dependencies
        for dep in plugin.dependencies() {
            if !self.plugins.contains_key(dep) {
                return Err(format!("Plugin '{}' depends on '{}' which is not registered", name, dep));
            }
        }
        
        // Initialize plugin
        if let Err(e) = plugin.initialize() {
            return Err(format!("Failed to initialize plugin '{}': {}", name, e));
        }
        
        // Register recognizers if plugin is enabled
        if plugin.is_enabled() {
            for recognizer in plugin.recognizers() {
                let category = recognizer.category_name().to_string();
                debug!("Registering recognizer '{}' from plugin '{}'", category, name);
                self.recognizers.insert(category.clone(), recognizer);
                self.enabled_categories.insert(category);
            }
        }
        
        info!("Registered plugin: {} v{}", plugin.name(), plugin.version());
        self.plugins.insert(name, plugin);
        Ok(())
    }
    
    /// Get recognizer by category
    pub fn get_recognizer(&self, category: &str) -> Option<&Box<dyn TokenRecognizer>> {
        self.recognizers.get(category)
    }
    
    /// List all enabled categories
    pub fn list_categories(&self) -> Vec<String> {
        self.enabled_categories.iter().cloned().collect()
    }
    
    /// Get statistics about registered plugins
    pub fn stats(&self) -> RegistryStats {
        RegistryStats {
            total_plugins: self.plugins.len(),
            total_recognizers: self.recognizers.len(),
            enabled_categories: self.enabled_categories.len(),
        }
    }
    
    /// Check if a token is handled by any recognizer
    pub fn is_token_handled(&self, token: &Token) -> bool {
        self.recognizers.values().any(|recognizer| recognizer.handles_token(token))
    }
    
    /// Get category name for a token
    pub fn get_token_category(&self, token: &Token) -> Option<String> {
        self.recognizers.iter()
            .find(|(_, recognizer)| recognizer.handles_token(token))
            .map(|(category, _)| category.clone())
    }
    
    /// Enable/disable a category
    pub fn set_category_enabled(&mut self, category: &str, enabled: bool) {
        if enabled {
            self.enabled_categories.insert(category.to_string());
        } else {
            self.enabled_categories.remove(category);
            warn!("Disabled token category: {}", category);
        }
    }
}

#[derive(Debug)]
pub struct RegistryStats {
    pub total_plugins: usize,
    pub total_recognizers: usize,
    pub enabled_categories: usize,
}

impl Default for LexerRegistry {
    fn default() -> Self {
        Self::new()
    }
}
