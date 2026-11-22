use crate::token::TokenSet;

// Layout tokens are reserved for the planned responsive layout primitives.
#[allow(dead_code)]
pub struct Layout<'a> {
    tokens: &'a TokenSet,
}

#[allow(dead_code)]
impl<'a> Layout<'a> {
    pub fn new(tokens: &'a TokenSet) -> Self {
        Self { tokens }
    }

    /// Retrieves the default grid gap token (fallback to 1rem).
    pub fn grid_gap(&self) -> &str {
        self.tokens.get("grid_gap").unwrap_or("1rem")
    }

    /// Retrieves the container max‑width token (fallback to 1200px).
    pub fn container_max_width(&self) -> &str {
        self.tokens.get("container_max_width").unwrap_or("1200px")
    }
}
