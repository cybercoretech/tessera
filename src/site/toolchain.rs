use crate::token::TokenSet;
use std::{
    collections::HashSet,
    fs,
    path::{Component, Path, PathBuf},
    process::Command,
};
use tessera_config::{ProjectConfig, SassCompiler, StyleSource, UtilityEngine};

#[derive(Debug, Clone)]
struct ToolPaths {
    tailwind: PathBuf,
    unocss: PathBuf,
    esbuild: PathBuf,
    tsc: PathBuf,
}

impl Default for ToolPaths {
    fn default() -> Self {
        Self {
            tailwind: PathBuf::from("tailwindcss"),
            unocss: PathBuf::from("unocss"),
            esbuild: PathBuf::from("esbuild"),
            tsc: PathBuf::from("tsc"),
        }
    }
}

/// Lists external tools required by the selected build configuration.
pub fn required_tools(config: &ProjectConfig) -> Vec<(&'static str, Option<u64>)> {
    let mut tools = Vec::new();
    if config.style.source == StyleSource::Scss
        && config.style.sass_compiler == SassCompiler::DartSass
    {
        tools.push(("sass", None));
    }
    match config.style.utility {
        UtilityEngine::None => {}
        UtilityEngine::Tailwind => tools.push(("tailwindcss", Some(4))),
        UtilityEngine::UnoCss => tools.push(("unocss", None)),
    }
    if config.scripts.enabled {
        tools.push(("esbuild", None));
        if config.scripts.typecheck {
            tools.push(("tsc", None));
        }
    }
    tools
}

/// Checks an installed tool and optionally verifies its major version.
pub fn check_tool(program: &str, expected_major: Option<u64>) -> Result<String, String> {
    let output = Command::new(program)
        .arg("--version")
        .output()
        .map_err(|error| format!("{program} is unavailable: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "{program} --version exited with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let version = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if let Some(expected_major) = expected_major {
        let major = version
            .split(|character: char| !character.is_ascii_digit())
            .find(|part| !part.is_empty())
            .and_then(|part| part.parse::<u64>().ok());
        if major != Some(expected_major) {
            return Err(format!(
                "{program} version is {version:?}; expected major version {expected_major}"
            ));
        }
    }
    Ok(version)
}

/// Builds the configured optional utility-engine stylesheet, if one is selected.
pub fn compile_utility_css(
    config: &ProjectConfig,
    tokens: &TokenSet,
    project_root: &Path,
    scratch_dir: &Path,
) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    compile_utility_css_with_tools(
        config,
        tokens,
        project_root,
        scratch_dir,
        &ToolPaths::default(),
    )
}

fn compile_utility_css_with_tools(
    config: &ProjectConfig,
    tokens: &TokenSet,
    project_root: &Path,
    scratch_dir: &Path,
    tools: &ToolPaths,
) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    match config.style.utility {
        UtilityEngine::None => Ok(None),
        UtilityEngine::Tailwind => {
            compile_tailwind(tokens, project_root, scratch_dir, tools).map(Some)
        }
        UtilityEngine::UnoCss => compile_unocss(config, project_root, scratch_dir, tools).map(Some),
    }
}

fn compile_tailwind(
    tokens: &TokenSet,
    project_root: &Path,
    scratch_dir: &Path,
    tools: &ToolPaths,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let input_path = scratch_dir.join("tailwind-input.css");
    let output_path = scratch_dir.join("tailwind-output.css");
    let mut input = String::from("@import \"tailwindcss\";\n");
    let theme_tokens = tokens
        .iter()
        .filter_map(|(name, _)| {
            let normalized = normalize_token_name(name)?;
            let (namespace, name) = if let Some(name) = normalized.strip_prefix("font-") {
                ("font", name)
            } else if let Some(name) = normalized.strip_prefix("spacing-") {
                ("spacing", name)
            } else {
                ("color", normalized.as_str())
            };
            Some(format!("  --{namespace}-{name}: var(--ts-{normalized});\n"))
        })
        .collect::<String>();
    if !theme_tokens.is_empty() {
        input.push_str("@theme inline {\n");
        input.push_str(&theme_tokens);
        input.push_str("}\n");
    }
    fs::write(&input_path, input)?;

    let output = Command::new(&tools.tailwind)
        .current_dir(project_root)
        .arg("-i")
        .arg(&input_path)
        .arg("-o")
        .arg(&output_path)
        .output()
        .map_err(|error| {
            format!("could not start Tailwind CLI `tailwindcss`; install it to enable style.utility = \"tailwind\": {error}")
        })?;
    ensure_success("Tailwind", &output)?;
    Ok(fs::read_to_string(output_path)?)
}

