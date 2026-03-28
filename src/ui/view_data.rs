//! ViewData — unified UI data layer.
//!
//! Separates "what to show" from "how to draw". Each view has a typed
//! struct containing pre-formatted strings ready for any renderer
//! (TUI, Web, X11) to consume without accessing [`App`] state directly.

use serde::Serialize;

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
}

#[derive(Debug, Clone, Serialize)]
pub struct DashboardData {
    pub load: LoadSection,
    pub memory: MemorySection,
    pub cpu: CpuSection,
    pub network: NetworkSection,
    pub selected_section: usize,
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
    pub title: String,
    pub footer: String,
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
            View::Dashboard => {
                if l == Locale::Ja { "ダッシュボード" } else { "DASHBOARD" }
            }
            View::Welcome => {
                if l == Locale::Ja { "ようこそ" } else { "WELCOME" }
            }
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

        // Network section
        let net_headers = if l == Locale::Ja {
            vec![
                "IF".to_string(),
                "RX バイト".to_string(),
                "RX パケット".to_string(),
                "TX バイト".to_string(),
                "TX パケット".to_string(),
            ]
        } else {
            vec![
                "Interface".to_string(),
                "RX Bytes".to_string(),
                "RX Pkts".to_string(),
                "TX Bytes".to_string(),
                "TX Pkts".to_string(),
            ]
        };

        let net_rows: Vec<Vec<String>> = self
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

        DashboardData {
            load,
            memory: MemorySection { items: mem_items },
            cpu: CpuSection { items: cpu_items },
            network: NetworkSection {
                headers: net_headers,
                rows: net_rows,
            },
            selected_section: self.selected_dashboard_section,
        }
    }

    fn build_welcome_data(&self) -> WelcomeData {
        let l = self.locale;
        let (title, footer, keybindings) = if l == Locale::Ja {
            (
                "syslenz — /proc の全てを構造化データで".to_string(),
                "D でダッシュボード、O でクラシックモード".to_string(),
                vec![
                    ("D".into(), "ダッシュボード（システム概要）".into()),
                    ("O".into(), "クラシックモード（全ソース一覧）".into()),
                    ("j/k ↑/↓".into(), "ソース / フィールド移動".into()),
                    ("Enter →".into(), "ドリルイン（詳細表示）".into()),
                    ("BS ←".into(), "戻る".into()),
                    ("d".into(), "差分ビュー".into()),
                    ("g".into(), "グラフ（数値フィールドの推移）".into()),
                    ("/".into(), "ソース検索".into()),
                    ("?".into(), "ヘルプパネル（フィールド説明）".into()),
                    ("L".into(), "言語切り替え (EN/JA)".into()),
                    ("e".into(), "JSON エクスポート".into()),
                    ("q".into(), "終了".into()),
                ],
            )
        } else {
            (
                "syslenz — Wireshark for /proc".to_string(),
                "Press D for Dashboard, O for Classic mode".to_string(),
                vec![
                    ("D".into(), "Dashboard (system overview)".into()),
                    ("O".into(), "Classic mode (all sources list)".into()),
                    ("j/k ↑/↓".into(), "Navigate sources / fields".into()),
                    ("Enter →".into(), "Drill in (detail view)".into()),
                    ("BS ←".into(), "Go back".into()),
                    ("d".into(), "Diff view".into()),
                    ("g".into(), "Graph (sparkline of numeric field)".into()),
                    ("/".into(), "Search sources".into()),
                    ("?".into(), "Help panel (field descriptions)".into()),
                    ("L".into(), "Toggle language (EN/JA)".into()),
                    ("e".into(), "Export snapshot as JSON".into()),
                    ("q".into(), "Quit".into()),
                ],
            )
        };

        WelcomeData {
            keybindings,
            title,
            footer,
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
                        FieldRow {
                            name: name_display,
                            value: f.value.display(),
                            unit: f.unit.clone().unwrap_or_default(),
                            description: f.description.clone(),
                            color,
                            is_table,
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
                }
            })
            .collect();

        DiagnosticsData {
            findings: rows,
            title,
        }
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
