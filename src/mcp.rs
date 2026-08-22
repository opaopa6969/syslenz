//! MCP (Model Context Protocol) server — Streamable HTTP over axum.
//!
//! Implements JSON-RPC 2.0 over HTTP POST to `/mcp`, wrapping the existing
//! syslenz HTTP API as MCP tools. Also provides resources (`syslenz://spec`,
//! `syslenz://guide`, etc.) and a prompt.
//!
//! This module is gated by the `web` feature (same as the rest of the axum server).

#[cfg(feature = "web")]
use crate::web::AppState;
#[cfg(feature = "web")]
use axum::{
    Json,
    http::{StatusCode, header::CONTENT_ENCODING, HeaderName},
    response::IntoResponse,
};
#[cfg(feature = "web")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "web")]
use serde_json::{Value, json};
#[cfg(feature = "web")]
use std::sync::Arc;

#[cfg(feature = "web")]
const NAMESPACE: &str = "syslenz";
#[cfg(feature = "web")]
const VERSION: &str = env!("CARGO_PKG_VERSION");

// ---------------------------------------------------------------------------
// JSON-RPC types
// ---------------------------------------------------------------------------

#[cfg(feature = "web")]
#[derive(Debug, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    #[serde(default)]
    pub params: Value,
}

#[cfg(feature = "web")]
#[derive(Debug, Serialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

#[cfg(feature = "web")]
#[derive(Debug, Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[cfg(feature = "web")]
impl JsonRpcResponse {
    fn ok(id: Option<Value>, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id,
            result: Some(result),
            error: None,
        }
    }

    fn err(id: Option<Value>, code: i32, message: &str, data: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id,
            result: None,
            error: Some(JsonRpcError {
                code,
                message: message.into(),
                data,
            }),
        }
    }
}

// ---------------------------------------------------------------------------
// Tool definitions
// ---------------------------------------------------------------------------

#[cfg(feature = "web")]
fn tool_definitions() -> Vec<Value> {
    vec![
        json!({
            "name": "snapshot",
            "description": "現在のシステムスナップショットを取得する（50+ ソース、構造化・型付き JSON）",
            "inputSchema": { "type": "object", "properties": {} },
            "annotations": { "readOnlyHint": true, "openWorldHint": false }
        }),
        json!({
            "name": "history",
            "description": "スナップショット履歴（最大60件、1秒間隔）を取得する",
            "inputSchema": { "type": "object", "properties": {} },
            "annotations": { "readOnlyHint": true, "openWorldHint": false }
        }),
        json!({
            "name": "sources",
            "description": "利用可能なデータソース一覧を取得する（50+ source names）",
            "inputSchema": { "type": "object", "properties": {} },
            "annotations": { "readOnlyHint": true, "openWorldHint": false }
        }),
        json!({
            "name": "view",
            "description": "指定ビューの描画済みデータを取得する（TUI と同等の描画済みデータ）",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "view": { "type": "string", "description": "welcome/detail/diff/table/graph/diagnostics 等" },
                    "locale": { "type": "string", "enum": ["en", "ja"] },
                    "source": { "type": "string", "description": "source 名（detail ビューで選択）" },
                    "field": { "type": "string", "description": "field 名" },
                    "pid": { "type": "string", "description": "PID（ProcessDetail ビュー）" }
                }
            },
            "annotations": { "readOnlyHint": true, "openWorldHint": false }
        }),
        json!({
            "name": "field_help",
            "description": "フィールドのヘルプ・説明を取得する（normal/detailed/extra 説明、see_also、contextual_hint）",
            "inputSchema": {
                "type": "object",
                "required": ["source", "field"],
                "properties": {
                    "source": { "type": "string" },
                    "field": { "type": "string" },
                    "lang": { "type": "string", "enum": ["en", "ja"] }
                }
            },
            "annotations": { "readOnlyHint": true, "openWorldHint": false }
        }),
        json!({
            "name": "article",
            "description": "メトリクスの教育記事を取得する（691 メトリクス × EN+JA）",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "source": { "type": "string" },
                    "field": { "type": "string" },
                    "id": { "type": "string" },
                    "locale": { "type": "string", "enum": ["en", "ja"] }
                }
            },
            "annotations": { "readOnlyHint": true, "openWorldHint": false }
        }),
        json!({
            "name": "diagnostics",
            "description": "自動診断を実行する（27 チェック、40+ パターン、severity/source/title/detail/suggestion/runbook_url）",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "fresh": { "type": "boolean", "description": "true なら新規キャプチャ（デフォルト false = キャッシュから）" }
                }
            },
            "annotations": { "readOnlyHint": true, "openWorldHint": false }
        }),
        json!({
            "name": "get_settings",
            "description": "現在の設定を取得する（alert_rules, history, diagnostic_runbooks）",
            "inputSchema": { "type": "object", "properties": {} },
            "annotations": { "readOnlyHint": true, "openWorldHint": false }
        }),
        json!({
            "name": "set_alerts",
            "description": "アラートルールを設定する（config.toml に書き込み）。confirm=false なら dry-run（差分プレビューを返す）。confirm=true で実際に書き込む",
            "inputSchema": {
                "type": "object",
                "required": ["rules"],
                "properties": {
                    "rules": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "source": { "type": "string" },
                                "field": { "type": "string" },
                                "condition": { "type": "string" },
                                "severity": { "type": "string", "enum": ["info", "warning", "critical"] },
                                "message": { "type": "string" }
                            }
                        }
                    },
                    "confirm": { "type": "boolean", "description": "true で実際に書き込む。未指定(=false)なら dry-run" }
                }
            },
            "annotations": { "destructiveHint": true, "idempotentHint": true, "openWorldHint": false }
        }),
    ]
}

