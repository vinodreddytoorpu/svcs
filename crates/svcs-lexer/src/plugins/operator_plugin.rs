use crate::traits::{TokenRecognizer, LexerPlugin};
use crate::Token;

#[derive(Clone)]
pub struct OperatorTokenRecognizer {
    tokens: Vec<Token>,
    keywords: Vec<&'static str>,
}

impl Default for OperatorTokenRecognizer {
    fn default() -> Self {
        Self {
            tokens: vec![
                Token::Plus, Token::Minus, Token::Multiply, Token::Divide,
                Token::Modulo, Token::Power, Token::Equal, Token::NotEqual,
                Token::CaseEqual, Token::CaseNotEqual, Token::LessThan,
                Token::GreaterThan, Token::LessEqual, Token::GreaterEqual,
                Token::LogicalAnd, Token::LogicalOr, Token::LogicalNot,
                Token::BitwiseAnd, Token::BitwiseOr, Token::BitwiseXor,
                Token::BitwiseNot, Token::LeftShift, Token::RightShift,
                Token::ArithmeticLeftShift, Token::ArithmeticRightShift,
                Token::Assign, Token::PlusAssign, Token::MinusAssign,
                Token::MultiplyAssign, Token::DivideAssign, Token::ModuloAssign,
                Token::AndAssign, Token::OrAssign, Token::XorAssign,
                Token::LeftShiftAssign, Token::RightShiftAssign,
                Token::Increment, Token::Decrement,
            ],
            keywords: vec![], // Operators don't have keyword forms
        }
    }
}

impl TokenRecognizer for OperatorTokenRecognizer {
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
        "operator"
    }
}

pub struct OperatorPlugin {
    recognizer: Option<OperatorTokenRecognizer>,
}

impl Default for OperatorPlugin {
    fn default() -> Self {
        Self { recognizer: None }
    }
}

impl LexerPlugin for OperatorPlugin {
    fn name(&self) -> &'static str {
        "SystemVerilog Operator Plugin"
    }
    
    fn version(&self) -> &'static str {
        "1.0.0"
    }
    
    fn description(&self) -> &'static str {
        "Recognizes SystemVerilog operator tokens"
    }
    
    fn recognizers(&self) -> Vec<Box<dyn TokenRecognizer>> {
        if let Some(ref recognizer) = self.recognizer {
            vec![Box::new(recognizer.clone())]
        } else {
            vec![]
        }
    }
    
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.recognizer = Some(OperatorTokenRecognizer::default());
        tracing::info!("Initialized {} v{}", self.name(), self.version());
        Ok(())
    }
}
