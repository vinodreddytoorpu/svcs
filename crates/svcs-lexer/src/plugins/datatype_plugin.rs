use crate::traits::{TokenRecognizer, LexerPlugin};
use crate::Token;

#[derive(Clone)]
pub struct DataTypeTokenRecognizer {
    tokens: Vec<Token>,
    keywords: Vec<&'static str>,
}

impl Default for DataTypeTokenRecognizer {
    fn default() -> Self {
        Self {
            tokens: vec![
                Token::Logic, Token::Bit, Token::Byte, Token::Int,
                Token::Integer, Token::Real, Token::String, Token::Wire,
                Token::Reg, Token::Input, Token::Output, Token::Inout,
                Token::Signed, Token::Unsigned, Token::Packed,
                Token::Struct, Token::Union, Token::Enum, Token::Typedef,
                Token::Parameter, Token::LocalParam, Token::Const,
                Token::Static, Token::Automatic,
            ],
            keywords: vec![
                "logic", "bit", "byte", "int", "integer", "real", "string",
                "wire", "reg", "input", "output", "inout", "signed", "unsigned",
                "packed", "struct", "union", "enum", "typedef", "parameter",
                "localparam", "const", "static", "automatic"
            ],
        }
    }
}

impl TokenRecognizer for DataTypeTokenRecognizer {
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
        "datatype"
    }
}

pub struct DataTypePlugin {
    recognizer: Option<DataTypeTokenRecognizer>,
}

impl Default for DataTypePlugin {
    fn default() -> Self {
        Self { recognizer: None }
    }
}

impl LexerPlugin for DataTypePlugin {
    fn name(&self) -> &'static str {
        "SystemVerilog DataType Plugin"
    }
    
    fn version(&self) -> &'static str {
        "1.0.0"
    }
    
    fn description(&self) -> &'static str {
        "Recognizes SystemVerilog data type tokens"
    }
    
    fn recognizers(&self) -> Vec<Box<dyn TokenRecognizer>> {
        if let Some(ref recognizer) = self.recognizer {
            vec![Box::new(recognizer.clone())]
        } else {
            vec![]
        }
    }
    
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.recognizer = Some(DataTypeTokenRecognizer::default());
        tracing::info!("Initialized {} v{}", self.name(), self.version());
        Ok(())
    }
}