// ---------------------------------------------------------------------------
// Resource definitions
// ---------------------------------------------------------------------------

#[cfg(feature = "web")]
fn resource_definitions() -> Vec<Value> {
    vec![
        json!({
            "uri": "syslenz://spec",
            "name": "spec",
            "description": "能力仕様（JSON）",
            "mimeType": "application/json"
        }),
        json!({
            "uri": "syslenz://guide",
            "name": "guide",
            "description": "使い方ガイド（Markdown）",
            "mimeType": "text/markdown"
        }),
        json!({
            "uri": "syslenz://sources",
            "name": "sources",
            "description": "データソース一覧と分類",
            "mimeType": "application/json"
        }),
        json!({
            "uri": "syslenz://diagnostics",
            "name": "diagnostics",
            "description": "診断チェック一覧",
            "mimeType": "application/json"
        }),
        json!({
            "uri": "syslenz://metric-kinds",
            "name": "metric-kinds",
            "description": "MetricKind 8種の分類体系",
            "mimeType": "application/json"
        }),
    ]
}

// ---------------------------------------------------------------------------
// Prompt definitions
// ---------------------------------------------------------------------------

#[cfg(feature = "web")]
fn prompt_definitions() -> Vec<Value> {
    vec![json!({
        "name": "monitor-and-diagnose",
        "description": "スナップショット取得 → 診断実行 → 結果解釈のプロンプトテンプレート",
        "arguments": []
    })]
}

// ---------------------------------------------------------------------------
// spec resource builder
// ---------------------------------------------------------------------------

