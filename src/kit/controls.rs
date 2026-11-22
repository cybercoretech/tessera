use crate::token::TokenSet;

pub struct Controls<'a> {
    tokens: &'a TokenSet,
}

impl<'a> Controls<'a> {
    pub fn new(tokens: &'a TokenSet) -> Self {
        Self { tokens }
    }

    /// Default button background color (fallback to primary).
    pub fn button_bg(&self) -> &str {
        self.tokens.get("primary").unwrap_or("#000000")
    }

    /// Default input border color (fallback to secondary).
    pub fn input_border(&self) -> &str {
        self.tokens.get("secondary").unwrap_or("#777777")
    }
}