fn compile_unocss(
    config: &ProjectConfig,
    project_root: &Path,
    scratch_dir: &Path,
    tools: &ToolPaths,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let options = &config.style.unocss;
    let config_path = options.config.as_ref().map(|path| project_root.join(path));
    let detected_config = [
        "uno.config.ts",
        "uno.config.js",
        "uno.config.mts",
        "uno.config.mjs",
        "unocss.config.ts",
        "unocss.config.js",
        "unocss.config.mts",
        "unocss.config.mjs",
    ]
    .iter()
    .map(|path| project_root.join(path))
    .find(|path| path.is_file());
    let selected_config = config_path.or(detected_config);

    if let Some(path) = &selected_config {
        if !path.is_file() {
            return Err(format!("UnoCSS config does not exist: {}", path.display()).into());
        }
    } else if options.preset.is_none() {
        return Err(
            "UnoCSS requires an explicit style.unocss.preset or a uno.config.* file".into(),
        );
    }

    let output_path = scratch_dir.join("unocss-output.css");
    let mut command = Command::new(&tools.unocss);
    command.current_dir(project_root);
    for pattern in [
        "content/**/*.md",
        "content/**/*.html",
        "layouts/**/*.html",
        "templates/**/*.html",
        "src/**/*.{html,js,jsx,ts,tsx,vue,svelte}",
        "scripts/**/*.{js,jsx,ts,tsx}",
    ] {
        command.arg(pattern);
    }
    command.arg("--out-file").arg(&output_path);
    if let Some(path) = selected_config {
        command.arg("--config").arg(path);
    }
    if let Some(preset) = &options.preset {
        if !matches!(preset.as_str(), "wind3" | "wind4") {
            return Err(format!(
                "unsupported UnoCSS CLI preset {preset:?}; choose wind3 or wind4, or use a config file"
            )
            .into());
        }
        command.arg("--preset").arg(preset);
    }

    let output = command.output().map_err(|error| {
        format!("could not start UnoCSS CLI `unocss`; install @unocss/cli to enable style.utility = \"uno-css\": {error}")
    })?;
    ensure_success("UnoCSS", &output)?;
    Ok(fs::read_to_string(output_path)?)
}

/// Bundles configured browser script entry points into the staged output directory.
pub fn bundle_scripts(
    config: &ProjectConfig,
    project_root: &Path,
    output_root: &Path,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error + Send + Sync>> {
    bundle_scripts_with_tools(config, project_root, output_root, &ToolPaths::default())
}

fn bundle_scripts_with_tools(
    config: &ProjectConfig,
    project_root: &Path,
    output_root: &Path,
    tools: &ToolPaths,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error + Send + Sync>> {
    let scripts = &config.scripts;
    if !scripts.enabled {
        return Ok(Vec::new());
    }

    let entries = scripts
        .entry_points
        .iter()
        .map(|entry| validate_entry(project_root, entry))
        .collect::<Result<Vec<_>, _>>()?;
    if entries.is_empty() {
        return Err("scripts.enabled is true but scripts.entry_points is empty".into());
    }

    let mut outputs = Vec::with_capacity(entries.len());
    let mut output_names = HashSet::new();
    for entry in &entries {
        let extension = entry.extension().and_then(|value| value.to_str());
        if !matches!(extension, Some("js" | "jsx" | "ts" | "tsx" | "mts" | "cts")) {
            return Err(
                format!("unsupported script entry extension for {}", entry.display()).into(),
            );
        }
        let stem = entry
            .file_stem()
            .and_then(|value| value.to_str())
            .ok_or_else(|| format!("invalid script entry name: {}", entry.display()))?;
        if !stem
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_'))
        {
            return Err(format!("script entry has an unsafe output name: {stem:?}").into());
        }
        if !output_names.insert(stem.to_owned()) {
            return Err(format!(
                "multiple script entry points produce the same output name: {stem}.js"
            )
            .into());
        }
        let output = output_root.join("assets").join(format!("{stem}.js"));
        fs::create_dir_all(output.parent().expect("asset output has parent"))?;

        let result = Command::new(&tools.esbuild)
            .current_dir(project_root)
            .arg(entry)
            .arg("--bundle")
            .arg("--format=esm")
            .arg("--target=es2020")
            .arg(format!("--outfile={}", output.display()))
            .output()
            .map_err(|error| {
                format!("could not start esbuild; install it to enable scripts.enabled: {error}")
            })?;
        ensure_success("esbuild", &result)?;
        if !output.is_file() {
            return Err(format!("esbuild did not create {}", output.display()).into());
        }
        outputs.push(PathBuf::from("assets").join(format!("{stem}.js")));
    }

    if scripts.typecheck {
        if !entries.iter().any(|entry| {
            matches!(
                entry.extension().and_then(|value| value.to_str()),
                Some("ts" | "tsx" | "mts" | "cts")
            )
        }) {
            return Err(
                "scripts.typecheck is enabled but no TypeScript entry point is configured".into(),
            );
        }
        let mut command = Command::new(&tools.tsc);
        command.current_dir(project_root).arg("--noEmit");
        command.args(&entries);
        let result = command.output().map_err(|error| {
            format!("could not start TypeScript compiler `tsc`; install TypeScript to enable scripts.typecheck: {error}")
        })?;
        ensure_success("TypeScript type check", &result)?;
    }

    Ok(outputs)
}

fn validate_entry(
    project_root: &Path,
    entry: &Path,
) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    if entry.is_absolute()
        || entry
            .components()
            .any(|component| !matches!(component, Component::Normal(_) | Component::CurDir))
    {
        return Err(format!(
            "script entry must be a project-relative path without traversal: {}",
            entry.display()
        )
        .into());
    }
    let full_path = project_root.join(entry);
    if !full_path.is_file() {
        return Err(format!("script entry does not exist: {}", entry.display()).into());
    }
    Ok(full_path)
}

fn normalize_token_name(name: &str) -> Option<String> {
    let normalized = name.replace('_', "-");
    if !normalized.is_empty()
        && normalized
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '-')
    {
        Some(normalized)
    } else {
        None
    }
}