#[cfg(feature = "web")]
fn build_spec() -> Value {
    json!({
        "namespace": NAMESPACE,
        "name": "syslenz",
        "version": VERSION,
        "summary": "Wireshark for /proc — 50+ ソースの構造化・型付き JSON、27 診断チェック、691 メトリクス教育記事",
        "capabilities": tool_definitions().iter().map(|t| json!({
            "kind": "tool",
            "name": t["name"],
            "summary": t["description"],
            "input": t["inputSchema"],
            "output": "JSON",
            "side_effect": if t["name"] == "set_alerts" { "write" } else { "read" },
            "long_running": false,
            "dry_run": t["name"] == "set_alerts",
            "min_role": "MEMBER"
        })).collect::<Vec<_>>(),
        "compositions": [
            { "title": "異常検知フロー", "flow": ["syslenz__snapshot", "syslenz__diagnostics", "index__agent_send"], "note": "snapshot → diagnostics → 通知" },
            { "title": "ヘルス突合せ", "flow": ["syslenz__snapshot", "volta__svc_health"], "note": "負荷メトリクスと他サービスのヘルスを相関分析" },
            { "title": "アラート自動生成", "flow": ["syslenz__snapshot", "syslenz__set_alerts"], "note": "LLM 解析からアラートルールを自動生成" }
        ],
        "depends_on": [
            { "namespace": "index", "capability": "index__agent_send" }
        ],
        "health": "/healthz",
        "docs": ["syslenz://guide"]
    })
}

#[cfg(feature = "web")]
fn build_guide() -> String {
    format!(
        r#"# syslenz MCP ガイド

## namespace
`syslenz`

## tools
- `snapshot` — 現在のシステムスナップショット（50+ ソース、構造化 JSON）
- `history` — スナップショット履歴（最大60件）
- `sources` — データソース一覧
- `view` — 指定ビューの描画済みデータ
- `field_help` — フィールドのヘルプ・説明
- `article` — メトリクスの教育記事（691 × EN+JA）
- `diagnostics` — 自動診断（27 checks, 40+ patterns）
- `get_settings` — 現在の設定
- `set_alerts` — アラートルール設定（confirm 必須、dry-run 既定）

## resources
- `syslenz://spec` — 能力仕様
- `syslenz://guide` — このガイド
- `syslenz://sources` — データソース一覧
- `syslenz://diagnostics` — 診断チェック一覧
- `syslenz://metric-kinds` — MetricKind 分類

## 組み合わせ例
1. `syslenz__snapshot` → `syslenz__diagnostics` → `index__agent_send`
2. `syslenz__snapshot` → `volta__svc_health`
3. `syslenz__snapshot` → LLM 解析 → `syslenz__set_alerts`

## 起動
```
syslenz --web <port> --lang en
```
MCP エンドポイント: `http://<host>:<port>/mcp` (Streamable HTTP, JSON-RPC 2.0)

## バージョン
{VERSION}
"#,
        VERSION = VERSION
    )
}

// ---------------------------------------------------------------------------
// Tool call dispatcher
// ---------------------------------------------------------------------------

