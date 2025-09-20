//! SVCS SystemVerilog Lexer with Plugin Architecture
//! 
//! A modular, extensible lexer for SystemVerilog with plug-and-play token recognition.

pub mod error;
pub mod traits;
pub mod registry;
pub mod lexer;
pub mod plugins;
pub mod utils;

// Re-export core types
pub use error::LexError;
pub use traits::{TokenRecognizer, LexerPlugin, TokenConfig};
pub use registry::LexerRegistry;
pub use lexer::{SystemVerilogLexer, LexerBuilder};

use logos::Logos;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Logos, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[logos(skip r"[ \t\r\n\f]+")]
#[logos(skip r"//[^\n]*")]
#[logos(skip r"/\*([^*]|\*[^/])*\*/")]
pub enum Token {
    // Module tokens
    #[token("module")]
    Module,
    #[token("endmodule")]
    EndModule,
    #[token("package")]
    Package,
    #[token("endpackage")]
    EndPackage,
    #[token("import")]
    Import,
    #[token("export")]
    Export,
    #[token("generate")]
    Generate,
    #[token("endgenerate")]
    EndGenerate,
    #[token("genvar")]
    GenVar,

    // Function/task/class tokens
    #[token("function")]
    Function,
    #[token("endfunction")]
    EndFunction,
    #[token("task")]
    Task,
    #[token("endtask")]
    EndTask,
    #[token("class")]
    Class,
    #[token("endclass")]
    EndClass,

    // Interface tokens
    #[token("interface")]
    Interface,
    #[token("endinterface")]
    EndInterface,
    #[token("modport")]
    ModPort,
    #[token("clocking")]
    Clocking,
    #[token("endclocking")]
    EndClocking,
    #[token("virtual")]
    Virtual,
    
    // Data type tokens
    #[token("logic")]
    Logic,
    #[token("bit")]
    Bit,
    #[token("byte")]
    Byte,
    #[token("int")]
    Int,
    #[token("integer")]
    Integer,
    #[token("real")]
    Real,
    #[token("string")]
    String,
    #[token("wire")]
    Wire,
    #[token("reg")]
    Reg,
    #[token("input")]
    Input,
    #[token("output")]
    Output,
    #[token("inout")]
    Inout,
    #[token("signed")]
    Signed,
    #[token("unsigned")]
    Unsigned,
    #[token("packed")]
    Packed,
    #[token("struct")]
    Struct,
    #[token("union")]
    Union,
    #[token("enum")]
    Enum,
    #[token("typedef")]
    Typedef,
    #[token("parameter")]
    Parameter,
    #[token("localparam")]
    LocalParam,
    #[token("const")]
    Const,
    #[token("static")]
    Static,
    #[token("automatic")]
    Automatic,
    
    // Control flow tokens
    #[token("always")]
    Always,
    #[token("always_comb")]
    AlwaysComb,
    #[token("always_ff")]
    AlwaysFF,
    #[token("always_latch")]
    AlwaysLatch,
    #[token("initial")]
    Initial,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("case")]
    Case,
    #[token("casex")]
    CaseX,
    #[token("casez")]
    CaseZ,
    #[token("endcase")]
    EndCase,
    #[token("default")]
    Default,
    #[token("for")]
    For,
    #[token("foreach")]
    ForEach,
    #[token("while")]
    While,
    #[token("do")]
    Do,
    #[token("repeat")]
    Repeat,
    #[token("forever")]
    Forever,
    #[token("begin")]
    Begin,
    #[token("end")]
    End,
    #[token("fork")]
    Fork,
    #[token("join")]
    Join,
    #[token("join_any")]
    JoinAny,
    #[token("join_none")]
    JoinNone,
    #[token("wait")]
    Wait,
    #[token("disable")]
    Disable,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("return")]
    Return,
    
    // Operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,
    #[token("%")]
    Modulo,
    #[token("**")]
    Power,
    #[token("==")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token("===")]
    CaseEqual,
    #[token("!==")]
    CaseNotEqual,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,
    #[token("&&")]
    LogicalAnd,
    #[token("||")]
    LogicalOr,
    #[token("!")]
    LogicalNot,
    #[token("&")]
    BitwiseAnd,
    #[token("|")]
    BitwiseOr,
    #[token("^")]
    BitwiseXor,
    #[token("~")]
    BitwiseNot,
    #[token("<<")]
    LeftShift,
    #[token(">>")]
    RightShift,
    #[token("<<<")]
    ArithmeticLeftShift,
    #[token(">>>")]
    ArithmeticRightShift,
    #[token("=")]
    Assign,
    #[token("+=")]
    PlusAssign,
    #[token("-=")]
    MinusAssign,
    #[token("*=")]
    MultiplyAssign,
    #[token("/=")]
    DivideAssign,
    #[token("%=")]
    ModuloAssign,
    #[token("&=")]
    AndAssign,
    #[token("|=")]
    OrAssign,
    #[token("^=")]
    XorAssign,
    #[token("<<=")]
    LeftShiftAssign,
    #[token(">>=")]
    RightShiftAssign,
    #[token("++")]
    Increment,
    #[token("--")]
    Decrement,
    
    // General tokens
    #[token("assign")]
    AssignKeyword,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_$]*")]
    Identifier,
    #[regex(r"\$[a-zA-Z_][a-zA-Z0-9_$]*")]
    SystemIdentifier,
    #[regex(r"[0-9]*'[bdhoDBDHO][0-9a-fA-FxzXZ_]*")]
    SizedNumber,
    #[regex(r"[0-9]+")]
    UnsizedNumber,
    #[regex(r"[0-9]*\.[0-9]+([eE][+-]?[0-9]+)?")]
    RealNumber,
    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token("::")]
    Scope,
    #[token("?")]
    Question,
    #[token("#")]
    Hash,
    #[token("@")]
    At,
    #[regex(r"`[a-zA-Z_][a-zA-Z0-9_]*")]
    PreprocessorDirective,
    Error,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Easy-to-use API for creating a lexer with default plugins
pub fn create_default_lexer(input: &str, file_path: String) -> SystemVerilogLexer<'_> {
    LexerBuilder::new()
        .with_default_plugins()
        .build(input, file_path)
}

/// Easy-to-use API for creating a minimal lexer
pub fn create_minimal_lexer(input: &str, file_path: String) -> SystemVerilogLexer<'_> {
    LexerBuilder::new()
        .with_plugin(Box::new(plugins::GeneralPlugin::default()))
        .build(input, file_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_module() {
        let input = "module test_module(input clk, output data); assign data = clk; endmodule";
        let mut lexer = create_default_lexer(input, "test.sv".to_string());
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].0, Token::Module);
        assert_eq!(tokens[1].0, Token::Identifier);
        assert_eq!(tokens[2].0, Token::LeftParen);
    }
}
