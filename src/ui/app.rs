use crate::alert::{self, AlertEvent, AlertRule};
use crate::i18n::Locale;
use crate::proc::{diff_snapshots, DiffItem, FieldValue, Snapshot};
use std::collections::BTreeMap;
use std::sync::mpsc;
use std::time::{Instant, SystemTime};

#[derive(PartialEq)]
pub enum Focus {
    Sidebar,
    Content,
}

#[derive(PartialEq, Clone, Copy)]
pub enum HelpLevel {
    Off,
    Normal,
    Detailed,
    ExtraDetailed,
}

impl HelpLevel {
    pub fn next(self) -> Self {
        match self {
            HelpLevel::Off => HelpLevel::Normal,
            HelpLevel::Normal => HelpLevel::Detailed,
            HelpLevel::Detailed => HelpLevel::ExtraDetailed,
            HelpLevel::ExtraDetailed => HelpLevel::Off,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            HelpLevel::Off => "OFF",
            HelpLevel::Normal => "NORMAL",
            HelpLevel::Detailed => "DETAILED",
            HelpLevel::ExtraDetailed => "EXTRA",
        }
    }
}

pub enum View {
    Dashboard,
    Welcome,
    Overview,
    Detail,
    Diff,
    TableView,
    Graph,
    Diagnostics,
    CategoryGuide,
}

pub struct App {
    pub snapshots: Vec<Snapshot>,
    pub current: Snapshot,
    pub diffs: Vec<DiffItem>,
    pub view: View,
    pub focus: Focus,
    pub selected_source: usize,
    pub source_keys: Vec<String>,
    pub selected_field: usize,
    pub sidebar_scroll: usize,
    pub field_scroll: usize,
    pub table_scroll: usize,
    pub running: bool,
    pub last_refresh: Instant,
    pub auto_refresh: bool,
    pub refresh_interval_ms: u64,
    pub search_query: String,
    pub searching: bool,
    pub filtered_keys: Option<Vec<usize>>,
    pub graph_field: Option<(String, String)>,
    pub status_message: Option<String>,
    pub remote_host: Option<String>,
    pub remote_rx: Option<mpsc::Receiver<Snapshot>>,
    pub locale: Locale,
    pub help_level: HelpLevel,
    pub selected_dashboard_section: usize,
    pub came_from_dashboard: bool,
    pub selected_category: usize,
    pub category_scroll: usize,
    pub help_scroll: usize,
    /// Total line count of the last rendered category guide content (set by render)
    pub category_content_lines: usize,
    /// Visible height of category content panel (set by render)
    pub category_visible_height: usize,
    /// Total line count of help panel content (set by render)
    pub help_content_lines: usize,
    /// Visible height of help panel (set by render)
    pub help_visible_height: usize,
    /// Time-travel diff: None = compare with previous snapshot, Some(i) = compare with snapshots[i]
    pub diff_target_index: Option<usize>,
    /// Alert rules loaded from config
    pub alert_rules: Vec<AlertRule>,
    /// Currently active (firing) alerts
    pub active_alerts: Vec<AlertEvent>,

