use crate::alert::{self, AlertEvent, AlertRule};
use crate::i18n::Locale;
use crate::proc::{diff_snapshots, DiffItem, FieldValue, Snapshot};
use std::collections::BTreeMap;
use std::sync::mpsc;
use std::time::{Instant, SystemTime};

// ---------------------------------------------------------------------------
// BL-034: ConnectionStatus — tracks SSH / remote connection state
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionStatus {
    /// Monitoring the local machine (no remote connection).
    Local,
    /// Successfully receiving data from a remote host.
    Connected { last_seen: Instant },
    /// Connection was lost; `last_seen` is the last successful data time,
    /// `since` is when the disconnect was detected.
    Disconnected { last_seen: Instant, since: Instant },
    /// Currently attempting to (re)connect.
    Connecting,
}

impl ConnectionStatus {
    /// Short label for the status bar.
    pub fn label(&self) -> &'static str {
        match self {
            ConnectionStatus::Local => "local",
            ConnectionStatus::Connected { .. } => "connected",
            ConnectionStatus::Disconnected { .. } => "disconnected",
            ConnectionStatus::Connecting => "connecting",
        }
    }
}

// ---------------------------------------------------------------------------
// BL-030: HostState — per-host state container
// ---------------------------------------------------------------------------

/// Per-host monitoring state. In multi-host mode each tab owns one instance.
pub struct HostState {
    /// Identifier: "localhost", "ssh:user@host", "docker:container", etc.
    pub label: String,
    /// Current (latest) snapshot for this host.
    pub current: Snapshot,
    /// Historical snapshots (ring buffer, max `max_snapshots` entries).
    pub snapshots: Vec<Snapshot>,
    /// Maximum number of historical snapshots to retain.
    pub max_snapshots: usize,
    /// Receiver channel for remote data (None for local-only).
    pub receiver: Option<mpsc::Receiver<Snapshot>>,
    /// Connection status for this host.
    pub connection_status: ConnectionStatus,
    /// Alert events currently firing on this host.
    pub alert_events: Vec<AlertEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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
    /// BL-074: Interactive tutorial mode
    Tutorial,
}

pub struct App {
    // === Active view state (mirrors the active host) ===
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

    // === BL-030: Multi-host state (HostState per host) ===
    /// Per-host state containers. `hosts[0]` is always localhost or the primary host.
    pub hosts: Vec<HostState>,
    /// Which host tab is currently displayed.
    pub active_host: usize,
    /// Connection status for the primary host (kept for backward compat in single-host mode).
    pub connection_status: ConnectionStatus,

    // === BL-074: Tutorial mode ===
    /// Current tutorial step (0-based). None if not in tutorial mode.
    pub tutorial_step: Option<usize>,

