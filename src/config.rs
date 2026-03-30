use crate::alert::AlertRule;
use crate::history::HistoryConfig;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub general: GeneralConfig,
    pub otel: OtelConfig,
    pub web: WebConfig,
    pub ssh: SshConfig,
    pub history: HistoryTomlConfig,
    #[serde(default)]
    pub alert: Vec<AlertRule>,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct HistoryTomlConfig {
    pub enabled: bool,
    pub interval_secs: u64,
    pub retention_days: u32,
    pub path: Option<String>,
}

impl Default for HistoryTomlConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_secs: 60,
            retention_days: 7,
            path: None,
        }
    }
}

impl From<&HistoryTomlConfig> for HistoryConfig {
    fn from(toml: &HistoryTomlConfig) -> Self {
        Self {
            enabled: toml.enabled,
            interval_secs: toml.interval_secs,
            retention_days: toml.retention_days,
            path: toml.path.as_ref().map(PathBuf::from),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct GeneralConfig {
    pub lang: String,
    pub interval_ms: u64,
    pub default_view: String,
    pub history_size: usize,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            lang: "en".to_string(),
            interval_ms: 1000,
            default_view: "dashboard".to_string(),
            history_size: 60,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct OtelConfig {
    pub endpoint: String,
    pub interval_secs: u64,
}

impl Default for OtelConfig {
    fn default() -> Self {
        Self {
            endpoint: "http://localhost:4317".to_string(),
            interval_secs: 5,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct WebConfig {
    pub port: u16,
}

impl Default for WebConfig {
    fn default() -> Self {
        Self { port: 3000 }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct SshConfig {
    pub hosts: Vec<String>,
}

impl Config {
    pub fn load() -> Self {
        let path = match Self::config_path() {
            Some(p) => p,
            None => return Self::default(),
        };

        if !path.exists() {
            return Self::default();
        }

        let contents = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("warning: could not read {}: {}", path.display(), e);
                return Self::default();
            }
        };

        match toml::from_str(&contents) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("warning: could not parse {}: {}", path.display(), e);
                Self::default()
            }
        }
    }

    fn config_path() -> Option<PathBuf> {
        // Try $XDG_CONFIG_HOME first, then fall back to ~/.config
        let config_dir = if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            PathBuf::from(xdg)
        } else {
            let home = std::env::var("HOME").ok()?;
            PathBuf::from(home).join(".config")
        };

        Some(config_dir.join("syslenz").join("config.toml"))
    }
}