    // Multi-host support
    /// Labels for each monitored host (e.g. "local", "ssh:host1", "docker:web")
    pub host_labels: Vec<String>,
    /// Receiver channels for each host (None for local)
    pub host_receivers: Vec<Option<mpsc::Receiver<Snapshot>>>,
    /// Current snapshot per host
    pub host_snapshots: Vec<Snapshot>,
    /// Snapshot history per host (ring buffers)
    pub host_snapshot_histories: Vec<Vec<Snapshot>>,
    /// Which host tab is currently displayed
    pub active_host: usize,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let current = Snapshot::capture()?;
        let source_keys: Vec<String> = current.entries.keys().cloned().collect();
        Ok(App {
            snapshots: Vec::new(),
            current,
            diffs: Vec::new(),
            view: View::Dashboard,
            focus: Focus::Content,
            selected_source: 0,
            source_keys,
            selected_field: 0,
            sidebar_scroll: 0,
            field_scroll: 0,
            table_scroll: 0,
            running: true,
            last_refresh: Instant::now(),
            auto_refresh: true,
            refresh_interval_ms: 1000,
            search_query: String::new(),
            searching: false,
            filtered_keys: None,
            graph_field: None,
            status_message: None,
            remote_host: None,
            remote_rx: None,
            locale: Locale::En,
            help_level: HelpLevel::Off,
            selected_dashboard_section: 0,
            came_from_dashboard: false,
            selected_category: 0,
            category_scroll: 0,
            help_scroll: 0,
            category_content_lines: 0,
            category_visible_height: 0,
            help_content_lines: 0,
            help_visible_height: 0,
            diff_target_index: None,
            alert_rules: Vec::new(),
            active_alerts: Vec::new(),
            host_labels: Vec::new(),
            host_receivers: Vec::new(),
            host_snapshots: Vec::new(),
            host_snapshot_histories: Vec::new(),
            active_host: 0,
        })
    }

    pub fn from_remote(host: &str, rx: mpsc::Receiver<Snapshot>) -> anyhow::Result<Self> {
        // Block until we receive the first snapshot
        let current = rx.recv().map_err(|_| anyhow::anyhow!(
            "Failed to receive initial snapshot from '{}'", host
        ))?;
        let source_keys: Vec<String> = current.entries.keys().cloned().collect();
        Ok(App {
            snapshots: Vec::new(),
            current,
            diffs: Vec::new(),
            view: View::Dashboard,
            focus: Focus::Content,
            selected_source: 0,
            source_keys,
            selected_field: 0,
            sidebar_scroll: 0,
            field_scroll: 0,
            table_scroll: 0,
            running: true,
            last_refresh: Instant::now(),
            auto_refresh: true,
            refresh_interval_ms: 1000,
            search_query: String::new(),
            searching: false,
            filtered_keys: None,
            graph_field: None,
            status_message: None,
            remote_host: Some(host.to_owned()),
            remote_rx: Some(rx),
            locale: Locale::En,
            help_level: HelpLevel::Off,
            selected_dashboard_section: 0,
            came_from_dashboard: false,
            selected_category: 0,
            category_scroll: 0,
            help_scroll: 0,
            category_content_lines: 0,
            category_visible_height: 0,
            help_content_lines: 0,
            help_visible_height: 0,
            diff_target_index: None,
            alert_rules: Vec::new(),
            active_alerts: Vec::new(),
            host_labels: Vec::new(),
            host_receivers: Vec::new(),
            host_snapshots: Vec::new(),
            host_snapshot_histories: Vec::new(),
            active_host: 0,
        })
    }

    pub fn from_imported(snapshots: Vec<Snapshot>) -> anyhow::Result<Self> {
        let current = snapshots.last().cloned()
            .ok_or_else(|| anyhow::anyhow!("No snapshots to import"))?;
        let source_keys: Vec<String> = current.entries.keys().cloned().collect();
        let previous = if snapshots.len() > 1 {
            snapshots[..snapshots.len() - 1].to_vec()
        } else {
            Vec::new()
        };
        Ok(App {
            snapshots: previous,
            current,
            diffs: Vec::new(),
            view: View::Overview,
            focus: Focus::Sidebar,
            selected_source: 0,
            source_keys,
            selected_field: 0,
            sidebar_scroll: 0,
            field_scroll: 0,
            table_scroll: 0,
            running: true,
            last_refresh: Instant::now(),
            auto_refresh: false,
            refresh_interval_ms: 1000,
            search_query: String::new(),
            searching: false,
            filtered_keys: None,
            graph_field: None,
            status_message: None,
            remote_host: None,
            remote_rx: None,
            locale: Locale::En,
            help_level: HelpLevel::Off,
            selected_dashboard_section: 0,
            came_from_dashboard: false,
            selected_category: 0,
            category_scroll: 0,
            help_scroll: 0,
            category_content_lines: 0,
            category_visible_height: 0,
            help_content_lines: 0,
            help_visible_height: 0,
            diff_target_index: None,
            alert_rules: Vec::new(),
            active_alerts: Vec::new(),
            host_labels: Vec::new(),
            host_receivers: Vec::new(),
            host_snapshots: Vec::new(),
            host_snapshot_histories: Vec::new(),
            active_host: 0,
        })
    }

    /// Initialize multi-host monitoring. Call after creating the App.
    /// `hosts` is a list of (label, receiver) pairs. The App's own current/remote_rx
    /// becomes host index 0 (local or the single remote that was used to construct App).
    pub fn init_multi_host(&mut self, hosts: Vec<(String, Option<mpsc::Receiver<Snapshot>>)>) {
        if hosts.is_empty() {
            return;
        }
        // Store the current App state as host 0
        let mut labels = Vec::with_capacity(hosts.len() + 1);
        let mut receivers = Vec::with_capacity(hosts.len() + 1);
        let mut snapshots = Vec::with_capacity(hosts.len() + 1);
        let mut histories = Vec::with_capacity(hosts.len() + 1);

        // Host 0: whatever the App was initialized with
        let host0_label = self.remote_host.clone().unwrap_or_else(|| "local".to_string());
        labels.push(host0_label);
        receivers.push(self.remote_rx.take());
        snapshots.push(self.current.clone());
        histories.push(self.snapshots.clone());

        // Additional hosts
        for (label, rx) in hosts {
            labels.push(label);
            // Try to get an initial snapshot from the receiver
            if let Some(ref receiver) = rx {
                if let Ok(snap) = receiver.try_recv() {
                    snapshots.push(snap);
                } else {
                    snapshots.push(Snapshot { timestamp: SystemTime::now(), entries: BTreeMap::new() });
                }
            } else {
                snapshots.push(Snapshot { timestamp: SystemTime::now(), entries: BTreeMap::new() });
            }
            receivers.push(rx);
            histories.push(Vec::new());
        }

        self.host_labels = labels;
        self.host_receivers = receivers;
        self.host_snapshots = snapshots;
        self.host_snapshot_histories = histories;
        self.active_host = 0;
    }

    /// Switch to a different host tab. Returns true if switched.
    pub fn switch_host(&mut self, index: usize) -> bool {
        if self.host_labels.is_empty() || index >= self.host_labels.len() || index == self.active_host {
            return false;
        }

        // Save current host state
        self.host_snapshots[self.active_host] = self.current.clone();
        self.host_snapshot_histories[self.active_host] = self.snapshots.clone();

        // Load new host state
        self.active_host = index;
        self.current = self.host_snapshots[index].clone();
        self.snapshots = self.host_snapshot_histories[index].clone();
        self.source_keys = self.current.entries.keys().cloned().collect();

        // Reset view state for new host
        self.selected_source = 0;
        self.selected_field = 0;
        self.sidebar_scroll = 0;
        self.field_scroll = 0;
        self.table_scroll = 0;
        self.diff_target_index = None;
        self.diffs.clear();
        self.filtered_keys = None;
        self.search_query.clear();
        self.searching = false;

        // Update remote_host for status bar display
        let label = &self.host_labels[index];
        self.remote_host = if label == "local" { None } else { Some(label.clone()) };

        true
    }

    /// Returns true if multi-host mode is active (more than 1 host).
    pub fn is_multi_host(&self) -> bool {
        self.host_labels.len() > 1
    }

    pub fn refresh(&mut self) -> anyhow::Result<()> {
        // In multi-host mode, use the active host's receiver
        let active_rx = if self.is_multi_host() {
            self.host_receivers.get(self.active_host).and_then(|r| r.as_ref())
        } else {
            self.remote_rx.as_ref()
        };
        let new_snapshot = if let Some(rx) = active_rx {
            // Remote mode: drain channel and use the latest snapshot
            let mut latest = None;
            while let Ok(snap) = rx.try_recv() {
                latest = Some(snap);
            }
            match latest {
                Some(s) => s,
                None => return Ok(()), // No new data yet
            }
        } else {
            Snapshot::capture()?
        };

        let old = self.current.clone();
        self.current = new_snapshot;
        self.source_keys = self.current.entries.keys().cloned().collect();

        // Compute diff against the selected target snapshot (time-travel) or previous
        let diff_base = if let Some(idx) = self.diff_target_index {
            self.snapshots.get(idx)
        } else {
            // Default: compare with most recent previous snapshot (the one we just replaced)
            None
        };
        self.diffs = if let Some(base) = diff_base {
            diff_snapshots(base, &self.current)
        } else {
            diff_snapshots(&old, &self.current)
        };

        self.snapshots.push(old);
        if self.snapshots.len() > 60 {
            self.snapshots.remove(0);
        }

        // Adjust diff_target_index after buffer shift (if we removed the oldest)
        if self.snapshots.len() >= 60 {
            if let Some(ref mut idx) = self.diff_target_index {
                if *idx == 0 {
                    self.diff_target_index = None; // target was evicted
                } else {
                    *idx -= 1;
                }
            }
        }

        // Evaluate alerts
        let prev_firing: Vec<usize> = self.active_alerts.iter()
            .filter(|a| a.firing)
            .map(|a| a.rule_index)
            .collect();
        self.active_alerts = alert::evaluate_alerts(
            &self.current,
            &self.alert_rules,
            &prev_firing,
        );

        // Multi-host: keep the active host snapshot in sync
        if self.is_multi_host() {
            self.host_snapshots[self.active_host] = self.current.clone();
            self.host_snapshot_histories[self.active_host] = self.snapshots.clone();
        }

        // Multi-host: drain snapshots from non-active hosts so channels don't fill up
        if self.is_multi_host() {
            for i in 0..self.host_receivers.len() {
                if i == self.active_host {
                    continue;
                }
                if let Some(ref rx) = self.host_receivers[i] {
                    let mut latest = None;
                    while let Ok(snap) = rx.try_recv() {
                        latest = Some(snap);
                    }
                    if let Some(snap) = latest {
                        // Push old snapshot into history
                        let old = self.host_snapshots[i].clone();
                        self.host_snapshots[i] = snap;
                        self.host_snapshot_histories[i].push(old);
                        if self.host_snapshot_histories[i].len() > 60 {
                            self.host_snapshot_histories[i].remove(0);
                        }
                    }
                }
            }
        }

        self.last_refresh = Instant::now();
        Ok(())
    }

    pub fn current_entry_fields(&self) -> Option<&Vec<crate::proc::Field>> {
        let key = self.source_keys.get(self.selected_source)?;
        self.current.entries.get(key).map(|e| &e.fields)
    }

    pub fn current_source_name(&self) -> &str {
        self.source_keys
            .get(self.selected_source)
            .map(|s| s.as_str())
            .unwrap_or("")
    }

    const DASHBOARD_SECTIONS: usize = 5; // load, mem, cpu, net, sys
    const DASHBOARD_SOURCES: [&str; 5] = ["loadavg", "meminfo", "stat", "net/dev", "df"];

    pub fn move_up(&mut self) {
        match self.focus {
            Focus::Sidebar => {
                if self.selected_source > 0 {
                    self.selected_source -= 1;
                    self.selected_field = 0;
                    self.field_scroll = 0;
                    self.table_scroll = 0;
                }
            }
            Focus::Content => match self.view {
                View::Dashboard => {
                    if self.selected_dashboard_section > 0 {
                        self.selected_dashboard_section -= 1;
                    }
                }
                View::TableView => {
                    self.table_scroll = self.table_scroll.saturating_sub(1);
                }
                View::CategoryGuide => {
                    self.category_scroll = self.category_scroll.saturating_sub(1);
                }
                _ => {
                    if self.selected_field > 0 {
                        self.selected_field -= 1;
                    }
                }
            },
        }
    }

    pub fn move_down(&mut self) {
        match self.focus {
            Focus::Sidebar => {
                if self.selected_source + 1 < self.source_keys.len() {
                    self.selected_source += 1;
                    self.selected_field = 0;
                    self.field_scroll = 0;
                    self.table_scroll = 0;
                }
            }
            Focus::Content => match self.view {
                View::Dashboard => {
                    if self.selected_dashboard_section + 1 < Self::DASHBOARD_SECTIONS {
                        self.selected_dashboard_section += 1;
                    }
                }
                View::TableView => {
                    self.table_scroll += 1;
                }
                View::CategoryGuide => {
                    let max_scroll = self.category_content_lines
                        .saturating_sub(self.category_visible_height);
                    if self.category_scroll < max_scroll {
                        self.category_scroll += 1;
                    }
                }
                _ => {
                    if let Some(fields) = self.current_entry_fields() {
                        if self.selected_field + 1 < fields.len() {
                            self.selected_field += 1;
                        }
                    }
                }
            },
        }
    }

    pub fn enter_selected(&mut self) {
        match self.focus {
            Focus::Sidebar => {
                self.focus = Focus::Content;
                self.view = View::Detail;
            }
            Focus::Content => {
                match self.view {
                    View::Dashboard => {
                        // Drill into the selected dashboard section's source
                        let source = Self::DASHBOARD_SOURCES
                            .get(self.selected_dashboard_section)
                            .unwrap_or(&"meminfo");
                        if let Some(idx) = self.source_keys.iter().position(|k| k == source) {
                            self.selected_source = idx;
                            self.selected_field = 0;
                            self.field_scroll = 0;
                            self.came_from_dashboard = true;
                            self.focus = Focus::Content;
                            self.view = View::Detail;
                        }
                    }
                    View::Welcome => {
                        // Enter from welcome goes to Dashboard
                        self.view = View::Dashboard;
                    }
                    View::Overview | View::Detail => {
                        if self.selected_field_is_table() {
                            self.table_scroll = 0;
                            self.view = View::TableView;
                        }
                    }
                    View::TableView | View::Diff | View::Graph | View::Diagnostics | View::CategoryGuide => {}
                }
            }
        }
    }

    pub fn go_back(&mut self) {
        match self.focus {
            Focus::Content => {
                match self.view {
                    View::TableView | View::Graph => {
                        self.view = View::Detail;
                    }
                    View::Detail if self.came_from_dashboard => {
                        self.view = View::Dashboard;
                        self.came_from_dashboard = false;
                    }
                    View::Welcome | View::Dashboard | View::CategoryGuide => {
                        // Stay on dashboard/welcome/category guide, no back
                    }
                    _ => {
                        self.focus = Focus::Sidebar;
                        self.view = View::Overview;
                    }
                }
            }
            Focus::Sidebar => {}
        }
    }

    fn selected_field_is_table(&self) -> bool {
        self.current_entry_fields()
            .and_then(|fields| fields.get(self.selected_field))
            .is_some_and(|f| matches!(f.value, FieldValue::Table(_)))
    }

    pub fn scroll_page_up(&mut self) {
        match self.view {
            View::CategoryGuide => {
                let page = self.category_visible_height.max(1);
                self.category_scroll = self.category_scroll.saturating_sub(page);
            }
            _ => {}
        }
        // Also scroll help if active
        if self.help_level != HelpLevel::Off {
            let page = self.help_visible_height.max(1);
            self.help_scroll = self.help_scroll.saturating_sub(page);
        }
    }

    pub fn scroll_page_down(&mut self) {
        match self.view {
            View::CategoryGuide => {
                let page = self.category_visible_height.max(1);
                let max_scroll = self.category_content_lines
                    .saturating_sub(self.category_visible_height);
                self.category_scroll = (self.category_scroll + page).min(max_scroll);
            }
            _ => {}
        }
        if self.help_level != HelpLevel::Off {
            let page = self.help_visible_height.max(1);
            let max_scroll = self.help_content_lines
                .saturating_sub(self.help_visible_height);
            self.help_scroll = (self.help_scroll + page).min(max_scroll);
        }
    }

    pub fn select_prev_category(&mut self) {
        if self.selected_category > 0 {
            self.selected_category -= 1;
            self.category_scroll = 0;
        }
    }

    pub fn select_next_category(&mut self) {
        let cat_count = crate::education::Category::all().len();
        if self.selected_category + 1 < cat_count {
            self.selected_category += 1;
            self.category_scroll = 0;
        }
    }

    pub fn start_search(&mut self) {
        self.searching = true;
        self.search_query.clear();
    }

    pub fn apply_search(&mut self) {
        self.searching = false;
        if self.search_query.is_empty() {
            self.filtered_keys = None;
            return;
        }
        let q = self.search_query.to_lowercase();
        let matches: Vec<usize> = self
            .source_keys
            .iter()
            .enumerate()
            .filter(|(_, k)| k.to_lowercase().contains(&q))
            .map(|(i, _)| i)
            .collect();

        if let Some(&first) = matches.first() {
            self.selected_source = first;
            self.selected_field = 0;
            self.field_scroll = 0;
        }
        self.filtered_keys = Some(matches);
    }

    pub fn cancel_search(&mut self) {
        self.searching = false;
        self.search_query.clear();
        self.filtered_keys = None;
    }

    pub fn start_graph(&mut self) {
        let source_key = self.current_source_name().to_string();
        if let Some(fields) = self.current_entry_fields() {
            if let Some(field) = fields.get(self.selected_field) {
                let is_numeric = matches!(
                    field.value,
                    FieldValue::Bytes(_)
                        | FieldValue::Integer(_)
                        | FieldValue::Float(_)
                        | FieldValue::Duration(_)
                );
                if is_numeric {
                    self.graph_field = Some((source_key, field.name.clone()));
                    self.view = View::Graph;
                }
            }
        }
    }

    /// Move diff comparison target to an older snapshot.
    pub fn diff_older(&mut self) {
        if self.snapshots.is_empty() {
            return;
        }
        let last_idx = self.snapshots.len() - 1;
        match self.diff_target_index {
            None => {
                // Currently comparing with the most recent previous snapshot.
                // Move to the one before that (second to last).
                if last_idx > 0 {
                    self.diff_target_index = Some(last_idx - 1);
                    self.recompute_diff();
                }
            }
            Some(idx) => {
                if idx > 0 {
                    self.diff_target_index = Some(idx - 1);
                    self.recompute_diff();
                }
            }
        }
    }

    /// Move diff comparison target to a newer snapshot.
    pub fn diff_newer(&mut self) {
        if self.snapshots.is_empty() {
            return;
        }
        let last_idx = self.snapshots.len() - 1;
        match self.diff_target_index {
            Some(idx) => {
                if idx >= last_idx {
                    // Back to default (compare with most recent previous)
                    self.diff_target_index = None;
                    self.recompute_diff();
                } else {
                    self.diff_target_index = Some(idx + 1);
                    self.recompute_diff();
                }
            }
            None => {
                // Already at most recent, do nothing
            }
        }
    }

    /// How many snapshots back we are diffing. 1 = previous (default), N = T-N.
    pub fn diff_offset(&self) -> usize {
        match self.diff_target_index {
            None => 1,
            Some(idx) => self.snapshots.len().saturating_sub(idx),
        }
    }

    /// Recompute diffs based on current diff_target_index.
    fn recompute_diff(&mut self) {
        if let Some(idx) = self.diff_target_index {
            if let Some(base) = self.snapshots.get(idx) {
                self.diffs = diff_snapshots(base, &self.current);
                return;
            }
        }
        // Default: compare with last snapshot
        if let Some(last) = self.snapshots.last() {
            self.diffs = diff_snapshots(last, &self.current);
        }
    }

    /// Get text that can be copied to clipboard based on current view/selection.
    pub fn get_copyable_text(&self) -> Option<String> {
        match self.view {
            View::Detail | View::Overview => {
                // Copy selected field's value
                self.current_entry_fields()
                    .and_then(|fields| fields.get(self.selected_field))
                    .map(|f| format!("{}: {}", f.name, f.value.display()))
            }
            View::Dashboard => {
                // Copy the whole dashboard section as text
                let source = Self::DASHBOARD_SOURCES
                    .get(self.selected_dashboard_section)?;
                let entry = self.current.entries.get(*source)?;
                let lines: Vec<String> = entry.fields.iter()
                    .map(|f| format!("{}: {} {}", f.name, f.value.display(),
                        f.unit.as_deref().unwrap_or("")))
                    .collect();
                Some(format!("[{}]\n{}", source, lines.join("\n")))
            }
            View::Diagnostics => {
                // Copy all diagnostics as text
                let findings = crate::diagnostics::analyze(&self.current, self.locale);
                if findings.is_empty() {
                    Some("No diagnostic findings.".to_string())
                } else {
                    let lines: Vec<String> = findings.iter()
                        .map(|f| format!("[{}] {} — {}\n  {}",
                            f.severity.label(self.locale), f.title, f.detail, f.suggestion))
                        .collect();
                    Some(lines.join("\n\n"))
                }
            }
            _ => None,
        }
    }
}
