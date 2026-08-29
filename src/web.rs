//! Web UI server using Axum.
//!
//! Serves a single-page dashboard with live-updating system metrics,
//! using Server-Sent Events for real-time streaming.

#[cfg(feature = "web")]
use crate::article;
#[cfg(feature = "web")]
use crate::i18n::Locale;
#[cfg(feature = "web")]
use crate::proc::Snapshot;

#[cfg(feature = "web")]
use axum::{
    Router,
    extract::{Query, State},
    http::StatusCode,
    response::{
        Html, IntoResponse, Json,
        sse::{Event, Sse},
    },
    routing::{get, post},
};
#[cfg(feature = "web")]
use std::sync::{Arc, Mutex};
#[cfg(feature = "web")]
use std::time::Duration;
#[cfg(feature = "web")]
use tokio::sync::broadcast;
#[cfg(feature = "web")]
use tokio_stream::StreamExt;
#[cfg(feature = "web")]
use tokio_stream::wrappers::BroadcastStream;

#[cfg(feature = "web")]
pub(crate) struct AppState {
    pub(crate) current: Mutex<Snapshot>,
    pub(crate) history: Mutex<Vec<Snapshot>>,
    pub(crate) tx: broadcast::Sender<String>,
    pub(crate) locale: Locale,
    pub(crate) config_path: Option<std::path::PathBuf>,
    pub(crate) alert_rules: Mutex<Vec<crate::alert::AlertRule>>,
    pub(crate) history_config: crate::config::HistoryTomlConfig,
    pub(crate) diagnostic_runbooks: Vec<crate::config::RunbookConfig>,
    pub(crate) web_config: crate::config::WebConfig,
}

#[cfg(feature = "web")]
pub fn run_web_server(bind: &str, locale: Locale) -> anyhow::Result<()> {
    let addr = normalize_web_bind(bind);
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let (tx, _) = broadcast::channel::<String>(64);
        let initial = Snapshot::capture()?;

        let cfg = crate::config::Config::load();
        let web_cfg = cfg.web.clone();
        let state = Arc::new(AppState {
            current: Mutex::new(initial),
            history: Mutex::new(Vec::new()),
            tx: tx.clone(),
            locale,
            config_path: crate::config::Config::config_path(),
            alert_rules: Mutex::new(cfg.alert.clone()),
            history_config: cfg.history.clone(),
            diagnostic_runbooks: cfg.diagnostic_runbook.clone(),
            web_config: web_cfg.clone(),
        });

        // 起動時にメモリ上限をログに出す（確認用）
        eprintln!(
            "syslenz web config: capture_interval={}s, max_history_count={}, max_history_bytes={}MB, truncate_tables={}",
            web_cfg.capture_interval_secs,
            web_cfg.max_history_count,
            web_cfg.max_history_bytes / 1024 / 1024,
            web_cfg.truncate_large_tables,
        );

        // Background task: capture snapshots periodically
        let bg_state = state.clone();
        tokio::spawn(async move {
            let interval = Duration::from_secs(bg_state.web_config.capture_interval_secs.max(1));
            let max_count = bg_state.web_config.max_history_count;
            let max_bytes = bg_state.web_config.max_history_bytes;
            let truncate = bg_state.web_config.truncate_large_tables;
            let truncate_rows = bg_state.web_config.truncate_table_rows;
            let mut trim_counter: u64 = 0;
            loop {
                tokio::time::sleep(interval).await;
                if let Ok(snapshot) = Snapshot::capture() {
                    let json = serde_json::to_string(&snapshot).unwrap_or_default();
                    {
                        let mut history = bg_state.history.lock().unwrap();
                        let old = bg_state.current.lock().unwrap().clone();
                        // 古いスナップショットを履歴に追加する際、巨大テーブルを縮約
                        let mut old_for_history = old;
                        if truncate {
                            truncate_snapshot_tables(&mut old_for_history, truncate_rows);
                        }
                        history.push(old_for_history);
                        // 件数上限
                        while history.len() > max_count {
                            history.remove(0);
                        }
                        // バイト数上限（概算: 各 Snapshot の JSON サイズで見積もり）
                        if max_bytes > 0 {
                            let mut total_bytes: usize = history
                                .iter()
                                .map(|s| approx_snapshot_bytes(s))
                                .sum();
                            while total_bytes > max_bytes && history.len() > 1 {
                                let removed = history.remove(0);
                                total_bytes = total_bytes.saturating_sub(approx_snapshot_bytes(&removed));
                            }
                        }
                    }
                    *bg_state.current.lock().unwrap() = snapshot;
                    let _ = bg_state.tx.send(json);
                }
                // 60 秒ごとに malloc_trim を呼んで、glibc malloc が
                // カーネルに返さない断片化メモリを解放する。
                // （毎秒呼ぶとオーバーヘッドが大きいため 60 秒間隔）
                trim_counter += 1;
                if trim_counter >= 60 {
                    trim_counter = 0;
                    #[cfg(target_os = "linux")]
                    unsafe {
                        libc::malloc_trim(0);
                    }
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
            .route("/api/field-help", get(field_help_handler))
            .route("/settings", get(settings_page_handler))
            .route("/api/v1/settings", get(settings_api_handler))
            .route("/api/v1/settings/alerts", post(settings_alerts_handler))
            .route("/api/article", get(article_handler))
            // Phase 2: MCP Streamable HTTP endpoint (JSON-RPC 2.0 over HTTP)
            .route("/mcp", post(crate::mcp::mcp_handler))
            // Phase 2: Diagnostics HTTP API (was TUI-internal only)
            .route("/api/diagnostics", get(diagnostics_handler))
            // Liveness probe for external monitoring. Intentionally exposes no
            // system information — body is a fixed "ok" string.
            .route("/healthz", get(healthz_handler))
            .with_state(state);

        eprintln!("syslenz web UI listening on http://{}", addr);
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    })
}

#[cfg(feature = "web")]
fn normalize_web_bind(bind: &str) -> String {
    match bind.parse::<u16>() {
        Ok(port) => format!("0.0.0.0:{}", port),
        Err(_) => bind.to_owned(),
    }
}

/// Health check endpoint for external monitoring.
///
/// Returns a small JSON object with liveness ("ok") plus memory-guard
/// configuration so operators can confirm the bounds are active.
/// No system information (process list, etc.) is exposed.
#[cfg(feature = "web")]
async fn healthz_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let history_len = state.history.lock().unwrap().len();
    let wc = &state.web_config;
    let body = serde_json::json!({
        "status": "ok",
        "history_len": history_len,
        "max_history_count": wc.max_history_count,
        "max_history_bytes": wc.max_history_bytes,
        "capture_interval_secs": wc.capture_interval_secs,
        "truncate_large_tables": wc.truncate_large_tables,
    });
    (StatusCode::OK, Json(body))
}

/// 履歴保持用に Snapshot 内の巨大テーブルを縮約する。
/// 最新の1件（current）はフルで保持されるため、この関数は履歴に
/// 追加する *古い* スナップショットにのみ適用される。
///
/// `max_rows` を超える Table フィールドは、先頭 `max_rows` 行＋
/// `[truncated: N rows]` の1行に置き換わり、`truncated: true` を示す
/// ダミー行が残る。これにより「データが省かれた」ことが分かる。
#[cfg(feature = "web")]
fn truncate_snapshot_tables(snap: &mut crate::proc::Snapshot, max_rows: usize) {
    use crate::proc::FieldValue;
    for entry in snap.entries.values_mut() {
        for field in &mut entry.fields {
            if let FieldValue::Table(rows) = &mut field.value {
                if rows.len() > max_rows {
                    let truncated_count = rows.len() - max_rows;
                    let mut kept = rows.drain(..max_rows).collect::<Vec<_>>();
                    kept.push(vec![format!(
                        "[truncated: {} rows]",
                        truncated_count
                    )]);
                    *rows = kept;
                }
            }
        }
    }
}

/// Snapshot の概算メモリサイズ（バイト）を返す。
/// バイト数上限の判定に使う。正確な値ではなく、文字列・テーブルの
/// バイト数の和で見積もる。Vec のオーバーヘッドは含まない。
#[cfg(feature = "web")]
fn approx_snapshot_bytes(snap: &crate::proc::Snapshot) -> usize {
    use crate::proc::FieldValue;
    let mut total = 0usize;
    for (k, e) in &snap.entries {
        total += k.len();
        total += e.source.len();
        for f in &e.fields {
            total += f.name.len();
            total += f.description.len();
            if let Some(u) = &f.unit {
                total += u.len();
            }
            match &f.value {
                FieldValue::Text(s) => total += s.len(),
                FieldValue::Table(rows) => {
                    for r in rows {
                        for c in r {
                            total += c.len();
                        }
                    }
                }
                _ => {}
            }
        }
    }
    total
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
    Sse::new(stream)
        .keep_alive(axum::response::sse::KeepAlive::new().interval(Duration::from_secs(10)))
}

#[cfg(feature = "web")]
#[derive(serde::Deserialize)]
struct ViewQuery {
    view: Option<String>,
    locale: Option<String>,
    /// G20-10: URL state sharing — host selector (reserved for multi-host)
    #[allow(dead_code)]
    host: Option<String>,
    /// G20-10: URL state sharing — select source by name
    source: Option<String>,
    /// G20-10: URL state sharing — select field by name
    field: Option<String>,
    /// PID for ProcessDetail view
    pid: Option<String>,
}

#[cfg(feature = "web")]
async fn view_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ViewQuery>,
) -> impl IntoResponse {
    use crate::ui::app::View;

    let locale = params
        .locale
        .as_deref()
        .map(Locale::from_str)
        .unwrap_or(state.locale);

    let snapshot = state.current.lock().unwrap().clone();
    let history = state.history.lock().unwrap().clone();

    // Build a minimal App to use the ViewData builders
    let source_keys: Vec<String> = snapshot.entries.keys().cloned().collect();

    // Reject path-traversal in pid early: it is interpolated into /proc paths.
    let safe_pid = params.pid.and_then(|p| {
        if !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit()) {
            Some(p)
        } else {
            None
        }
    });

    // G20-10: If source/field are specified, auto-select detail view
    let view = if safe_pid.is_some() {
        View::ProcessDetail
    } else if params.source.is_some() {
        View::Detail
    } else {
        match params.view.as_deref() {
            Some("welcome") => View::Welcome,
            Some("detail") => View::Detail,
            Some("diff") => View::Diff,
            Some("table") => View::TableView,
            Some("graph") => View::Graph,
            Some("diagnostics") => View::Diagnostics,
            Some("category") => View::CategoryGuide,
            Some("processdetail") => View::ProcessDetail,
            _ => View::Dashboard,
        }
    };

    // G20-10: Resolve source index from query param
    let selected_source = params
        .source
        .as_deref()
        .and_then(|s| source_keys.iter().position(|k| k == s))
        .unwrap_or(0);

    // G20-10: Resolve field index from query param
    let selected_field = if let (Some(src_name), Some(field_name)) =
        (params.source.as_deref(), params.field.as_deref())
    {
        snapshot
            .entries
            .get(src_name)
            .and_then(|entry| entry.fields.iter().position(|f| f.name == field_name))
            .unwrap_or(0)
    } else {
        0
    };

    let app = build_minimal_app(
        snapshot,
        history,
        source_keys,
        view,
        locale,
        selected_source,
        selected_field,
        safe_pid,
    );

    let view_data = app.build_view_data();
    Json(view_data)
}

