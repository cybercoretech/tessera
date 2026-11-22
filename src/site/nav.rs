use std::collections::HashMap;
use crate::token::TokenSet;

#[derive(Debug, Default)]
pub struct Nav {
    // Simple map from page slug to title (could be expanded later)
    pub items: HashMap<String, String>,
}

impl Nav {
    pub fn new(_tokens: &TokenSet) -> Self {
        // For now we return an empty navigation structure.
        // Future implementation will read a [navigation] table from tessera.toml.
        Self { items: HashMap::new() }
    }
}