#[cfg(feature = "web")]
fn call_tool(
    name: &str,
    args: &Value,
    state: &AppState,
) -> Result<Value, (i32, String, Option<Value>)> {
    match name {
        "snapshot" => {
            let snapshot = state.current.lock().unwrap().clone();
            let json_str = serde_json::to_string(&snapshot)
                .map_err(|e| (-32603, format!("serialize error: {e}"), None))?;
            let val: Value = serde_json::from_str(&json_str)
                .map_err(|e| (-32603, format!("re-parse error: {e}"), None))?;
            Ok(json!({
                "content": [{ "type": "text", "text": json_str }],
                "structuredContent": val
            }))
        }
        "history" => {
            let history = state.history.lock().unwrap().clone();
            let json_str = serde_json::to_string(&history)
                .map_err(|e| (-32603, format!("serialize error: {e}"), None))?;
            let val: Value = serde_json::from_str(&json_str)
                .map_err(|e| (-32603, format!("re-parse error: {e}"), None))?;
            Ok(json!({
                "content": [{ "type": "text", "text": json_str }],
                "structuredContent": val
            }))
        }
        "sources" => {
            let snapshot = state.current.lock().unwrap().clone();
            let sources: Vec<String> = snapshot.entries.keys().cloned().collect();
            let val = json!(sources);
            Ok(json!({
                "content": [{ "type": "text", "text": serde_json::to_string(&sources).unwrap_or_default() }],
                "structuredContent": val
            }))
        }
        "view" => {
            let locale = args
                .get("locale")
                .and_then(|v| v.as_str())
                .map(crate::i18n::Locale::from_str)
                .unwrap_or(state.locale);
            let snapshot = state.current.lock().unwrap().clone();
            let history = state.history.lock().unwrap().clone();

            let mut snapshots = history.clone();
            snapshots.push(snapshot.clone());

            let app = crate::ui::app::App::from_imported(snapshots)
                .map_err(|e| (-32603, format!("App creation error: {e}"), None))?;
            let mut app = app;
            app.locale = locale;

            let view = if args.get("pid").and_then(|v| v.as_str()).is_some() {
                crate::ui::app::View::ProcessDetail
            } else if args.get("source").and_then(|v| v.as_str()).is_some() {
                crate::ui::app::View::Detail
            } else {
                match args.get("view").and_then(|v| v.as_str()) {
                    Some("welcome") => crate::ui::app::View::Welcome,
                    Some("detail") => crate::ui::app::View::Detail,
                    Some("diff") => crate::ui::app::View::Diff,
                    Some("table") => crate::ui::app::View::TableView,
                    Some("graph") => crate::ui::app::View::Graph,
                    Some("diagnostics") => crate::ui::app::View::Diagnostics,
                    _ => crate::ui::app::View::Dashboard,
                }
            };

            if let Some(src) = args.get("source").and_then(|v| v.as_str()) {
                if let Some(idx) = app.source_keys.iter().position(|k| k == src) {
                    app.selected_source = idx;
                }
            }
            if let Some(fld) = args.get("field").and_then(|v| v.as_str()) {
                if let Some(entry) = app.current.entries.get(app.current_source_name()) {
                    if let Some(idx) = entry.fields.iter().position(|f| f.name == fld) {
                        app.selected_field = idx;
                    }
                }
            }
            if let Some(p) = args.get("pid").and_then(|v| v.as_str()) {
                app.detailed_pid = Some(p.to_string());
            }
            app.view = view;

            let view_data = app.build_view_data();
            let json_str = serde_json::to_string(&view_data)
                .map_err(|e| (-32603, format!("serialize error: {e}"), None))?;
            let val: Value = serde_json::from_str(&json_str)
                .map_err(|e| (-32603, format!("re-parse error: {e}"), None))?;
            Ok(json!({
                "content": [{ "type": "text", "text": json_str }],
                "structuredContent": val
            }))
        }
        "field_help" => {
            let source = args
                .get("source")
                .and_then(|v| v.as_str())
                .ok_or((-32602, "source is required".into(), None))?;
            let field = args
                .get("field")
                .and_then(|v| v.as_str())
                .ok_or((-32602, "field is required".into(), None))?;
            let locale = args
                .get("lang")
                .and_then(|v| v.as_str())
                .map(crate::i18n::Locale::from_str)
                .unwrap_or(state.locale);

            let desc = crate::i18n::field_description(locale, source, field);
            let (normal, detailed, extra) = desc.unwrap_or(("(no description)", "", ""));
            let see_also = crate::i18n::see_also(locale, source, field);
            let see_also_json: Vec<Value> = see_also
                .iter()
                .map(|(s, f, d)| json!({"source": s, "field": f, "description": d}))
                .collect();

            let val = json!({
                "description": { "normal": normal, "detailed": detailed, "extra": extra },
                "see_also": see_also_json
            });
            let json_str = serde_json::to_string(&val).unwrap_or_default();
            Ok(json!({
                "content": [{ "type": "text", "text": json_str }],
                "structuredContent": val
            }))
        }
        "article" => {
            let source = args.get("source").and_then(|v| v.as_str());
            let field = args.get("field").and_then(|v| v.as_str());
            let id = args.get("id").and_then(|v| v.as_str());
            let locale = args
                .get("locale")
                .and_then(|v| v.as_str())
                .map(crate::i18n::Locale::from_str)
                .unwrap_or(state.locale);

            let article_ref = if let Some(id) = id {
                crate::article::find_article_by_id(id)
                    .unwrap_or_else(crate::article::fallback_article)
            } else if let (Some(s), Some(f)) = (source, field) {
                crate::article::resolve_article(s, f)
            } else {
                crate::article::fallback_article()
            };
            let api = crate::article::to_api_article(article_ref, locale);
            let val = serde_json::to_value(&api)
                .map_err(|e| (-32603, format!("serialize error: {e}"), None))?;
            let json_str = serde_json::to_string(&val).unwrap_or_default();
            Ok(json!({
                "content": [{ "type": "text", "text": json_str }],
                "structuredContent": val
            }))
        }
        "diagnostics" => {
            let fresh = args
                .get("fresh")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let snapshot = if fresh {
                crate::proc::Snapshot::capture()
                    .unwrap_or_else(|_| state.current.lock().unwrap().clone())
            } else {
                state.current.lock().unwrap().clone()
            };
            let findings =
                crate::diagnostics::analyze(&snapshot, state.locale, &state.diagnostic_runbooks);
            let val = serde_json::to_value(&findings)
                .map_err(|e| (-32603, format!("serialize error: {e}"), None))?;
            let json_str = serde_json::to_string(&val).unwrap_or_default();
            Ok(json!({
                "content": [{ "type": "text", "text": json_str }],
                "structuredContent": val
            }))
        }
        "get_settings" => {
            let rules = state.alert_rules.lock().unwrap().clone();
            let val = json!({
                "alert_rules": rules,
                "history": {
                    "enabled": state.history_config.enabled,
                    "interval_secs": state.history_config.interval_secs,
                    "retention_days": state.history_config.retention_days
                },
                "diagnostic_runbooks": state.diagnostic_runbooks
            });
            let json_str = serde_json::to_string(&val).unwrap_or_default();
            Ok(json!({
                "content": [{ "type": "text", "text": json_str }],
                "structuredContent": val
            }))
        }
        "set_alerts" => {
            let confirm = args
                .get("confirm")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let rules_val = args
                .get("rules")
                .ok_or((-32602, "rules is required".into(), None))?;
            let new_rules: Vec<crate::alert::AlertRule> = serde_json::from_value(rules_val.clone())
                .map_err(|e| (-32602, format!("invalid rules: {e}"), None))?;

            let current = state.alert_rules.lock().unwrap().clone();
            let diff = format!(
                "Current rules: {}\nNew rules: {}\nAdded: {}\nRemoved: {}",
                current.len(),
                new_rules.len(),
                new_rules
                    .iter()
                    .filter(|n| !current.iter().any(|c| c.source == n.source && c.field == n.field))
                    .count(),
                current
                    .iter()
                    .filter(|c| !new_rules.iter().any(|n| n.source == c.source && n.field == c.field))
                    .count()
            );

            if !confirm {
                return Ok(json!({
                    "content": [{ "type": "text", "text": format!("dry-run: {diff}") }],
                    "structuredContent": json!({
                        "dry_run": true,
                        "diff": diff,
                        "current_count": current.len(),
                        "new_count": new_rules.len()
                    })
                }));
            }

            // Apply: update in-memory + write config.toml
            {
                *state.alert_rules.lock().unwrap() = new_rules.clone();
            }
            let written = if let Some(ref path) = state.config_path {
                let contents = std::fs::read_to_string(path).unwrap_or_default();
                let updated = replace_alert_section_in_toml(&contents, &new_rules);
                if let Some(parent) = path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                match std::fs::write(path, &updated) {
                    Ok(_) => true,
                    Err(e) => {
                        return Err((
                            -32603,
                            format!("Failed to write config: {e}"),
                            None,
                        ));
                    }
                }
            } else {
                false
            };

            Ok(json!({
                "content": [{ "type": "text", "text": format!("Alerts updated: {} rules, config written: {}", new_rules.len(), written) }],
                "structuredContent": json!({
                    "dry_run": false,
                    "alert_rules": new_rules,
                    "config_written": written
                })
            }))
        }
        _ => Err((-32601, format!("unknown tool: {name}"), None)),
    }
}

