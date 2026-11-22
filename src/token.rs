use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct TokenSet {
    pub tokens: HashMap<String, String>,
}

impl TokenSet {
    /// Retrieve a token value by name. Returns `None` if the token is not defined.
    pub fn get(&self, name: &str) -> Option<&str> {
        self.tokens.get(name).map(|s| s.as_str())
    }

    /// Returns an iterator over `(name, value)` pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.tokens.iter()
    }
}
