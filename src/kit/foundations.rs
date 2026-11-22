use crate::token::TokenSet;

pub struct Foundations<'a> {
    tokens: &'a TokenSet,
}

// The full foundation token set is consumed as the Kit expands.
#[allow(dead_code)]
impl<'a> Foundations<'a> {
    pub fn new(tokens: &'a TokenSet) -> Self {
        Self { tokens }
    }

    /// Primary brand color (fallback to black).
    pub fn primary_color(&self) -> &str {
        self.tokens.get("primary").unwrap_or("#000000")
    }

    /// Secondary brand color (fallback to gray).
    pub fn secondary_color(&self) -> &str {
        self.tokens.get("secondary").unwrap_or("#777777")
    }

    /// Medium spacing token (fallback to 1rem).
    pub fn spacing_md(&self) -> &str {
        self.tokens.get("spacing_md").unwrap_or("1rem")
    }
}