// ---------------------------------------------------------------------------
// Resource reader
// ---------------------------------------------------------------------------

#[cfg(feature = "web")]
fn read_resource(uri: &str, state: &AppState) -> Result<Value, (i32, String, Option<Value>)> {
    match uri {
        "syslenz://spec" => {
            let spec = build_spec();
            let json_str = serde_json::to_string_pretty(&spec).unwrap_or_default();
            Ok(json!({
                "contents": [{
                    "uri": uri,
                    "mimeType": "application/json",
                    "text": json_str
                }]
            }))
        }
        "syslenz://guide" => Ok(json!({
            "contents": [{
                "uri": uri,
                "mimeType": "text/markdown",
                "text": build_guide()
            }]
        })),
        "syslenz://sources" => {
            let snapshot = state.current.lock().unwrap().clone();
            let sources: Vec<String> = snapshot.entries.keys().cloned().collect();
            Ok(json!({
                "contents": [{
                    "uri": uri,
                    "mimeType": "application/json",
                    "text": serde_json::to_string_pretty(&sources).unwrap_or_default()
                }]
            }))
        }
        "syslenz://diagnostics" => {
            let snapshot = state.current.lock().unwrap().clone();
            let findings =
                crate::diagnostics::analyze(&snapshot, state.locale, &state.diagnostic_runbooks);
            Ok(json!({
                "contents": [{
                    "uri": uri,
                    "mimeType": "application/json",
                    "text": serde_json::to_string_pretty(&findings).unwrap_or_default()
                }]
            }))
        }
        "syslenz://metric-kinds" => {
            let kinds = ["Memory", "Cpu", "Network", "Storage", "Process", "System", "Power", "Security"];
            Ok(json!({
                "contents": [{
                    "uri": uri,
                    "mimeType": "application/json",
                    "text": serde_json::to_string_pretty(&kinds).unwrap_or_default()
                }]
            }))
        }
        _ => Err((-32602, format!("unknown resource: {uri}"), None)),
    }
}