/// Build a minimal App for ViewData generation (shared by view_handler and field_help_handler).
#[cfg(feature = "web")]
fn build_minimal_app(
    snapshot: Snapshot,
    history: Vec<Snapshot>,
    source_keys: Vec<String>,
    view: crate::ui::app::View,
    locale: Locale,
    selected_source: usize,
    selected_field: usize,
    detailed_pid: Option<String>,
) -> crate::ui::app::App {
    use crate::ui::app::{App, ConnectionStatus, Focus, HostState};

    let host0 = HostState {
        label: "localhost".to_string(),
        current: snapshot.clone(),
        snapshots: history.clone(),
        max_snapshots: 60,
        receiver: None,
        connection_status: ConnectionStatus::Local,
        alert_events: Vec::new(),
    };

    App {
        snapshots: history,
        current: snapshot.clone(),
        diffs: Vec::new(),
        view,
        focus: Focus::Content,
        selected_source,
        source_keys,
        selected_field,
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
        diagnostic_runbooks: Vec::new(),
        hosts: vec![host0],
        active_host: 0,
        connection_status: ConnectionStatus::Local,
        tutorial_step: None,
        view_history: Vec::new(),
        selected_diagnostic: 0,
        selected_related_metric: None,
        sidebar_tree: false,
        graph_time_window: 60,
        table_view_source: None,
        detailed_pid,
        article_overlay: None,
        article_content_lines: 0,
        article_visible_height: 0,
        dash_zero_axis: false,
        pins: crate::pins::PinSet::new(Vec::new()),
        pin_filter: false,
    }
}

/// G20-7: /api/field-help endpoint — P-A4 spec
/// GET /api/field-help?source=meminfo&field=MemAvailable&lang=ja
#[cfg(feature = "web")]
#[derive(serde::Deserialize)]
struct FieldHelpQuery {
    source: String,
    field: String,
    lang: Option<String>,
}

#[cfg(feature = "web")]
#[derive(serde::Serialize)]
struct FieldHelpResponse {
    source: String,
    field: String,
    description: FieldHelpDescription,
    see_also: Vec<FieldHelpLink>,
    breadcrumbs: Vec<FieldHelpLink>,
    contextual_hint: Option<String>,
}

#[cfg(feature = "web")]
#[derive(serde::Serialize)]
struct FieldHelpDescription {
    normal: String,
    detailed: String,
    extra: String,
}

#[cfg(feature = "web")]
#[derive(serde::Serialize)]
struct FieldHelpLink {
    source: String,
    field: String,
    reason: String,
}

#[cfg(feature = "web")]
async fn field_help_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<FieldHelpQuery>,
) -> impl IntoResponse {
    use crate::education;
    use crate::i18n;

    let locale = params
        .lang
        .as_deref()
        .map(Locale::from_str)
        .unwrap_or(state.locale);

    let source = &params.source;
    let field = &params.field;

    // Description (normal, detailed, extra) from i18n
    let description = match i18n::field_description(locale, source, field) {
        Some((normal, detailed, extra)) => FieldHelpDescription {
            normal: normal.to_string(),
            detailed: detailed.to_string(),
            extra: extra.to_string(),
        },
        None => FieldHelpDescription {
            normal: String::new(),
            detailed: String::new(),
            extra: String::new(),
        },
    };

    // See-also cross-links from i18n
    let see_also: Vec<FieldHelpLink> = i18n::see_also(locale, source, field)
        .into_iter()
        .map(|(src, fld, reason)| FieldHelpLink {
            source: src.to_string(),
            field: fld.to_string(),
            reason: reason.to_string(),
        })
        .collect();

    // Breadcrumbs (learning path) from education
    let breadcrumbs: Vec<FieldHelpLink> = education::breadcrumbs(locale, source, field)
        .into_iter()
        .map(|(src, fld, reason)| FieldHelpLink {
            source: src.to_string(),
            field: fld.to_string(),
            reason: reason.to_string(),
        })
        .collect();

    // Contextual hint: computed from live snapshot data
    let contextual_hint = {
        let snapshot = state.current.lock().unwrap().clone();
        let history = state.history.lock().unwrap().clone();
        let source_keys: Vec<String> = snapshot.entries.keys().cloned().collect();
        let selected_source = source_keys.iter().position(|k| k == source).unwrap_or(0);
        let selected_field_idx = snapshot
            .entries
            .get(source.as_str())
            .and_then(|entry| entry.fields.iter().position(|f| f.name == *field))
            .unwrap_or(0);
        let app = build_minimal_app(
            snapshot,
            history,
            source_keys,
            crate::ui::app::View::Detail,
            locale,
            selected_source,
            selected_field_idx,
            None,
        );
        crate::ui::render::get_contextual_hint_for_api(&app, source, field)
    };

    Json(FieldHelpResponse {
        source: source.to_string(),
        field: field.to_string(),
        description,
        see_also,
        breadcrumbs,
        contextual_hint,
    })
}

#[cfg(feature = "web")]
#[derive(serde::Deserialize)]
struct ArticleQuery {
    source: Option<String>,
    field: Option<String>,
    id: Option<String>,
    locale: Option<String>,
}

#[cfg(feature = "web")]
async fn article_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ArticleQuery>,
) -> impl IntoResponse {
    let locale = params
        .locale
        .as_deref()
        .map(Locale::from_str)
        .unwrap_or(state.locale);

    let mut found = true;
    let article_ref = if let Some(id) = params.id.as_deref() {
        if let Some(article) = article::find_article_by_id(id) {
            article
        } else {
            found = false;
            article::fallback_article()
        }
    } else if let (Some(source), Some(field)) = (params.source.as_deref(), params.field.as_deref())
    {
        let resolved_id = article::resolve_article_id(source, field);
        if resolved_id == article::fallback_article().id {
            let direct_id = format!("{source}.{field}");
            found = article::find_article_by_id(&direct_id).is_some();
        }
        article::resolve_article(source, field)
    } else {
        found = false;
        article::fallback_article()
    };

    let mut api = article::to_api_article(article_ref, locale);
    api.found = found;
    Json(api)
}

