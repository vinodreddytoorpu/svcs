use crate::traits::{TokenRecognizer, LexerPlugin};
use crate::Token;

#[derive(Clone)]
pub struct ModuleTokenRecognizer {
    tokens: Vec<Token>,
    keywords: Vec<&'static str>,
}

impl Default for ModuleTokenRecognizer {
    fn default() -> Self {
        Self {
            tokens: vec![
                Token::Module, Token::EndModule, Token::Package, 
                Token::EndPackage, Token::Import, Token::Export,
                Token::Generate, Token::EndGenerate, Token::GenVar,
            ],
            keywords: vec![
                "module", "endmodule", "package", "endpackage",
                "import", "export", "generate", "endgenerate", "genvar"
            ],
        }
    }
}

impl TokenRecognizer for ModuleTokenRecognizer {
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
        "module"
    }
}

pub struct ModulePlugin {
    recognizer: Option<ModuleTokenRecognizer>,
}

impl Default for ModulePlugin {
    fn default() -> Self {
        Self { recognizer: None }
    }
}

impl LexerPlugin for ModulePlugin {
    fn name(&self) -> &'static str {
        "SystemVerilog Module Plugin"
    }
    
    fn version(&self) -> &'static str {
        "1.0.0"
    }
    
    fn description(&self) -> &'static str {
        "Recognizes SystemVerilog module-related tokens"
    }
    
    fn recognizers(&self) -> Vec<Box<dyn TokenRecognizer>> {
        if let Some(ref recognizer) = self.recognizer {
            vec![Box::new(recognizer.clone())]
        } else {
            vec![]
        }
    }
    
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.recognizer = Some(ModuleTokenRecognizer::default());
        tracing::info!("Initialized {} v{}", self.name(), self.version());
        Ok(())
    }
}
