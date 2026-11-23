use std::{collections::BTreeMap, fs, path::Path, path::PathBuf};

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
    pub scripts: ScriptConfig,
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
    #[serde(default)]
    pub source: StyleSource,
    #[serde(default)]
    pub sass_compiler: SassCompiler,
    #[serde(default)]
    pub utility: UtilityEngine,
    #[serde(default)]
    pub unocss: UnoCssConfig,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StyleMode {
    #[default]
    Scss,
    Css,
    Tailwind,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StyleSource {
    #[default]
    Css,
    Scss,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SassCompiler {
    #[default]
    Grass,
    DartSass,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UtilityEngine {
    #[default]
    None,
    Tailwind,
    UnoCss,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnoCssConfig {
    #[serde(default)]
    pub preset: Option<String>,
    #[serde(default)]
    pub config: Option<PathBuf>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScriptConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub entry_points: Vec<PathBuf>,
    #[serde(default)]
    pub typecheck: bool,
    #[serde(default)]
    pub interactions: bool,
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
    #[error("style.mode {mode:?} conflicts with style.source {source_name:?}")]
    ConflictingStyleSource { mode: String, source_name: String },
    #[error("style.mode = \"tailwind\" conflicts with style.utility {utility:?}")]
    ConflictingUtilityEngine { utility: String },
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
        let mut value: toml::Value = toml::from_str(contents)?;
        let normalize_style_mode = value
            .get("style")
            .and_then(toml::Value::as_table)
            .is_some_and(|style| {
                style.contains_key("mode")
                    || style.contains_key("source")
                    || style.contains_key("utility")
            });

        if let Some(style) = value.get_mut("style").and_then(toml::Value::as_table_mut) {
            if let Some(mode_value) = style.get("mode") {
                let mode = mode_value
                    .as_str()
                    .ok_or_else(|| ConfigError::InvalidStyleMode {
                        value: mode_value.to_string(),
                    })?;
                let (legacy_source, legacy_utility) = match mode {
                    "scss" => ("scss", None),
                    "css" => ("css", None),
                    "tailwind" => ("scss", Some("tailwind")),
                    _ => {
                        return Err(ConfigError::InvalidStyleMode {
                            value: mode.to_owned(),
                        });
                    }
                };

                if let Some(source_value) = style.get("source") {
                    if let Some(source_value) = source_value.as_str() {
                        if source_value != legacy_source {
                            return Err(ConfigError::ConflictingStyleSource {
                                mode: mode.to_owned(),
                                source_name: source_value.to_owned(),
                            });
                        }
                    }
                } else {
                    style.insert(
                        "source".to_owned(),
                        toml::Value::String(legacy_source.to_owned()),
                    );
                }

                if let Some(legacy_utility) = legacy_utility {
                    if let Some(utility_value) = style.get("utility") {
                        if let Some(utility_value) = utility_value.as_str() {
                            if utility_value != legacy_utility {
                                return Err(ConfigError::ConflictingUtilityEngine {
                                    utility: utility_value.to_owned(),
                                });
                            }
                        }
                    } else {
                        style.insert(
                            "utility".to_owned(),
                            toml::Value::String(legacy_utility.to_owned()),
                        );
                    }
                }
            }
        }

        let mut config: Self = value.try_into()?;
        if normalize_style_mode {
            config.style.mode = match config.style.utility {
                UtilityEngine::Tailwind => StyleMode::Tailwind,
                UtilityEngine::None | UtilityEngine::UnoCss => match config.style.source {
                    StyleSource::Css => StyleMode::Css,
                    StyleSource::Scss => StyleMode::Scss,
                },
            };
        }
        Ok(config)
    }

    pub fn theme_tokens(&self) -> ThemeTokens {
        let mut tokens = self.legacy_tokens.0.clone();
        tokens.extend(self.theme.tokens.0.clone());
        ThemeTokens(tokens)
    }
}
