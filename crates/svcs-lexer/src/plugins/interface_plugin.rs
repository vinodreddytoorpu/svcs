use crate::traits::{TokenRecognizer, LexerPlugin};
use crate::Token;

#[derive(Clone)]
pub struct InterfaceTokenRecognizer {
    tokens: Vec<Token>,
    keywords: Vec<&'static str>,
}

impl Default for InterfaceTokenRecognizer {
    fn default() -> Self {
        Self {
            tokens: vec![
                Token::Interface, Token::EndInterface, Token::ModPort,
                Token::Clocking, Token::EndClocking, Token::Virtual,
            ],
            keywords: vec![
                "interface", "endinterface", "modport", "clocking", 
                "endclocking", "virtual"
            ],
        }
    }
}

impl TokenRecognizer for InterfaceTokenRecognizer {
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
        "interface"
    }
}

pub struct InterfacePlugin {
    recognizer: Option<InterfaceTokenRecognizer>,
}

impl Default for InterfacePlugin {
    fn default() -> Self {
        Self { recognizer: None }
    }
}

impl LexerPlugin for InterfacePlugin {
    fn name(&self) -> &'static str {
        "SystemVerilog Interface Plugin"
    }
    
    fn version(&self) -> &'static str {
        "1.0.0"
    }
    
    fn description(&self) -> &'static str {
        "Recognizes SystemVerilog interface-related tokens"
    }
    
    fn recognizers(&self) -> Vec<Box<dyn TokenRecognizer>> {
        if let Some(ref recognizer) = self.recognizer {
            vec![Box::new(recognizer.clone())]
        } else {
            vec![]
        }
    }
    
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.recognizer = Some(InterfaceTokenRecognizer::default());
        tracing::info!("Initialized {} v{}", self.name(), self.version());
        Ok(())
    }
}
