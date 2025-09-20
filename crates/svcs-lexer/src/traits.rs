use crate::Token;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Trait for token recognition modules
pub trait TokenRecognizer: Send + Sync {
    /// Get all tokens this recognizer handles
    fn tokens(&self) -> Vec<Token>;
    
    /// Get all keywords this recognizer handles  
    fn keywords(&self) -> Vec<&'static str>;
    
    /// Check if this recognizer handles a specific token
    fn handles_token(&self, token: &Token) -> bool;
    
    /// Get token category name for statistics
    fn category_name(&self) -> &'static str;
    
    /// Get priority for this recognizer (higher = more important)
    fn priority(&self) -> u8 { 50 }
    
    /// Custom token validation logic
    fn validate_token(&self, _token: &Token, _text: &str) -> bool { true }
}

/// Trait for lexer plugins - allows dynamic registration
pub trait LexerPlugin: Send + Sync {
    /// Plugin name
    fn name(&self) -> &'static str;
    
    /// Plugin version
    fn version(&self) -> &'static str;
    
    /// Plugin description
    fn description(&self) -> &'static str;
    
    /// Get token recognizers provided by this plugin
    fn recognizers(&self) -> Vec<Box<dyn TokenRecognizer>>;
    
    /// Initialize plugin (called once during lexer setup)
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Check if plugin is enabled
    fn is_enabled(&self) -> bool { true }
    
    /// Plugin dependencies (other plugin names)
    fn dependencies(&self) -> Vec<&'static str> { vec![] }
}

/// Configuration for token recognition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenConfig {
    pub ignore_case: bool,
    pub custom_keywords: HashMap<String, Token>,
    pub disabled_tokens: Vec<Token>,
}

pub struct TokenConfigSerdeWrapper(pub TokenConfig);

impl Default for TokenConfig {
    fn default() -> Self {
        Self {
            ignore_case: false,
            custom_keywords: HashMap::new(),
            disabled_tokens: Vec::new(),
        }
    }
}
