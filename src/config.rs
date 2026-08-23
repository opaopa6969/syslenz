use crate::alert::AlertRule;
use crate::history::HistoryConfig;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct RunbookConfig {
    pub pattern: String,
    pub url: String,
}

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
    #[serde(default)]
    pub diagnostic_runbook: Vec<RunbookConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct WebConfig {
    pub port: u16,
    /// キャプチャ間隔（秒）。既定 1。
    pub capture_interval_secs: u64,
    /// メモリ内履歴の件数上限。既定 60。
    pub max_history_count: usize,
    /// メモリ内履歴の合計バイト数上限（概算）。既定 64 * 1024 * 1024 (64 MB)。
    /// 0 でバイト数上限を無効化（件数上限のみ）。
    pub max_history_bytes: usize,
    /// 履歴内の巨大テーブルを縮約するか。既定 true。
    /// 縮約時、履歴（最新以外）の Table フィールドは
    /// 先頭数行＋ "[truncated: N rows]" の要約に置き換わる。
    pub truncate_large_tables: bool,
    /// 履歴保持中にテーブルを縮約する行数しきい値。既定 20。
    pub truncate_table_rows: usize,
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            capture_interval_secs: 1,
            max_history_count: 60,
            max_history_bytes: 64 * 1024 * 1024,
            truncate_large_tables: true,
            truncate_table_rows: 20,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct SshConfig {
    pub hosts: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Default values ---

    #[test]
    fn general_config_defaults() {
        let cfg = GeneralConfig::default();
        assert_eq!(cfg.lang, "en");
        assert_eq!(cfg.interval_ms, 1000);
        assert_eq!(cfg.default_view, "dashboard");
        assert_eq!(cfg.history_size, 60);
    }

    #[test]
    fn otel_config_defaults() {
        let cfg = OtelConfig::default();
        assert_eq!(cfg.endpoint, "http://localhost:4317");
        assert_eq!(cfg.interval_secs, 5);
    }

    #[test]
    fn web_config_default_port() {
        let cfg = WebConfig::default();
        assert_eq!(cfg.port, 3000);
        assert_eq!(cfg.capture_interval_secs, 1);
        assert_eq!(cfg.max_history_count, 60);
        assert_eq!(cfg.max_history_bytes, 64 * 1024 * 1024);
        assert!(cfg.truncate_large_tables);
        assert_eq!(cfg.truncate_table_rows, 20);
    }

    #[test]
    fn history_toml_config_defaults() {
        let cfg = HistoryTomlConfig::default();
        assert!(cfg.enabled);
        assert_eq!(cfg.interval_secs, 60);
        assert_eq!(cfg.retention_days, 7);
        assert!(cfg.path.is_none());
    }

    #[test]
    fn config_default_has_empty_alert_and_runbook() {
        let cfg = Config::default();
        assert!(cfg.alert.is_empty());
        assert!(cfg.diagnostic_runbook.is_empty());
        assert!(cfg.ssh.hosts.is_empty());
    }

    // --- TOML parsing ---

    #[test]
    fn parse_general_section_overrides_defaults() {
        let toml = r#"
[general]
lang = "ja"
interval_ms = 500
default_view = "dashboard"
history_size = 120
"#;
        let cfg: Config = toml::from_str(toml).unwrap();
        assert_eq!(cfg.general.lang, "ja");
        assert_eq!(cfg.general.interval_ms, 500);
        assert_eq!(cfg.general.history_size, 120);
    }

    #[test]
    fn parse_otel_section() {
        let toml = r#"
[otel]
endpoint = "http://otel-collector:4317"
interval_secs = 10
"#;
        let cfg: Config = toml::from_str(toml).unwrap();
        assert_eq!(cfg.otel.endpoint, "http://otel-collector:4317");
        assert_eq!(cfg.otel.interval_secs, 10);
    }

    #[test]
    fn parse_web_section() {
        let toml = r#"
[web]
port = 8080
"#;
        let cfg: Config = toml::from_str(toml).unwrap();
        assert_eq!(cfg.web.port, 8080);
    }

    #[test]
    fn parse_history_section_with_path() {
        let toml = r#"
[history]
enabled = false
interval_secs = 30
retention_days = 14
path = "/var/log/syslenz"
"#;
        let cfg: Config = toml::from_str(toml).unwrap();
        assert!(!cfg.history.enabled);
        assert_eq!(cfg.history.interval_secs, 30);
        assert_eq!(cfg.history.retention_days, 14);
        assert_eq!(cfg.history.path.as_deref(), Some("/var/log/syslenz"));
    }

    #[test]
    fn parse_ssh_hosts() {
        let toml = r#"
[ssh]
hosts = ["192.168.1.10", "192.168.1.11"]
"#;
        let cfg: Config = toml::from_str(toml).unwrap();
        assert_eq!(cfg.ssh.hosts.len(), 2);
        assert_eq!(cfg.ssh.hosts[0], "192.168.1.10");
    }

    #[test]
    fn parse_alert_rules() {
        let toml = r#"
[[alert]]
source = "loadavg"
field = "load_1min"
condition = "> 8.0"
severity = "warning"
message = "High load"

[[alert]]
source = "meminfo"
field = "MemAvailable"
condition = "< 500000000"
severity = "critical"
message = "Low memory"
"#;
        let cfg: Config = toml::from_str(toml).unwrap();
        assert_eq!(cfg.alert.len(), 2);
        assert_eq!(cfg.alert[0].source, "loadavg");
        assert_eq!(cfg.alert[1].severity, "critical");
    }

    #[test]
    fn parse_diagnostic_runbook() {
        let toml = r#"
[[diagnostic_runbook]]
pattern = "MemAvailable"
url = "https://wiki.example.com/memory"
"#;
        let cfg: Config = toml::from_str(toml).unwrap();
        assert_eq!(cfg.diagnostic_runbook.len(), 1);
        assert_eq!(cfg.diagnostic_runbook[0].pattern, "MemAvailable");
        assert!(cfg.diagnostic_runbook[0].url.contains("memory"));
    }

    #[test]
    fn empty_toml_uses_all_defaults() {
        let cfg: Config = toml::from_str("").unwrap();
        assert_eq!(cfg.general.lang, "en");
        assert_eq!(cfg.otel.interval_secs, 5);
        assert_eq!(cfg.web.port, 3000);
    }

    // --- HistoryTomlConfig -> HistoryConfig conversion ---

    #[test]
    fn history_toml_to_history_config_no_path() {
        let toml_cfg = HistoryTomlConfig::default();
        let hist: HistoryConfig = (&toml_cfg).into();
        assert!(hist.enabled);
        assert_eq!(hist.interval_secs, 60);
        assert_eq!(hist.retention_days, 7);
        assert!(hist.path.is_none());
    }

    #[test]
    fn history_toml_to_history_config_with_path() {
        let toml_cfg = HistoryTomlConfig {
            enabled: false,
            interval_secs: 120,
            retention_days: 30,
            path: Some("/tmp/syslenz-history".to_string()),
        };
        let hist: HistoryConfig = (&toml_cfg).into();
        assert!(!hist.enabled);
        assert_eq!(hist.interval_secs, 120);
        assert_eq!(hist.path, Some(PathBuf::from("/tmp/syslenz-history")));
    }

    // --- config_path resolution ---

    #[test]
    fn config_path_uses_xdg_config_home() {
        // Set XDG_CONFIG_HOME to a known value and verify the path is composed correctly.
        // We use a temp dir to avoid any side effects.
        let tmp = tempfile::TempDir::new().unwrap();
        let xdg_path = tmp.path().to_str().unwrap().to_string();

        // Temporarily override env var using a separate scope to avoid test interference.
        // SAFETY: single-threaded test; no other threads read this env var concurrently.
        unsafe {
            std::env::set_var("XDG_CONFIG_HOME", &xdg_path);
        }
        let path = Config::config_path().unwrap();
        unsafe {
            std::env::remove_var("XDG_CONFIG_HOME");
        }

        assert!(path.starts_with(tmp.path()));
        assert!(path.ends_with("syslenz/config.toml"));
    }

    #[test]
    fn runbook_config_roundtrip_serialization() {
        let rc = RunbookConfig {
            pattern: "cpu_usage".to_string(),
            url: "https://runbook.example.com/cpu".to_string(),
        };
        let json = serde_json::to_string(&rc).unwrap();
        let rc2: RunbookConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(rc2.pattern, "cpu_usage");
        assert_eq!(rc2.url, "https://runbook.example.com/cpu");
    }
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

    pub fn config_path() -> Option<PathBuf> {
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
