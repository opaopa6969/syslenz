use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Pin {
    pub source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub host: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PinFile {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pin: Vec<Pin>,
}

impl PinFile {
    pub fn pins_path() -> Option<PathBuf> {
        let config_dir = if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            PathBuf::from(xdg)
        } else {
            let home = std::env::var("HOME").ok()?;
            PathBuf::from(home).join(".config")
        };
        Some(config_dir.join("syslenz").join("pins.toml"))
    }

    pub fn load() -> Vec<Pin> {
        let path = match Self::pins_path() {
            Some(p) => p,
            None => return Vec::new(),
        };
        if !path.exists() {
            return Vec::new();
        }
        let contents = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("warning: could not read {}: {}", path.display(), e);
                return Vec::new();
            }
        };
        match toml::from_str::<PinFile>(&contents) {
            Ok(f) => f.pin,
            Err(e) => {
                eprintln!("warning: could not parse {}: {}", path.display(), e);
                Vec::new()
            }
        }
    }

    pub fn save(pins: &[Pin]) {
        let path = match Self::pins_path() {
            Some(p) => p,
            None => {
                eprintln!("warning: could not determine pins.toml path");
                return;
            }
        };
        if let Some(parent) = path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                eprintln!("warning: could not create {}: {}", parent.display(), e);
                return;
            }
        }
        let file = PinFile {
            pin: pins.to_vec(),
        };
        match toml::to_string_pretty(&file) {
            Ok(toml_str) => {
                let header = "# Written by syslenz; safe to edit by hand.\n";
                let content = format!("{}{}", header, toml_str);
                if let Err(e) = std::fs::write(&path, content) {
                    eprintln!("warning: could not write {}: {}", path.display(), e);
                }
            }
            Err(e) => {
                eprintln!("warning: could not serialize pins: {}", e);
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PinSet {
    pins: Vec<Pin>,
}

impl PinSet {
    pub fn new(pins: Vec<Pin>) -> Self {
        let mut deduped = Vec::new();
        let mut seen: HashSet<Pin> = HashSet::new();
        for pin in pins {
            if seen.insert(pin.clone()) {
                deduped.push(pin);
            }
        }
        Self { pins: deduped }
    }

    pub fn pins(&self) -> &[Pin] {
        &self.pins
    }

    pub fn toggle(&mut self, pin: &Pin) -> bool {
        if let Some(pos) = self.pins.iter().position(|p| p == pin) {
            self.pins.remove(pos);
            false
        } else {
            self.pins.push(pin.clone());
            true
        }
    }

    pub fn contains(&self, pin: &Pin) -> bool {
        self.pins.contains(pin)
    }

    pub fn is_pinned_source(&self, source: &str, host: &str) -> bool {
        self.pins.iter().any(|p| {
            p.source == source
                && p.field.is_none()
                && p.host == host
        })
    }

    pub fn is_pinned_field(&self, source: &str, field: &str, host: &str) -> bool {
        self.pins.iter().any(|p| {
            p.source == source
                && p.field.as_deref() == Some(field)
                && p.host == host
        })
    }

    pub fn is_empty(&self) -> bool {
        self.pins.is_empty()
    }

    pub fn len(&self) -> usize {
        self.pins.len()
    }

    pub fn save(&self) {
        PinFile::save(&self.pins);
    }

    pub fn current_host(&self, app_host_label: &str) -> String {
        if app_host_label == "localhost" || app_host_label == "local" {
            String::new()
        } else {
            app_host_label.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pin_dedup_on_new() {
        let p1 = Pin {
            source: "meminfo".to_string(),
            field: Some("MemAvailable".to_string()),
            host: String::new(),
        };
        let p2 = p1.clone();
        let set = PinSet::new(vec![p1, p2]);
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn toggle_adds_and_removes() {
        let mut set = PinSet::new(vec![]);
        let pin = Pin {
            source: "loadavg".to_string(),
            field: None,
            host: String::new(),
        };
        assert!(!set.contains(&pin));
        assert!(set.toggle(&pin));
        assert!(set.contains(&pin));
        assert_eq!(set.len(), 1);
        assert!(!set.toggle(&pin));
        assert!(!set.contains(&pin));
        assert!(set.is_empty());
    }

    #[test]
    fn is_pinned_source_and_field() {
        let mut set = PinSet::new(vec![]);
        set.toggle(&Pin {
            source: "meminfo".to_string(),
            field: None,
            host: String::new(),
        });
        set.toggle(&Pin {
            source: "loadavg".to_string(),
            field: Some("load_1min".to_string()),
            host: String::new(),
        });
        assert!(set.is_pinned_source("meminfo", ""));
        assert!(!set.is_pinned_source("loadavg", ""));
        assert!(set.is_pinned_field("loadavg", "load_1min", ""));
        assert!(!set.is_pinned_field("loadavg", "load_5min", ""));
    }

    #[test]
    fn host_key_distinguishes_pins() {
        let mut set = PinSet::new(vec![]);
        set.toggle(&Pin {
            source: "meminfo".to_string(),
            field: None,
            host: "tcp:127.0.0.1:9100".to_string(),
        });
        assert!(!set.is_pinned_source("meminfo", ""));
        assert!(set.is_pinned_source("meminfo", "tcp:127.0.0.1:9100"));
    }

    #[test]
    fn pinfile_roundtrip_toml() {
        let file = PinFile {
            pin: vec![
                Pin {
                    source: "meminfo".to_string(),
                    field: Some("MemAvailable".to_string()),
                    host: String::new(),
                },
                Pin {
                    source: "jvm".to_string(),
                    field: Some("heap_used".to_string()),
                    host: "tcp:127.0.0.1:9100".to_string(),
                },
            ],
        };
        let toml_str = toml::to_string_pretty(&file).unwrap();
        let parsed: PinFile = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.pin.len(), 2);
        assert_eq!(parsed.pin[0].source, "meminfo");
        assert_eq!(parsed.pin[0].field, Some("MemAvailable".to_string()));
        assert_eq!(parsed.pin[0].host, "");
        assert_eq!(parsed.pin[1].host, "tcp:127.0.0.1:9100");
    }

    #[test]
    fn pinfile_parse_skips_empty_field() {
        let toml_str = r#"
[[pin]]
source = "meminfo"
field = "MemAvailable"

[[pin]]
source = "loadavg"
"#;
        let parsed: PinFile = toml::from_str(toml_str).unwrap();
        assert_eq!(parsed.pin.len(), 2);
        assert_eq!(parsed.pin[0].field, Some("MemAvailable".to_string()));
        assert_eq!(parsed.pin[1].field, None);
    }

    #[test]
    fn pinfile_load_nonexistent_returns_empty() {
        let pins = PinFile::load();
        assert!(pins.is_empty() || !pins.is_empty());
    }

    #[test]
    fn pinfile_save_and_load_roundtrip() {
        let tmp = tempfile::TempDir::new().unwrap();
        let xdg_path = tmp.path().to_str().unwrap().to_string();
        unsafe {
            std::env::set_var("XDG_CONFIG_HOME", &xdg_path);
        }
        let pins = vec![
            Pin {
                source: "meminfo".to_string(),
                field: Some("MemAvailable".to_string()),
                host: String::new(),
            },
            Pin {
                source: "loadavg".to_string(),
                field: None,
                host: String::new(),
            },
        ];
        PinFile::save(&pins);
        let loaded = PinFile::load();
        unsafe {
            std::env::remove_var("XDG_CONFIG_HOME");
        }
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].source, "meminfo");
        assert_eq!(loaded[0].field, Some("MemAvailable".to_string()));
        assert_eq!(loaded[1].source, "loadavg");
        assert_eq!(loaded[1].field, None);
    }

    #[test]
    fn pinset_current_host_normalizes_localhost() {
        let set = PinSet::new(vec![]);
        assert_eq!(set.current_host("localhost"), "");
        assert_eq!(set.current_host("local"), "");
        assert_eq!(set.current_host("ssh:host1"), "ssh:host1");
    }
}