#[cfg(feature = "web")]
fn build_html(lang: &str) -> String {
    let initial_locale = if lang == "ja" { "ja" } else { "en" };
    format!(
        r##"<!DOCTYPE html>
<html lang="{lang}">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>syslenz</title>
<script src="https://cdn.jsdelivr.net/npm/chart.js@4"></script>
<script src="https://cdn.jsdelivr.net/npm/marked/marked.min.js"></script>
<script src="https://cdn.jsdelivr.net/npm/dompurify@3.1.6/dist/purify.min.js"></script>
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

/* Article overlay */
#article-overlay{{display:none;position:fixed;inset:6vh 6vw;background:var(--bg-dark);border:2px solid var(--blue);border-radius:10px;z-index:140;overflow:hidden;box-shadow:0 16px 48px rgba(0,0,0,.45)}}
#article-overlay.show{{display:flex;flex-direction:column}}
#article-overlay .head{{padding:10px 14px;border-bottom:1px solid var(--border);display:flex;justify-content:space-between;align-items:center;gap:8px}}
#article-overlay .title{{color:var(--blue);font-weight:bold;font-size:14px}}
#article-overlay .meta{{color:var(--fg-dim);font-size:11px}}
#article-overlay .body{{flex:1;overflow-y:auto;padding:14px;font-size:13px;line-height:1.65}}
#article-overlay .md-content h1,#article-overlay .md-content h2,#article-overlay .md-content h3{{color:var(--blue);margin:10px 0 6px 0}}
#article-overlay .md-content h4,#article-overlay .md-content h5{{color:var(--yellow);margin:10px 0 6px 0}}
#article-overlay .md-content p{{margin:0 0 8px 0}}
#article-overlay .md-content ul,#article-overlay .md-content ol{{margin:0 0 10px 20px}}
#article-overlay .md-content li{{margin:2px 0}}
#article-overlay .md-content code{{background:var(--bg-hl);padding:1px 4px;border-radius:4px;color:var(--cyan)}}
#article-overlay .md-content pre{{background:var(--bg-hl);padding:10px;border-radius:8px;overflow:auto;margin:8px 0}}
#article-overlay .md-content pre code{{background:transparent;padding:0}}
#article-overlay .md-content blockquote{{border-left:3px solid var(--border);padding-left:10px;color:var(--fg-dim);margin:8px 0}}
#article-overlay .body h4{{color:var(--yellow);font-size:12px;margin:12px 0 6px 0}}
#article-overlay .body pre{{white-space:pre-wrap;font-family:inherit}}
#article-overlay .article-toc{{border:1px solid var(--border);border-radius:8px;background:var(--bg);padding:10px;margin-bottom:12px}}
#article-overlay .article-toc .toc-title{{color:var(--yellow);font-size:11px;font-weight:bold;margin-bottom:6px;text-transform:uppercase;letter-spacing:.5px}}
#article-overlay .article-toc .toc-item{{display:block;width:100%;text-align:left;padding:4px 6px;background:transparent;border:none;color:var(--cyan);font:inherit;font-size:12px;cursor:pointer;border-radius:6px}}
#article-overlay .article-toc .toc-item:hover{{background:var(--bg-hl)}}
#article-overlay .article-toc .toc-item.l3{{padding-left:18px;color:var(--fg)}}
#article-overlay .links{{display:flex;flex-direction:column;gap:4px;margin-top:8px}}
#article-overlay .link{{padding:4px 8px;border-radius:6px;border:1px solid transparent;cursor:pointer;color:var(--cyan);font-size:12px}}
#article-overlay .link:hover{{background:var(--bg-hl)}}
#article-overlay .link.active{{background:var(--yellow);color:var(--bg-dark);font-weight:bold}}
#article-overlay .foot{{padding:8px 12px;border-top:1px solid var(--border);font-size:11px;color:var(--fg-dim)}}

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

/* Sidebar tree mode */
.src-dir{{padding:4px 8px;font-size:11px;color:var(--yellow);font-weight:bold;cursor:default;user-select:none;letter-spacing:.3px}}
.src-item.depth1{{padding-left:22px}}
.tree-toggle{{background:none;border:none;color:var(--fg-dim);font:inherit;font-size:10px;cursor:pointer;padding:0 4px}}
.tree-toggle:hover{{color:var(--blue)}}

/* Time window buttons */
.time-win-btns{{display:flex;gap:4px;margin-top:6px;flex-wrap:wrap}}
.time-win-btn{{background:var(--bg-sel);color:var(--fg-dim);border:1px solid var(--border);padding:2px 7px;border-radius:4px;cursor:pointer;font:inherit;font-size:10px}}
.time-win-btn:hover{{background:var(--border)}}
.time-win-btn.active{{background:var(--blue);color:var(--bg-dark);font-weight:bold;border-color:var(--blue)}}

/* Table view */
.table-view-header{{color:var(--blue);font-size:14px;margin-bottom:8px;display:flex;align-items:center;gap:12px}}
.table-view-back{{color:var(--fg-dim);font-size:11px;cursor:pointer}}
.table-view-back:hover{{color:var(--blue)}}
.table-back-btn{{background:none;border:none;color:var(--fg-dim);font:inherit;font-size:12px;cursor:pointer;padding:4px 8px;border-radius:4px}}
.table-back-btn:hover{{background:var(--bg-hl);color:var(--blue)}}

/* Process detail */
.proc-detail-field{{display:flex;padding:5px 8px;border-bottom:1px solid var(--bg-sel);font-size:13px;cursor:pointer}}
.proc-detail-field:hover{{background:var(--bg-hl)}}
.proc-detail-field.selected{{background:var(--bg-sel)}}
.proc-detail-name{{color:var(--purple);width:200px;flex-shrink:0;font-weight:bold}}
.proc-detail-value{{flex:1;color:var(--cyan)}}
.proc-detail-desc{{color:var(--fg-dim);font-size:11px;margin-left:12px;font-style:italic}}
.proc-detail-subtable{{margin:4px 0 8px 200px;font-size:12px;border-collapse:collapse;width:calc(100% - 200px)}}
.proc-detail-subtable th{{color:var(--yellow);padding:3px 8px;border-bottom:1px solid var(--border);text-align:left;font-size:10px;text-transform:uppercase}}
.proc-detail-subtable td{{padding:3px 8px;border-bottom:1px solid var(--bg-sel)}}

/* Toast */
#toast{{position:fixed;top:20px;right:20px;background:var(--blue);color:var(--bg-dark);padding:8px 16px;border-radius:6px;font-size:13px;display:none;z-index:200;font-weight:bold}}
#toast.show{{display:block;animation:fadeout 2s forwards}}
@keyframes fadeout{{0%,70%{{opacity:1}}100%{{opacity:0;display:none}}}}
</style>
</head>
<body>
<div id="sidebar">
  <div id="sidebar-header"><span>Sources (<span id="source-count">0</span>)</span><button class="tree-toggle" id="tree-toggle-btn" onclick="S.toggleSidebarTree()" title="Toggle tree/flat (t)">flat</button></div>
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
      <span class="badge" id="btn-article" onclick="S.toggleArticleForSelection()">A</span>
      <span class="badge" id="btn-refresh" onclick="S.toggleAutoRefresh()">RT</span>
      <span class="badge" id="btn-axis" onclick="S.toggleAxisMode()">AXIS</span>
      <span class="badge" id="btn-help" onclick="S.cycleHelp()">?</span>
      <span class="badge" id="btn-lang" onclick="S.toggleLang()">EN</span>
    </div>
  </div>
  <div id="content"></div>
  <div id="help-overlay"></div>
  <div id="article-overlay">
    <div class="head">
      <div>
        <div class="title" id="article-title">Article</div>
        <div class="meta" id="article-meta"></div>
      </div>
      <span class="badge" onclick="closeArticleOverlay()">Esc</span>
    </div>
    <div class="body" id="article-body"></div>
    <div class="foot" id="article-foot"></div>
  </div>
  <div id="graph-overlay"><h3>Graph: <span id="graph-title-field"></span> <span style="color:var(--fg-dim);font-size:11px" id="graph-win-label"></span></h3><div class="time-win-btns" id="graph-win-btns"></div><div class="graph-canvas-wrap"><canvas id="graph-canvas"></canvas></div></div>
  <div id="statusbar">
    <div class="left">
      <span id="sb-view">Dashboard</span>
      <span id="sb-snaps">0 snaps</span>
      <span id="sb-source">-</span>
    </div>
    <div class="right">
      <span id="sb-keys">? Help | A Article | D Dashboard | O Classic | X Diag | L Lang</span>
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
      ['A', 'Article overlay'],
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
      ['r', 'Toggle real-time updates'],
      ['s', 'Toggle axis scaling'],
    ],
    axis_auto: 'Axis (auto range)',
    axis_zero: 'Axis (zero baseline)',
    axis_toggle: 'Toggle axis scaling',
    real_time_on: 'Real-time updates resumed',
    real_time_off: 'Real-time updates paused',
    real_time_resume_label: 'Pause real-time updates',
    real_time_pause_label: 'Resume real-time updates',
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
      ['A', '記事オーバーレイ'],
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
      ['r', 'リアルタイム更新の切替'],
      ['s', '軸スケールの切替'],
    ],
    axis_auto: '軸: 自動レンジ',
    axis_zero: '軸: ゼロ基点',
    axis_toggle: '軸スケール切替',
    real_time_on: 'リアルタイム更新を再開しました',
    real_time_off: 'リアルタイム更新を停止しました',
    real_time_resume_label: 'リアルタイム更新を停止',
    real_time_pause_label: 'リアルタイム更新を再開',
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
  axisZero: false,
  evtSource: null,
  connected: false,
  dashCharts: {{}},
  viewDataCache: null,
  autoGraphChart: null,
  selectedCategory: 0,
  categoryScroll: 0,
  sidebarTree: false,
  tableViewSource: null,
  tableViewFieldIdx: null,
  tableViewScroll: 0,
  graphTimeWindow: 60,
  graphAllTimeMin: {{}},
  graphAllTimeMax: {{}},
  detailedPid: null,
  articleOverlay: {{
    open: false,
    loading: false,
    article: null,
    selectedLink: 0,
  }},

  t(key) {{ return I[this.locale][key] || key; }},

  setView(v) {{
    if (this.graphField) this.closeGraph();
    if (this.autoGraphChart) {{ this.autoGraphChart.destroy(); this.autoGraphChart = null; }}
    if (v === 'classic') v = 'detail';
    if (v !== 'table') {{ this.tableViewSource = null; this.tableViewFieldIdx = null; }}
    if (v !== 'processdetail') this.detailedPid = null;
    this.view = v;
    if (v === 'detail' || v === 'diff') this.focus = 'sidebar';
    else this.focus = 'content';
    this.selectedField = 0;
    render();
  }},

  toggleSidebarTree() {{
    this.sidebarTree = !this.sidebarTree;
    const btn = document.getElementById('tree-toggle-btn');
    if (btn) btn.textContent = this.sidebarTree ? 'tree' : 'flat';
    renderSidebar();
  }},

  cycleTimeWindow(delta) {{
    const windows = [30, 60, 120, 300, 900, 3600];
    const idx = windows.indexOf(this.graphTimeWindow);
    const next = Math.max(0, Math.min(windows.length - 1, (idx < 0 ? 1 : idx) + delta));
    this.graphTimeWindow = windows[next];
    updateAutoGraph();
    if (this.graphField) {{
      const src = this.currentSourceName();
      this.rebuildFieldGraphData(src, this.graphField);
      renderFieldChart();
    }}
    renderGraphWinBtns();
  }},

  rebuildFieldGraphData(source, fieldName) {{
    const windows = this.graphTimeWindow;
    const allSnaps = [...this.history, this.snapshot].filter(Boolean);
    const sliced = allSnaps.slice(-windows);
    this.graphData = [];
    sliced.forEach(snap => {{
      const v = getFieldNumeric(snap, source, fieldName);
      if (v !== null) this.graphData.push({{ t: new Date(snap.timestamp).toLocaleTimeString(), v }});
    }});
  }},

  toggleLang() {{
    this.locale = this.locale === 'en' ? 'ja' : 'en';
    document.documentElement.lang = this.locale;
    document.getElementById('btn-lang').textContent = this.locale.toUpperCase();
    render();
  }},

  toggleAutoRefresh() {{
    this.autoRefresh = !this.autoRefresh;
    toast(this.autoRefresh ? I[this.locale].real_time_on : I[this.locale].real_time_off);
    renderTopbar();
  }},

  toggleAxisMode() {{
    this.axisZero = !this.axisZero;
    toast(this.axisZero ? I[this.locale].axis_zero : I[this.locale].axis_auto);
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

  async toggleArticleForSelection() {{
    if (this.articleOverlay.open) {{
      closeArticleOverlay();
      return;
    }}
    await openArticleForSelection();
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

function slugifyHeading(text) {{
  const s = String(text || '').trim().toLowerCase()
    .replace(/[^\p{{L}}\p{{N}}\s-]/gu, '')
    .replace(/\s+/g, '-')
    .replace(/-+/g, '-')
    .replace(/^-|-$/g, '');
  return s;
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

  // Collect ALL history for stable min/max (prevents pikon y-axis rescaling)
  const allSnaps = [...S.history, S.snapshot].filter(Boolean);
  const allVals = allSnaps.map(s => getFieldNumeric(s, src, f.name)).filter(v => v !== null);
  if (allVals.length < 2) {{ panel.classList.remove('show'); return; }}

  // Track all-time min/max so axis never shrinks
  const stateKey = src + '.' + f.name;
  const prevMin = S.graphAllTimeMin[stateKey] ?? Infinity;
  const prevMax = S.graphAllTimeMax[stateKey] ?? -Infinity;
  S.graphAllTimeMin[stateKey] = Math.min(prevMin, Math.min(...allVals));
  S.graphAllTimeMax[stateKey] = Math.max(prevMax, Math.max(...allVals));
  const yMin = S.graphAllTimeMin[stateKey];
  const yMax = S.graphAllTimeMax[stateKey];
  const pad = (yMax - yMin) * 0.05 || 1;

  // Visible window limited to graphTimeWindow snapshots
  const windowedSnaps = allSnaps.slice(-S.graphTimeWindow);
  const points = windowedSnaps
    .map(snap => {{ const v = getFieldNumeric(snap, src, f.name); return v !== null ? {{ t: new Date(snap.timestamp).toLocaleTimeString(), v }} : null; }})
    .filter(Boolean);

  const vals = points.map(p => p.v);
  const current = vals[vals.length - 1];
  const avg = vals.reduce((a, b) => a + b, 0) / vals.length;

  panel.classList.add('show');
  const title = panel.querySelector('h4');
  if (title) title.textContent = src + '.' + f.name + ' [' + graphWinLabel(S.graphTimeWindow) + ']';
  const statsEl = panel.querySelector('.auto-graph-stats');
  if (statsEl) statsEl.innerHTML = 'Min: <span>' + yMin.toFixed(2) + '</span> | Max: <span>' + yMax.toFixed(2) + '</span> | Avg: <span>' + avg.toFixed(2) + '</span> | Current: <span>' + current.toFixed(2) + '</span>';

  // Render time window buttons inside panel
  let twHtml = document.getElementById('auto-graph-time-btns');
  if (!twHtml) {{
    const statsDiv = panel.querySelector('.auto-graph-stats');
    if (statsDiv) {{
      const btnWrap = document.createElement('div');
      btnWrap.id = 'auto-graph-time-btns';
      btnWrap.className = 'time-win-btns';
      panel.insertBefore(btnWrap, statsDiv.nextSibling);
    }}
  }}
  const btnWrap2 = document.getElementById('auto-graph-time-btns');
  if (btnWrap2) {{
    const wins = [30,60,120,300,900,3600];
    btnWrap2.innerHTML = wins.map(w =>
      '<button class="time-win-btn' + (w === S.graphTimeWindow ? ' active' : '') + '" onclick="S.cycleTimeWindow(' + (wins.indexOf(w) - wins.indexOf(S.graphTimeWindow)) + ');updateAutoGraph()">' + graphWinLabel(w) + '</button>'
    ).join('');
  }}

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
      scales: {{
        x: {{ ticks: {{ color: '#565f89', maxTicksLimit: 10, font: {{ size: 10 }} }} }},
        y: {{ min: yMin - pad, max: yMax + pad, ticks: {{ color: '#565f89', font: {{ size: 10 }} }} }}
      }},
      plugins: {{ legend: {{ labels: {{ color: '#c0caf5', font: {{ size: 10 }} }} }} }},
      animation: {{ duration: 0 }}
    }}
  }});
}}

