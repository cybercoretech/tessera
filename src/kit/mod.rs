pub mod catalog;
pub mod controls;
pub mod foundations;
pub mod layout;

/// Core, dependency-free CSS shipped with every generated Tessera site.
pub const STYLES: &str = concat!(
    include_str!("foundations.css"),
    "\n",
    include_str!("layout.css"),
    "\n",
    include_str!("controls.css"),
    "\n",
    include_str!("content.css"),
    "\n",
    include_str!("disclosure.css"),
    "\n",
    include_str!("feedback.css")
);

use crate::token::TokenSet;

// These themed APIs are staged for the planned Kit/style integration.
#[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::{catalog, STYLES};

    #[test]
    fn core_styles_match_snapshot() {
        assert_eq!(STYLES, include_str!("kit.css.snap"));
    }

    #[test]
    fn kit_class_names_use_the_ts_prefix() {
        for entry in catalog::ENTRIES {
            assert!(
                entry.class_name.starts_with("ts-"),
                "catalog class `{}` must start with `ts-`",
                entry.class_name
            );
        }

        let mut selectors = Vec::new();
        let mut prelude = String::new();
        for character in STYLES.chars() {
            match character {
                '{' => {
                    let selector = prelude.trim();
                    if !selector.starts_with('@') {
                        let bytes = selector.as_bytes();
                        let mut index = 0;
                        while index < bytes.len() {
                            if bytes[index] == b'.' {
                                let start = index + 1;
                                let mut end = start;
                                while end < bytes.len()
                                    && (bytes[end].is_ascii_alphanumeric()
                                        || matches!(bytes[end], b'-' | b'_'))
                                {
                                    end += 1;
                                }
                                if end > start {
                                    selectors.push(selector[start..end].to_owned());
                                }
                                index = end;
                            } else {
                                index += 1;
                            }
                        }
                    }
                    prelude.clear();
                }
                '}' => prelude.clear(),
                _ => prelude.push(character),
            }
        }

        assert!(
            !selectors.is_empty(),
            "Kit CSS should contain class selectors"
        );
        for class_name in selectors {
            assert!(
                class_name.starts_with("ts-"),
                "Kit CSS class selector `.{class_name}` must start with `.ts-`"
            );
        }
    }
}
