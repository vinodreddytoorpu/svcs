use crate::{Token, LexerRegistry};
use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct TokenStats {
    pub total_tokens: usize,
    pub category_counts: HashMap<String, usize>,
    pub token_type_counts: HashMap<String, usize>,
    pub file_path: String,
}

impl TokenStats {
    pub fn from_tokens(tokens: &[(Token, Range<usize>)], registry: &LexerRegistry) -> Self {
        let mut category_counts = HashMap::new();
        let mut token_type_counts = HashMap::new();
        
        for (token, _) in tokens {
            // Count by category
            if let Some(category) = registry.get_token_category(token) {
                *category_counts.entry(category).or_insert(0) += 1;
            }
            
            // Count by token type
            let token_name = format!("{:?}", token);
            *token_type_counts.entry(token_name).or_insert(0) += 1;
        }
        
        Self {
            total_tokens: tokens.len(),
            category_counts,
            token_type_counts,
            file_path: "unknown".to_string(),
        }
    }
    
    pub fn with_file_path(mut self, file_path: String) -> Self {
        self.file_path = file_path;
        self
    }
    
    pub fn print_summary(&self) {
        println!("Token Statistics for {}", self.file_path);
        println!("Total tokens: {}", self.total_tokens);
        println!("\nBy category:");
        for (category, count) in &self.category_counts {
            println!("  {}: {}", category, count);
        }
    }
}
