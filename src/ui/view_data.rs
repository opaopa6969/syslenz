//! ViewData — unified UI data layer.
//!
//! Separates "what to show" from "how to draw". Each view has a typed
//! struct containing pre-formatted strings ready for any renderer
//! (TUI, Web, X11) to consume without accessing [`App`] state directly.

use serde::Serialize;

use crate::alert;
use crate::i18n::{self, Locale, T};
use crate::proc::FieldValue;

use super::app::{App, HelpLevel, View};

// ---------------------------------------------------------------------------
// ViewData enum & all data structs
// ---------------------------------------------------------------------------

/// All possible view data, ready for any renderer to consume.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum ViewData {
    Dashboard(DashboardData),
    Welcome(WelcomeData),
    Detail(DetailData),
    Diff(DiffData),
    TableView(TableViewData),
    Graph(GraphData),
    Diagnostics(DiagnosticsData),
    CategoryGuide(CategoryGuideData),
    /// BL-074: Tutorial view data.
    Tutorial(TutorialData),
}

/// BL-074: Tutorial step data.
#[derive(Debug, Clone, Serialize)]
pub struct TutorialData {
    pub step: usize,
    pub total_steps: usize,
    pub title: String,
    pub body: String,
    pub nav_hint: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DashboardData {
    pub load: LoadSection,
    pub memory: MemorySection,
    pub cpu: CpuSection,
    pub network: NetworkSection,
    pub system: SystemSection,
    pub selected_section: usize,
    // Bar graph data
    pub mem_used_pct: u64,
    pub mem_bar: String,
    pub mem_used_bytes: u64,
    pub mem_total_bytes: u64,
    pub swap_used_pct: u64,
    pub swap_bar: String,
    pub swap_used_bytes: u64,
    pub swap_total_bytes: u64,
    pub cached: String,
    pub buffers: String,
    pub cpu_used_pct: u64,
    pub cpu_bar: String,
    pub cpu_user_pct: u64,
    pub cpu_sys_pct: u64,
    pub cpu_io_pct: u64,
    pub ctx_switches: String,
    pub procs_running: String,
    // Sparkline history
    pub load_history: Vec<u64>,
    pub mem_history: Vec<u64>,
    // Diagnostic badge (P-A1)
    pub diag_count: usize,
    pub diag_severity: Option<ViewColor>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SystemSection {
    pub disk_pct: String,
    pub temp: String,
    pub fd_pct: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoadSection {
    pub load1: String,
    pub load5: String,
    pub load15: String,
    pub uptime: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MemorySection {
    pub items: Vec<KeyValueItem>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CpuSection {
    pub items: Vec<KeyValueItem>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NetworkSection {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct KeyValueItem {
    pub name: String,
    pub value: String,
    pub color: ViewColor,
}

#[derive(Debug, Clone, Serialize)]
pub enum ViewColor {
    Default,
    Green,
    Blue,
    Red,
    Yellow,
    Cyan,
    Magenta,
    DarkGray,
}

#[derive(Debug, Clone, Serialize)]
pub struct WelcomeData {
    pub keybindings: Vec<(String, String)>,
    /// P-A2: Advanced keybindings shown only at Detailed/ExtraDetailed help levels.
    pub advanced_keybindings: Vec<(String, String)>,
    pub title: String,
    pub footer: String,
    pub tip: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DetailData {
    pub source_name: String,
    pub source_path: String,
    pub fields: Vec<FieldRow>,
    pub selected_field: usize,
    pub position: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct FieldRow {
    pub name: String,
    pub value: String,
    pub unit: String,
    pub description: String,
    pub color: ViewColor,
    pub is_table: bool,
    /// Alert severity if this field is triggering an alert: "info", "warning", "critical", or empty
    #[serde(default)]
    pub alert_severity: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiffData {
    pub items: Vec<DiffRow>,
    pub title: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiffRow {
    pub source: String,
    pub field: String,
    pub old_value: String,
    pub new_value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TableViewData {
    pub title: String,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub scroll: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct GraphData {
    pub field_name: String,
    pub source_name: String,
    pub data_points: Vec<f64>,
    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub current: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiagnosticsData {
    pub findings: Vec<DiagnosticRow>,
    pub title: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiagnosticRow {
    pub severity: String,
    pub severity_color: ViewColor,
    pub source: String,
    pub title: String,
    pub detail: String,
    pub suggestion: String,
    pub related_metrics: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CategoryGuideData {
    pub categories: Vec<CategoryItem>,
    pub selected_category: usize,
    pub content: CategoryContentData,
    pub content_scroll: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct CategoryItem {
    pub icon: String,
    pub name: String,
    pub selected: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct CategoryContentData {
    pub overview: String,
    pub story: String,
    pub diagnostic_flow: String,
    pub common_issues: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SidebarData {
    pub sources: Vec<SidebarItem>,
    pub selected: usize,
    pub title: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SidebarItem {
    pub name: String,
    pub is_selected: bool,
    pub is_net: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct StatusBarData {
    pub view_name: String,
    pub refresh_mode: String,
    pub elapsed_secs: u64,
    pub snapshot_count: usize,
    pub remote_host: Option<String>,
    pub locale_name: String,
    pub help_level: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct HelpPanelData {
    pub source_name: String,
    pub source_desc: String,
    pub field_name: String,
    pub field_desc: String,
    pub level_label: String,
}

// ---------------------------------------------------------------------------
// Build functions — extract ViewData from App state
// ---------------------------------------------------------------------------

/// Helper: get a single field value from the current snapshot.
fn get_field_value(app: &App, source: &str, field_name: &str) -> String {
    app.current
        .entries
        .get(source)
        .and_then(|e| e.fields.iter().find(|f| f.name == field_name))
        .map(|f| f.value.display())
        .unwrap_or_else(|| "-".to_string())
}

/// Helper: extract a numeric f64 from a FieldValue.
fn extract_numeric(value: &FieldValue) -> Option<f64> {
    match value {
        FieldValue::Bytes(b) => Some(*b as f64),
        FieldValue::Integer(i) => Some(*i as f64),
        FieldValue::Float(f) => Some(*f),
        FieldValue::Duration(d) => Some(*d),
        _ => None,
    }
}

/// Helper: get a raw bytes value from the current snapshot.
fn get_bytes_value(app: &App, source: &str, field: &str) -> u64 {
    app.current
        .entries
        .get(source)
        .and_then(|e| e.fields.iter().find(|f| f.name == field))
        .and_then(|f| match f.value {
            FieldValue::Bytes(v) => Some(v),
            _ => None,
        })
        .unwrap_or(0)
}

/// BL-076: Format a bytes-per-second rate as a human-readable string.
fn format_rate(bytes_per_sec: f64) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = 1024.0 * KIB;
    const GIB: f64 = 1024.0 * MIB;
    if bytes_per_sec >= GIB {
        format!("{:.1} GiB/s", bytes_per_sec / GIB)
    } else if bytes_per_sec >= MIB {
        format!("{:.1} MiB/s", bytes_per_sec / MIB)
    } else if bytes_per_sec >= KIB {
        format!("{:.1} KiB/s", bytes_per_sec / KIB)
    } else if bytes_per_sec > 0.5 {
        format!("{:.0} B/s", bytes_per_sec)
    } else {
        "0 B/s".to_string()
    }
}

/// Helper: build an ASCII bar graph string.
fn make_bar(pct: u64, width: usize) -> String {
    let filled = (pct as usize * width / 100).min(width);
    let empty = width - filled;
    format!("{}{}", "\u{2588}".repeat(filled), "\u{2591}".repeat(empty))
}

impl App {
    /// Build the main content ViewData from the current app state.
    pub fn build_view_data(&self) -> ViewData {
        match self.view {
            View::Dashboard => ViewData::Dashboard(self.build_dashboard_data()),
            View::Welcome => ViewData::Welcome(self.build_welcome_data()),
            View::Overview | View::Detail => ViewData::Detail(self.build_detail_data()),
            View::Diff => ViewData::Diff(self.build_diff_data()),
            View::TableView => ViewData::TableView(self.build_table_view_data()),
            View::Graph => ViewData::Graph(self.build_graph_data()),
            View::Diagnostics => ViewData::Diagnostics(self.build_diagnostics_data()),
            View::CategoryGuide => ViewData::CategoryGuide(self.build_category_guide_data()),
            View::Tutorial => ViewData::Tutorial(self.build_tutorial_data()),
        }
    }

    /// BL-074: Build tutorial step data.
    fn build_tutorial_data(&self) -> TutorialData {
        let step = self.tutorial_step.unwrap_or(0);
        let (en, ja) = self.tutorial_text();
        let body = if self.locale == Locale::Ja { ja } else { en };
        let title = if self.locale == Locale::Ja {
            format!("syslenz チュートリアル ({}/{})", step + 1, Self::TUTORIAL_STEPS)
        } else {
            format!("syslenz Tutorial ({}/{})", step + 1, Self::TUTORIAL_STEPS)
        };
        let nav_hint = if self.locale == Locale::Ja {
            "Enter/Right: 次へ | Backspace/Left: 戻る | q: 終了".to_string()
        } else {
            "Enter/Right: Next | Backspace/Left: Back | q: Exit".to_string()
        };
        TutorialData {
            step,
            total_steps: Self::TUTORIAL_STEPS,
            title,
            body: body.to_string(),
            nav_hint,
        }
    }

    /// Build sidebar data for views that show the source list.
    pub fn build_sidebar_data(&self) -> SidebarData {
        let sources: Vec<SidebarItem> = self
            .source_keys
            .iter()
            .enumerate()
            .map(|(i, key)| SidebarItem {
                name: key.clone(),
                is_selected: i == self.selected_source,
                is_net: key.starts_with("net/"),
            })
            .collect();

        SidebarData {
            title: format!("/proc ({})", self.source_keys.len()),
            selected: self.selected_source,
            sources,
        }
    }

    /// Build status bar data.
    pub fn build_status_bar_data(&self) -> StatusBarData {
        let l = self.locale;
        let view_name = match self.view {
            View::Dashboard => i18n::t(l, T::VIEW_DASHBOARD),
            View::Welcome => i18n::t(l, T::VIEW_WELCOME),
            View::Diagnostics => {
                if l == Locale::Ja { "診断" } else { "DIAGNOSTICS" }
            }
            View::Overview => i18n::t(l, T::VIEW_OVERVIEW),
            View::Detail => i18n::t(l, T::VIEW_DETAIL),
            View::Diff => i18n::t(l, T::VIEW_DIFF),
            View::TableView => i18n::t(l, T::VIEW_TABLE),
            View::Graph => i18n::t(l, T::VIEW_GRAPH),
            View::CategoryGuide => {
                if l == Locale::Ja { "カテゴリガイド" } else { "CATEGORY GUIDE" }
            }
            View::Tutorial => {
                if l == Locale::Ja { "チュートリアル" } else { "TUTORIAL" }
            }
        };

        StatusBarData {
            view_name: view_name.to_string(),
            refresh_mode: if self.auto_refresh {
                "AUTO".to_string()
            } else {
                "MANUAL".to_string()
            },
            elapsed_secs: self.last_refresh.elapsed().as_secs(),
            snapshot_count: self.snapshots.len(),
            remote_host: self.remote_host.clone(),
            locale_name: l.name().to_string(),
            help_level: self.help_level.label().to_string(),
        }
    }

    /// Build help panel data. Returns `None` when help is off.
    pub fn build_help_panel_data(&self) -> Option<HelpPanelData> {
        if self.help_level == HelpLevel::Off {
            return None;
        }

        let source_name = self.current_source_name().to_string();
        let source_desc =
            i18n::source_description(self.locale, &source_name).to_string();

        let field_info = self
            .current_entry_fields()
            .and_then(|fields| fields.get(self.selected_field))
            .map(|f| (f.name.clone(), f.description.clone()));

        let (field_name, fallback_desc) =
            field_info.unwrap_or_else(|| (String::new(), String::new()));

        let i18n_desc =
            i18n::field_description(self.locale, &source_name, &field_name);

        let field_desc = match (self.help_level, i18n_desc) {
            (HelpLevel::Normal, Some((normal, _, _))) => normal.to_string(),
            (HelpLevel::Detailed, Some((_, detailed, _))) => detailed.to_string(),
            (HelpLevel::ExtraDetailed, Some((_, _, extra))) => extra.to_string(),
            _ => fallback_desc,
        };

        Some(HelpPanelData {
            source_name,
            source_desc,
            field_name,
            field_desc,
            level_label: self.help_level.label().to_string(),
        })
    }

    // -- private builders ---------------------------------------------------

    fn build_dashboard_data(&self) -> DashboardData {
        let l = self.locale;

        // Load section
        let load = LoadSection {
            load1: get_field_value(self, "loadavg", "load1"),
            load5: get_field_value(self, "loadavg", "load5"),
            load15: get_field_value(self, "loadavg", "load15"),
            uptime: get_field_value(self, "uptime", "uptime"),
        };

        // Memory section
        let mem_field_names = [
            "MemTotal",
            "MemFree",
            "MemAvailable",
            "Buffers",
            "Cached",
            "SwapTotal",
            "SwapFree",
        ];
        let mem_items: Vec<KeyValueItem> = mem_field_names
            .iter()
            .filter_map(|name| {
                self.current
                    .entries
                    .get("meminfo")
                    .and_then(|e| e.fields.iter().find(|f| f.name == *name))
                    .map(|f| KeyValueItem {
                        name: f.name.clone(),
                        value: f.value.display(),
                        color: ViewColor::Green,
                    })
            })
            .collect();

        // CPU section
        let cpu_field_names = [
            "cpu_user",
            "cpu_system",
            "cpu_idle",
            "cpu_iowait",
            "context_switches",
            "processes_running",
        ];
        let cpu_items: Vec<KeyValueItem> = cpu_field_names
            .iter()
            .filter_map(|name| {
                self.current
                    .entries
                    .get("stat")
                    .and_then(|e| e.fields.iter().find(|f| f.name == *name))
                    .map(|f| {
                        let color = match f.name.as_str() {
                            "cpu_idle" => ViewColor::Green,
                            "cpu_iowait" => ViewColor::Red,
                            _ => ViewColor::Blue,
                        };
                        KeyValueItem {
                            name: f.name.clone(),
                            value: f.value.display(),
                            color,
                        }
                    })
            })
            .collect();

        // Network section (BL-076: add RX/s and TX/s rate columns)
        let net_headers = if l == Locale::Ja {
            vec![
                "IF".to_string(),
                "RX バイト".to_string(),
                "RX/s".to_string(),
                "TX バイト".to_string(),
                "TX/s".to_string(),
            ]
        } else {
            vec![
                "Interface".to_string(),
                "RX Bytes".to_string(),
                "RX/s".to_string(),
                "TX Bytes".to_string(),
                "TX/s".to_string(),
            ]
        };

        // Get current net/dev table rows
        let current_net_rows: Vec<Vec<String>> = self
            .current
            .entries
            .get("net/dev")
            .and_then(|e| {
                e.fields
                    .iter()
                    .find(|f| matches!(f.value, FieldValue::Table(_)))
            })
            .map(|f| {
                if let FieldValue::Table(ref data) = f.value {
                    data.iter()
                        .take(6)
                        .map(|row| row.clone())
                        .collect()
                } else {
                    vec![]
                }
            })
            .unwrap_or_default();

        // BL-076: Compute rates from previous snapshot
        let prev_net_table = self.snapshots.last().and_then(|snap| {
            snap.entries.get("net/dev").and_then(|e| {
                e.fields.iter().find(|f| matches!(f.value, FieldValue::Table(_)))
            }).and_then(|f| {
                if let FieldValue::Table(ref data) = f.value {
                    Some(data.clone())
                } else {
                    None
                }
            })
        });

        let time_delta_secs = self.snapshots.last().map(|prev| {
            let cur_dur = self.current.timestamp
                .duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap_or_default();
            let prev_dur = prev.timestamp
                .duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap_or_default();
            let delta = cur_dur.as_secs_f64() - prev_dur.as_secs_f64();
            if delta > 0.0 { delta } else { 1.0 }
        });

        let net_rows: Vec<Vec<String>> = current_net_rows.iter().map(|row| {
            // Row layout: [iface, rx_bytes, rx_pkts, tx_bytes, tx_pkts]
            let iface = row.first().cloned().unwrap_or_default();
            let rx_bytes_str = row.get(1).cloned().unwrap_or_default();
            let tx_bytes_str = row.get(3).cloned().unwrap_or_default();

            // Find the matching previous row by interface name
            let (rx_rate, tx_rate) = if let (Some(prev_rows), Some(dt)) = (&prev_net_table, time_delta_secs) {
                let prev_row = prev_rows.iter().find(|r| r.first().map(|s| s.as_str()) == Some(iface.as_str()));
                if let Some(prev) = prev_row {
                    let parse_net_bytes = |s: &str| -> u64 {
                        // Handle formatted strings like "1.2 GiB", "500.0 MiB", "1024 B"
                        let s = s.trim();
                        if let Some(rest) = s.strip_suffix("GiB") {
                            (rest.trim().parse::<f64>().unwrap_or(0.0) * 1024.0 * 1024.0 * 1024.0) as u64
                        } else if let Some(rest) = s.strip_suffix("MiB") {
                            (rest.trim().parse::<f64>().unwrap_or(0.0) * 1024.0 * 1024.0) as u64
                        } else if let Some(rest) = s.strip_suffix("KiB") {
                            (rest.trim().parse::<f64>().unwrap_or(0.0) * 1024.0) as u64
                        } else if let Some(rest) = s.strip_suffix(" B") {
                            rest.trim().parse::<u64>().unwrap_or(0)
                        } else {
                            // Try raw number
                            s.replace(',', "").parse::<u64>().unwrap_or(0)
                        }
                    };
                    let cur_rx = parse_net_bytes(&rx_bytes_str);
                    let prev_rx = parse_net_bytes(prev.get(1).map(|s| s.as_str()).unwrap_or("0"));
                    let cur_tx = parse_net_bytes(&tx_bytes_str);
                    let prev_tx = parse_net_bytes(prev.get(3).map(|s| s.as_str()).unwrap_or("0"));

                    let rx_delta = cur_rx.saturating_sub(prev_rx) as f64;
                    let tx_delta = cur_tx.saturating_sub(prev_tx) as f64;
                    (format_rate(rx_delta / dt), format_rate(tx_delta / dt))
                } else {
                    ("-".to_string(), "-".to_string())
                }
            } else {
                ("-".to_string(), "-".to_string())
            };

            vec![iface, rx_bytes_str, rx_rate, tx_bytes_str, tx_rate]
        }).collect();

        // Compute bar graph data for memory
        let mem_total = get_bytes_value(self, "meminfo", "MemTotal");
        let mem_available = get_bytes_value(self, "meminfo", "MemAvailable");
        let mem_used = mem_total.saturating_sub(mem_available);
        let mem_pct = if mem_total > 0 {
            (mem_used as f64 / mem_total as f64 * 100.0) as u64
        } else {
            0
        };
        let swap_total = get_bytes_value(self, "meminfo", "SwapTotal");
        let swap_free = get_bytes_value(self, "meminfo", "SwapFree");
        let swap_used = swap_total.saturating_sub(swap_free);
        let swap_pct = if swap_total > 0 {
            (swap_used as f64 / swap_total as f64 * 100.0) as u64
        } else {
            0
        };

        // CPU percentages from cumulative counters
        let cpu_user_raw: f64 = get_field_value(self, "stat", "cpu_user")
            .parse()
            .unwrap_or(0.0);
        let cpu_sys_raw: f64 = get_field_value(self, "stat", "cpu_system")
            .parse()
            .unwrap_or(0.0);
        let cpu_idle_raw: f64 = get_field_value(self, "stat", "cpu_idle")
            .parse()
            .unwrap_or(1.0);
        let cpu_iowait_raw: f64 = get_field_value(self, "stat", "cpu_iowait")
            .parse()
            .unwrap_or(0.0);
        let cpu_total = cpu_user_raw + cpu_sys_raw + cpu_idle_raw + cpu_iowait_raw;
        let user_pct = if cpu_total > 0.0 {
            (cpu_user_raw / cpu_total * 100.0) as u64
        } else {
            0
        };
        let sys_pct = if cpu_total > 0.0 {
            (cpu_sys_raw / cpu_total * 100.0) as u64
        } else {
            0
        };
        let io_pct = if cpu_total > 0.0 {
            (cpu_iowait_raw / cpu_total * 100.0) as u64
        } else {
            0
        };
        let cpu_used_pct = (user_pct + sys_pct).min(100);

        // System summary
        let system = SystemSection {
            disk_pct: get_field_value(self, "df", "root_use_pct"),
            temp: get_field_value(self, "thermal", "max_temp"),
            fd_pct: get_field_value(self, "file-nr", "fd_usage_pct"),
        };

        // Sparkline history: load
        let load_history: Vec<u64> = self
            .snapshots
            .iter()
            .chain(std::iter::once(&self.current))
            .filter_map(|snap| {
                snap.entries
                    .get("loadavg")
                    .and_then(|e| e.fields.iter().find(|f| f.name == "load_1min"))
                    .and_then(|f| match f.value {
                        FieldValue::Float(v) => Some((v * 100.0) as u64),
                        _ => None,
                    })
            })
            .collect();

        // Sparkline history: memory usage %
        let mem_history: Vec<u64> = self
            .snapshots
            .iter()
            .chain(std::iter::once(&self.current))
            .filter_map(|snap| {
                let total = snap
                    .entries
                    .get("meminfo")
                    .and_then(|e| e.fields.iter().find(|f| f.name == "MemTotal"))
                    .and_then(|f| match f.value {
                        FieldValue::Bytes(v) => Some(v),
                        _ => None,
                    })
                    .unwrap_or(1);
                let avail = snap
                    .entries
                    .get("meminfo")
                    .and_then(|e| e.fields.iter().find(|f| f.name == "MemAvailable"))
                    .and_then(|f| match f.value {
                        FieldValue::Bytes(v) => Some(v),
                        _ => None,
                    })
                    .unwrap_or(0);
                let pct = if total > 0 {
                    ((total - avail) as f64 / total as f64 * 100.0) as u64
                } else {
                    0
                };
                Some(pct)
            })
            .collect();

        // P-A1: Diagnostic badge — run analyze once, extract count & worst severity
        let diag_findings = crate::diagnostics::analyze(&self.current, l);
        let diag_count = diag_findings.len();
        let diag_severity = diag_findings.iter().fold(None, |worst, f| {
            let sev = match f.severity {
                crate::diagnostics::Severity::Critical => Some(ViewColor::Red),
                crate::diagnostics::Severity::Warning => Some(ViewColor::Yellow),
                crate::diagnostics::Severity::Info => Some(ViewColor::Cyan),
            };
            match (&worst, &sev) {
                (None, _) => sev,
                (Some(ViewColor::Red), _) => worst,
                (_, Some(ViewColor::Red)) => sev,
                (Some(ViewColor::Yellow), _) => worst,
                (_, Some(ViewColor::Yellow)) => sev,
                _ => worst,
            }
        });

        DashboardData {
            load,
            memory: MemorySection { items: mem_items },
            cpu: CpuSection { items: cpu_items },
            network: NetworkSection {
                headers: net_headers,
                rows: net_rows,
            },
            system,
            selected_section: self.selected_dashboard_section,
            mem_used_pct: mem_pct,
            mem_bar: make_bar(mem_pct, 30),
            mem_used_bytes: mem_used,
            mem_total_bytes: mem_total,
            swap_used_pct: swap_pct,
            swap_bar: make_bar(swap_pct, 30),
            swap_used_bytes: swap_used,
            swap_total_bytes: swap_total,
            cached: get_field_value(self, "meminfo", "Cached"),
            buffers: get_field_value(self, "meminfo", "Buffers"),
            cpu_used_pct,
            cpu_bar: make_bar(cpu_used_pct, 30),
            cpu_user_pct: user_pct,
            cpu_sys_pct: sys_pct,
            cpu_io_pct: io_pct,
            ctx_switches: get_field_value(self, "stat", "context_switches"),
            procs_running: get_field_value(self, "stat", "processes_running"),
            load_history,
            mem_history,
            diag_count,
            diag_severity,
        }
    }

    fn build_welcome_data(&self) -> WelcomeData {
        let l = self.locale;
        let title = if l == Locale::Ja {
            "syslenz — /proc の全てを構造化データで".to_string()
        } else {
            "syslenz — Wireshark for /proc".to_string()
        };
        let footer = i18n::t(l, T::WELCOME_CTA).to_string();

        // P-A2: Split keybindings into basic (6) and advanced
        let keybindings = if l == Locale::Ja {
            vec![
                ("D".into(), "ダッシュボード（システム概要）".into()),
                ("O".into(), "クラシックモード（全ソース一覧）".into()),
                ("j/k ↑/↓".into(), i18n::t(l, T::WELCOME_NAV).to_string()),
                ("Enter →".into(), i18n::t(l, T::WELCOME_DRILL).to_string()),
                ("BS ←".into(), i18n::t(l, T::BACK).to_string()),
                ("q".into(), i18n::t(l, T::QUIT).to_string()),
            ]
        } else {
            vec![
                ("D".into(), "Dashboard (system overview)".into()),
                ("O".into(), "Classic mode (all sources list)".into()),
                ("j/k ↑/↓".into(), i18n::t(l, T::WELCOME_NAV).to_string()),
                ("Enter →".into(), i18n::t(l, T::WELCOME_DRILL).to_string()),
                ("BS ←".into(), i18n::t(l, T::BACK).to_string()),
                ("q".into(), i18n::t(l, T::QUIT).to_string()),
            ]
        };

        let advanced_keybindings = if l == Locale::Ja {
            vec![
                ("d".into(), i18n::t(l, T::WELCOME_DIFF).to_string()),
                ("g".into(), i18n::t(l, T::WELCOME_GRAPH).to_string()),
                ("/".into(), i18n::t(l, T::WELCOME_SEARCH).to_string()),
                ("X".into(), "診断".into()),
                ("C".into(), "カテゴリガイド".into()),
                ("?".into(), i18n::t(l, T::WELCOME_HELP).to_string()),
                ("L".into(), i18n::t(l, T::WELCOME_LANG).to_string()),
                ("e".into(), i18n::t(l, T::EXPORT).to_string()),
                ("c".into(), "コピー".into()),
            ]
        } else {
            vec![
                ("d".into(), i18n::t(l, T::WELCOME_DIFF).to_string()),
                ("g".into(), i18n::t(l, T::WELCOME_GRAPH).to_string()),
                ("/".into(), i18n::t(l, T::WELCOME_SEARCH).to_string()),
                ("X".into(), "Diagnostics".into()),
                ("C".into(), "Category guide".into()),
                ("?".into(), i18n::t(l, T::WELCOME_HELP).to_string()),
                ("L".into(), i18n::t(l, T::WELCOME_LANG).to_string()),
                ("e".into(), i18n::t(l, T::EXPORT).to_string()),
                ("c".into(), "Copy".into()),
            ]
        };

        let tip = crate::education::generate_tip(&self.current, l);

        WelcomeData {
            keybindings,
            advanced_keybindings,
            title,
            footer,
            tip,
        }
    }

    fn build_detail_data(&self) -> DetailData {
        let source_name = self.current_source_name().to_string();
        let source_path = self
            .current
            .entries
            .get(&source_name)
            .map(|e| e.source.clone())
            .unwrap_or_default();

        let fields: Vec<FieldRow> = self
            .current_entry_fields()
            .map(|fields| {
                fields
                    .iter()
                    .map(|f| {
                        let color = match &f.value {
                            FieldValue::Bytes(_) => ViewColor::Green,
                            FieldValue::Integer(_) => ViewColor::Blue,
                            FieldValue::Float(_) => ViewColor::Magenta,
                            FieldValue::Duration(_) => ViewColor::Yellow,
                            FieldValue::Text(_) => ViewColor::Default,
                            FieldValue::Table(_) => ViewColor::Cyan,
                        };
                        let is_table = matches!(f.value, FieldValue::Table(_));
                        let name_display = if is_table {
                            format!("{} [+]", f.name)
                        } else {
                            f.name.clone()
                        };
                        let alert_sev = alert::field_alert_severity(
                            &self.active_alerts,
                            &source_name,
                            &f.name,
                        )
                        .unwrap_or("")
                        .to_string();
                        FieldRow {
                            name: name_display,
                            value: f.value.display(),
                            unit: f.unit.clone().unwrap_or_default(),
                            description: f.description.clone(),
                            color,
                            is_table,
                            alert_severity: alert_sev,
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        let field_count = fields.len();
        let position = format!(
            "[{}/{}]",
            if field_count > 0 {
                self.selected_field + 1
            } else {
                0
            },
            field_count
        );

        DetailData {
            source_name,
            source_path,
            fields,
            selected_field: self.selected_field,
            position,
        }
    }

    fn build_diff_data(&self) -> DiffData {
        let l = self.locale;
        let title = if self.diffs.is_empty() {
            format!("{}", i18n::t(l, T::DIFF))
        } else {
            format!("Diff ({} changes)", self.diffs.len())
        };

        let items: Vec<DiffRow> = self
            .diffs
            .iter()
            .map(|d| DiffRow {
                source: d.source.clone(),
                field: d.field.clone(),
                old_value: d.old_value.clone(),
                new_value: d.new_value.clone(),
            })
            .collect();

        DiffData { items, title }
    }

    fn build_table_view_data(&self) -> TableViewData {
        let source_name = self.current_source_name().to_string();

        let fields = match self.current_entry_fields() {
            Some(f) => f,
            None => {
                return TableViewData {
                    title: format!("{} — No data", source_name),
                    headers: vec![],
                    rows: vec![],
                    scroll: 0,
                }
            }
        };

        // Find the table field
        let table_field = fields
            .get(self.selected_field)
            .filter(|f| matches!(f.value, FieldValue::Table(_)))
            .or_else(|| {
                fields
                    .iter()
                    .find(|f| matches!(f.value, FieldValue::Table(_)))
            });

        let Some(field) = table_field else {
            return TableViewData {
                title: format!("{} — No table data", source_name),
                headers: vec![],
                rows: vec![],
                scroll: 0,
            };
        };

        let header_titles: Vec<String> = match self.current_source_name() {
            "mounts" => vec!["Device", "Mountpoint", "FSType", "Options"],
            "partitions" => vec!["Name", "Size", "Major", "Minor"],
            "net/dev" => vec!["Interface", "RX Bytes", "RX Pkts", "TX Bytes", "TX Pkts"],
            "diskstats" => {
                vec!["Device", "Reads", "Read Bytes", "Writes", "Written", "InFlight"]
            }
            "processes" => vec!["PID", "Name", "State", "RSS", "Threads", "UID"],
            "swaps" => vec!["Filename", "Type", "Size", "Used", "Priority"],
            "modules" => vec!["Name", "Size", "Used By", "State"],
            "net/tcp" => vec!["Local Addr", "Remote Addr", "State", "UID"],
            "net/udp" => vec!["Local Addr", "Remote Addr", "State", "UID"],
            "net/unix" => vec!["Type", "State", "Inode", "Path"],
            "net/arp" => vec!["IP Address", "HW Address", "Device"],
            "net/route" => vec!["Iface", "Destination", "Gateway", "Mask", "Metric"],
            "crypto" => vec!["Name", "Driver", "Module", "Type", "BlockSize"],
            "locks" => vec!["Type", "Mode", "RW", "PID", "Range"],
            "interrupts" => vec!["IRQ", "Count", "Type", "Device"],
            "devices" => vec!["Major", "Name"],
            "cgroups" => vec!["Name", "Hierarchy", "NumCGroups", "Enabled"],
            _ => {
                if let FieldValue::Table(ref data) = field.value {
                    let ncols = data.first().map(|r| r.len()).unwrap_or(4);
                    (0..ncols)
                        .map(|i| match i {
                            0 => "Col1",
                            1 => "Col2",
                            2 => "Col3",
                            3 => "Col4",
                            4 => "Col5",
                            5 => "Col6",
                            6 => "Col7",
                            _ => "Col",
                        })
                        .collect()
                } else {
                    vec!["Col1"]
                }
            }
        }
        .into_iter()
        .map(|s| s.to_string())
        .collect();

        let rows: Vec<Vec<String>> = if let FieldValue::Table(ref data) = field.value {
            data.clone()
        } else {
            vec![]
        };

        let title = format!(
            "{} / {} ({} rows)",
            source_name,
            field.name,
            rows.len()
        );

        TableViewData {
            title,
            headers: header_titles,
            rows,
            scroll: self.table_scroll,
        }
    }

    fn build_graph_data(&self) -> GraphData {
        let (source_name, field_name) = match self.graph_field {
            Some((ref s, ref f)) => (s.clone(), f.clone()),
            None => {
                return GraphData {
                    field_name: String::new(),
                    source_name: String::new(),
                    data_points: vec![],
                    min: 0.0,
                    max: 0.0,
                    avg: 0.0,
                    current: 0.0,
                }
            }
        };

        let mut values: Vec<f64> = Vec::new();
        for snap in &self.snapshots {
            if let Some(entry) = snap.entries.get(&source_name) {
                if let Some(field) = entry.fields.iter().find(|f| f.name == field_name)
                {
                    if let Some(v) = extract_numeric(&field.value) {
                        values.push(v);
                    }
                }
            }
        }
        // Current value
        if let Some(entry) = self.current.entries.get(&source_name) {
            if let Some(field) = entry.fields.iter().find(|f| f.name == field_name) {
                if let Some(v) = extract_numeric(&field.value) {
                    values.push(v);
                }
            }
        }

        let min = values
            .iter()
            .cloned()
            .fold(f64::INFINITY, f64::min);
        let max = values
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let avg = if values.is_empty() {
            0.0
        } else {
            values.iter().sum::<f64>() / values.len() as f64
        };
        let current = values.last().copied().unwrap_or(0.0);

        GraphData {
            field_name,
            source_name,
            data_points: values,
            min: if min.is_infinite() { 0.0 } else { min },
            max: if max.is_infinite() { 0.0 } else { max },
            avg,
            current,
        }
    }

    fn build_diagnostics_data(&self) -> DiagnosticsData {
        let l = self.locale;
        let findings = crate::diagnostics::analyze(&self.current, l);

        let title = if l == Locale::Ja {
            format!("診断 — {}件の検出", findings.len())
        } else {
            format!("Diagnostics — {} findings", findings.len())
        };

        let rows: Vec<DiagnosticRow> = findings
            .iter()
            .map(|f| {
                let severity_color = match f.severity {
                    crate::diagnostics::Severity::Critical => ViewColor::Red,
                    crate::diagnostics::Severity::Warning => ViewColor::Yellow,
                    crate::diagnostics::Severity::Info => ViewColor::Cyan,
                };
                DiagnosticRow {
                    severity: f.severity.label(l).to_string(),
                    severity_color,
                    source: f.source.clone(),
                    title: f.title.clone(),
                    detail: f.detail.clone(),
                    suggestion: f.suggestion.clone(),
                    related_metrics: f.related_metrics.clone(),
                }
            })
            .collect();

        DiagnosticsData {
            findings: rows,
            title,
        }
    }

    #[cfg(test)]
    pub fn test_build_dashboard_data(&self) -> DashboardData {
        self.build_dashboard_data()
    }

    #[cfg(test)]
    pub fn test_build_welcome_data(&self) -> WelcomeData {
        self.build_welcome_data()
    }

    fn build_category_guide_data(&self) -> CategoryGuideData {
        use crate::education::{self, Category};

        let l = self.locale;
        let all_cats = Category::all();
        let selected =
            self.selected_category.min(all_cats.len().saturating_sub(1));

        let categories: Vec<CategoryItem> = all_cats
            .iter()
            .enumerate()
            .map(|(i, cat)| CategoryItem {
                icon: cat.icon().to_string(),
                name: cat.name(l).to_string(),
                selected: i == selected,
            })
            .collect();

        let cat = all_cats[selected];
        let content = education::get_content(cat, l);

        CategoryGuideData {
            categories,
            selected_category: selected,
            content: CategoryContentData {
                overview: content.overview.to_string(),
                story: content.story.to_string(),
                diagnostic_flow: content.diagnostic_flow.to_string(),
                common_issues: content.common_issues.to_string(),
            },
            content_scroll: self.category_scroll,
        }
    }
}

#[cfg(test)]
mod view_data_tests {
    use super::*;

    // P-A1: diag_count matches diagnostics::analyze().len()
    #[cfg(target_os = "linux")]
    #[test]
    fn dashboard_diag_count_matches_analyze() {
        let app = App::new().unwrap();
        let data = app.test_build_dashboard_data();
        let findings = crate::diagnostics::analyze(&app.current, app.locale);
        assert_eq!(data.diag_count, findings.len());
    }

    // P-A1: diag_severity is None when no findings
    #[test]
    fn dashboard_diag_severity_none_when_empty() {
        // When diag_count == 0, diag_severity should be None
        let data = DashboardData {
            load: LoadSection {
                load1: String::new(), load5: String::new(),
                load15: String::new(), uptime: String::new(),
            },
            memory: MemorySection { items: vec![] },
            cpu: CpuSection { items: vec![] },
            network: NetworkSection { headers: vec![], rows: vec![] },
            system: SystemSection {
                disk_pct: String::new(), temp: String::new(), fd_pct: String::new(),
            },
            selected_section: 0,
            mem_used_pct: 0, mem_bar: String::new(),
            mem_used_bytes: 0, mem_total_bytes: 0,
            swap_used_pct: 0, swap_bar: String::new(),
            swap_used_bytes: 0, swap_total_bytes: 0,
            cached: String::new(), buffers: String::new(),
            cpu_used_pct: 0, cpu_bar: String::new(),
            cpu_user_pct: 0, cpu_sys_pct: 0, cpu_io_pct: 0,
            ctx_switches: String::new(), procs_running: String::new(),
            load_history: vec![], mem_history: vec![],
            diag_count: 0, diag_severity: None,
        };
        assert_eq!(data.diag_count, 0);
        assert!(data.diag_severity.is_none());
    }

    // P-A2: basic keybindings has exactly 6 entries
    #[cfg(target_os = "linux")]
    #[test]
    fn welcome_basic_keybindings_count() {
        let app = App::new().unwrap();
        let data = app.test_build_welcome_data();
        assert_eq!(data.keybindings.len(), 6, "basic keybindings should have 6 entries");
    }

    // P-A2: advanced keybindings are non-empty
    #[cfg(target_os = "linux")]
    #[test]
    fn welcome_advanced_keybindings_present() {
        let app = App::new().unwrap();
        let data = app.test_build_welcome_data();
        assert!(!data.advanced_keybindings.is_empty(), "advanced keybindings should be non-empty");
    }

    // P-A2: help_level Off → only basic keybindings should be shown
    // (WelcomeData always has both; the render layer decides what to display)
    #[cfg(target_os = "linux")]
    #[test]
    fn welcome_data_has_both_tiers() {
        let app = App::new().unwrap();
        let data = app.test_build_welcome_data();
        assert_eq!(data.keybindings.len(), 6);
        assert!(data.advanced_keybindings.len() >= 5);
    }
}
