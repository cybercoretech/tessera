pub mod foundations;
pub mod layout;
pub mod controls;

use crate::token::TokenSet;

pub struct Theme<'a> {
    pub foundations: foundations::Foundations<'a>,
    pub layout: layout::Layout<'a>,
    pub controls: controls::Controls<'a>,
}

impl<'a> Theme<'a> {
    /// Build a Theme from a shared TokenSet.
    pub fn new(tokens: &'a TokenSet) -> Self {
        Self {
            foundations: foundations::Foundations::new(tokens),
            layout: layout::Layout::new(tokens),
            controls: controls::Controls::new(tokens),
        }
    }
}