// ---------------------------------------------------------------------------
// Axum handler — POST /mcp
// ---------------------------------------------------------------------------

#[cfg(feature = "web")]
pub async fn mcp_handler(
    state: axum::extract::State<Arc<AppState>>,
    Json(req): Json<JsonRpcRequest>,
) -> axum::response::Response {
    let is_initialize = req.method == "initialize";
    let is_notification = req.id.is_none();

    // JSON-RPC notifications (no id) → HTTP 202 Accepted, empty body
    if is_notification {
        return axum::response::Response::builder()
            .status(StatusCode::ACCEPTED)
            .header(CONTENT_ENCODING, "identity")
            .body(axum::body::Body::empty())
            .unwrap();
    }

    let response = handle_jsonrpc(req, &state);

    if is_initialize {
        let session_id = uuid_v4();
        (
            [
                (CONTENT_ENCODING, "identity"),
                (HeaderName::from_static("mcp-session-id"), session_id.as_str()),
            ],
            Json(response),
        )
            .into_response()
    } else {
        ([(CONTENT_ENCODING, "identity")], Json(response)).into_response()
    }
}

#[cfg(feature = "web")]
fn uuid_v4() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("syslenz-{now:016x}")
}

#[cfg(feature = "web")]
fn handle_jsonrpc(req: JsonRpcRequest, state: &AppState) -> JsonRpcResponse {
    let id = req.id.clone();

    match req.method.as_str() {
        "initialize" => JsonRpcResponse::ok(
            id,
            json!({
                "protocolVersion": "2025-11-25",
                "capabilities": {
                    "tools": { "listChanged": false },
                    "resources": { "listChanged": false },
                    "prompts": { "listChanged": false }
                },
                "serverInfo": {
                    "name": "syslenz",
                    "version": VERSION
                }
            }),
        ),

        "notifications/initialized" => JsonRpcResponse::ok(id, json!({})),

        "tools/list" => JsonRpcResponse::ok(id, json!({
            "tools": tool_definitions()
        })),

        "tools/call" => {
            let name = req
                .params
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let args = req
                .params
                .get("arguments")
                .cloned()
                .unwrap_or(json!({}));
            match call_tool(name, &args, state) {
                Ok(result) => JsonRpcResponse::ok(id, result),
                Err((code, msg, data)) => JsonRpcResponse::err(id, code, &msg, data),
            }
        }

        "resources/list" => JsonRpcResponse::ok(id, json!({
            "resources": resource_definitions()
        })),

        "resources/read" => {
            let uri = req
                .params
                .get("uri")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            match read_resource(uri, state) {
                Ok(result) => JsonRpcResponse::ok(id, result),
                Err((code, msg, data)) => JsonRpcResponse::err(id, code, &msg, data),
            }
        }

        "resources/templates/list" => JsonRpcResponse::ok(id, json!({
            "resourceTemplates": []
        })),

        "prompts/list" => JsonRpcResponse::ok(id, json!({
            "prompts": prompt_definitions()
        })),

        "prompts/get" => {
            let name = req
                .params
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if name == "monitor-and-diagnose" {
                JsonRpcResponse::ok(id, json!({
                    "description": "スナップショット取得 → 診断実行 → 結果解釈",
                    "messages": [{
                        "role": "user",
                        "content": {
                            "type": "text",
                            "text": "syslenz の snapshot ツールで現在のシステム状態を取得し、diagnostics ツールで自動診断を実行して、結果を解釈してください。Critical または Warning の Finding があれば、対応策を提案してください。"
                        }
                    }]
                }))
            } else {
                JsonRpcResponse::err(id, -32601, &format!("unknown prompt: {name}"), None)
            }
        }

        "ping" => JsonRpcResponse::ok(id, json!({})),

        _ => JsonRpcResponse::err(
            id,
            -32601,
            &format!("method not found: {}", req.method),
            None,
        ),
    }
}

