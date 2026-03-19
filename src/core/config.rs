use serde::Deserialize;
use std::path::{PathBuf};

const DEFAULT_CSS: &str = include_str!("../../assets/default.css");

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub theme: ThemeConfig,

    #[serde(default)]
    pub shortcuts: Vec<ShortcutConfig>,
}

#[derive(Debug, Deserialize, Default)]
pub struct ThemeConfig {
    pub css_file: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ShortcutConfig {
    pub label: String,
    pub path: String,
}

impl AppConfig {
    /// Tries to load config from the standard locations in order:
    ///   1. `~/.config/cheese-manager/config.toml`
    ///   2. `/etc/cheese-manager/config.toml`
    ///
    /// Returns a default [`AppConfig`] (no shortcuts, default theme) if
    /// neither file exists or can be parsed.
    pub fn load() -> Self {
        let candidates = config_candidates();

        for path in candidates {
            if let Ok(raw) = std::fs::read_to_string(&path) {
                match toml::from_str::<AppConfig>(&raw) {
                    Ok(cfg) => {
                        log::info!("Loaded config from {}", path.display());
                        return cfg;
                    }
                    Err(e) => {
                        log::warn!("Failed to parse {}: {e}", path.display());
                    }
                }
            }
        }

        log::info!("No config file found, using defaults.");
        AppConfig::default()
    }

    /// Returns the CSS string to apply: user theme if provided and readable,
    /// otherwise the compiled-in default.
    pub fn css(&self) -> String {
        if let Some(ref css_path) = self.theme.css_file {
            let expanded = expand_tilde(css_path);
            if let Ok(user_css) = std::fs::read_to_string(&expanded) {
                log::info!("Using user CSS from {}", expanded.display());
                return user_css;
            }
            log::warn!(
                "Could not read theme CSS at '{}', falling back to default.",
                expanded.display()
            );
        }
        DEFAULT_CSS.to_owned()
    }

    /// Returns shortcuts whose paths actually exist on this machine.
    pub fn valid_shortcuts(&self) -> Vec<ShortcutConfig> {
        self.shortcuts
            .iter()
            .filter(|s| expand_tilde(&s.path).exists())
            .cloned()
            .collect()
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: ThemeConfig::default(),
            shortcuts: Vec::new(),
        }
    }
}

fn config_candidates() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Some(home) = home_dir() {
        paths.push(home.join(".config/cheese-manager/config.toml"));
    }
    paths.push(PathBuf::from("/etc/cheese-manager/config.toml"));

    paths
}

/// Expands a leading `~` to the user's home directory.
pub fn expand_tilde(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/") {
        if let Some(home) = home_dir() {
            return home.join(rest);
        }
    }
    PathBuf::from(path)
}

fn home_dir() -> Option<PathBuf> {
    std::env::var("HOME").ok().map(PathBuf::from)
}

/// Resolves a shortcut path string to an absolute [`PathBuf`],
/// expanding `~` if present.
pub fn resolve_path(raw: &str) -> PathBuf {
    expand_tilde(raw)
}

pub fn default_start_path() -> PathBuf {
    home_dir().unwrap_or_else(|| PathBuf::from("/"))
}