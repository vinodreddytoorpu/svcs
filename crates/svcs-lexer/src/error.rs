use thiserror::Error;
use serde::{Deserialize, Serialize};
use std::ops::Range;

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum LexError {
    #[error("Invalid token in file {file} at {span:?}: '{text}'")]
    InvalidToken {
        file: String,
        span: Range<usize>,
        text: String,
    },
    
    #[error("Unexpected end of file in {file}")]
    UnexpectedEof {
        file: String,
    },
    
    #[error("Invalid number format in {file} at {span:?}: '{text}'")]
    InvalidNumber {
        file: String,
        span: Range<usize>, 
        text: String,
    },
    
    #[error("Invalid string literal in {file} at {span:?}: '{text}'")]
    InvalidString {
        file: String,
        span: Range<usize>,
        text: String,
    },
    
    #[error("Plugin error: {message}")]
    PluginError {
        message: String,
    },
    
    #[error("Registry error: {message}")]
    RegistryError {
        message: String,
    },
}