// ---------------------------------------------------------------------------
// Helpers (reused from web.rs)
// ---------------------------------------------------------------------------

#[cfg(feature = "web")]
fn replace_alert_section_in_toml(
    existing: &str,
    rules: &[crate::alert::AlertRule],
) -> String {
    // Remove existing [[alert]] blocks
    let mut result = String::new();
    let mut skip = false;

    for line in existing.lines() {
        if line.trim_start().starts_with("[[alert]]") {
            skip = true;
            continue;
        }
        if skip {
            if line.starts_with('[') {
                skip = false;
                result.push_str(line);
                result.push('\n');
            }
            continue;
        }
        result.push_str(line);
        result.push('\n');
    }

    // Append new [[alert]] blocks
    for rule in rules {
        result.push_str(&format!(
            "[[alert]]\nsource = \"{}\"\nfield = \"{}\"\ncondition = \"{}\"\nseverity = \"{}\"\nmessage = \"{}\"\n\n",
            rule.source,
            rule.field,
            rule.condition,
            rule.severity,
            rule.message.replace('"', "\\\"")
        ));
    }

    result
}

// ---------------------------------------------------------------------------
// Non-web stub
// ---------------------------------------------------------------------------

#[cfg(not(feature = "web"))]
pub fn mcp_handler_stub() {}