fn ensure_success(
    tool: &str,
    output: &std::process::Output,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "{tool} exited with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        )
        .into())
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::{bundle_scripts_with_tools, compile_utility_css_with_tools, ToolPaths};
    use crate::test_support::{exec_lock, fake_program};
    use crate::token::TokenSet;
    use std::{fs, path::Path};
    use tempfile::tempdir;
    use tessera_config::ProjectConfig;

    fn tool_paths(root: &Path) -> ToolPaths {
        ToolPaths {
            tailwind: root.join("tailwindcss"),
            unocss: root.join("unocss"),
            esbuild: root.join("esbuild"),
            tsc: root.join("tsc"),
        }
    }

    #[test]
    fn tailwind_uses_token_theme_and_emits_cli_output() {
        let _exec = exec_lock();
        let root = tempdir().unwrap();
        let scratch = root.path().join("scratch");
        fs::create_dir(&scratch).unwrap();
        let tools = tool_paths(root.path());
        fake_program(
            &tools.tailwind,
            "while [ \"$#\" -gt 0 ]; do if [ \"$1\" = \"-o\" ]; then shift; printf '/* tailwind output */' > \"$1\"; fi; shift; done",
        );
        let config =
            ProjectConfig::parse("name = \"test\"\n[style]\nutility = \"tailwind\"\n").unwrap();
        let tokens =
            TokenSet::from_toml("name = \"test\"\n[theme.tokens]\nprimary = \"#123456\"\n")
                .unwrap();

        let css = compile_utility_css_with_tools(&config, &tokens, root.path(), &scratch, &tools)
            .unwrap()
            .unwrap();

        assert!(css.contains("tailwind output"));
        let input = fs::read_to_string(scratch.join("tailwind-input.css")).unwrap();
        assert!(input.contains("@import \"tailwindcss\""));
        assert!(input.contains("--color-primary: var(--ts-primary)"));
    }

    #[test]
    fn reports_missing_tailwind_cli_without_installing_it() {
        let _exec = exec_lock();
        let root = tempdir().unwrap();
        let scratch = root.path().join("scratch");
        fs::create_dir(&scratch).unwrap();
        let config =
            ProjectConfig::parse("name = \"test\"\n[style]\nutility = \"tailwind\"\n").unwrap();
        let tools = tool_paths(root.path());

        let error = compile_utility_css_with_tools(
            &config,
            &TokenSet::default(),
            root.path(),
            &scratch,
            &tools,
        )
        .unwrap_err()
        .to_string();

        assert!(error.contains("install it to enable style.utility"));
    }

    #[test]
    fn reports_tailwind_cli_failure_output() {
        let _exec = exec_lock();
        let root = tempdir().unwrap();
        let scratch = root.path().join("scratch");
        fs::create_dir(&scratch).unwrap();
        let tools = tool_paths(root.path());
        fake_program(&tools.tailwind, "echo expected failure >&2\nexit 7");
        let config =
            ProjectConfig::parse("name = \"test\"\n[style]\nutility = \"tailwind\"\n").unwrap();

        let error = compile_utility_css_with_tools(
            &config,
            &TokenSet::default(),
            root.path(),
            &scratch,
            &tools,
        )
        .unwrap_err()
        .to_string();

        assert!(error.contains("Tailwind exited with"));
        assert!(error.contains("expected failure"));
    }

    #[test]
    fn check_tool_rejects_unsupported_tailwind_major_version() {
        let _exec = exec_lock();
        let root = tempdir().unwrap();
        let executable = root.path().join("tailwindcss");
        fake_program(&executable, "echo 'tailwindcss v3.4.0'");

        let error = super::check_tool(executable.to_str().unwrap(), Some(4)).unwrap_err();

        assert!(error.contains("expected major version 4"), "{error}");
    }

    #[test]
    fn unocss_requires_explicit_preset_or_configuration() {
        let root = tempdir().unwrap();
        let scratch = root.path().join("scratch");
        fs::create_dir(&scratch).unwrap();
        let config =
            ProjectConfig::parse("name = \"test\"\n[style]\nutility = \"uno-css\"\n").unwrap();

        let error = compile_utility_css_with_tools(
            &config,
            &TokenSet::default(),
            root.path(),
            &scratch,
            &tool_paths(root.path()),
        )
        .unwrap_err()
        .to_string();

        assert!(error.contains("explicit style.unocss.preset"));
    }

    #[test]
    fn unocss_uses_selected_preset_and_emits_cli_output() {
        let _exec = exec_lock();
        let root = tempdir().unwrap();
        let scratch = root.path().join("scratch");
        fs::create_dir(&scratch).unwrap();
        let tools = tool_paths(root.path());
        fake_program(
            &tools.unocss,
            "while [ \"$#\" -gt 0 ]; do if [ \"$1\" = \"--out-file\" ]; then shift; printf '/* uno output */' > \"$1\"; fi; shift; done",
        );
        let config = ProjectConfig::parse(
            "name = \"test\"\n[style]\nutility = \"uno-css\"\n[style.unocss]\npreset = \"wind4\"\n",
        )
        .unwrap();

        let css = compile_utility_css_with_tools(
            &config,
            &TokenSet::default(),
            root.path(),
            &scratch,
            &tools,
        )
        .unwrap()
        .unwrap();

        assert!(css.contains("uno output"));
    }

    #[test]
    fn reports_missing_unocss_cli_without_installing_it() {
        let _exec = exec_lock();
        let root = tempdir().unwrap();
        let scratch = root.path().join("scratch");
        fs::create_dir(&scratch).unwrap();
        let tools = tool_paths(root.path());
        let config = ProjectConfig::parse(
            "name = \"test\"\n[style]\nutility = \"uno-css\"\n[style.unocss]\npreset = \"wind4\"\n",
        )
        .unwrap();

        let error = compile_utility_css_with_tools(
            &config,
            &TokenSet::default(),
            root.path(),
            &scratch,
            &tools,
        )
        .unwrap_err()
        .to_string();

        assert!(error.contains("install @unocss/cli"));
    }

    #[test]
    fn reports_unocss_cli_failure_output() {
        let _exec = exec_lock();
        let root = tempdir().unwrap();
        let scratch = root.path().join("scratch");
        fs::create_dir(&scratch).unwrap();
        let tools = tool_paths(root.path());
        fake_program(&tools.unocss, "echo expected UnoCSS failure >&2\nexit 9");
        let config = ProjectConfig::parse(
            "name = \"test\"\n[style]\nutility = \"uno-css\"\n[style.unocss]\npreset = \"wind4\"\n",
        )
        .unwrap();

        let error = compile_utility_css_with_tools(
            &config,
            &TokenSet::default(),
            root.path(),
            &scratch,
            &tools,
        )
        .unwrap_err()
        .to_string();

        assert!(error.contains("UnoCSS exited with"));
        assert!(error.contains("expected UnoCSS failure"));
    }

    #[test]
    fn esbuild_and_tsc_are_separately_invoked_for_typescript() {
        let _exec = exec_lock();
        let root = tempdir().unwrap();
        let scripts_dir = root.path().join("scripts");
        fs::create_dir(&scripts_dir).unwrap();
        fs::write(scripts_dir.join("main.ts"), "const answer: number = 42;").unwrap();
        let output_dir = root.path().join("dist");
        let tools = tool_paths(root.path());
        fake_program(
            &tools.esbuild,
            "for arg in \"$@\"; do case \"$arg\" in --outfile=*) out=${arg#--outfile=}; printf 'export {};\\n' > \"$out\" ;; esac; done",
        );
        fake_program(&tools.tsc, "exit 0");
        let config = ProjectConfig::parse(
            "name = \"test\"\n[scripts]\nenabled = true\nentry_points = [\"scripts/main.ts\"]\ntypecheck = true\n",
        )
        .unwrap();

        let outputs = bundle_scripts_with_tools(&config, root.path(), &output_dir, &tools).unwrap();

        assert_eq!(outputs, [Path::new("assets/main.js")]);
        assert!(output_dir.join("assets/main.js").is_file());
    }

    #[test]
    fn reports_missing_esbuild_and_typescript_tools() {
        let _exec = exec_lock();
        let root = tempdir().unwrap();
        let scripts_dir = root.path().join("scripts");
        fs::create_dir(&scripts_dir).unwrap();
        fs::write(scripts_dir.join("main.ts"), "export {};").unwrap();
        let config = ProjectConfig::parse(
            "name = \"test\"\n[scripts]\nenabled = true\nentry_points = [\"scripts/main.ts\"]\ntypecheck = true\n",
        )
        .unwrap();
        let tools = tool_paths(root.path());

        let error =
            bundle_scripts_with_tools(&config, root.path(), &root.path().join("dist"), &tools)
                .unwrap_err()
                .to_string();

        assert!(error.contains("could not start esbuild"));
    }

    #[test]
    fn reports_typecheck_failure_after_bundling() {
        let _exec = exec_lock();
        let root = tempdir().unwrap();
        let scripts_dir = root.path().join("scripts");
        fs::create_dir(&scripts_dir).unwrap();
        fs::write(scripts_dir.join("main.ts"), "export {};").unwrap();
        let output_dir = root.path().join("dist");
        let tools = tool_paths(root.path());
        fake_program(
            &tools.esbuild,
            "for arg in \"$@\"; do case \"$arg\" in --outfile=*) out=${arg#--outfile=}; printf 'export {};\\n' > \"$out\" ;; esac; done",
        );
        fake_program(&tools.tsc, "echo expected type error >&2\nexit 2");
        let config = ProjectConfig::parse(
            "name = \"test\"\n[scripts]\nenabled = true\nentry_points = [\"scripts/main.ts\"]\ntypecheck = true\n",
        )
        .unwrap();

        let error =
            bundle_scripts_with_tools(&config, root.path(), &output_dir, &tools).unwrap_err();

        assert!(error
            .to_string()
            .contains("TypeScript type check exited with"));
        assert!(error.to_string().contains("expected type error"));
        assert!(output_dir.join("assets/main.js").is_file());
    }

    #[test]
    fn reports_missing_typescript_compiler_after_bundling() {
        let _exec = exec_lock();
        let root = tempdir().unwrap();
        let scripts_dir = root.path().join("scripts");
        fs::create_dir(&scripts_dir).unwrap();
        fs::write(scripts_dir.join("main.ts"), "export {};").unwrap();
        let output_dir = root.path().join("dist");
        let tools = tool_paths(root.path());
        fake_program(
            &tools.esbuild,
            "for arg in \"$@\"; do case \"$arg\" in --outfile=*) out=${arg#--outfile=}; printf 'export {};\\n' > \"$out\" ;; esac; done",
        );
        let config = ProjectConfig::parse(
            "name = \"test\"\n[scripts]\nenabled = true\nentry_points = [\"scripts/main.ts\"]\ntypecheck = true\n",
        )
        .unwrap();

        let error =
            bundle_scripts_with_tools(&config, root.path(), &output_dir, &tools).unwrap_err();

        assert!(error
            .to_string()
            .contains("could not start TypeScript compiler"));
    }

    #[test]
    fn rejects_script_paths_outside_the_project() {
        let root = tempdir().unwrap();
        let config = ProjectConfig::parse(
            "name = \"test\"\n[scripts]\nenabled = true\nentry_points = [\"../outside.ts\"]\n",
        )
        .unwrap();

        let error = bundle_scripts_with_tools(
            &config,
            root.path(),
            &root.path().join("dist"),
            &tool_paths(root.path()),
        )
        .unwrap_err()
        .to_string();

        assert!(error.contains("without traversal"));
    }

    #[test]
    fn disabled_script_pipeline_does_not_require_or_invoke_tools() {
        let root = tempdir().unwrap();
        let config = ProjectConfig::parse("name = \"test\"\n").unwrap();

        let outputs = bundle_scripts_with_tools(
            &config,
            root.path(),
            &root.path().join("dist"),
            &tool_paths(root.path()),
        )
        .unwrap();

        assert!(outputs.is_empty());
    }
}
