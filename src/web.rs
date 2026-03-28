//! Web UI server using Axum.
//!
//! Serves a single-page dashboard with live-updating system metrics,
//! using Server-Sent Events for real-time streaming.

#[cfg(feature = "web")]
use crate::i18n::Locale;
#[cfg(feature = "web")]
use crate::proc::Snapshot;

#[cfg(feature = "web")]
use axum::{
    extract::{Query, State},
    response::{
        sse::{Event, Sse},
        Html, IntoResponse, Json,
    },
    routing::get,
    Router,
};
#[cfg(feature = "web")]
use std::sync::{Arc, Mutex};
#[cfg(feature = "web")]
use std::time::Duration;
#[cfg(feature = "web")]
use tokio::sync::broadcast;
#[cfg(feature = "web")]
use tokio_stream::wrappers::BroadcastStream;
#[cfg(feature = "web")]
use tokio_stream::StreamExt;

#[cfg(feature = "web")]
struct AppState {
    current: Mutex<Snapshot>,
    history: Mutex<Vec<Snapshot>>,
    tx: broadcast::Sender<String>,
    locale: Locale,
}

#[cfg(feature = "web")]
pub fn run_web_server(port: u16, locale: Locale) -> anyhow::Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let (tx, _) = broadcast::channel::<String>(64);
        let initial = Snapshot::capture()?;

        let state = Arc::new(AppState {
            current: Mutex::new(initial),
            history: Mutex::new(Vec::new()),
            tx: tx.clone(),
            locale,
        });

        // Background task: capture snapshots periodically
        let bg_state = state.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                if let Ok(snapshot) = Snapshot::capture() {
                    let json = serde_json::to_string(&snapshot).unwrap_or_default();
                    {
                        let mut history = bg_state.history.lock().unwrap();
                        let old = bg_state.current.lock().unwrap().clone();
                        history.push(old);
                        if history.len() > 60 {
                            history.remove(0);
                        }
                    }
                    *bg_state.current.lock().unwrap() = snapshot;
                    let _ = bg_state.tx.send(json);
                }
            }
        });

        let app = Router::new()
            .route("/", get(index_handler))
            .route("/api/snapshot", get(snapshot_handler))
            .route("/api/history", get(history_handler))
            .route("/api/sources", get(sources_handler))
            .route("/api/stream", get(sse_handler))
            .route("/api/view", get(view_handler))
            .with_state(state);

        let addr = format!("0.0.0.0:{}", port);
        eprintln!("syslenz web UI: http://localhost:{}", port);
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    })
}

#[cfg(feature = "web")]
async fn index_handler(State(state): State<Arc<AppState>>) -> Html<String> {
    let lang = match state.locale {
        Locale::Ja => "ja",
        Locale::En => "en",
    };
    Html(build_html(lang))
}

#[cfg(feature = "web")]
async fn snapshot_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let snapshot = state.current.lock().unwrap().clone();
    Json(snapshot)
}

#[cfg(feature = "web")]
async fn history_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let history = state.history.lock().unwrap().clone();
    Json(history)
}

#[cfg(feature = "web")]
async fn sources_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let snapshot = state.current.lock().unwrap().clone();
    let sources: Vec<String> = snapshot.entries.keys().cloned().collect();
    Json(sources)
}

#[cfg(feature = "web")]
async fn sse_handler(
    State(state): State<Arc<AppState>>,
) -> Sse<impl tokio_stream::Stream<Item = Result<Event, std::convert::Infallible>>> {
    let rx = state.tx.subscribe();
    let stream = BroadcastStream::new(rx)
        .filter_map(|result: Result<String, _>| result.ok())
        .map(|json| Ok(Event::default().data(json)));
    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(10))
    )
}

#[cfg(feature = "web")]
#[derive(serde::Deserialize)]
struct ViewQuery {
    view: Option<String>,
    locale: Option<String>,
}

#[cfg(feature = "web")]
async fn view_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ViewQuery>,
) -> impl IntoResponse {
    use crate::ui::app::{App, View, Focus};

    let locale = params
        .locale
        .as_deref()
        .map(Locale::from_str)
        .unwrap_or(state.locale);

    let snapshot = state.current.lock().unwrap().clone();
    let history = state.history.lock().unwrap().clone();

    // Build a minimal App to use the ViewData builders
    let source_keys: Vec<String> = snapshot.entries.keys().cloned().collect();
    let view = match params.view.as_deref() {
        Some("welcome") => View::Welcome,
        Some("detail") => View::Detail,
        Some("diff") => View::Diff,
        Some("table") => View::TableView,
        Some("graph") => View::Graph,
        Some("diagnostics") => View::Diagnostics,
        Some("category") => View::CategoryGuide,
        _ => View::Dashboard,
    };

    let app = App {
        snapshots: history,
        current: snapshot,
        diffs: Vec::new(),
        view,
        focus: Focus::Content,
        selected_source: 0,
        source_keys,
        selected_field: 0,
        sidebar_scroll: 0,
        field_scroll: 0,
        table_scroll: 0,
        running: false,
        last_refresh: std::time::Instant::now(),
        auto_refresh: false,
        refresh_interval_ms: 1000,
        search_query: String::new(),
        searching: false,
        filtered_keys: None,
        graph_field: None,
        status_message: None,
        remote_host: None,
        remote_rx: None,
        locale,
        help_level: crate::ui::app::HelpLevel::Off,
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
    };

    let view_data = app.build_view_data();
    Json(view_data)
}

