use crate::{Token, LexError, LexerRegistry};
use crate::traits::{LexerPlugin, TokenConfig};
use crate::utils::TokenStats;
use logos::Logos;
use std::ops::Range;

pub struct SystemVerilogLexer<'input> {
    lexer: logos::Lexer<'input, Token>,
    file_path: String,
    registry: LexerRegistry,
    config: TokenConfig,
}

impl<'input> SystemVerilogLexer<'input> {
    pub fn new(input: &'input str, file_path: String, registry: LexerRegistry) -> Self {
        Self {
            lexer: Token::lexer(input),
            file_path,
            registry,
            config: TokenConfig::default(),
        }
    }
    
    pub fn with_config(mut self, config: TokenConfig) -> Self {
        self.config = config;
        self
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<(Token, Range<usize>)>, LexError> {
        let mut tokens = Vec::new();
        
        while let Some(token) = self.lexer.next() {
            let span = self.lexer.span();
            let text = self.lexer.slice();
            
            match token {
                Ok(tok) => {
                    // Skip disabled tokens
                    if self.config.disabled_tokens.contains(&tok) {
                        continue;
                    }
                    
                    // Validate token with registered recognizers
                    if let Some(category) = self.registry.get_token_category(&tok) {
                        if let Some(recognizer) = self.registry.get_recognizer(&category) {
                            if !recognizer.validate_token(&tok, text) {
                                return Err(LexError::InvalidToken {
                                    file: self.file_path.clone(),
                                    span: span.clone(),
                                    text: text.to_string(),
                                });
                            }
                        }
                    }
                    
                    tokens.push((tok, span));
                }
                Err(_) => {
                    return Err(LexError::InvalidToken {
                        file: self.file_path.clone(),
                        span: span.clone(),
                        text: text.to_string(),
                    });
                }
            }
        }
        
        tracing::debug!("Tokenized {} tokens from {}", tokens.len(), self.file_path);
        Ok(tokens)
    }
    
    pub fn tokenize_with_stats(&mut self) -> Result<(Vec<(Token, Range<usize>)>, TokenStats), LexError> {
        let tokens = self.tokenize()?;
        let stats = TokenStats::from_tokens(&tokens, &self.registry);
        Ok((tokens, stats))
    }
    
    pub fn registry(&self) -> &LexerRegistry {
        &self.registry
    }
}

/// Builder for creating SystemVerilog lexers with plugins
pub struct LexerBuilder {
    registry: LexerRegistry,
    config: TokenConfig,
}

impl LexerBuilder {
    pub fn new() -> Self {
        Self {
            registry: LexerRegistry::new(),
            config: TokenConfig::default(),
        }
    }
    
    /// Add a plugin to the lexer
    pub fn with_plugin(mut self, plugin: Box<dyn LexerPlugin>) -> Self {
        if let Err(e) = self.registry.register_plugin(plugin) {
            tracing::error!("Failed to register plugin: {}", e);
        }
        self
    }
    
    /// Add all default plugins
    pub fn with_default_plugins(mut self) -> Self {
        use crate::plugins::*;
        
        self = self.with_plugin(Box::new(ModulePlugin::default()));
        self = self.with_plugin(Box::new(InterfacePlugin::default()));
        self = self.with_plugin(Box::new(DataTypePlugin::default()));
        self = self.with_plugin(Box::new(ControlPlugin::default()));
        self = self.with_plugin(Box::new(OperatorPlugin::default()));
        self = self.with_plugin(Box::new(GeneralPlugin::default()));
        
        self
    }
    
    /// Set token configuration
    pub fn with_config(mut self, config: TokenConfig) -> Self {
        self.config = config;
        self
    }
    
    /// Build the lexer
    pub fn build<'input>(self, input: &'input str, file_path: String) -> SystemVerilogLexer<'input> {
        SystemVerilogLexer::new(input, file_path, self.registry).with_config(self.config)
    }
}

impl Default for LexerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
