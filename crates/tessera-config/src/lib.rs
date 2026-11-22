use std::{collections::BTreeMap, fs, path::Path};

use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectConfig {
    pub name: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub style: StyleConfig,
    #[serde(default)]
    pub theme: ThemeConfig,
    #[serde(default, rename = "tokens")]
    legacy_tokens: ThemeTokens,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThemeConfig {
    #[serde(default)]
    pub tokens: ThemeTokens,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(transparent)]
pub struct ThemeTokens(BTreeMap<String, String>);

impl ThemeTokens {
    pub fn get(&self, name: &str) -> Option<&String> {
        self.0.get(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.0.iter()
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StyleConfig {
    #[serde(default)]
    pub mode: StyleMode,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StyleMode {
    #[default]
    Scss,
    Css,
    Tailwind,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("could not read {path}: {source}")]
    Read {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("invalid tessera.toml: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("invalid style.mode {value:?}; expected scss, css, or tailwind")]
    InvalidStyleMode { value: String },
}

impl ProjectConfig {
    pub fn load(root: &Path) -> Result<Self, ConfigError> {
        let manifest_path = root.join("tessera.toml");
        let contents = fs::read_to_string(&manifest_path).map_err(|source| ConfigError::Read {
            path: manifest_path.display().to_string(),
            source,
        })?;
        Self::parse(&contents)
    }

    pub fn parse(contents: &str) -> Result<Self, ConfigError> {
        let value: toml::Value = toml::from_str(contents)?;

        let mode = value
            .get("style")
            .and_then(toml::Value::as_table)
            .and_then(|style| style.get("mode"))
            .and_then(toml::Value::as_str);
        if let Some(mode) = mode {
            if !matches!(mode, "scss" | "css" | "tailwind") {
                return Err(ConfigError::InvalidStyleMode {
                    value: mode.to_owned(),
                });
            }
        }

        Ok(value.try_into()?)
    }

    pub fn theme_tokens(&self) -> ThemeTokens {
        let mut tokens = self.legacy_tokens.0.clone();
        tokens.extend(self.theme.tokens.0.clone());
        ThemeTokens(tokens)
    }
}