function graphWinLabel(secs) {{
  if (secs < 60) return secs + 's';
  if (secs < 3600) return (secs/60|0) + 'm';
  return (secs/3600|0) + 'h';
}}

function renderGraphWinBtns() {{
  const wrap = document.getElementById('graph-win-btns');
  if (!wrap) return;
  const wins = [30,60,120,300,900,3600];
  wrap.innerHTML = wins.map(w =>
    '<button class="time-win-btn' + (w === S.graphTimeWindow ? ' active' : '') + '" onclick="S.graphTimeWindow=' + w + ';renderFieldChart()">' + graphWinLabel(w) + '</button>'
  ).join('');
  const lbl = document.getElementById('graph-win-label');
  if (lbl) lbl.textContent = '[' + graphWinLabel(S.graphTimeWindow) + ']';
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
  renderArticleOverlay();
  updateGraphData();
}}

function buildTreeRows(keys) {{
  // Group keys by first path segment (e.g. net/dev → prefix=net, child=dev)
  const groups = {{}};
  const singles = [];
  keys.forEach((k, i) => {{
    const slash = k.indexOf('/');
    if (slash > 0) {{
      const prefix = k.slice(0, slash);
      if (!groups[prefix]) groups[prefix] = [];
      groups[prefix].push({{ key: k, idx: i }});
    }} else {{
      singles.push({{ key: k, idx: i }});
    }}
  }});
  const rows = [];
  // Merge singles and group headers alphabetically
  const allItems = [
    ...singles.map(s => ({{ type: 'leaf', key: s.key, idx: s.idx }})),
    ...Object.keys(groups).sort().map(p => ({{ type: 'dir', prefix: p, children: groups[p] }}))
  ].sort((a, b) => {{
    const ka = a.type === 'leaf' ? a.key : a.prefix + '/';
    const kb = b.type === 'leaf' ? b.key : b.prefix + '/';
    return ka.localeCompare(kb);
  }});
  allItems.forEach(item => {{
    if (item.type === 'leaf') {{
      rows.push({{ type: 'leaf', key: item.key, idx: item.idx, label: item.key, depth: 0 }});
    }} else {{
      rows.push({{ type: 'dir', label: item.prefix + '/', depth: 0 }});
      item.children.sort((a, b) => a.key.localeCompare(b.key)).forEach(c => {{
        rows.push({{ type: 'leaf', key: c.key, idx: c.idx, label: c.key.slice(item.prefix.length + 1), depth: 1 }});
      }});
    }}
  }});
  return rows;
}}