    // === G18-10: Diagnostics → Detail jump navigation ===
    /// View history stack for back-navigation from diagnostic jumps.
    pub view_history: Vec<(View, Focus, usize, usize)>,
    /// Selected diagnostic finding index in Diagnostics view.
    pub selected_diagnostic: usize,
    /// When viewing related_metrics picker, which metric is selected.
    pub selected_related_metric: Option<usize>,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let current = Snapshot::capture()?;
        let source_keys: Vec<String> = current.entries.keys().cloned().collect();
        let host0 = HostState {
            label: "localhost".to_string(),
            current: current.clone(),
            snapshots: Vec::new(),
            max_snapshots: 60,
            receiver: None,
            connection_status: ConnectionStatus::Local,
            alert_events: Vec::new(),
        };
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
            hosts: vec![host0],
            active_host: 0,
            connection_status: ConnectionStatus::Local,
            tutorial_step: None,
            view_history: Vec::new(),
            selected_diagnostic: 0,
            selected_related_metric: None,
        })
    }

    pub fn from_remote(host: &str, rx: mpsc::Receiver<Snapshot>) -> anyhow::Result<Self> {
        // Block until we receive the first snapshot
        let current = rx.recv().map_err(|_| anyhow::anyhow!(
            "Failed to receive initial snapshot from '{}'", host
        ))?;
        let source_keys: Vec<String> = current.entries.keys().cloned().collect();
        let now = Instant::now();
        let conn_status = ConnectionStatus::Connected { last_seen: now };
        let host0 = HostState {
            label: host.to_owned(),
            current: current.clone(),
            snapshots: Vec::new(),
            max_snapshots: 60,
            receiver: Some(rx),
            connection_status: conn_status.clone(),
            alert_events: Vec::new(),
        };
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
            remote_rx: None, // Receiver now stored in HostState
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
            hosts: vec![host0],
            active_host: 0,
            connection_status: conn_status,
            tutorial_step: None,
            view_history: Vec::new(),
            selected_diagnostic: 0,
            selected_related_metric: None,
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
        let host0 = HostState {
            label: "imported".to_string(),
            current: current.clone(),
            snapshots: previous.clone(),
            max_snapshots: 60,
            receiver: None,
            connection_status: ConnectionStatus::Local,
            alert_events: Vec::new(),
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
            hosts: vec![host0],
            active_host: 0,
            connection_status: ConnectionStatus::Local,
            tutorial_step: None,
            view_history: Vec::new(),
            selected_diagnostic: 0,
            selected_related_metric: None,
        })
    }

    /// Initialize multi-host monitoring. Call after creating the App.
    /// `additional_hosts` is a list of (label, receiver) pairs beyond the primary host.
    /// The App's `hosts[0]` is already set by the constructor.
    pub fn init_multi_host(&mut self, additional_hosts: Vec<(String, Option<mpsc::Receiver<Snapshot>>)>) {
        if additional_hosts.is_empty() {
            return;
        }

        // Additional hosts
        for (label, rx) in additional_hosts {
            let initial_snap = if let Some(ref receiver) = rx {
                receiver.try_recv().unwrap_or_else(|_| Snapshot {
                    timestamp: SystemTime::now(),
                    entries: BTreeMap::new(),
                })
            } else {
                Snapshot { timestamp: SystemTime::now(), entries: BTreeMap::new() }
            };
            let conn_status = if rx.is_some() {
                ConnectionStatus::Connecting
            } else {
                ConnectionStatus::Local
            };
            self.hosts.push(HostState {
                label,
                current: initial_snap,
                snapshots: Vec::new(),
                max_snapshots: 60,
                receiver: rx,
                connection_status: conn_status,
                alert_events: Vec::new(),
            });
        }

        self.active_host = 0;
    }

    /// Switch to a different host tab. Returns true if switched.
    pub fn switch_host(&mut self, index: usize) -> bool {
        if self.hosts.len() <= 1 || index >= self.hosts.len() || index == self.active_host {
            return false;
        }

        // Save current host state back into HostState
        self.hosts[self.active_host].current = self.current.clone();
        self.hosts[self.active_host].snapshots = self.snapshots.clone();
        self.hosts[self.active_host].alert_events = self.active_alerts.clone();

        // Load new host state
        self.active_host = index;
        self.current = self.hosts[index].current.clone();
        self.snapshots = self.hosts[index].snapshots.clone();
        self.active_alerts = self.hosts[index].alert_events.clone();
        self.connection_status = self.hosts[index].connection_status.clone();
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
        let label = &self.hosts[index].label;
        self.remote_host = if label == "localhost" || label == "local" { None } else { Some(label.clone()) };

        true
    }

    /// Returns true if multi-host mode is active (more than 1 host).
    pub fn is_multi_host(&self) -> bool {
        self.hosts.len() > 1
    }

    /// Returns the active host's HostState.
    pub fn active_host_state(&self) -> &HostState {
        &self.hosts[self.active_host]
    }

    /// Returns a mutable reference to the active host's HostState.
    pub fn active_host_state_mut(&mut self) -> &mut HostState {
        &mut self.hosts[self.active_host]
    }

    /// Get labels for all hosts (for tab bar rendering).
    pub fn host_labels(&self) -> Vec<&str> {
        self.hosts.iter().map(|h| h.label.as_str()).collect()
    }

    pub fn refresh(&mut self) -> anyhow::Result<()> {
        // Use the active host's receiver (from HostState)
        let active_host = &self.hosts[self.active_host];
        let has_receiver = active_host.receiver.is_some();

        let new_snapshot = if has_receiver {
            let rx = self.hosts[self.active_host].receiver.as_ref().unwrap();
            // Remote mode: drain channel and use the latest snapshot
            let mut latest = None;
            while let Ok(snap) = rx.try_recv() {
                latest = Some(snap);
            }
            match latest {
                Some(s) => {
                    // Update connection status: we got data
                    self.hosts[self.active_host].connection_status =
                        ConnectionStatus::Connected { last_seen: Instant::now() };
                    self.connection_status = self.hosts[self.active_host].connection_status.clone();
                    s
                }
                None => {
                    // No new data — check if we should mark as disconnected
                    if let ConnectionStatus::Connected { last_seen } = &self.hosts[self.active_host].connection_status {
                        if last_seen.elapsed().as_secs() > 10 {
                            let status = ConnectionStatus::Disconnected {
                                last_seen: *last_seen,
                                since: Instant::now(),
                            };
                            self.hosts[self.active_host].connection_status = status.clone();
                            self.connection_status = status;
                        }
                    }
                    return Ok(());
                }
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

        // BL-071: Execute external actions for newly-firing alerts
        alert::execute_actions(&self.alert_rules, &self.active_alerts, &prev_firing);

        // Sync active host state
        self.hosts[self.active_host].current = self.current.clone();
        self.hosts[self.active_host].snapshots = self.snapshots.clone();
        self.hosts[self.active_host].alert_events = self.active_alerts.clone();

        // Multi-host: drain snapshots from non-active hosts so channels don't fill up
        if self.is_multi_host() {
            for i in 0..self.hosts.len() {
                if i == self.active_host {
                    continue;
                }
                if let Some(ref rx) = self.hosts[i].receiver {
                    let mut latest = None;
                    while let Ok(snap) = rx.try_recv() {
                        latest = Some(snap);
                    }
                    if let Some(snap) = latest {
                        // Push old snapshot into history
                        let old = self.hosts[i].current.clone();
                        self.hosts[i].current = snap;
                        self.hosts[i].snapshots.push(old);
                        if self.hosts[i].snapshots.len() > self.hosts[i].max_snapshots {
                            self.hosts[i].snapshots.remove(0);
                        }
                        self.hosts[i].connection_status =
                            ConnectionStatus::Connected { last_seen: Instant::now() };
                    } else {
                        // Check for disconnect on non-active hosts too
                        if let ConnectionStatus::Connected { last_seen } = &self.hosts[i].connection_status {
                            if last_seen.elapsed().as_secs() > 10 {
                                self.hosts[i].connection_status = ConnectionStatus::Disconnected {
                                    last_seen: *last_seen,
                                    since: Instant::now(),
                                };
                            }
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
                View::Diagnostics => {
                    if let Some(ref mut sel) = self.selected_related_metric {
                        *sel = sel.saturating_sub(1);
                    } else if self.selected_diagnostic > 0 {
                        self.selected_diagnostic -= 1;
                    }
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
                View::Diagnostics => {
                    if let Some(ref mut sel) = self.selected_related_metric {
                        let findings = crate::diagnostics::analyze(&self.current, self.locale);
                        if let Some(f) = findings.get(self.selected_diagnostic) {
                            if *sel + 1 < f.related_metrics.len() {
                                *sel += 1;
                            }
                        }
                    } else {
                        let count = crate::diagnostics::analyze(&self.current, self.locale).len();
                        if self.selected_diagnostic + 1 < count {
                            self.selected_diagnostic += 1;
                        }
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
                    View::Diagnostics => {
                        let findings = crate::diagnostics::analyze(&self.current, self.locale);
                        if let Some(finding) = findings.get(self.selected_diagnostic) {
                            if finding.related_metrics.is_empty() {
                                // No related metrics to jump to
                            } else if let Some(sel) = self.selected_related_metric {
                                // Picker is open, jump to selected metric
                                if let Some((source, field)) = finding.related_metrics.get(sel) {
                                    self.jump_to_metric(source, field);
                                }
                            } else if finding.related_metrics.len() == 1 {
                                // Single metric: jump directly
                                let (source, field) = &finding.related_metrics[0];
                                self.jump_to_metric(&source.clone(), &field.clone());
                            } else {
                                // Multiple metrics: open picker
                                self.selected_related_metric = Some(0);
                            }
                        }
                    }
                    View::TableView | View::Diff | View::Graph | View::CategoryGuide | View::Tutorial => {}
                }
            }
        }
    }

    /// Jump from Diagnostics view to Detail view for a specific source/field.
    fn jump_to_metric(&mut self, source: &str, field: &str) {
        if let Some(source_idx) = self.source_keys.iter().position(|k| k == source) {
            let field_idx = self.current.entries.get(source)
                .and_then(|entry| entry.fields.iter().position(|f| f.name == field))
                .unwrap_or(0);
            // Push current state to history for back-navigation
            self.view_history.push((
                View::Diagnostics,
                Focus::Content,
                self.selected_source,
                self.selected_field,
            ));
            self.selected_source = source_idx;
            self.selected_field = field_idx;
            self.field_scroll = 0;
            self.selected_related_metric = None;
            self.focus = Focus::Content;
            self.view = View::Detail;
        }
    }

    pub fn go_back(&mut self) {
        // If related_metrics picker is open, close it first
        if self.selected_related_metric.is_some() {
            self.selected_related_metric = None;
            return;
        }
        // If we have a view_history entry, pop it and restore state
        if let Some((prev_view, prev_focus, prev_source, prev_field)) = self.view_history.pop() {
            self.view = prev_view;
            self.focus = prev_focus;
            self.selected_source = prev_source;
            self.selected_field = prev_field;
            self.selected_related_metric = None;
            return;
        }
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

    // === BL-074: Tutorial mode ===

    /// Total number of tutorial steps.
    pub const TUTORIAL_STEPS: usize = 6;

    /// Start tutorial mode at step 0.
    pub fn start_tutorial(&mut self) {
        self.tutorial_step = Some(0);
        self.view = View::Tutorial;
        self.focus = Focus::Content;
    }

    /// Advance to the next tutorial step. If finished, exit tutorial.
    pub fn tutorial_next(&mut self) {
        if let Some(step) = self.tutorial_step {
            if step + 1 >= Self::TUTORIAL_STEPS {
                self.tutorial_finish();
            } else {
                self.tutorial_step = Some(step + 1);
                // Change the view to match the tutorial step
                match step + 1 {
                    1 => {
                        // Step 1: Navigate the sidebar
                        self.view = View::Overview;
                        self.focus = Focus::Sidebar;
                    }
                    2 => {
                        // Step 2: Enter detail view
                        self.view = View::Detail;
                        self.focus = Focus::Content;
                    }
                    3 => {
                        // Step 3: Diff view
                        self.view = View::Diff;
                        self.focus = Focus::Content;
                    }
                    4 => {
                        // Step 4: Graph view
                        self.start_graph();
                        // Fallback if no numeric field selected
                        if !matches!(self.view, View::Graph) {
                            self.view = View::Tutorial;
                        }
                    }
                    5 => {
                        // Step 5: Dashboard
                        self.view = View::Dashboard;
                        self.focus = Focus::Content;
                    }
                    _ => {}
                }
            }
        }
    }

    /// Go back to the previous tutorial step.
    pub fn tutorial_prev(&mut self) {
        if let Some(step) = self.tutorial_step {
            if step > 0 {
                self.tutorial_step = Some(step - 1);
            }
        }
    }

    /// Exit tutorial mode, transition to normal Dashboard view.
    pub fn tutorial_finish(&mut self) {
        self.tutorial_step = None;
        self.view = View::Dashboard;
        self.focus = Focus::Content;
        self.status_message = Some(
            if self.locale == Locale::Ja {
                "チュートリアル完了! 自由に操作してください".to_string()
            } else {
                "Tutorial complete! Explore freely".to_string()
            }
        );
    }

    /// Get tutorial step text (English and Japanese).
    pub fn tutorial_text(&self) -> (&'static str, &'static str) {
        let step = self.tutorial_step.unwrap_or(0);
        match step {
            0 => (
                "Welcome to syslenz! This tutorial will guide you through the key features.\n\n\
                 syslenz reads data from /proc and other system interfaces\n\
                 to show you structured, live system information.\n\n\
                 Press Enter or Right arrow to proceed to the next step.\n\
                 Press Backspace or Left arrow to go back.\n\
                 Press q to exit the tutorial at any time.",
                "syslenz へようこそ! このチュートリアルで主な機能を体験します。\n\n\
                 syslenz は /proc などからシステム情報を読み取り、\n\
                 構造化されたライブデータとして表示します。\n\n\
                 Enter または右矢印で次のステップへ進みます。\n\
                 Backspace または左矢印で戻ります。\n\
                 q でチュートリアルを終了します。",
            ),
            1 => (
                "Step 1: Source List (Sidebar)\n\n\
                 The left sidebar lists all /proc sources (meminfo, loadavg, stat, etc.).\n\
                 Use j/k (or Up/Down) to navigate the list.\n\
                 Each source represents a different aspect of your system.\n\n\
                 Press Enter to continue.",
                "ステップ 1: ソースリスト (サイドバー)\n\n\
                 左のサイドバーに全 /proc ソース (meminfo, loadavg, stat 等) が表示されます。\n\
                 j/k (または上下矢印) でリストを移動します。\n\
                 各ソースはシステムの異なる側面を表しています。\n\n\
                 Enter で次へ進みます。",
            ),
            2 => (
                "Step 2: Detail View\n\n\
                 Press Enter on a source to see its fields.\n\
                 Each field has a name, value, unit, and description.\n\
                 Use j/k to browse fields. Press Tab to switch focus.\n\n\
                 Press Enter to continue.",
                "ステップ 2: 詳細ビュー\n\n\
                 ソースで Enter を押すとフィールドが表示されます。\n\
                 各フィールドには名前、値、単位、説明があります。\n\
                 j/k でフィールドを閲覧。Tab でフォーカスを切り替えます。\n\n\
                 Enter で次へ進みます。",
            ),
            3 => (
                "Step 3: Diff View (d key)\n\n\
                 Press d to enter diff mode.\n\
                 This shows what changed between the current and previous snapshot.\n\
                 Green = increased, Red = decreased.\n\
                 Use [ and ] to time-travel through snapshot history.\n\n\
                 Press Enter to continue.",
                "ステップ 3: Diff ビュー (d キー)\n\n\
                 d を押すと diff モードに入ります。\n\
                 現在と前回のスナップショットの差分が表示されます。\n\
                 緑 = 増加、赤 = 減少。\n\
                 [ と ] でスナップショット履歴をタイムトラベルできます。\n\n\
                 Enter で次へ進みます。",
            ),
            4 => (
                "Step 4: Graph View (g key)\n\n\
                 Select a numeric field and press g to see a time-series graph.\n\
                 The graph shows how the value has changed over recent snapshots.\n\
                 Press Backspace to return from the graph.\n\n\
                 Press Enter to continue.",
                "ステップ 4: グラフビュー (g キー)\n\n\
                 数値フィールドを選択して g を押すと時系列グラフが表示されます。\n\
                 最近のスナップショットでの値の変化が確認できます。\n\
                 Backspace でグラフから戻ります。\n\n\
                 Enter で次へ進みます。",
            ),
            5 => (
                "Step 5: Dashboard (D key)\n\n\
                 Press D to see the Dashboard — a live overview of key metrics:\n\
                 Load average, Memory, CPU, Network, and Disk.\n\
                 Use j/k to select sections, Enter to drill into details.\n\n\
                 This is the end of the tutorial! Press Enter to finish\n\
                 and explore syslenz on your own.\n\n\
                 Other useful keys: ? for help, / for search, e to export,\n\
                 X for diagnostics, C for category guide, L to toggle language.",
                "ステップ 5: ダッシュボード (D キー)\n\n\
                 D でダッシュボード表示 — 主要メトリクスの概要:\n\
                 負荷平均、メモリ、CPU、ネットワーク、ディスク。\n\
                 j/k でセクション選択、Enter で詳細へドリルイン。\n\n\
                 これでチュートリアル終了です! Enter で完了し\n\
                 自由に syslenz を操作してください。\n\n\
                 便利なキー: ? ヘルプ、/ 検索、e エクスポート、\n\
                 X 診断、C カテゴリガイド、L 言語切り替え。",
            ),
            _ => ("", ""),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proc::{Field, FieldValue, ProcEntry, Snapshot};
    use std::collections::BTreeMap;
    use std::time::SystemTime;

    fn make_snapshot(fields: Vec<(&str, &str, FieldValue)>) -> Snapshot {
        let mut entries = BTreeMap::new();
        for (source, name, value) in fields {
            let entry = entries.entry(source.to_string()).or_insert_with(|| ProcEntry {
                source: format!("/proc/{}", source),
                fields: Vec::new(),
            });
            entry.fields.push(Field {
                name: name.to_string(),
                value,
                unit: None,
                description: String::new(),
            });
        }
        Snapshot { timestamp: SystemTime::now(), entries }
    }

    // BL-030: HostState struct exists and holds per-host data
    #[test]
    fn host_state_holds_snapshots() {
        let snap = make_snapshot(vec![("meminfo", "MemTotal", FieldValue::Bytes(1024))]);
        let hs = HostState {
            label: "test-host".to_string(),
            current: snap.clone(),
            snapshots: vec![snap],
            max_snapshots: 60,
            receiver: None,
            connection_status: ConnectionStatus::Local,
            alert_events: Vec::new(),
        };
        assert_eq!(hs.label, "test-host");
        assert_eq!(hs.snapshots.len(), 1);
        assert_eq!(hs.max_snapshots, 60);
    }

    // BL-030: App always has at least one HostState after construction
    #[cfg(target_os = "linux")]
    #[test]
    fn app_new_has_one_host() {
        let app = App::new().unwrap();
        assert_eq!(app.hosts.len(), 1);
        assert_eq!(app.hosts[0].label, "localhost");
        assert!(matches!(app.hosts[0].connection_status, ConnectionStatus::Local));
        assert!(!app.is_multi_host());
    }

    // BL-030: active_host_state returns correct host
    #[cfg(target_os = "linux")]
    #[test]
    fn active_host_state_returns_first() {
        let app = App::new().unwrap();
        assert_eq!(app.active_host_state().label, "localhost");
    }

    // BL-034: ConnectionStatus enum variants
    #[test]
    fn connection_status_labels() {
        assert_eq!(ConnectionStatus::Local.label(), "local");
        let now = Instant::now();
        assert_eq!(ConnectionStatus::Connected { last_seen: now }.label(), "connected");
        assert_eq!(ConnectionStatus::Disconnected { last_seen: now, since: now }.label(), "disconnected");
        assert_eq!(ConnectionStatus::Connecting.label(), "connecting");
    }

    // BL-034: ConnectionStatus equality
    #[test]
    fn connection_status_equality() {
        assert_eq!(ConnectionStatus::Local, ConnectionStatus::Local);
        assert_eq!(ConnectionStatus::Connecting, ConnectionStatus::Connecting);
        assert_ne!(ConnectionStatus::Local, ConnectionStatus::Connecting);
    }

    // BL-031: time-travel diff with empty snapshots doesn't panic
    #[cfg(target_os = "linux")]
    #[test]
    fn diff_older_empty_snapshots_no_panic() {
        let mut app = App::new().unwrap();
        app.diff_older(); // should be a no-op, not panic
        assert!(app.diff_target_index.is_none());
    }

    // BL-031: diff_offset returns 1 by default
    #[cfg(target_os = "linux")]
    #[test]
    fn diff_offset_default_is_one() {
        let app = App::new().unwrap();
        assert_eq!(app.diff_offset(), 1);
    }

    // BL-031: time-travel diff navigation
    #[cfg(target_os = "linux")]
    #[test]
    fn diff_navigation() {
        let mut app = App::new().unwrap();
        // Add some snapshots
        for _ in 0..5 {
            let _ = app.refresh();
        }
        if app.snapshots.len() >= 3 {
            app.diff_older(); // should set diff_target_index
            assert!(app.diff_target_index.is_some());
            let offset_after_older = app.diff_offset();
            assert!(offset_after_older > 1);

            app.diff_newer(); // move back towards most recent
            let offset_after_newer = app.diff_offset();
            assert!(offset_after_newer <= offset_after_older);
        }
    }

    // BL-032: alert evaluation with matching rule fires
    #[test]
    fn alert_fires_on_threshold() {
        let snap = make_snapshot(vec![
            ("loadavg", "load_1min", FieldValue::Float(12.0)),
        ]);
        let rules = vec![crate::alert::AlertRule {
            source: "loadavg".to_string(),
            field: "load_1min".to_string(),
            condition: "> 8.0".to_string(),
            severity: "warning".to_string(),
            message: "High load".to_string(),
            action: None,
        }];
        let events = crate::alert::evaluate_alerts(&snap, &rules, &[]);
        assert_eq!(events.len(), 1);
        assert!(events[0].firing);
        assert_eq!(events[0].severity, "warning");
    }

    // BL-032: alert does not fire when below threshold
    #[test]
    fn alert_does_not_fire_below_threshold() {
        let snap = make_snapshot(vec![
            ("loadavg", "load_1min", FieldValue::Float(2.0)),
        ]);
        let rules = vec![crate::alert::AlertRule {
            source: "loadavg".to_string(),
            field: "load_1min".to_string(),
            condition: "> 8.0".to_string(),
            severity: "warning".to_string(),
            message: "High load".to_string(),
            action: None,
        }];
        let events = crate::alert::evaluate_alerts(&snap, &rules, &[]);
        // Should produce no firing events
        let firing: Vec<_> = events.iter().filter(|e| e.firing).collect();
        assert!(firing.is_empty());
    }

    // BL-032: debounce — previously firing rule does not duplicate
    #[test]
    fn alert_debounce() {
        let snap = make_snapshot(vec![
            ("meminfo", "MemAvailable", FieldValue::Bytes(100_000_000)),
        ]);
        let rules = vec![crate::alert::AlertRule {
            source: "meminfo".to_string(),
            field: "MemAvailable".to_string(),
            condition: "< 500000000".to_string(),
            severity: "critical".to_string(),
            message: "Low memory".to_string(),
            action: None,
        }];
        // First evaluation
        let events1 = crate::alert::evaluate_alerts(&snap, &rules, &[]);
        assert_eq!(events1.len(), 1);
        assert!(events1[0].firing);
        // Second evaluation with prev_firing
        let prev_firing: Vec<usize> = events1.iter().filter(|e| e.firing).map(|e| e.rule_index).collect();
        let events2 = crate::alert::evaluate_alerts(&snap, &rules, &prev_firing);
        // Still one event, still firing — not duplicated
        let firing2: Vec<_> = events2.iter().filter(|e| e.firing).collect();
        assert_eq!(firing2.len(), 1);
    }

    // BL-033: init_multi_host adds hosts
    #[cfg(target_os = "linux")]
    #[test]
    fn multi_host_init() {
        let mut app = App::new().unwrap();
        assert!(!app.is_multi_host());

        // Add two "additional" hosts (no actual receivers — just None)
        app.init_multi_host(vec![
            ("ssh:host1".to_string(), None),
            ("ssh:host2".to_string(), None),
        ]);
        assert!(app.is_multi_host());
        assert_eq!(app.hosts.len(), 3); // localhost + 2 remotes
        assert_eq!(app.hosts[0].label, "localhost");
        assert_eq!(app.hosts[1].label, "ssh:host1");
        assert_eq!(app.hosts[2].label, "ssh:host2");
    }

    // BL-033: switch_host changes active state
    #[cfg(target_os = "linux")]
    #[test]
    fn multi_host_switch() {
        let mut app = App::new().unwrap();
        app.init_multi_host(vec![
            ("ssh:host1".to_string(), None),
        ]);
        assert_eq!(app.active_host, 0);
        assert!(app.switch_host(1));
        assert_eq!(app.active_host, 1);
        assert_eq!(app.remote_host, Some("ssh:host1".to_string()));
    }

    // BL-033: switch_host to same index returns false
    #[cfg(target_os = "linux")]
    #[test]
    fn multi_host_switch_same_returns_false() {
        let mut app = App::new().unwrap();
        app.init_multi_host(vec![("ssh:host1".to_string(), None)]);
        assert!(!app.switch_host(0)); // already on 0
    }

    // BL-033: switch to invalid index returns false
    #[cfg(target_os = "linux")]
    #[test]
    fn multi_host_switch_invalid_returns_false() {
        let mut app = App::new().unwrap();
        app.init_multi_host(vec![("ssh:host1".to_string(), None)]);
        assert!(!app.switch_host(99)); // out of bounds
    }

    // BL-033: single host switch_host always returns false
    #[cfg(target_os = "linux")]
    #[test]
    fn single_host_switch_returns_false() {
        let mut app = App::new().unwrap();
        assert!(!app.switch_host(0));
        assert!(!app.switch_host(1));
    }

    // BL-074: Tutorial mode starts and progresses
    #[cfg(target_os = "linux")]
    #[test]
    fn tutorial_start_and_progress() {
        let mut app = App::new().unwrap();
        assert!(app.tutorial_step.is_none());

        app.start_tutorial();
        assert_eq!(app.tutorial_step, Some(0));
        assert!(matches!(app.view, View::Tutorial));

        app.tutorial_next();
        assert_eq!(app.tutorial_step, Some(1));

        app.tutorial_prev();
        assert_eq!(app.tutorial_step, Some(0));

        // prev at step 0 stays at 0
        app.tutorial_prev();
        assert_eq!(app.tutorial_step, Some(0));
    }

    // BL-074: Tutorial finish exits tutorial mode
    #[cfg(target_os = "linux")]
    #[test]
    fn tutorial_finish_exits() {
        let mut app = App::new().unwrap();
        app.start_tutorial();
        app.tutorial_finish();
        assert!(app.tutorial_step.is_none());
        assert!(matches!(app.view, View::Dashboard));
        assert!(app.status_message.is_some());
    }

    // BL-074: Tutorial text returns non-empty for all steps
    #[cfg(target_os = "linux")]
    #[test]
    fn tutorial_text_all_steps() {
        let mut app = App::new().unwrap();
        for step in 0..App::TUTORIAL_STEPS {
            app.tutorial_step = Some(step);
            let (en, ja) = app.tutorial_text();
            assert!(!en.is_empty(), "Step {} English text empty", step);
            assert!(!ja.is_empty(), "Step {} Japanese text empty", step);
        }
    }

    // BL-073: OtelLevel parsing
    #[test]
    fn otel_level_from_str() {
        use crate::otel::OtelLevel;
        assert_eq!(OtelLevel::from_str("core"), OtelLevel::Core);
        assert_eq!(OtelLevel::from_str("Core"), OtelLevel::Core);
        assert_eq!(OtelLevel::from_str("CORE"), OtelLevel::Core);
        assert_eq!(OtelLevel::from_str("full"), OtelLevel::Full);
        assert_eq!(OtelLevel::from_str("anything"), OtelLevel::Full);
    }
}