#[cfg(feature = "web")]
fn build_html(lang: &str) -> String {
    let initial_locale = if lang == "ja" { "ja" } else { "en" };
    format!(r##"<!DOCTYPE html>
<html lang="{lang}">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>syslenz</title>
<script src="https://cdn.jsdelivr.net/npm/chart.js@4"></script>
<style>
*{{margin:0;padding:0;box-sizing:border-box}}
:root{{
  --bg:#1a1b26;--bg-dark:#16161e;--bg-hl:#1f2335;--bg-sel:#292e42;
  --fg:#c0caf5;--fg-dim:#565f89;--border:#3b4261;
  --blue:#7aa2f7;--green:#9ece6a;--yellow:#e0af68;--red:#f7768e;
  --cyan:#7dcfff;--purple:#bb9af7;--orange:#ff9e64;
}}
html,body{{height:100%;overflow:hidden}}
body{{font-family:'Consolas','Monaco','Fira Code',monospace;background:var(--bg);color:var(--fg);display:flex;height:100vh}}
#sidebar{{width:220px;background:var(--bg-dark);border-right:1px solid var(--border);display:flex;flex-direction:column;flex-shrink:0;transition:transform .2s}}
#sidebar.hidden{{transform:translateX(-220px);width:0;overflow:hidden;border:none}}
#sidebar-header{{padding:10px 12px;font-size:13px;color:var(--blue);border-bottom:1px solid var(--border);display:flex;justify-content:space-between;align-items:center;font-weight:bold}}
#search-box{{width:100%;padding:7px 12px;background:var(--bg);border:none;border-bottom:1px solid var(--border);color:var(--fg);font-family:inherit;font-size:12px;outline:none}}
#search-box::placeholder{{color:var(--fg-dim)}}
#search-box:focus{{border-bottom-color:var(--blue)}}
#source-list{{flex:1;overflow-y:auto;scrollbar-width:thin;scrollbar-color:var(--border) transparent}}
.src-item{{padding:6px 12px;cursor:pointer;font-size:12px;border-left:3px solid transparent;white-space:nowrap;overflow:hidden;text-overflow:ellipsis}}
.src-item:hover{{background:var(--bg-hl)}}
.src-item.active{{background:var(--bg-hl);border-left-color:var(--blue);color:var(--blue);font-weight:bold}}
.src-item.net{{color:var(--cyan)}}
.src-item.net.active{{color:var(--blue)}}
#main{{flex:1;display:flex;flex-direction:column;overflow:hidden}}
#topbar{{display:flex;justify-content:space-between;align-items:center;padding:6px 16px;border-bottom:1px solid var(--border);font-size:12px;color:var(--fg-dim);flex-shrink:0;gap:8px}}
#topbar .left{{display:flex;gap:12px;align-items:center}}
#topbar .right{{display:flex;gap:8px;align-items:center}}
.badge{{background:var(--bg-sel);color:var(--blue);padding:2px 8px;border-radius:4px;cursor:pointer;font-size:11px;user-select:none}}
.badge:hover{{background:var(--border)}}
.badge.active{{background:var(--blue);color:var(--bg-dark)}}
.conn-dot{{width:8px;height:8px;border-radius:50%;display:inline-block}}
.conn-dot.ok{{background:var(--green)}}
.conn-dot.err{{background:var(--red)}}
.conn-dot.connecting{{background:var(--yellow);animation:pulse 1s infinite}}
@keyframes pulse{{0%,100%{{opacity:1}}50%{{opacity:.3}}}}
#content{{flex:1;overflow-y:auto;padding:16px;scrollbar-width:thin;scrollbar-color:var(--border) transparent}}

/* Detail table */
.field-table{{width:100%;border-collapse:collapse;font-size:13px}}
.field-table th{{text-align:left;padding:8px;color:var(--purple);border-bottom:2px solid var(--border);font-size:11px;text-transform:uppercase;letter-spacing:.5px;position:sticky;top:0;background:var(--bg);z-index:1}}
.field-table td{{padding:5px 8px;border-bottom:1px solid var(--bg-sel)}}
.field-table tr{{cursor:pointer}}
.field-table tr:hover{{background:var(--bg-hl)}}
.field-table tr.selected{{background:var(--bg-sel)}}
.val-bytes{{color:var(--green)}}.val-int{{color:var(--blue)}}.val-float{{color:var(--purple)}}
.val-dur{{color:var(--yellow)}}.val-text{{color:var(--fg)}}.val-table{{color:var(--cyan)}}
.diff-up{{color:var(--green);font-weight:bold}}.diff-down{{color:var(--red);font-weight:bold}}

/* Dashboard */
.dash-grid{{display:grid;grid-template-columns:1fr 1fr;gap:12px}}
.dash-card{{background:var(--bg-dark);border:1px solid var(--border);border-radius:8px;padding:14px;overflow:hidden}}
.dash-card.full{{grid-column:1/-1}}
.dash-card h3{{color:var(--blue);font-size:13px;margin-bottom:10px;display:flex;align-items:center;gap:6px}}
.dash-card h3 .icon{{font-size:16px}}
.dash-metric{{display:flex;justify-content:space-between;padding:3px 0;font-size:13px}}
.dash-metric .label{{color:var(--fg-dim)}}.dash-metric .val{{font-weight:bold}}
.dash-chart-box{{height:150px;position:relative}}

/* Welcome */
.welcome-box{{max-width:700px;margin:40px auto}}
.welcome-box h1{{color:var(--blue);font-size:22px;margin-bottom:4px}}
.welcome-box .subtitle{{color:var(--fg-dim);font-size:14px;margin-bottom:24px}}
.key-row{{display:flex;padding:5px 0;font-size:13px}}
.key-row .key{{color:var(--yellow);width:110px;flex-shrink:0;font-weight:bold}}
.key-row .desc{{color:var(--fg)}}

/* Diagnostics */
.diag-table{{width:100%;border-collapse:collapse;font-size:13px}}
.diag-table th{{text-align:left;padding:8px;color:var(--yellow);border-bottom:2px solid var(--border);font-size:11px;text-transform:uppercase}}
.diag-table td{{padding:6px 8px;border-bottom:1px solid var(--bg-sel);vertical-align:top}}
.sev-crit{{color:var(--red);font-weight:bold}}.sev-warn{{color:var(--yellow)}}.sev-info{{color:var(--cyan)}}

/* Diff view */
.diff-table{{width:100%;border-collapse:collapse;font-size:13px}}
.diff-table th{{text-align:left;padding:8px;color:var(--purple);border-bottom:2px solid var(--border);font-size:11px;text-transform:uppercase}}
.diff-table td{{padding:5px 8px;border-bottom:1px solid var(--bg-sel)}}

/* Help overlay */
#help-overlay{{display:none;position:fixed;bottom:40px;left:0;right:0;background:var(--bg-dark);border-top:2px solid var(--blue);padding:12px 20px;font-size:12px;z-index:100;max-height:40vh;overflow-y:auto}}
#help-overlay.show{{display:block}}
.help-grid{{display:grid;grid-template-columns:repeat(auto-fill,minmax(250px,1fr));gap:4px 20px}}

/* Graph panel */
#graph-overlay{{display:none;position:fixed;bottom:0;left:220px;right:0;height:250px;background:var(--bg-dark);border-top:2px solid var(--blue);padding:12px 20px;z-index:90}}
#graph-overlay.show{{display:block}}
#graph-overlay h3{{color:var(--blue);font-size:13px;margin-bottom:8px}}
.graph-canvas-wrap{{height:190px}}

/* Status bar */
#statusbar{{height:28px;background:var(--bg-dark);border-top:1px solid var(--border);display:flex;align-items:center;padding:0 16px;font-size:11px;color:var(--fg-dim);flex-shrink:0;gap:16px;justify-content:space-between}}
#statusbar .left{{display:flex;gap:16px}}
#statusbar .right{{display:flex;gap:12px}}

/* Category Guide */
.cat-layout{{display:flex;gap:16px;height:calc(100vh - 120px)}}
.cat-sidebar{{width:200px;flex-shrink:0;overflow-y:auto;border-right:1px solid var(--border);padding-right:12px}}
.cat-item{{padding:8px 10px;cursor:pointer;font-size:13px;border-radius:6px;margin-bottom:2px;display:flex;align-items:center;gap:8px}}
.cat-item:hover{{background:var(--bg-hl)}}
.cat-item.active{{background:var(--bg-sel);color:var(--blue);font-weight:bold}}
.cat-content{{flex:1;overflow-y:auto;padding:0 8px;font-size:13px;line-height:1.7}}
.cat-content h4{{color:var(--blue);margin-top:16px;margin-bottom:6px;font-size:14px}}
.cat-content p,.cat-content pre{{margin-bottom:10px}}
.cat-content pre{{background:var(--bg-dark);padding:10px;border-radius:6px;overflow-x:auto;font-size:12px;color:var(--cyan)}}

/* Auto-graph inline */
#auto-graph-panel{{margin-top:12px;border:1px solid var(--border);border-radius:8px;padding:12px;background:var(--bg-dark);display:none}}
#auto-graph-panel.show{{display:block}}
#auto-graph-panel h4{{color:var(--blue);font-size:12px;margin-bottom:6px}}
.auto-graph-stats{{display:flex;gap:16px;font-size:11px;color:var(--fg-dim);margin-top:6px}}
.auto-graph-stats span{{color:var(--fg)}}
.auto-graph-canvas-wrap{{height:160px}}

/* Enter to expand hint */
.table-hint{{color:var(--cyan);font-size:11px;margin-left:6px;opacity:0.7}}

/* Toast */
#toast{{position:fixed;top:20px;right:20px;background:var(--blue);color:var(--bg-dark);padding:8px 16px;border-radius:6px;font-size:13px;display:none;z-index:200;font-weight:bold}}
#toast.show{{display:block;animation:fadeout 2s forwards}}
@keyframes fadeout{{0%,70%{{opacity:1}}100%{{opacity:0;display:none}}}}
</style>
</head>
<body>
<div id="sidebar">
  <div id="sidebar-header"><span>Sources (<span id="source-count">0</span>)</span></div>
  <input type="text" id="search-box" placeholder="Search sources... (/)">
  <div id="source-list"></div>
</div>
<div id="main">
  <div id="topbar">
    <div class="left">
      <span class="conn-dot connecting" id="conn-dot" title="SSE"></span>
      <span id="view-label">Dashboard</span>
      <span style="color:var(--fg-dim)" id="time-display">--:--:--</span>
    </div>
    <div class="right">
      <span class="badge" id="btn-dash" onclick="S.setView('dashboard')">D</span>
      <span class="badge" id="btn-classic" onclick="S.setView('detail')">O</span>
      <span class="badge" id="btn-diag" onclick="S.setView('diagnostics')">X</span>
      <span class="badge" id="btn-help" onclick="S.cycleHelp()">?</span>
      <span class="badge" id="btn-lang" onclick="S.toggleLang()">EN</span>
    </div>
  </div>
  <div id="content"></div>
  <div id="help-overlay"></div>
  <div id="graph-overlay"><h3>Graph: <span id="graph-title-field"></span></h3><div class="graph-canvas-wrap"><canvas id="graph-canvas"></canvas></div></div>
  <div id="statusbar">
    <div class="left">
      <span id="sb-view">Dashboard</span>
      <span id="sb-snaps">0 snaps</span>
      <span id="sb-source">-</span>
    </div>
    <div class="right">
      <span id="sb-keys">? Help | D Dashboard | O Classic | X Diag | L Lang</span>
    </div>
  </div>
</div>
<div id="toast"></div>

<script>
// ---- i18n ----
const I = {{
  en: {{
    title: 'syslenz - System Information Viewer',
    sources: 'Sources', field: 'Field', value: 'Value', unit: 'Unit', desc: 'Description',
    search: 'Search sources... (/)', noData: 'No data', snaps: 'snaps',
    dashboard: 'Dashboard', classic: 'Classic', detail: 'Detail', diff: 'Diff',
    diagnostics: 'Diagnostics', welcome: 'Welcome', help: 'Help', graph: 'Graph',
    loadUptime: 'Load / Uptime', memory: 'Memory', cpu: 'CPU', network: 'Network',
    load: 'Load', up: 'Up',
    memTotal: 'Total', memFree: 'Free', memAvail: 'Available', buffers: 'Buffers',
    cached: 'Cached', swapTotal: 'Swap Total', swapFree: 'Swap Free',
    cpuUser: 'User', cpuSystem: 'System', cpuIdle: 'Idle', cpuIowait: 'IO Wait',
    ctxSw: 'Ctx Switch', procsRun: 'Running',
    sev: 'Sev', source: 'Source', issue: 'Issue', suggestion: 'Suggestion',
    noIssues: 'No issues detected',
    oldVal: 'Old Value', newVal: 'New Value', noDiff: 'No changes detected (need 2+ snapshots)',
    welcomeTitle: 'syslenz', welcomeSub: 'Wireshark for /proc',
    copied: 'Copied to clipboard',
    helpKeys: [
      ['D', 'Dashboard (system overview)'],
      ['O', 'Classic mode (all sources list)'],
      ['j/k', 'Navigate sources / fields'],
      ['Enter', 'Drill in (detail view)'],
      ['Backspace', 'Go back'],
      ['d', 'Diff view'],
      ['/', 'Search sources'],
      ['?', 'Help panel (cycle levels)'],
      ['L', 'Toggle language (EN/JA)'],
      ['X', 'Diagnostics view'],
      ['c', 'Copy to clipboard'],
    ],
    helpKeysDetailed: [
      ['g', 'Toggle graph for selected field'],
      ['Tab', 'Switch focus sidebar/content'],
      ['Home/End', 'Jump to first/last'],
      ['a', 'Toggle auto-refresh'],
    ],
  }},
  ja: {{
    title: 'syslenz - システム情報ビューア',
    sources: 'ソース', field: 'フィールド', value: '値', unit: '単位', desc: '説明',
    search: 'ソース検索... (/)', noData: 'データなし', snaps: '件',
    dashboard: 'ダッシュボード', classic: 'クラシック', detail: '詳細', diff: '差分',
    diagnostics: '診断', welcome: 'ようこそ', help: 'ヘルプ', graph: 'グラフ',
    loadUptime: '負荷 / 稼働時間', memory: 'メモリ', cpu: 'CPU', network: 'ネットワーク',
    load: '負荷', up: '稼働',
    memTotal: '合計', memFree: '空き', memAvail: '利用可能', buffers: 'バッファ',
    cached: 'キャッシュ', swapTotal: 'Swap合計', swapFree: 'Swap空き',
    cpuUser: 'ユーザ', cpuSystem: 'システム', cpuIdle: 'アイドル', cpuIowait: 'IO待ち',
    ctxSw: 'コンテキストSW', procsRun: '実行中',
    sev: '重要度', source: 'ソース', issue: '問題', suggestion: '対処法',
    noIssues: '問題は検出されませんでした',
    oldVal: '旧値', newVal: '新値', noDiff: '変更なし（2件以上のスナップショットが必要）',
    welcomeTitle: 'syslenz', welcomeSub: '/proc の全てを構造化データで',
    copied: 'クリップボードにコピーしました',
    helpKeys: [
      ['D', 'ダッシュボード（システム概要）'],
      ['O', 'クラシックモード（全ソース一覧）'],
      ['j/k', 'ソース / フィールド移動'],
      ['Enter', 'ドリルイン（詳細表示）'],
      ['BS', '戻る'],
      ['d', '差分ビュー'],
      ['/', 'ソース検索'],
      ['?', 'ヘルプパネル（レベル切替）'],
      ['L', '言語切り替え (EN/JA)'],
      ['X', '診断ビュー'],
      ['c', 'クリップボードにコピー'],
    ],
    helpKeysDetailed: [
      ['g', '選択フィールドのグラフ表示'],
      ['Tab', 'サイドバー/コンテンツ切替'],
      ['Home/End', '先頭/末尾へ'],
      ['a', '自動更新の切替'],
    ],
  }}
}};

// ---- State ----
const S = {{
  snapshot: null,
  prevSnapshot: null,
  history: [],
  locale: '{initial_locale}',
  view: 'dashboard',   // dashboard|welcome|detail|diff|diagnostics|category
  selectedSource: 0,
  selectedField: 0,
  sourceKeys: [],
  filteredKeys: [],
  searchQuery: '',
  searching: false,
  focus: 'content',     // sidebar|content
  helpLevel: 0,         // 0=off,1=normal,2=detailed,3=extra
  autoRefresh: true,
  graphField: null,
  graphData: [],
  chart: null,
  evtSource: null,
  connected: false,
  dashCharts: {{}},
  viewDataCache: null,
  autoGraphChart: null,
  selectedCategory: 0,
  categoryScroll: 0,

  t(key) {{ return I[this.locale][key] || key; }},

  setView(v) {{
    if (this.graphField) this.closeGraph();
    if (this.autoGraphChart) {{ this.autoGraphChart.destroy(); this.autoGraphChart = null; }}
    if (v === 'classic') v = 'detail';
    this.view = v;
    if (v === 'detail' || v === 'diff') this.focus = 'sidebar';
    else this.focus = 'content';
    this.selectedField = 0;
    render();
  }},

  toggleLang() {{
    this.locale = this.locale === 'en' ? 'ja' : 'en';
    document.documentElement.lang = this.locale;
    document.getElementById('btn-lang').textContent = this.locale.toUpperCase();
    render();
  }},

  cycleHelp() {{
    this.helpLevel = (this.helpLevel + 1) % 4;
    renderHelp();
  }},

  closeGraph() {{
    this.graphField = null;
    this.graphData = [];
    if (this.chart) {{ this.chart.destroy(); this.chart = null; }}
    document.getElementById('graph-overlay').classList.remove('show');
  }},

  currentSourceName() {{
    return this.filteredKeys[this.selectedSource] || this.sourceKeys[0] || '';
  }},
}};

// ---- Helpers ----
function formatValue(v) {{
  if (v.Bytes !== undefined) return [formatBytes(v.Bytes), 'val-bytes'];
  if (v.Integer !== undefined) return [v.Integer.toLocaleString(), 'val-int'];
  if (v.Float !== undefined) return [v.Float.toFixed(2), 'val-float'];
  if (v.Duration !== undefined) return [formatDuration(v.Duration), 'val-dur'];
  if (v.Text !== undefined) return [escapeHtml(v.Text), 'val-text'];
  if (v.Table !== undefined) return ['[' + v.Table.length + ' rows]', 'val-table'];
  return ['?', 'val-text'];
}}

function formatBytes(b) {{
  if (b >= 1073741824) return (b / 1073741824).toFixed(2) + ' GiB';
  if (b >= 1048576) return (b / 1048576).toFixed(1) + ' MiB';
  if (b >= 1024) return (b / 1024).toFixed(1) + ' KiB';
  return b + ' B';
}}

function formatDuration(s) {{
  const d = Math.floor(s / 86400), h = Math.floor((s % 86400) / 3600),
        m = Math.floor((s % 3600) / 60), sec = Math.floor(s % 60);
  if (d > 0) return d + 'd ' + h + 'h ' + m + 'm';
  if (h > 0) return h + 'h ' + m + 'm ' + sec + 's';
  if (m > 0) return m + 'm ' + sec + 's';
  return s.toFixed(1) + 's';
}}

function extractNumeric(v) {{
  if (v.Bytes !== undefined) return v.Bytes;
  if (v.Integer !== undefined) return v.Integer;
  if (v.Float !== undefined) return v.Float;
  if (v.Duration !== undefined) return v.Duration;
  return null;
}}

function escapeHtml(s) {{
  if (typeof s !== 'string') return String(s);
  return s.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;');
}}

function getField(snap, source, name) {{
  const e = snap && snap.entries && snap.entries[source];
  if (!e) return null;
  return e.fields.find(f => f.name === name) || null;
}}

function getFieldVal(snap, source, name) {{
  const f = getField(snap, source, name);
  if (!f) return '-';
  const [d] = formatValue(f.value);
  return d;
}}

function getFieldNumeric(snap, source, name) {{
  const f = getField(snap, source, name);
  return f ? extractNumeric(f.value) : null;
}}

function diffDirection(prev, cur, source, fieldName) {{
  if (!prev) return '';
  const oldF = getField(prev, source, fieldName);
  const newF = getField(cur, source, fieldName);
  if (!oldF || !newF) return '';
  const ov = extractNumeric(oldF.value), nv = extractNumeric(newF.value);
  if (ov === null || nv === null) return '';
  if (nv > ov) return 'diff-up';
  if (nv < ov) return 'diff-down';
  return '';
}}

function toast(msg) {{
  const el = document.getElementById('toast');
  el.textContent = msg;
  el.classList.remove('show');
  void el.offsetWidth;
  el.classList.add('show');
  setTimeout(() => el.classList.remove('show'), 2200);
}}

// ---- ViewData API ----
async function fetchViewData(view, locale) {{
  try {{
    const url = '/api/view?view=' + encodeURIComponent(view) + '&locale=' + encodeURIComponent(locale || S.locale);
    const resp = await fetch(url);
    if (!resp.ok) return null;
    const data = await resp.json();
    S.viewDataCache = data;
    return data;
  }} catch (e) {{
    return null;
  }}
}}

// Auto-graph: show graph inline for numeric fields in detail view
function updateAutoGraph() {{
  const panel = document.getElementById('auto-graph-panel');
  if (!panel || S.view !== 'detail') {{ if (panel) panel.classList.remove('show'); return; }}

  const src = S.currentSourceName();
  const entry = S.snapshot && S.snapshot.entries[src];
  if (!entry || !entry.fields[S.selectedField]) {{ panel.classList.remove('show'); return; }}

  const f = entry.fields[S.selectedField];
  const numVal = extractNumeric(f.value);
  if (numVal === null) {{ panel.classList.remove('show'); return; }}

  // Collect history for this field
  const allSnaps = [...S.history, S.snapshot].filter(Boolean);
  const points = [];
  allSnaps.forEach(snap => {{
    const v = getFieldNumeric(snap, src, f.name);
    if (v !== null) points.push({{ t: new Date(snap.timestamp).toLocaleTimeString(), v }});
  }});
  if (points.length < 2) {{ panel.classList.remove('show'); return; }}

  const vals = points.map(p => p.v);
  const min = Math.min(...vals);
  const max = Math.max(...vals);
  const avg = vals.reduce((a, b) => a + b, 0) / vals.length;
  const current = vals[vals.length - 1];

  panel.classList.add('show');
  const title = panel.querySelector('h4');
  if (title) title.textContent = src + '.' + f.name;
  const statsEl = panel.querySelector('.auto-graph-stats');
  if (statsEl) statsEl.innerHTML = 'Min: <span>' + min.toFixed(2) + '</span> | Max: <span>' + max.toFixed(2) + '</span> | Avg: <span>' + avg.toFixed(2) + '</span> | Current: <span>' + current.toFixed(2) + '</span>';

  const canvas = document.getElementById('auto-graph-canvas');
  if (!canvas) return;
  if (S.autoGraphChart) S.autoGraphChart.destroy();
  S.autoGraphChart = new Chart(canvas.getContext('2d'), {{
    type: 'line',
    data: {{
      labels: points.map(d => d.t),
      datasets: [{{ label: f.name, data: vals, borderColor: '#7aa2f7', backgroundColor: 'rgba(122,162,247,0.1)', fill: true, tension: 0.3, pointRadius: 1 }}]
    }},
    options: {{
      responsive: true, maintainAspectRatio: false,
      scales: {{ x: {{ ticks: {{ color: '#565f89', maxTicksLimit: 10, font: {{ size: 10 }} }} }}, y: {{ ticks: {{ color: '#565f89', font: {{ size: 10 }} }} }} }},
      plugins: {{ legend: {{ labels: {{ color: '#c0caf5', font: {{ size: 10 }} }} }} }},
      animation: {{ duration: 200 }}
    }}
  }});
}}

// ---- SSE ----
function startSSE() {{
  const dot = document.getElementById('conn-dot');
  dot.className = 'conn-dot connecting';
  S.evtSource = new EventSource('/api/stream');
  S.evtSource.onopen = () => {{
    S.connected = true;
    dot.className = 'conn-dot ok';
  }};
  S.evtSource.onerror = () => {{
    S.connected = false;
    dot.className = 'conn-dot err';
    S.evtSource.close();
    setTimeout(startSSE, 3000);
  }};
  S.evtSource.onmessage = (e) => {{
    if (!S.autoRefresh) return;
    try {{
      const snap = JSON.parse(e.data);
      S.prevSnapshot = S.snapshot;
      if (S.snapshot) {{
        S.history.push(S.snapshot);
        if (S.history.length > 60) S.history.shift();
      }}
      S.snapshot = snap;
      render();
    }} catch(ex) {{}}
  }};
}}

// ---- Init ----
async function init() {{
  const [snapRes, histRes] = await Promise.all([
    fetch('/api/snapshot').then(r => r.json()),
    fetch('/api/history').then(r => r.json()),
  ]);
  S.snapshot = snapRes;
  S.history = histRes;
  updateSourceKeys();
  render();
  startSSE();
}}

function updateSourceKeys() {{
  if (!S.snapshot) return;
  S.sourceKeys = Object.keys(S.snapshot.entries).sort();
  filterSources();
}}

function filterSources() {{
  const q = S.searchQuery.toLowerCase();
  S.filteredKeys = q ? S.sourceKeys.filter(k => k.toLowerCase().includes(q)) : [...S.sourceKeys];
  if (S.selectedSource >= S.filteredKeys.length) S.selectedSource = Math.max(0, S.filteredKeys.length - 1);
}}

// ---- Render ----
function render() {{
  updateSourceKeys();
  renderSidebar();
  renderContent();
  renderTopbar();
  renderStatusbar();
  renderHelp();
  updateGraphData();
}}

function renderSidebar() {{
  document.getElementById('source-count').textContent = S.sourceKeys.length;
  const list = document.getElementById('source-list');
  const show = S.view === 'detail' || S.view === 'diff';
  document.getElementById('sidebar').classList.toggle('hidden', !show);
  if (!show) return;
  let html = '';
  S.filteredKeys.forEach((k, i) => {{
    const cls = (i === S.selectedSource ? 'active ' : '') + (k.startsWith('net/') ? 'net ' : '');
    html += '<div class="src-item ' + cls + '" data-idx="' + i + '">' + escapeHtml(k) + '</div>';
  }});
  list.innerHTML = html;
  // scroll active into view
  const active = list.querySelector('.active');
  if (active) active.scrollIntoView({{ block: 'nearest' }});
  // click handlers
  list.querySelectorAll('.src-item').forEach(el => {{
    el.onclick = () => {{
      S.selectedSource = parseInt(el.dataset.idx);
      S.selectedField = 0;
      if (S.graphField) S.closeGraph();
      render();
    }};
  }});
}}

function renderTopbar() {{
  const t = S.t.bind(S);
  const viewNames = {{ dashboard: t('dashboard'), detail: t('classic'), diff: t('diff'), diagnostics: t('diagnostics'), welcome: t('welcome'), category: S.locale === 'ja' ? 'カテゴリガイド' : 'Category Guide' }};
  document.getElementById('view-label').textContent = viewNames[S.view] || S.view;
  document.getElementById('time-display').textContent = S.snapshot ? new Date(S.snapshot.timestamp).toLocaleTimeString() : '--:--:--';
  // highlight active view button
  ['btn-dash','btn-classic','btn-diag'].forEach(id => document.getElementById(id).classList.remove('active'));
  if (S.view === 'dashboard') document.getElementById('btn-dash').classList.add('active');
  else if (S.view === 'detail' || S.view === 'diff') document.getElementById('btn-classic').classList.add('active');
  else if (S.view === 'diagnostics') document.getElementById('btn-diag').classList.add('active');
}}

function renderStatusbar() {{
  const t = S.t.bind(S);
  document.getElementById('sb-view').textContent = S.view;
  document.getElementById('sb-snaps').textContent = S.history.length + ' ' + t('snaps');
  document.getElementById('sb-source').textContent = (S.view === 'detail' || S.view === 'diff') ? S.currentSourceName() : '';
}}

function renderContent() {{
  const el = document.getElementById('content');
  switch (S.view) {{
    case 'dashboard': el.innerHTML = renderDashboard(); initDashCharts(); break;
    case 'detail': el.innerHTML = renderDetail(); setTimeout(updateAutoGraph, 0); break;
    case 'diff': el.innerHTML = renderDiff(); break;
    case 'diagnostics': el.innerHTML = renderDiagnostics(); break;
    case 'welcome': el.innerHTML = renderWelcome(); break;
    case 'category': renderCategoryGuide(el); break;
    default: el.innerHTML = renderDashboard(); initDashCharts(); break;
  }}
}}

// ---- Dashboard ----
function renderDashboard() {{
  if (!S.snapshot) return '<p style="color:var(--fg-dim)">Loading...</p>';
  const t = S.t.bind(S);
  const snap = S.snapshot;

  // Load
  const l1 = getFieldVal(snap, 'loadavg', 'load1');
  const l5 = getFieldVal(snap, 'loadavg', 'load5');
  const l15 = getFieldVal(snap, 'loadavg', 'load15');
  const up = getFieldVal(snap, 'uptime', 'uptime');

  // Memory
  const memFields = [
    ['MemTotal', t('memTotal')], ['MemFree', t('memFree')], ['MemAvailable', t('memAvail')],
    ['Buffers', t('buffers')], ['Cached', t('cached')], ['SwapTotal', t('swapTotal')], ['SwapFree', t('swapFree')]
  ];
  let memHtml = '';
  memFields.forEach(([name, label]) => {{
    const v = getFieldVal(snap, 'meminfo', name);
    const dc = diffDirection(S.prevSnapshot, snap, 'meminfo', name);
    memHtml += '<div class="dash-metric"><span class="label">' + label + '</span><span class="val val-bytes ' + dc + '">' + v + '</span></div>';
  }});

  // CPU
  const cpuFields = [
    ['cpu_user', t('cpuUser'), '--blue'], ['cpu_system', t('cpuSystem'), '--purple'],
    ['cpu_idle', t('cpuIdle'), '--green'], ['cpu_iowait', t('cpuIowait'), '--red'],
    ['context_switches', t('ctxSw'), '--cyan'], ['processes_running', t('procsRun'), '--yellow']
  ];
  let cpuHtml = '';
  cpuFields.forEach(([name, label]) => {{
    const v = getFieldVal(snap, 'stat', name);
    const dc = diffDirection(S.prevSnapshot, snap, 'stat', name);
    cpuHtml += '<div class="dash-metric"><span class="label">' + label + '</span><span class="val val-int ' + dc + '">' + v + '</span></div>';
  }});

  // Network table
  let netHtml = '';
  const netEntry = snap.entries['net/dev'];
  if (netEntry) {{
    const tableField = netEntry.fields.find(f => f.value.Table);
    if (tableField) {{
      const rows = tableField.value.Table;
      netHtml += '<table class="field-table" style="font-size:12px"><thead><tr>';
      const netHeaders = S.locale === 'ja'
        ? ['IF','RX バイト','RX パケット','TX バイト','TX パケット']
        : ['Interface','RX Bytes','RX Pkts','TX Bytes','TX Pkts'];
      netHeaders.forEach(h => {{ netHtml += '<th>' + h + '</th>'; }});
      netHtml += '</tr></thead><tbody>';
      rows.slice(0, 8).forEach(row => {{
        netHtml += '<tr>' + row.slice(0, 5).map(c => '<td>' + escapeHtml(c) + '</td>').join('') + '</tr>';
      }});
      netHtml += '</tbody></table>';
    }}
  }}

  return '<div class="dash-grid">' +
    '<div class="dash-card full"><h3><span class="icon">&#9889;</span> ' + t('loadUptime') + '</h3>' +
      '<div style="display:flex;gap:40px;flex-wrap:wrap">' +
        '<div><span style="color:var(--fg-dim)">' + t('load') + ':</span> <span style="color:var(--green);font-weight:bold">' + l1 + '</span> / <span style="color:var(--yellow)">' + l5 + '</span> / <span style="color:var(--fg-dim)">' + l15 + '</span></div>' +
        '<div><span style="color:var(--fg-dim)">' + t('up') + ':</span> <span style="color:var(--cyan)">' + up + '</span></div>' +
      '</div>' +
      '<div class="dash-chart-box" style="margin-top:8px"><canvas id="dash-load-chart"></canvas></div>' +
    '</div>' +
    '<div class="dash-card"><h3><span class="icon">&#128190;</span> ' + t('memory') + '</h3>' + memHtml +
      '<div class="dash-chart-box" style="margin-top:8px"><canvas id="dash-mem-chart"></canvas></div>' +
    '</div>' +
    '<div class="dash-card"><h3><span class="icon">&#9881;</span> ' + t('cpu') + '</h3>' + cpuHtml +
      '<div class="dash-chart-box" style="margin-top:8px"><canvas id="dash-cpu-chart"></canvas></div>' +
    '</div>' +
    '<div class="dash-card full"><h3><span class="icon">&#127760;</span> ' + t('network') + '</h3>' + (netHtml || '<span style="color:var(--fg-dim)">-</span>') + '</div>' +
  '</div>';
}}

function initDashCharts() {{
  if (S.view !== 'dashboard' || !S.snapshot) return;
  // Destroy old charts
  Object.values(S.dashCharts).forEach(c => {{ if (c) c.destroy(); }});
  S.dashCharts = {{}};

  const chartOpts = {{
    responsive: true, maintainAspectRatio: false,
    scales: {{ x: {{ display: true, ticks: {{ color: '#565f89', maxTicksLimit: 8, font: {{ size: 10 }} }} }}, y: {{ ticks: {{ color: '#565f89', font: {{ size: 10 }} }} }} }},
    plugins: {{ legend: {{ labels: {{ color: '#c0caf5', font: {{ size: 10 }} }} }} }},
    animation: {{ duration: 300 }}
  }};

  // Load average trend
  const loadEl = document.getElementById('dash-load-chart');
  if (loadEl) {{
    const allSnaps = [...S.history, S.snapshot].filter(Boolean);
    const labels = allSnaps.map(s => new Date(s.timestamp).toLocaleTimeString());
    const l1d = allSnaps.map(s => getFieldNumeric(s, 'loadavg', 'load1') || 0);
    const l5d = allSnaps.map(s => getFieldNumeric(s, 'loadavg', 'load5') || 0);
    const l15d = allSnaps.map(s => getFieldNumeric(s, 'loadavg', 'load15') || 0);
    S.dashCharts.load = new Chart(loadEl.getContext('2d'), {{
      type: 'line',
      data: {{ labels, datasets: [
        {{ label: 'load1', data: l1d, borderColor: '#9ece6a', backgroundColor: 'rgba(158,206,106,0.1)', fill: true, tension: 0.3, pointRadius: 1 }},
        {{ label: 'load5', data: l5d, borderColor: '#e0af68', fill: false, tension: 0.3, pointRadius: 0 }},
        {{ label: 'load15', data: l15d, borderColor: '#565f89', fill: false, tension: 0.3, pointRadius: 0 }},
      ]}},
      options: chartOpts
    }});
  }}

  // Memory donut
  const memEl = document.getElementById('dash-mem-chart');
  if (memEl) {{
    const total = getFieldNumeric(S.snapshot, 'meminfo', 'MemTotal') || 1;
    const free = getFieldNumeric(S.snapshot, 'meminfo', 'MemFree') || 0;
    const buffers = getFieldNumeric(S.snapshot, 'meminfo', 'Buffers') || 0;
    const cached = getFieldNumeric(S.snapshot, 'meminfo', 'Cached') || 0;
    const used = Math.max(0, total - free - buffers - cached);
    S.dashCharts.mem = new Chart(memEl.getContext('2d'), {{
      type: 'doughnut',
      data: {{
        labels: ['Used', 'Buffers', 'Cached', 'Free'],
        datasets: [{{ data: [used, buffers, cached, free], backgroundColor: ['#f7768e','#7aa2f7','#e0af68','#9ece6a'], borderWidth: 0 }}]
      }},
      options: {{
        responsive: true, maintainAspectRatio: false,
        plugins: {{ legend: {{ position: 'right', labels: {{ color: '#c0caf5', font: {{ size: 10 }}, padding: 8 }} }} }},
        cutout: '55%',
        animation: {{ duration: 300 }}
      }}
    }});
  }}

  // CPU bar chart
  const cpuEl = document.getElementById('dash-cpu-chart');
  if (cpuEl) {{
    const user = getFieldNumeric(S.snapshot, 'stat', 'cpu_user') || 0;
    const sys = getFieldNumeric(S.snapshot, 'stat', 'cpu_system') || 0;
    const idle = getFieldNumeric(S.snapshot, 'stat', 'cpu_idle') || 0;
    const iow = getFieldNumeric(S.snapshot, 'stat', 'cpu_iowait') || 0;
    S.dashCharts.cpu = new Chart(cpuEl.getContext('2d'), {{
      type: 'bar',
      data: {{
        labels: ['User', 'System', 'Idle', 'IO Wait'],
        datasets: [{{ data: [user, sys, idle, iow], backgroundColor: ['#7aa2f7','#bb9af7','#9ece6a','#f7768e'], borderWidth: 0 }}]
      }},
      options: {{
        responsive: true, maintainAspectRatio: false,
        indexAxis: 'y',
        scales: {{ x: {{ ticks: {{ color: '#565f89' }} }}, y: {{ ticks: {{ color: '#c0caf5', font: {{ size: 10 }} }} }} }},
        plugins: {{ legend: {{ display: false }} }},
        animation: {{ duration: 300 }}
      }}
    }});
  }}
}}

// ---- Detail view ----
function renderDetail() {{
  if (!S.snapshot) return '';
  const t = S.t.bind(S);
  const src = S.currentSourceName();
  const entry = S.snapshot.entries[src];
  if (!entry) return '<p style="color:var(--fg-dim)">' + t('noData') + '</p>';
  const fields = entry.fields;

  let html = '<h3 style="color:var(--blue);margin-bottom:8px;font-size:14px">' + escapeHtml(src) + ' <span style="color:var(--fg-dim);font-size:11px">(' + escapeHtml(entry.source) + ')</span></h3>';
  html += '<table class="field-table"><thead><tr><th>' + t('field') + '</th><th>' + t('value') + '</th><th>' + t('unit') + '</th><th>' + t('desc') + '</th></tr></thead><tbody>';

  fields.forEach((f, i) => {{
    const [display, cls] = formatValue(f.value);
    const dc = diffDirection(S.prevSnapshot, S.snapshot, src, f.name);
    const sel = i === S.selectedField ? ' selected' : '';
    const isTable = f.value.Table !== undefined;
    html += '<tr class="' + sel + '" data-fidx="' + i + '">' +
      '<td>' + escapeHtml(f.name) + '</td>' +
      '<td class="' + cls + ' ' + dc + '">' + display + (isTable ? ' <span class="table-hint">[Enter to expand]</span>' : '') + '</td>' +
      '<td style="color:var(--fg-dim)">' + escapeHtml(f.unit || '') + '</td>' +
      '<td style="color:var(--fg-dim);max-width:300px;overflow:hidden;text-overflow:ellipsis">' + escapeHtml(f.description) + '</td></tr>';
  }});
  html += '</tbody></table>';
  html += '<div id="auto-graph-panel"><h4></h4><div class="auto-graph-canvas-wrap"><canvas id="auto-graph-canvas"></canvas></div><div class="auto-graph-stats"></div></div>';

  // Attach click after render
  setTimeout(() => {{
    document.querySelectorAll('.field-table tr[data-fidx]').forEach(tr => {{
      tr.onclick = () => {{
        const idx = parseInt(tr.dataset.fidx);
        S.selectedField = idx;
        const f = fields[idx];
        if (f.value.Table) {{
          showTableExpand(src, idx);
        }} else {{
          toggleFieldGraph(src, f.name);
        }}
        render();
      }};
    }});
  }}, 0);

  return html;
}}

function showTableExpand(source, idx) {{
  const entry = S.snapshot.entries[source];
  if (!entry) return;
  const field = entry.fields[idx];
  if (!field || !field.value.Table) return;
  const rows = field.value.Table;
  const el = document.getElementById('content');
  let html = '<h3 style="color:var(--blue);margin-bottom:8px;font-size:14px">' + escapeHtml(source) + '.' + escapeHtml(field.name) + '</h3>';
  html += '<table class="field-table"><tbody>';
  rows.forEach(row => {{
    html += '<tr>' + row.map(c => '<td>' + escapeHtml(c) + '</td>').join('') + '</tr>';
  }});
  html += '</tbody></table>';
  html += '<p style="margin-top:12px"><a href="#" onclick="render();return false" style="color:var(--blue)">&#8592; Back (Backspace)</a></p>';
  el.innerHTML = html;
}}

// ---- Diff view ----
function renderDiff() {{
  const t = S.t.bind(S);
  if (S.history.length === 0) return '<p style="color:var(--fg-dim)">' + t('noDiff') + '</p>';
  const prev = S.history[S.history.length - 1];
  const cur = S.snapshot;
  const diffs = computeDiffs(prev, cur);
  if (diffs.length === 0) return '<p style="color:var(--fg-dim)">' + t('noDiff') + '</p>';

  let html = '<table class="diff-table"><thead><tr><th>' + t('source') + '</th><th>' + t('field') + '</th><th>' + t('oldVal') + '</th><th>' + t('newVal') + '</th></tr></thead><tbody>';
  diffs.forEach(d => {{
    html += '<tr><td style="color:var(--cyan)">' + escapeHtml(d.source) + '</td><td>' + escapeHtml(d.field) + '</td>' +
      '<td style="color:var(--red)">' + escapeHtml(d.oldValue) + '</td>' +
      '<td style="color:var(--green)">' + escapeHtml(d.newValue) + '</td></tr>';
  }});
  html += '</tbody></table>';
  return html;
}}

function computeDiffs(oldSnap, newSnap) {{
  const diffs = [];
  if (!oldSnap || !newSnap) return diffs;
  for (const [key, newEntry] of Object.entries(newSnap.entries)) {{
    const oldEntry = oldSnap.entries[key];
    if (!oldEntry) continue;
    const len = Math.min(newEntry.fields.length, oldEntry.fields.length);
    for (let i = 0; i < len; i++) {{
      const of = oldEntry.fields[i], nf = newEntry.fields[i];
      const ov = extractNumeric(of.value), nv = extractNumeric(nf.value);
      let changed = false;
      if (ov !== null && nv !== null) {{
        changed = Math.abs(ov - nv) > 0.001;
      }} else {{
        const [od] = formatValue(of.value);
        const [nd] = formatValue(nf.value);
        changed = od !== nd;
      }}
      if (changed) {{
        const [ovd] = formatValue(of.value);
        const [nvd] = formatValue(nf.value);
        diffs.push({{ source: key, field: nf.name, oldValue: ovd, newValue: nvd }});
      }}
    }}
  }}
  return diffs;
}}

// ---- Diagnostics (uses ViewData API for consistent data with TUI) ----
function renderDiagnostics() {{
  if (!S.snapshot) return '';
  const t = S.t.bind(S);
  // Kick off ViewData fetch for diagnostics (async, will update when ready)
  fetchViewData('diagnostics', S.locale).then(data => {{
    if (!data || data.type !== 'Diagnostics' || S.view !== 'diagnostics') return;
    const el = document.getElementById('content');
    if (!el) return;
    const findings = data.findings || [];
    if (findings.length === 0) {{
      el.innerHTML = '<p style="color:var(--green);font-size:14px">&#10003; ' + t('noIssues') + '</p>';
      return;
    }}
    let html = '<h3 style="color:var(--yellow);margin-bottom:10px;font-size:14px">' + escapeHtml(data.title) + '</h3>';
    html += '<table class="diag-table"><thead><tr><th>' + t('sev') + '</th><th>' + t('source') + '</th><th>' + t('issue') + '</th><th>' + t('desc') + '</th><th>' + t('suggestion') + '</th></tr></thead><tbody>';
    findings.forEach(f => {{
      const sevColor = f.severity_color;
      const sevCls = sevColor === 'Red' ? 'sev-crit' : sevColor === 'Yellow' ? 'sev-warn' : 'sev-info';
      html += '<tr><td class="' + sevCls + '">' + escapeHtml(f.severity) + '</td><td style="color:var(--fg-dim)">' + escapeHtml(f.source) + '</td>' +
        '<td class="' + sevCls + '">' + escapeHtml(f.title) + '</td><td>' + escapeHtml(f.detail) + '</td>' +
        '<td style="color:var(--green)">' + escapeHtml(f.suggestion) + '</td></tr>';
    }});
    html += '</tbody></table>';
    el.innerHTML = html;
  }});
  // Show local fallback immediately while ViewData loads
  const findings = runDiagnostics(S.snapshot);
  if (findings.length === 0) return '<p style="color:var(--green);font-size:14px">&#10003; ' + t('noIssues') + '</p>';

  let html = '<table class="diag-table"><thead><tr><th>' + t('sev') + '</th><th>' + t('source') + '</th><th>' + t('issue') + '</th><th>' + t('desc') + '</th><th>' + t('suggestion') + '</th></tr></thead><tbody>';
  findings.forEach(f => {{
    const sevCls = f.severity === 'CRIT' ? 'sev-crit' : f.severity === 'WARN' ? 'sev-warn' : 'sev-info';
    html += '<tr><td class="' + sevCls + '">' + f.severity + '</td><td style="color:var(--fg-dim)">' + escapeHtml(f.source) + '</td>' +
      '<td class="' + sevCls + '">' + escapeHtml(f.title) + '</td><td>' + escapeHtml(f.detail) + '</td>' +
      '<td style="color:var(--green)">' + escapeHtml(f.suggestion) + '</td></tr>';
  }});
  html += '</tbody></table>';
  return html;
}}

function runDiagnostics(snap) {{
  const findings = [];
  const isJa = S.locale === 'ja';
  // Memory check
  const memTotal = getFieldNumeric(snap, 'meminfo', 'MemTotal');
  const memAvail = getFieldNumeric(snap, 'meminfo', 'MemAvailable');
  if (memTotal && memAvail) {{
    const pct = (memAvail / memTotal) * 100;
    if (pct < 5) findings.push({{ severity: 'CRIT', source: 'meminfo', title: isJa ? 'メモリ危機' : 'Memory critical', detail: pct.toFixed(1) + '% available', suggestion: isJa ? 'プロセスを終了してください' : 'Kill processes or add memory' }});
    else if (pct < 15) findings.push({{ severity: 'WARN', source: 'meminfo', title: isJa ? 'メモリ不足' : 'Low memory', detail: pct.toFixed(1) + '% available', suggestion: isJa ? 'メモリ使用量を確認' : 'Check memory usage' }});
  }}
  // Load check
  const load1 = getFieldNumeric(snap, 'loadavg', 'load1');
  if (load1 !== null && load1 > 4) {{
    findings.push({{ severity: load1 > 8 ? 'CRIT' : 'WARN', source: 'loadavg', title: isJa ? '高負荷' : 'High load', detail: 'load1=' + load1.toFixed(2), suggestion: isJa ? 'CPUバウンドプロセスを確認' : 'Check CPU-bound processes' }});
  }}
  // Swap check
  const swapTotal = getFieldNumeric(snap, 'meminfo', 'SwapTotal');
  const swapFree = getFieldNumeric(snap, 'meminfo', 'SwapFree');
  if (swapTotal && swapTotal > 0 && swapFree !== null) {{
    const used = swapTotal - swapFree;
    const pct = (used / swapTotal) * 100;
    if (pct > 80) findings.push({{ severity: 'WARN', source: 'meminfo', title: isJa ? 'Swap使用率高' : 'High swap usage', detail: pct.toFixed(1) + '% used', suggestion: isJa ? 'メモリリーク確認' : 'Check for memory leaks' }});
  }}
  // Sort: CRIT first
  const order = {{ CRIT: 0, WARN: 1, INFO: 2 }};
  findings.sort((a, b) => (order[a.severity] || 9) - (order[b.severity] || 9));
  return findings;
}}

// ---- Welcome ----
function renderWelcome() {{
  const t = S.t.bind(S);
  const keys = I[S.locale].helpKeys;
  let html = '<div class="welcome-box"><h1>' + t('welcomeTitle') + '</h1><p class="subtitle">' + t('welcomeSub') + '</p>';
  html += '<div style="margin-top:16px">';
  keys.forEach(([k, d]) => {{
    html += '<div class="key-row"><span class="key">' + escapeHtml(k) + '</span><span class="desc">' + escapeHtml(d) + '</span></div>';
  }});
  html += '</div></div>';
  return html;
}}

// ---- Category Guide (fetches from ViewData API) ----
async function renderCategoryGuide(el) {{
  el.innerHTML = '<p style="color:var(--fg-dim)">Loading category guide...</p>';
  const data = await fetchViewData('category', S.locale);
  if (!data || data.type !== 'CategoryGuide') {{
    el.innerHTML = '<p style="color:var(--fg-dim)">Failed to load category guide.</p>';
    return;
  }}
  const cats = data.categories || [];
  const content = data.content || {{}};
  const selected = S.selectedCategory;

  let sidebar = '';
  cats.forEach((cat, i) => {{
    const cls = i === selected ? 'active' : '';
    sidebar += '<div class="cat-item ' + cls + '" data-cidx="' + i + '">' + cat.icon + ' ' + escapeHtml(cat.name) + '</div>';
  }});

  function formatContent(text) {{
    if (!text) return '';
    return escapeHtml(text).replace(/\n/g, '<br>');
  }}

  const overviewLabel = S.locale === 'ja' ? '概要' : 'Overview';
  const storyLabel = S.locale === 'ja' ? 'ストーリー' : 'Story';
  const diagLabel = S.locale === 'ja' ? '診断フロー' : 'Diagnostic Flow';
  const issuesLabel = S.locale === 'ja' ? 'よくある問題' : 'Common Issues';

  let contentHtml = '';
  if (content.overview) contentHtml += '<h4>' + overviewLabel + '</h4><p>' + formatContent(content.overview) + '</p>';
  if (content.story) contentHtml += '<h4>' + storyLabel + '</h4><p>' + formatContent(content.story) + '</p>';
  if (content.diagnostic_flow) contentHtml += '<h4>' + diagLabel + '</h4><pre>' + escapeHtml(content.diagnostic_flow) + '</pre>';
  if (content.common_issues) contentHtml += '<h4>' + issuesLabel + '</h4><p>' + formatContent(content.common_issues) + '</p>';

  el.innerHTML = '<div class="cat-layout"><div class="cat-sidebar">' + sidebar + '</div><div class="cat-content" id="cat-content-scroll">' + contentHtml + '</div></div>';

  // Restore scroll position
  const scrollEl = document.getElementById('cat-content-scroll');
  if (scrollEl) scrollEl.scrollTop = S.categoryScroll;

  // Click handlers for category items
  el.querySelectorAll('.cat-item').forEach(item => {{
    item.onclick = () => {{
      S.selectedCategory = parseInt(item.dataset.cidx);
      S.categoryScroll = 0;
      renderCategoryGuide(el);
    }};
  }});

  // Track scroll position
  if (scrollEl) {{
    scrollEl.onscroll = () => {{ S.categoryScroll = scrollEl.scrollTop; }};
  }}
}}

// ---- Help panel ----
function renderHelp() {{
  const el = document.getElementById('help-overlay');
  if (S.helpLevel === 0) {{ el.classList.remove('show'); return; }}
  el.classList.add('show');
  const keys = I[S.locale].helpKeys;
  const extra = I[S.locale].helpKeysDetailed || [];
  let items = [...keys];
  if (S.helpLevel >= 2) items = items.concat(extra);

  let html = '<div class="help-grid">';
  items.forEach(([k, d]) => {{
    html += '<div class="key-row"><span class="key" style="width:80px">' + escapeHtml(k) + '</span><span class="desc">' + escapeHtml(d) + '</span></div>';
  }});
  html += '</div>';
  if (S.helpLevel >= 3) {{
    html += '<div style="margin-top:8px;color:var(--fg-dim);font-size:11px">' +
      'Source: ' + escapeHtml(S.currentSourceName()) + ' | Fields: ' +
      (S.snapshot && S.snapshot.entries[S.currentSourceName()] ? S.snapshot.entries[S.currentSourceName()].fields.length : 0) +
      ' | History: ' + S.history.length + ' snapshots</div>';
  }}
  el.innerHTML = html;
}}

// ---- Graph ----
function toggleFieldGraph(source, fieldName) {{
  if (S.graphField === fieldName) {{ S.closeGraph(); return; }}
  S.graphField = fieldName;
  S.graphData = [];
  // Collect from history
  const allSnaps = [...S.history, S.snapshot].filter(Boolean);
  allSnaps.forEach(snap => {{
    const v = getFieldNumeric(snap, source, fieldName);
    if (v !== null) S.graphData.push({{ t: new Date(snap.timestamp).toLocaleTimeString(), v }});
  }});
  if (S.graphData.length > 60) S.graphData = S.graphData.slice(-60);
  document.getElementById('graph-title-field').textContent = source + '.' + fieldName;
  document.getElementById('graph-overlay').classList.add('show');
  renderFieldChart();
}}

function updateGraphData() {{
  if (!S.graphField || S.view !== 'detail') return;
  const src = S.currentSourceName();
  const v = getFieldNumeric(S.snapshot, src, S.graphField);
  if (v !== null) {{
    S.graphData.push({{ t: new Date(S.snapshot.timestamp).toLocaleTimeString(), v }});
    if (S.graphData.length > 60) S.graphData.shift();
    renderFieldChart();
  }}
}}

function renderFieldChart() {{
  const canvas = document.getElementById('graph-canvas');
  if (!canvas) return;
  if (S.chart) S.chart.destroy();
  S.chart = new Chart(canvas.getContext('2d'), {{
    type: 'line',
    data: {{
      labels: S.graphData.map(d => d.t),
      datasets: [{{ label: S.graphField, data: S.graphData.map(d => d.v), borderColor: '#7aa2f7', backgroundColor: 'rgba(122,162,247,0.1)', fill: true, tension: 0.3, pointRadius: 1 }}]
    }},
    options: {{
      responsive: true, maintainAspectRatio: false,
      scales: {{ x: {{ ticks: {{ color: '#565f89', maxTicksLimit: 10 }} }}, y: {{ ticks: {{ color: '#565f89' }} }} }},
      plugins: {{ legend: {{ labels: {{ color: '#c0caf5' }} }} }},
      animation: {{ duration: 200 }}
    }}
  }});
}}

// ---- Keyboard ----
document.addEventListener('keydown', (e) => {{
  // If search is focused, handle differently
  const searchBox = document.getElementById('search-box');
  if (document.activeElement === searchBox) {{
    if (e.key === 'Escape') {{ searchBox.blur(); S.searching = false; return; }}
    if (e.key === 'Enter') {{
      searchBox.blur();
      S.searching = false;
      // select first filtered source
      if (S.filteredKeys.length > 0) {{ S.selectedSource = 0; S.selectedField = 0; }}
      render();
      return;
    }}
    // Let normal typing happen
    setTimeout(() => {{ S.searchQuery = searchBox.value; filterSources(); renderSidebar(); }}, 0);
    return;
  }}

  const key = e.key;

  // Global keys
  if (key === '/') {{ e.preventDefault(); S.searching = true; searchBox.focus(); return; }}
  if (key === 'D') {{ e.preventDefault(); S.setView('dashboard'); return; }}
  if (key === 'O') {{ e.preventDefault(); S.setView('classic'); return; }}
  if (key === 'W') {{ e.preventDefault(); S.setView('welcome'); return; }}
  if (key === 'X') {{ e.preventDefault(); S.setView('diagnostics'); return; }}
  if (key === 'C') {{ e.preventDefault(); S.setView('category'); return; }}
  if (key === '?') {{ e.preventDefault(); S.cycleHelp(); return; }}
  if (key === 'L') {{ e.preventDefault(); S.toggleLang(); return; }}
  if (key === 'd') {{ e.preventDefault(); S.setView('diff'); return; }}
  if (key === 'e') {{
    e.preventDefault();
    const json = JSON.stringify(S.snapshot, null, 2);
    const blob = new Blob([json], {{type: 'application/json'}});
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url; a.download = 'syslenz_snapshot.json'; a.click();
    URL.revokeObjectURL(url);
    toast(S.t('exported'));
    return;
  }}
  if (key === 'a') {{ e.preventDefault(); S.autoRefresh = !S.autoRefresh; toast(S.autoRefresh ? 'Auto ON' : 'Auto OFF'); return; }}
  if (key === 'c') {{
    e.preventDefault();
    let text = '';
    if (S.view === 'detail') {{
      const src = S.currentSourceName();
      const entry = S.snapshot && S.snapshot.entries[src];
      if (entry) text = JSON.stringify(entry, null, 2);
    }} else if (S.view === 'diff') {{
      text = JSON.stringify(computeDiffs(S.history[S.history.length - 1], S.snapshot), null, 2);
    }} else if (S.view === 'diagnostics') {{
      text = JSON.stringify(runDiagnostics(S.snapshot), null, 2);
    }} else {{
      text = JSON.stringify(S.snapshot, null, 2);
    }}
    if (text) {{ navigator.clipboard.writeText(text).then(() => toast(S.t('copied'))); }}
    return;
  }}

  if (key === 'Backspace') {{
    e.preventDefault();
    if (S.graphField) {{ S.closeGraph(); return; }}
    if (S.view === 'detail' || S.view === 'diff') S.setView('dashboard');
    else S.setView('dashboard');
    return;
  }}

  if (key === 'Tab') {{
    e.preventDefault();
    if (S.view === 'detail' || S.view === 'diff') {{
      S.focus = S.focus === 'sidebar' ? 'content' : 'sidebar';
      render();
    }}
    return;
  }}

  // Navigation
  const inSidebar = S.focus === 'sidebar' && (S.view === 'detail' || S.view === 'diff');
  if (key === 'j' || key === 'ArrowDown') {{
    e.preventDefault();
    if (S.view === 'category') {{
      S.selectedCategory = Math.min(S.selectedCategory + 1, 99);
      S.categoryScroll = 0;
      render();
    }} else if (inSidebar) {{
      S.selectedSource = Math.min(S.selectedSource + 1, S.filteredKeys.length - 1);
      S.selectedField = 0;
      if (S.graphField) S.closeGraph();
      render();
    }} else if (S.view === 'detail') {{
      const src = S.currentSourceName();
      const entry = S.snapshot && S.snapshot.entries[src];
      if (entry) S.selectedField = Math.min(S.selectedField + 1, entry.fields.length - 1);
      render();
    }}
    return;
  }}

  if (key === 'k' || key === 'ArrowUp') {{
    e.preventDefault();
    if (S.view === 'category') {{
      S.selectedCategory = Math.max(S.selectedCategory - 1, 0);
      S.categoryScroll = 0;
      render();
    }} else if (inSidebar) {{
      S.selectedSource = Math.max(S.selectedSource - 1, 0);
      S.selectedField = 0;
      if (S.graphField) S.closeGraph();
      render();
    }} else if (S.view === 'detail') {{
      S.selectedField = Math.max(S.selectedField - 1, 0);
      render();
    }}
    return;
  }}

  if (key === 'Home') {{
    e.preventDefault();
    if (inSidebar) S.selectedSource = 0;
    else S.selectedField = 0;
    render();
    return;
  }}
  if (key === 'End') {{
    e.preventDefault();
    if (inSidebar) S.selectedSource = Math.max(0, S.filteredKeys.length - 1);
    else {{
      const src = S.currentSourceName();
      const entry = S.snapshot && S.snapshot.entries[src];
      if (entry) S.selectedField = entry.fields.length - 1;
    }}
    render();
    return;
  }}

  if (key === 'PageDown') {{
    e.preventDefault();
    if (S.view === 'category') {{
      const scrollEl = document.getElementById('cat-content-scroll');
      if (scrollEl) {{ scrollEl.scrollTop += 300; S.categoryScroll = scrollEl.scrollTop; }}
    }} else if (inSidebar) {{
      S.selectedSource = Math.min(S.selectedSource + 10, S.filteredKeys.length - 1);
      S.selectedField = 0;
    }} else if (S.view === 'detail') {{
      const src = S.currentSourceName();
      const entry = S.snapshot && S.snapshot.entries[src];
      if (entry) S.selectedField = Math.min(S.selectedField + 10, entry.fields.length - 1);
    }} else {{
      const contentEl = document.getElementById('content');
      if (contentEl) contentEl.scrollTop += 300;
    }}
    render();
    return;
  }}

  if (key === 'PageUp') {{
    e.preventDefault();
    if (S.view === 'category') {{
      const scrollEl = document.getElementById('cat-content-scroll');
      if (scrollEl) {{ scrollEl.scrollTop -= 300; S.categoryScroll = scrollEl.scrollTop; }}
    }} else if (inSidebar) {{
      S.selectedSource = Math.max(S.selectedSource - 10, 0);
      S.selectedField = 0;
    }} else if (S.view === 'detail') {{
      S.selectedField = Math.max(S.selectedField - 10, 0);
    }} else {{
      const contentEl = document.getElementById('content');
      if (contentEl) contentEl.scrollTop -= 300;
    }}
    render();
    return;
  }}

  if (key === 'Enter') {{
    e.preventDefault();
    if (inSidebar) {{
      S.focus = 'content';
      S.selectedField = 0;
      render();
    }} else if (S.view === 'detail') {{
      const src = S.currentSourceName();
      const entry = S.snapshot && S.snapshot.entries[src];
      if (entry && entry.fields[S.selectedField]) {{
        const f = entry.fields[S.selectedField];
        if (f.value.Table) showTableExpand(src, S.selectedField);
        else toggleFieldGraph(src, f.name);
      }}
    }} else if (S.view === 'dashboard') {{
      S.setView('detail');
    }}
    return;
  }}

  if (key === 'g') {{
    e.preventDefault();
    if (S.view === 'detail') {{
      const src = S.currentSourceName();
      const entry = S.snapshot && S.snapshot.entries[src];
      if (entry && entry.fields[S.selectedField]) {{
        toggleFieldGraph(src, entry.fields[S.selectedField].name);
      }}
    }}
    return;
  }}

  if (key === 'Escape') {{
    if (S.graphField) {{ S.closeGraph(); return; }}
    if (S.helpLevel > 0) {{ S.helpLevel = 0; renderHelp(); return; }}
  }}
}});

// ---- Start ----
document.getElementById('btn-lang').textContent = S.locale.toUpperCase();
init();
</script>
</body>
</html>"##,
        lang = lang,
        initial_locale = initial_locale,
    )
}