function renderSidebar() {{
  document.getElementById('source-count').textContent = S.sourceKeys.length;
  const list = document.getElementById('source-list');
  const show = S.view === 'detail' || S.view === 'diff' || S.view === 'table' || S.view === 'processdetail';
  document.getElementById('sidebar').classList.toggle('hidden', !show);
  const toggleBtn = document.getElementById('tree-toggle-btn');
  if (toggleBtn) toggleBtn.textContent = S.sidebarTree ? 'tree' : 'flat';
  if (!show) return;
  let html = '';
  if (S.sidebarTree) {{
    const rows = buildTreeRows(S.filteredKeys);
    rows.forEach(row => {{
      if (row.type === 'dir') {{
        html += '<div class="src-dir">' + escapeHtml(row.label) + '</div>';
      }} else {{
        const active = row.idx === S.selectedSource;
        const depthCls = row.depth > 0 ? ' depth1' : '';
        html += '<div class="src-item' + depthCls + (active ? ' active' : '') + '" data-idx="' + row.idx + '">' + escapeHtml(row.label) + '</div>';
      }}
    }});
  }} else {{
    S.filteredKeys.forEach((k, i) => {{
      const cls = (i === S.selectedSource ? 'active ' : '') + (k.startsWith('net/') ? 'net ' : '');
      html += '<div class="src-item ' + cls + '" data-idx="' + i + '">' + escapeHtml(k) + '</div>';
    }});
  }}
  list.innerHTML = html;
  const active = list.querySelector('.active');
  if (active) active.scrollIntoView({{ block: 'nearest' }});
  list.querySelectorAll('.src-item').forEach(el => {{
    el.onclick = () => {{
      S.selectedSource = parseInt(el.dataset.idx);
      S.selectedField = 0;
      S.tableViewSource = null; S.tableViewFieldIdx = null;
      S.detailedPid = null;
      if (S.view === 'table' || S.view === 'processdetail') S.view = 'detail';
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
  ['btn-dash','btn-classic','btn-diag','btn-article'].forEach(id => document.getElementById(id).classList.remove('active'));
  if (S.view === 'dashboard') document.getElementById('btn-dash').classList.add('active');
  else if (S.view === 'detail' || S.view === 'diff') document.getElementById('btn-classic').classList.add('active');
  else if (S.view === 'diagnostics') document.getElementById('btn-diag').classList.add('active');
  if (S.articleOverlay.open) document.getElementById('btn-article').classList.add('active');
  const refreshBtn = document.getElementById('btn-refresh');
  if (refreshBtn) {{
    refreshBtn.classList.toggle('active', S.autoRefresh);
    refreshBtn.title = S.autoRefresh ? I[S.locale].real_time_resume_label : I[S.locale].real_time_pause_label;
  }}
  const axisBtn = document.getElementById('btn-axis');
  if (axisBtn) {{
    axisBtn.classList.toggle('active', S.axisZero);
    axisBtn.textContent = S.axisZero ? I[S.locale].axis_zero : I[S.locale].axis_auto;
    axisBtn.title = I[S.locale].axis_toggle;
  }}
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
    case 'table': renderTableView(el); break;
    case 'processdetail': renderProcessDetail(el); break;
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

  const yAxisOptions = {{
    display: true,
    ticks: {{ color: '#565f89', font: {{ size: 10 }} }},
    beginAtZero: S.axisZero,
  }};
  const chartOpts = {{
    responsive: true, maintainAspectRatio: false,
    scales: {{ x: {{ display: true, ticks: {{ color: '#565f89', maxTicksLimit: 8, font: {{ size: 10 }} }} }}, y: yAxisOptions }},
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
  // Pin source/field so RT refreshes don't wipe the table view
  S.tableViewSource = source;
  S.tableViewFieldIdx = idx;
  S.tableViewScroll = 0;
  S.view = 'table';
  render();
}}

const TABLE_HEADERS = {{
  'mounts':     ['Device','Mountpoint','FSType','Options'],
  'partitions': ['Name','Size','Major','Minor'],
  'net/dev':    ['Interface','RX Bytes','RX Pkts','TX Bytes','TX Pkts'],
  'diskstats':  ['Device','Reads','Read Bytes','Writes','Written','InFlight'],
  'processes':  ['PID','Name','State','RSS','Threads','UID','FDs'],
  'swaps':      ['Filename','Type','Size','Used','Priority'],
  'modules':    ['Name','Size','Used By','State'],
  'net/tcp':    ['Local Addr','Remote Addr','State','UID'],
  'net/udp':    ['Local Addr','Remote Addr','State','UID'],
  'net/unix':   ['Type','State','Inode','Path'],
  'net/arp':    ['IP Address','HW Address','Device'],
  'net/route':  ['Iface','Destination','Gateway','Mask','Metric'],
  'crypto':     ['Name','Driver','Module','Type','BlockSize'],
  'locks':      ['Type','Mode','RW','PID','Range'],
  'interrupts': ['IRQ','Count','Type','Device'],
  'devices':    ['Major','Name'],
  'cgroups':    ['Name','Hierarchy','NumCGroups','Enabled'],
}};

function renderTableView(el) {{
  const source = S.tableViewSource;
  const idx = S.tableViewFieldIdx;
  if (!source || idx === null) {{ S.view = 'detail'; renderContent(); return; }}

  const entry = S.snapshot && S.snapshot.entries[source];
  if (!entry) {{ el.innerHTML = '<p style="color:var(--fg-dim)">No data</p>'; return; }}
  const field = entry.fields[idx];
  if (!field || !field.value.Table) {{ el.innerHTML = '<p style="color:var(--fg-dim)">No table data</p>'; return; }}
  const rows = field.value.Table;
  const headers = TABLE_HEADERS[source] || Array.from({{length: (rows[0]||[]).length}}, (_, i) => 'Col'+(i+1));
  const isProcesses = source === 'processes';

  let html = '<div class="table-view-header">' +
    '<span>' + escapeHtml(source) + ' / ' + escapeHtml(field.name) + '</span>' +
    '<span style="color:var(--fg-dim);font-size:11px">(' + rows.length + ' rows)</span>' +
    (isProcesses ? '<span style="color:var(--cyan);font-size:11px">[Enter: detail]</span>' : '') +
    '<button class="table-back-btn" onclick="S.setView(\'detail\')">&#8592; Back (BS)</button>' +
    '</div>';
  html += '<table class="field-table"><thead><tr>';
  headers.forEach(h => {{ html += '<th>' + escapeHtml(h) + '</th>'; }});
  html += '</tr></thead><tbody>';
  rows.forEach((row, i) => {{
    const sel = (isProcesses && i === S.tableViewScroll) ? ' selected' : '';
    html += '<tr class="' + sel + '" data-row="' + i + '">' +
      row.map(c => '<td>' + escapeHtml(c) + '</td>').join('') + '</tr>';
  }});
  html += '</tbody></table>';
  el.innerHTML = html;

  // scroll selected into view
  const selRow = el.querySelector('tr.selected');
  if (selRow) selRow.scrollIntoView({{ block: 'nearest' }});

  // click handlers
  el.querySelectorAll('tr[data-row]').forEach(tr => {{
    tr.onclick = () => {{
      const i = parseInt(tr.dataset.row);
      S.tableViewScroll = i;
      if (isProcesses) {{
        const pid = rows[i] && rows[i][0];
        if (pid) {{ S.detailedPid = pid; S.view = 'processdetail'; render(); return; }}
      }}
      render();
    }};
  }});
}}

async function renderProcessDetail(el) {{
  const pid = S.detailedPid;
  if (!pid) {{ S.view = 'table'; renderContent(); return; }}
  el.innerHTML = '<p style="color:var(--fg-dim)">Loading /proc/' + escapeHtml(pid) + '...</p>';

  try {{
    const resp = await fetch('/api/view?view=processdetail&pid=' + encodeURIComponent(pid) + '&locale=' + S.locale);
    if (!resp.ok) throw new Error('HTTP ' + resp.status);
    const data = await resp.json();
    if (data.type !== 'ProcessDetail') throw new Error('unexpected: ' + data.type);
    const d = data;
    let html = '<div class="table-view-header">' +
      '<span>PID ' + escapeHtml(d.pid) + ' — ' + escapeHtml(d.comm) + '</span>' +
      '<button class="table-back-btn" onclick="S.view=\'table\';render()">&#8592; Back (BS)</button>' +
      '</div>';
    if (d.error) {{
      html += '<p style="color:var(--red)">' + escapeHtml(d.error) + '</p>';
      el.innerHTML = html;
      return;
    }}
    d.fields.forEach(f => {{
      html += '<div class="proc-detail-field">' +
        '<span class="proc-detail-name">' + escapeHtml(f.name) + '</span>' +
        '<span class="proc-detail-value">' + escapeHtml(f.value) + '</span>' +
        '<span class="proc-detail-desc">' + escapeHtml(f.description) + '</span>' +
        '</div>';
      if (f.table_rows && f.table_rows.length > 0) {{
        html += '<table class="proc-detail-subtable"><thead><tr>';
        f.table_headers.forEach(h => {{ html += '<th>' + escapeHtml(h) + '</th>'; }});
        html += '</tr></thead><tbody>';
        f.table_rows.slice(0, 20).forEach(row => {{
          html += '<tr>' + row.map(c => '<td>' + escapeHtml(c) + '</td>').join('') + '</tr>';
        }});
        if (f.table_rows.length > 20) html += '<tr><td colspan="' + f.table_headers.length + '" style="color:var(--fg-dim)">... ' + (f.table_rows.length - 20) + ' more</td></tr>';
        html += '</tbody></table>';
      }}
    }});
    el.innerHTML = html;
  }} catch(e) {{
    el.innerHTML = '<p style="color:var(--red)">Failed to load process detail: ' + escapeHtml(String(e)) + '</p>' +
      '<button class="table-back-btn" onclick="S.view=\'table\';render()">&#8592; Back</button>';
  }}
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

async function fetchArticle(params) {{
  const qs = new URLSearchParams(params);
  qs.set('locale', S.locale);
  const resp = await fetch('/api/article?' + qs.toString());
  if (!resp.ok) return null;
  return await resp.json();
}}

async function openArticleForSelection() {{
  const src = S.currentSourceName();
  const entry = S.snapshot && S.snapshot.entries[src];
  const field = entry && entry.fields[S.selectedField] ? entry.fields[S.selectedField].name : '';
  S.articleOverlay.loading = true;
  S.articleOverlay.open = true;
  S.articleOverlay.selectedLink = 0;
  renderArticleOverlay();

  const article = await fetchArticle({{ source: src, field }});
  if (!article) {{
    closeArticleOverlay();
    return;
  }}
  S.articleOverlay.loading = false;
  S.articleOverlay.article = article;
  S.articleOverlay.selectedLink = 0;
  renderArticleOverlay();
}}

async function openArticleById(id) {{
  if (!id) return;
  S.articleOverlay.loading = true;
  renderArticleOverlay();
  const article = await fetchArticle({{ id }});
  if (!article) {{
    closeArticleOverlay();
    return;
  }}
  S.articleOverlay.loading = false;
  S.articleOverlay.article = article;
  S.articleOverlay.selectedLink = 0;
  renderArticleOverlay();
}}

function closeArticleOverlay() {{
  S.articleOverlay.open = false;
  S.articleOverlay.loading = false;
  S.articleOverlay.article = null;
  S.articleOverlay.selectedLink = 0;
  const el = document.getElementById('article-overlay');
  if (el) el.classList.remove('show');
  renderTopbar();
}}

function jumpToMetric(source, field) {{
  if (!source || !field) return;
  updateSourceKeys();
  const idx = S.filteredKeys.indexOf(source);
  if (idx < 0) return;
  S.view = 'detail';
  S.focus = 'content';
  S.selectedSource = idx;
  S.selectedField = 0;
  const entry = S.snapshot && S.snapshot.entries[source];
  if (entry) {{
    const fidx = entry.fields.findIndex(f => f.name === field);
    if (fidx >= 0) S.selectedField = fidx;
  }}
  closeArticleOverlay();
  render();
}}

function activateArticleLink(link) {{
  if (!link) return;
  if (link.type === 'metric') {{
    jumpToMetric(link.source, link.field);
    return;
  }}
  if (link.type === 'article') {{
    openArticleById(link.id);
  }}
}}

function renderArticleOverlay() {{
  const wrap = document.getElementById('article-overlay');
  if (!wrap) return;
  if (!S.articleOverlay.open) {{
    wrap.classList.remove('show');
    return;
  }}
  wrap.classList.add('show');
  renderTopbar();

  const title = document.getElementById('article-title');
  const meta = document.getElementById('article-meta');
  const body = document.getElementById('article-body');
  const foot = document.getElementById('article-foot');

  if (S.articleOverlay.loading) {{
    title.textContent = S.locale === 'ja' ? '記事を読み込み中...' : 'Loading article...';
    meta.textContent = '';
    body.innerHTML = '<p style="color:var(--fg-dim)">...</p>';
    foot.textContent = '';
    return;
  }}

  const article = S.articleOverlay.article;
  if (!article) {{
    body.innerHTML = '';
    foot.textContent = '';
    return;
  }}

  title.textContent = article.title || 'Article';
  meta.textContent = '[' + (article.kind || '').toLowerCase() + '] ' + (article.id || '');
  let bodyHtml = '';
  if (typeof marked !== 'undefined') {{
    marked.setOptions({{
      gfm: true,
      breaks: true,
    }});
    const rendered = marked.parse(article.body || '');
    if (typeof DOMPurify !== 'undefined') {{
      bodyHtml = DOMPurify.sanitize(rendered);
    }} else {{
      bodyHtml = rendered;
    }}
  }} else {{
    // Fallback: keep text readable with simple linebreak-rich rendering
    const lines = escapeHtml(article.body || '').split('\n').map(l => l || '&nbsp;');
    bodyHtml = '<pre style="margin:0;white-space:pre-wrap">' + lines.join('\n') + '</pre>';
  }}
  let tocHtml = '';
  try {{
    const host = document.createElement('div');
    host.innerHTML = bodyHtml;
    const headings = Array.from(host.querySelectorAll('h2, h3'));
    const used = {{}};
    headings.forEach((h, i) => {{
      const label = (h.textContent || '').trim();
      let base = slugifyHeading(label);
      if (!base) base = 'section-' + (i + 1);
      let id = base;
      let n = 2;
      while (used[id]) {{
        id = base + '-' + n;
        n += 1;
      }}
      used[id] = true;
      h.id = id;
    }});

    const items = headings
      .map(h => {{
        const label = (h.textContent || '').trim();
        if (!label) return '';
        const cls = h.tagName === 'H3' ? 'toc-item l3' : 'toc-item';
        return '<button class="' + cls + '" data-anchor="' + escapeHtml(h.id) + '">' + escapeHtml(label) + '</button>';
      }})
      .filter(Boolean);
    bodyHtml = host.innerHTML;
    if (items.length > 0) {{
      const tocTitle = S.locale === 'ja' ? '目次' : 'Contents';
      tocHtml = '<div class="article-toc"><div class="toc-title">' + tocTitle + '</div>' + items.join('') + '</div>';
    }}
  }} catch (_e) {{
    // no-op
  }}

  let html = tocHtml + '<div class="md-content">' + bodyHtml + '</div>';
  const links = Array.isArray(article.links) ? article.links : [];
  if (links.length > 0) {{
    html += '<h4>SEE ALSO</h4><div class="links">';
    links.forEach((link, idx) => {{
      const active = idx === S.articleOverlay.selectedLink ? ' active' : '';
      html += '<div class="link' + active + '" data-link-idx="' + idx + '">' +
        escapeHtml(link.label || '') + '</div>';
    }});
    html += '</div>';
  }}
  body.innerHTML = html;
  body.querySelectorAll('[data-anchor]').forEach(node => {{
    node.onclick = () => {{
      const id = node.dataset.anchor;
      if (!id) return;
      const target = document.getElementById(id);
      if (!target) return;
      target.scrollIntoView({{ behavior: 'smooth', block: 'start' }});
    }};
  }});
  body.querySelectorAll('[data-link-idx]').forEach(node => {{
    node.onclick = () => {{
      const idx = parseInt(node.dataset.linkIdx, 10);
      S.articleOverlay.selectedLink = idx;
      renderArticleOverlay();
      activateArticleLink(links[idx]);
    }};
  }});

  foot.textContent = S.locale === 'ja'
    ? 'j/k: スクロール  PgUp/PgDn: ページ  Tab/Shift+Tab: 関連項目  Enter: 移動  A/Esc/q: 閉じる'
    : 'j/k: scroll  PgUp/PgDn: page  Tab/Shift+Tab: related  Enter: jump  A/Esc/q: close';
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
  renderGraphWinBtns();

  // Use windowed data
  const src = S.currentSourceName();
  if (S.graphField) S.rebuildFieldGraphData(src, S.graphField);

  // Stable y-axis from all-time min/max
  const stateKey = src + '.' + S.graphField;
  const allVals = S.graphData.map(d => d.v);
  if (allVals.length > 0) {{
    const prevMin = S.graphAllTimeMin[stateKey] ?? Infinity;
    const prevMax = S.graphAllTimeMax[stateKey] ?? -Infinity;
    S.graphAllTimeMin[stateKey] = Math.min(prevMin, Math.min(...allVals));
    S.graphAllTimeMax[stateKey] = Math.max(prevMax, Math.max(...allVals));
  }}
  const yMin = S.graphAllTimeMin[stateKey] ?? 0;
  const yMax = S.graphAllTimeMax[stateKey] ?? 100;
  const pad = (yMax - yMin) * 0.05 || 1;

  if (S.chart) S.chart.destroy();
  S.chart = new Chart(canvas.getContext('2d'), {{
    type: 'line',
    data: {{
      labels: S.graphData.map(d => d.t),
      datasets: [{{ label: S.graphField, data: S.graphData.map(d => d.v), borderColor: '#7aa2f7', backgroundColor: 'rgba(122,162,247,0.1)', fill: true, tension: 0.3, pointRadius: 1 }}]
    }},
    options: {{
      responsive: true, maintainAspectRatio: false,
      scales: {{
        x: {{ ticks: {{ color: '#565f89', maxTicksLimit: 10 }} }},
        y: {{ min: yMin - pad, max: yMax + pad, ticks: {{ color: '#565f89' }} }}
      }},
      plugins: {{ legend: {{ labels: {{ color: '#c0caf5' }} }} }},
      animation: {{ duration: 0 }}
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

  // Article overlay keys (highest priority while open)
  if (S.articleOverlay.open) {{
    const body = document.getElementById('article-body');
    const article = S.articleOverlay.article;
    const links = article && Array.isArray(article.links) ? article.links : [];
    if (key === 'Escape' || key === 'q' || key === 'A') {{
      e.preventDefault();
      closeArticleOverlay();
      return;
    }}
    if (key === 'j' || key === 'ArrowDown') {{
      e.preventDefault();
      if (body) body.scrollTop += 28;
      return;
    }}
    if (key === 'k' || key === 'ArrowUp') {{
      e.preventDefault();
      if (body) body.scrollTop -= 28;
      return;
    }}
    if (key === 'PageDown') {{
      e.preventDefault();
      if (body) body.scrollTop += Math.max(120, Math.floor(body.clientHeight * 0.8));
      return;
    }}
    if (key === 'PageUp') {{
      e.preventDefault();
      if (body) body.scrollTop -= Math.max(120, Math.floor(body.clientHeight * 0.8));
      return;
    }}
    if (key === 'Tab') {{
      e.preventDefault();
      if (links.length > 0) {{
        const delta = e.shiftKey ? -1 : 1;
        let next = S.articleOverlay.selectedLink + delta;
        if (next < 0) next = links.length - 1;
        if (next >= links.length) next = 0;
        S.articleOverlay.selectedLink = next;
        renderArticleOverlay();
      }}
      return;
    }}
    if (key === 'Enter') {{
      e.preventDefault();
      if (links.length > 0) activateArticleLink(links[S.articleOverlay.selectedLink]);
      return;
    }}
    return;
  }}

  // Global keys
  if (key === '/') {{ e.preventDefault(); S.searching = true; searchBox.focus(); return; }}
  if (key === 'A') {{ e.preventDefault(); S.toggleArticleForSelection(); return; }}
  if (key === 'D') {{ e.preventDefault(); S.setView('dashboard'); return; }}
  if (key === 'O') {{ e.preventDefault(); S.setView('classic'); return; }}
  if (key === 'W') {{ e.preventDefault(); S.setView('welcome'); return; }}
  if (key === 'X') {{ e.preventDefault(); S.setView('diagnostics'); return; }}
  if (key === 'C') {{ e.preventDefault(); S.setView('category'); return; }}
  if (key === 'r' || key === 'R') {{ e.preventDefault(); S.toggleAutoRefresh(); return; }}
  if (key === 's' || key === 'S') {{ e.preventDefault(); S.toggleAxisMode(); return; }}
  if (key === '?') {{ e.preventDefault(); S.cycleHelp(); return; }}
  if (key === 'L') {{ e.preventDefault(); S.toggleLang(); return; }}
  if (key === 'd') {{ e.preventDefault(); S.setView('diff'); return; }}
  if (key === 't') {{ e.preventDefault(); S.toggleSidebarTree(); return; }}
  if (key === '[') {{ e.preventDefault(); S.cycleTimeWindow(-1); return; }}
  if (key === ']') {{ e.preventDefault(); S.cycleTimeWindow(1); return; }}
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
    if (S.view === 'processdetail') {{ S.detailedPid = null; S.view = 'table'; render(); return; }}
    if (S.view === 'table') {{ S.tableViewSource = null; S.tableViewFieldIdx = null; S.view = 'detail'; render(); return; }}
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
  const inSidebar = S.focus === 'sidebar' && (S.view === 'detail' || S.view === 'diff' || S.view === 'table' || S.view === 'processdetail');
  if (key === 'j' || key === 'ArrowDown') {{
    e.preventDefault();
    if (S.view === 'category') {{
      S.selectedCategory = Math.min(S.selectedCategory + 1, 99);
      S.categoryScroll = 0;
      render();
    }} else if (S.view === 'table' && !inSidebar) {{
      const entry = S.snapshot && S.snapshot.entries[S.tableViewSource];
      const field = entry && entry.fields[S.tableViewFieldIdx||0];
      const rowCount = (field && field.value.Table) ? field.value.Table.length : 0;
      S.tableViewScroll = Math.min(S.tableViewScroll + 1, rowCount - 1);
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
    }} else if (S.view === 'table' && !inSidebar) {{
      S.tableViewScroll = Math.max(S.tableViewScroll - 1, 0);
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
    }} else if (S.view === 'table') {{
      // Enter on processes row → drill into ProcessDetail
      if (S.tableViewSource === 'processes') {{
        const entry = S.snapshot && S.snapshot.entries['processes'];
        if (entry) {{
          const field = entry.fields[S.tableViewFieldIdx||0];
          if (field && field.value.Table) {{
            const row = field.value.Table[S.tableViewScroll];
            if (row && row[0]) {{ S.detailedPid = row[0]; S.view = 'processdetail'; render(); }}
          }}
        }}
      }}
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
async fn settings_page_handler(State(state): State<Arc<AppState>>) -> Html<String> {
    let lang = match state.locale {
        Locale::Ja => "ja",
        Locale::En => "en",
    };
    Html(build_settings_html(lang))
}

/// GET /api/v1/settings — returns current config as JSON
#[cfg(feature = "web")]
async fn settings_api_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let rules = state.alert_rules.lock().unwrap().clone();

    #[derive(serde::Serialize)]
    struct SettingsResponse {
        alert_rules: Vec<crate::alert::AlertRule>,
        history: HistoryInfo,
        diagnostic_runbooks: Vec<crate::config::RunbookConfig>,
    }
    #[derive(serde::Serialize)]
    struct HistoryInfo {
        enabled: bool,
        interval_secs: u64,
        retention_days: u32,
    }

    Json(SettingsResponse {
        alert_rules: rules,
        history: HistoryInfo {
            enabled: state.history_config.enabled,
            interval_secs: state.history_config.interval_secs,
            retention_days: state.history_config.retention_days,
        },
        diagnostic_runbooks: state.diagnostic_runbooks.clone(),
    })
}

/// POST /api/v1/settings/alerts — accepts JSON array of alert rules, writes to config file
#[cfg(feature = "web")]
async fn settings_alerts_handler(
    State(state): State<Arc<AppState>>,
    Json(new_rules): Json<Vec<crate::alert::AlertRule>>,
) -> impl IntoResponse {
    // Update in-memory state
    {
        let mut rules = state.alert_rules.lock().unwrap();
        *rules = new_rules.clone();
    }

    // Write to config file
    if let Some(ref path) = state.config_path {
        // Read existing TOML, replace [[alert]] section, write back
        let contents = std::fs::read_to_string(path).unwrap_or_default();
        let updated = replace_alert_section_in_toml(&contents, &new_rules);

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        if let Err(e) = std::fs::write(path, &updated) {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("Failed to write config: {}", e)})),
            )
                .into_response();
        }
    }

    Json(serde_json::json!({"alert_rules": new_rules})).into_response()
}

/// Replace the [[alert]] sections in TOML with the new rules.
/// Preserves all other config sections.
#[cfg(feature = "web")]
fn replace_alert_section_in_toml(existing: &str, rules: &[crate::alert::AlertRule]) -> String {
    // Remove existing [[alert]] blocks
    let mut result = String::new();
    let mut skip = false;
    for line in existing.lines() {
        let trimmed = line.trim();
        if trimmed == "[[alert]]" {
            skip = true;
            continue;
        }
        // A new section header ends the skip
        if skip && trimmed.starts_with('[') {
            skip = false;
        }
        if skip {
            // Skip key=value lines and blank lines within an [[alert]] block
            if trimmed.is_empty() || trimmed.contains('=') {
                continue;
            }
            // Non key=value, non-empty, non-section line — stop skipping
            skip = false;
        }
        result.push_str(line);
        result.push('\n');
    }

    // Append new [[alert]] blocks
    if !rules.is_empty() {
        if !result.ends_with('\n') {
            result.push('\n');
        }
        for rule in rules {
            result.push_str("\n[[alert]]\n");
            result.push_str(&format!("source = {:?}\n", rule.source));
            result.push_str(&format!("field = {:?}\n", rule.field));
            result.push_str(&format!("condition = {:?}\n", rule.condition));
            result.push_str(&format!("severity = {:?}\n", rule.severity));
            result.push_str(&format!("message = {:?}\n", rule.message));
            if let Some(ref action) = rule.action {
                result.push_str(&format!("action = {:?}\n", action));
            }
            if !rule.notify.is_empty() {
                let notify_strs: Vec<String> =
                    rule.notify.iter().map(|n| format!("{:?}", n)).collect();
                result.push_str(&format!("notify = [{}]\n", notify_strs.join(", ")));
            }
        }
    }

    result
}

/// Build the HTML for the settings page (G20-15).
#[cfg(feature = "web")]
fn build_settings_html(lang: &str) -> String {
    let title = if lang == "ja" { "設定" } else { "Settings" };
    let rules_label = if lang == "ja" {
        "アラートルール"
    } else {
        "Alert Rules"
    };
    let add_label = if lang == "ja" {
        "新規ルール追加"
    } else {
        "Add New Rule"
    };
    let save_label = if lang == "ja" { "保存" } else { "Save" };
    let delete_label = if lang == "ja" { "削除" } else { "Delete" };
    let back_label = if lang == "ja" {
        "ダッシュボードに戻る"
    } else {
        "Back to Dashboard"
    };

    format!(
        r##"<!DOCTYPE html>
<html lang="{lang}">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>syslenz - {title}</title>
<style>
*{{margin:0;padding:0;box-sizing:border-box}}
:root{{
  --bg:#1a1b26;--bg-dark:#16161e;--bg-hl:#1f2335;--bg-sel:#292e42;
  --fg:#c0caf5;--fg-dim:#565f89;--border:#3b4261;
  --blue:#7aa2f7;--green:#9ece6a;--yellow:#e0af68;--red:#f7768e;
  --cyan:#7dcfff;--purple:#bb9af7;--orange:#ff9e64;
}}
html,body{{height:100%}}
body{{font-family:'Consolas','Monaco','Fira Code',monospace;background:var(--bg);color:var(--fg);padding:20px}}
h1{{color:var(--blue);font-size:20px;margin-bottom:16px;display:flex;align-items:center;gap:12px}}
h2{{color:var(--purple);font-size:15px;margin:20px 0 10px;text-transform:uppercase;letter-spacing:.5px}}
a{{color:var(--blue);text-decoration:none}}
a:hover{{text-decoration:underline}}
.container{{max-width:1100px;margin:0 auto}}

/* Table */
.rules-table{{width:100%;border-collapse:collapse;font-size:13px;margin-bottom:20px}}
.rules-table th{{text-align:left;padding:8px;color:var(--purple);border-bottom:2px solid var(--border);font-size:11px;text-transform:uppercase;letter-spacing:.5px}}
.rules-table td{{padding:6px 8px;border-bottom:1px solid var(--bg-sel);vertical-align:top}}
.rules-table tr:hover{{background:var(--bg-hl)}}
.sev-critical{{color:var(--red);font-weight:bold}}
.sev-warning{{color:var(--yellow)}}
.sev-info{{color:var(--cyan)}}

/* Form */
.form-card{{background:var(--bg-dark);border:1px solid var(--border);border-radius:8px;padding:16px;margin-bottom:20px}}
.form-row{{display:flex;gap:10px;margin-bottom:10px;flex-wrap:wrap;align-items:end}}
.form-group{{display:flex;flex-direction:column;gap:4px}}
.form-group label{{font-size:11px;color:var(--fg-dim);text-transform:uppercase;letter-spacing:.5px}}
.form-group input,.form-group select{{
  background:var(--bg);border:1px solid var(--border);color:var(--fg);
  padding:6px 10px;border-radius:4px;font-family:inherit;font-size:12px;
  outline:none;min-width:120px;
}}
.form-group input:focus,.form-group select:focus{{border-color:var(--blue)}}

/* Buttons */
.btn{{
  padding:6px 16px;border:1px solid var(--border);border-radius:4px;
  font-family:inherit;font-size:12px;cursor:pointer;
  background:var(--bg-sel);color:var(--fg);
}}
.btn:hover{{background:var(--border)}}
.btn-primary{{background:var(--blue);color:var(--bg-dark);border-color:var(--blue);font-weight:bold}}
.btn-primary:hover{{opacity:0.85}}
.btn-danger{{background:transparent;color:var(--red);border-color:var(--red)}}
.btn-danger:hover{{background:var(--red);color:var(--bg-dark)}}
.btn-sm{{padding:3px 10px;font-size:11px}}

/* Status */
.status-msg{{padding:8px 12px;border-radius:6px;font-size:12px;margin-bottom:12px;display:none}}
.status-msg.success{{display:block;background:rgba(158,206,106,0.15);color:var(--green);border:1px solid var(--green)}}
.status-msg.error{{display:block;background:rgba(247,118,142,0.15);color:var(--red);border:1px solid var(--red)}}

.empty-state{{color:var(--fg-dim);font-size:13px;padding:20px;text-align:center}}
</style>
</head>
<body>
<div class="container">
  <h1>
    <a href="/" title="{back_label}">syslenz</a>
    <span style="color:var(--fg-dim);font-size:14px">/</span>
    <span>{title}</span>
  </h1>

  <div id="status-msg" class="status-msg"></div>

  <h2>{rules_label}</h2>
  <table class="rules-table" id="rules-table">
    <thead>
      <tr>
        <th>Source</th><th>Field</th><th>Condition</th><th>Severity</th>
        <th>Message</th><th>Notify</th><th>Action</th><th></th>
      </tr>
    </thead>
    <tbody id="rules-tbody">
      <tr><td colspan="8" class="empty-state">Loading...</td></tr>
    </tbody>
  </table>

  <h2>{add_label}</h2>
  <div class="form-card">
    <div class="form-row">
      <div class="form-group">
        <label>Source</label>
        <input type="text" id="f-source" placeholder="meminfo">
      </div>
      <div class="form-group">
        <label>Field</label>
        <input type="text" id="f-field" placeholder="MemAvailable">
      </div>
      <div class="form-group">
        <label>Condition</label>
        <input type="text" id="f-condition" placeholder="< 500000000">
      </div>
      <div class="form-group">
        <label>Severity</label>
        <select id="f-severity">
          <option value="info">info</option>
          <option value="warning" selected>warning</option>
          <option value="critical">critical</option>
        </select>
      </div>
    </div>
    <div class="form-row">
      <div class="form-group" style="flex:1">
        <label>Message</label>
        <input type="text" id="f-message" placeholder="Low memory available" style="width:100%">
      </div>
    </div>
    <div class="form-row">
      <div class="form-group" style="flex:1">
        <label>Notify URLs (comma-separated, e.g. slack:https://..., webhook:https://...)</label>
        <input type="text" id="f-notify" placeholder="slack:https://hooks.slack.com/..." style="width:100%">
      </div>
    </div>
    <div class="form-row">
      <div class="form-group" style="flex:1">
        <label>Action (optional shell command)</label>
        <input type="text" id="f-action" placeholder="notify-send 'syslenz: {{message}}'" style="width:100%">
      </div>
    </div>
    <div class="form-row" style="margin-top:8px">
      <button class="btn btn-primary" onclick="addRule()">{add_label}</button>
    </div>
  </div>

  <div style="display:flex;gap:10px;margin-top:16px">
    <button class="btn btn-primary" onclick="saveRules()">{save_label}</button>
    <a href="/" class="btn">{back_label}</a>
  </div>
</div>

<script>
let rules = [];

async function loadSettings() {{
  try {{
    const resp = await fetch('/api/v1/settings');
    const data = await resp.json();
    rules = data.alert_rules || [];
    renderRules();
  }} catch(e) {{
    showStatus('Failed to load settings: ' + e.message, true);
  }}
}}

function renderRules() {{
  const tbody = document.getElementById('rules-tbody');
  if (rules.length === 0) {{
    tbody.innerHTML = '<tr><td colspan="8" class="empty-state">No alert rules configured</td></tr>';
    return;
  }}
  let html = '';
  rules.forEach((r, i) => {{
    const sevClass = 'sev-' + r.severity;
    const notify = (r.notify || []).join(', ');
    const action = r.action || '';
    html += '<tr>'
      + '<td>' + esc(r.source) + '</td>'
      + '<td>' + esc(r.field) + '</td>'
      + '<td><code>' + esc(r.condition) + '</code></td>'
      + '<td class="' + sevClass + '">' + esc(r.severity) + '</td>'
      + '<td>' + esc(r.message) + '</td>'
      + '<td style="font-size:11px;max-width:200px;overflow:hidden;text-overflow:ellipsis">' + esc(notify) + '</td>'
      + '<td style="font-size:11px;max-width:150px;overflow:hidden;text-overflow:ellipsis">' + esc(action) + '</td>'
      + '<td><button class="btn btn-danger btn-sm" onclick="deleteRule(' + i + ')">{delete_label}</button></td>'
      + '</tr>';
  }});
  tbody.innerHTML = html;
}}

function addRule() {{
  const source = document.getElementById('f-source').value.trim();
  const field = document.getElementById('f-field').value.trim();
  const condition = document.getElementById('f-condition').value.trim();
  const severity = document.getElementById('f-severity').value;
  const message = document.getElementById('f-message').value.trim();
  const notifyRaw = document.getElementById('f-notify').value.trim();
  const action = document.getElementById('f-action').value.trim();

  if (!source || !field || !condition || !message) {{
    showStatus('Source, Field, Condition, and Message are required.', true);
    return;
  }}

  const notify = notifyRaw ? notifyRaw.split(',').map(s => s.trim()).filter(Boolean) : [];
  const rule = {{ source, field, condition, severity, message, notify }};
  if (action) rule.action = action;

  rules.push(rule);
  renderRules();
  // Clear form
  ['f-source','f-field','f-condition','f-message','f-notify','f-action'].forEach(id => document.getElementById(id).value = '');
  showStatus('Rule added (not yet saved to file).', false);
}}

function deleteRule(idx) {{
  rules.splice(idx, 1);
  renderRules();
  showStatus('Rule removed (not yet saved to file).', false);
}}

async function saveRules() {{
  try {{
    const resp = await fetch('/api/v1/settings/alerts', {{
      method: 'POST',
      headers: {{ 'Content-Type': 'application/json' }},
      body: JSON.stringify(rules),
    }});
    if (!resp.ok) {{
      const err = await resp.json().catch(() => ({{}}));
      throw new Error(err.error || 'HTTP ' + resp.status);
    }}
    const data = await resp.json();
    rules = data.alert_rules || rules;
    renderRules();
    showStatus('Settings saved to config file.', false);
  }} catch(e) {{
    showStatus('Failed to save: ' + e.message, true);
  }}
}}

function showStatus(msg, isError) {{
  const el = document.getElementById('status-msg');
  el.textContent = msg;
  el.className = 'status-msg ' + (isError ? 'error' : 'success');
  setTimeout(() => {{ el.className = 'status-msg'; }}, 5000);
}}

function esc(s) {{
  if (typeof s !== 'string') return String(s);
  return s.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;').replace(/"/g,'&quot;');
}}

loadSettings();
</script>
</body>
</html>"##,
        lang = lang,
        title = title,
        rules_label = rules_label,
        add_label = add_label,
        save_label = save_label,
        delete_label = delete_label,
        back_label = back_label,
    )
}

/// GET /api/diagnostics — Phase 2: HTTP endpoint for diagnostics (was TUI-internal only)
#[cfg(feature = "web")]
async fn diagnostics_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let snapshot = state.current.lock().unwrap().clone();
    let findings = crate::diagnostics::analyze(&snapshot, state.locale, &state.diagnostic_runbooks);
    Json(findings)
}

#[cfg(all(test, feature = "web"))]
mod tests {
    use super::*;
    use crate::proc::{FieldValue, ProcEntry};
    use std::time::SystemTime;

    #[test]
    fn web_bind_port_preserves_legacy_all_interfaces_default() {
        assert_eq!(normalize_web_bind("3000"), "0.0.0.0:3000");
    }

    #[test]
    fn web_bind_explicit_address_is_preserved() {
        assert_eq!(normalize_web_bind("127.0.0.1:3000"), "127.0.0.1:3000");
        assert_eq!(normalize_web_bind("[::1]:3000"), "[::1]:3000");
    }

    fn make_snapshot_with_table(rows: usize) -> crate::proc::Snapshot {
        use std::collections::BTreeMap;
        let table_rows: Vec<Vec<String>> = (0..rows)
            .map(|i| vec![format!("pid_{}", i), format!("proc_{}", i), format!("{}", i)])
            .collect();
        let entry = ProcEntry {
            source: "processes".to_string(),
            fields: vec![crate::proc::Field {
                name: "process_list".to_string(),
                value: FieldValue::Table(table_rows),
                unit: None,
                description: "Process list".to_string(),
            }],
        };
        let mut entries = BTreeMap::new();
        entries.insert("processes".to_string(), entry);
        crate::proc::Snapshot {
            timestamp: SystemTime::now(),
            entries,
            alerts: Vec::new(),
        }
    }

    #[test]
    fn truncate_reduces_large_tables() {
        let mut snap = make_snapshot_with_table(100);
        truncate_snapshot_tables(&mut snap, 20);
        let entry = snap.entries.get("processes").unwrap();
        if let FieldValue::Table(rows) = &entry.fields[0].value {
            // 20 rows + 1 truncated marker = 21
            assert_eq!(rows.len(), 21, "should have 20 kept + 1 truncated marker");
            assert!(
                rows[20][0].contains("[truncated: 80 rows]"),
                "last row should be truncation marker, got: {}",
                rows[20][0]
            );
        } else {
            panic!("expected Table");
        }
    }

    #[test]
    fn truncate_leaves_small_tables_alone() {
        let mut snap = make_snapshot_with_table(10);
        truncate_snapshot_tables(&mut snap, 20);
        let entry = snap.entries.get("processes").unwrap();
        if let FieldValue::Table(rows) = &entry.fields[0].value {
            assert_eq!(rows.len(), 10, "small table should be unchanged");
            assert!(
                !rows.iter().any(|r| r[0].contains("[truncated")),
                "no truncation marker expected"
            );
        }
    }

    #[test]
    fn approx_bytes_counts_table_cells() {
        let snap = make_snapshot_with_table(100);
        let bytes = approx_snapshot_bytes(&snap);
        // 100 rows × ~3 cells × ~10 chars = ~3000+ bytes minimum
        assert!(
            bytes > 1000,
            "approx bytes should account for table cells, got {}",
            bytes
        );
    }

    /// 回帰テスト: 履歴がバイト数上限で頭打ちになることを確認。
    /// 1 件が 5KB のスナップショットを 100 件積み、上限を 20KB に設定した場合、
    /// 保持される件数が 20KB / 5KB = 4 件程度に抑えられることを確認する。
    #[test]
    fn history_respects_byte_limit() {
        let snap = make_snapshot_with_table(100);
        let snap_bytes = approx_snapshot_bytes(&snap);
        assert!(snap_bytes > 1000, "snap should be >1KB, got {}", snap_bytes);

        let max_bytes = snap_bytes * 4; // 4 件分程度
        let mut history: Vec<crate::proc::Snapshot> = Vec::new();
        for _ in 0..100 {
            let s = snap.clone();
            // 履歴用に縮約せずそのまま積む（テスト用）
            history.push(s.clone());
            let max_count = 60;
            while history.len() > max_count {
                history.remove(0);
            }
            // バイト数上限
            if max_bytes > 0 {
                let mut total: usize = history.iter().map(approx_snapshot_bytes).sum();
                while total > max_bytes && history.len() > 1 {
                    let removed = history.remove(0);
                    total = total.saturating_sub(approx_snapshot_bytes(&removed));
                }
            }
        }
        let total_bytes: usize = history.iter().map(approx_snapshot_bytes).sum();
        assert!(
            total_bytes <= max_bytes,
            "total {} should be <= max {}",
            total_bytes,
            max_bytes
        );
        assert!(
            history.len() <= 5,
            "history len {} should be small (around 4)",
            history.len()
        );
    }

    /// 回帰テスト: 件数上限がバイト数上限より小さい場合は件数で抑えられる。
    #[test]
    fn history_respects_count_limit() {
        let snap = make_snapshot_with_table(10); // small table
        let max_count = 30;
        let max_bytes = 1_000_000_000; // 1 GB — 実質無効
        let mut history: Vec<crate::proc::Snapshot> = Vec::new();
        for _ in 0..100 {
            history.push(snap.clone());
            while history.len() > max_count {
                history.remove(0);
            }
            if max_bytes > 0 {
                let mut total: usize = history.iter().map(approx_snapshot_bytes).sum();
                while total > max_bytes && history.len() > 1 {
                    let removed = history.remove(0);
                    total = total.saturating_sub(approx_snapshot_bytes(&removed));
                }
            }
        }
        assert_eq!(
            history.len(),
            max_count,
            "should be capped at count limit {}",
            max_count
        );
    }

    /// 回帰テスト: truncate_snapshot_tables を通した履歴は
    /// フルサイズより小さくなる。
    #[test]
    fn truncated_history_is_smaller() {
        let snap = make_snapshot_with_table(500);
        let full_bytes = approx_snapshot_bytes(&snap);
        let mut truncated = snap.clone();
        truncate_snapshot_tables(&mut truncated, 20);
        let truncated_bytes = approx_snapshot_bytes(&truncated);
        assert!(
            truncated_bytes < full_bytes,
            "truncated {} should be < full {}",
            truncated_bytes,
            full_bytes
        );
        // 500 rows → 21 rows なので大幅に小さいはず
        assert!(
            truncated_bytes < full_bytes / 5,
            "truncated {} should be much smaller than full {}",
            truncated_bytes,
            full_bytes
        );
    }
}
