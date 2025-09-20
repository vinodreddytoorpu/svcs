use crate::traits::{TokenRecognizer, LexerPlugin};
use crate::Token;

#[derive(Clone)]
pub struct GeneralTokenRecognizer {
    tokens: Vec<Token>,
    keywords: Vec<&'static str>,
}

impl Default for GeneralTokenRecognizer {
    fn default() -> Self {
        Self {
            tokens: vec![
                Token::Identifier, Token::SystemIdentifier, Token::SizedNumber,
                Token::UnsizedNumber, Token::RealNumber, Token::StringLiteral,
                Token::LeftParen, Token::RightParen, Token::LeftBracket,
                Token::RightBracket, Token::LeftBrace, Token::RightBrace,
                Token::Dot, Token::Comma, Token::Semicolon, Token::Colon,
                Token::Scope, Token::Question, Token::Hash, Token::At,
                Token::PreprocessorDirective,
            ],
            keywords: vec![], // General tokens are mostly symbols
        }
    }
}

impl TokenRecognizer for GeneralTokenRecognizer {
    fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
    
    fn keywords(&self) -> Vec<&'static str> {
        self.keywords.clone()
    }
    
    fn handles_token(&self, token: &Token) -> bool {
        self.tokens.contains(token)
    }
    
    fn category_name(&self) -> &'static str {
        "general"
    }
}

pub struct GeneralPlugin {
    recognizer: Option<GeneralTokenRecognizer>,
}

impl Default for GeneralPlugin {
    fn default() -> Self {
        Self { recognizer: None }
    }
}

impl LexerPlugin for GeneralPlugin {
    fn name(&self) -> &'static str {
        "SystemVerilog General Plugin"
    }
    
    fn version(&self) -> &'static str {
        "1.0.0"
    }
    
    fn description(&self) -> &'static str {
        "Recognizes general SystemVerilog tokens like identifiers, literals, and punctuation"
    }
    
    fn recognizers(&self) -> Vec<Box<dyn TokenRecognizer>> {
        if let Some(ref recognizer) = self.recognizer {
            vec![Box::new(recognizer.clone())]
        } else {
            vec![]
        }
    }
    
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.recognizer = Some(GeneralTokenRecognizer::default());
        tracing::info!("Initialized {} v{}", self.name(), self.version());
        Ok(())
    }
}
