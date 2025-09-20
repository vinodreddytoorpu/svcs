use crate::traits::{TokenRecognizer, LexerPlugin};
use crate::Token;

#[derive(Clone)]
pub struct ControlTokenRecognizer {
    tokens: Vec<Token>,
    keywords: Vec<&'static str>,
}

impl Default for ControlTokenRecognizer {
    fn default() -> Self {
        Self {
            tokens: vec![
                Token::Always, Token::AlwaysComb, Token::AlwaysFF,
                Token::AlwaysLatch, Token::Initial, Token::If, Token::Else,
                Token::Case, Token::CaseX, Token::CaseZ, Token::EndCase,
                Token::Default, Token::For, Token::ForEach, Token::While,
                Token::Do, Token::Repeat, Token::Forever, Token::Begin,
                Token::End, Token::Fork, Token::Join, Token::JoinAny,
                Token::JoinNone, Token::Wait, Token::Disable,
                Token::Break, Token::Continue, Token::Return,
            ],
            keywords: vec![
                "always", "always_comb", "always_ff", "always_latch", "initial",
                "if", "else", "case", "casex", "casez", "endcase", "default",
                "for", "foreach", "while", "do", "repeat", "forever",
                "begin", "end", "fork", "join", "join_any", "join_none",
                "wait", "disable", "break", "continue", "return"
            ],
        }
    }
}

impl TokenRecognizer for ControlTokenRecognizer {
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
        "control"
    }
}

pub struct ControlPlugin {
    recognizer: Option<ControlTokenRecognizer>,
}

impl Default for ControlPlugin {
    fn default() -> Self {
        Self { recognizer: None }
    }
}

impl LexerPlugin for ControlPlugin {
    fn name(&self) -> &'static str {
        "SystemVerilog Control Flow Plugin"
    }
    
    fn version(&self) -> &'static str {
        "1.0.0"
    }
    
    fn description(&self) -> &'static str {
        "Recognizes SystemVerilog control flow tokens"
    }
    
    fn recognizers(&self) -> Vec<Box<dyn TokenRecognizer>> {
        if let Some(ref recognizer) = self.recognizer {
            vec![Box::new(recognizer.clone())]
        } else {
            vec![]
        }
    }
    
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.recognizer = Some(ControlTokenRecognizer::default());
        tracing::info!("Initialized {} v{}", self.name(), self.version());
        Ok(())
    }
}
