# syslenz MCP 化設計（Phase 2）

## 1. namespace と種別

- **namespace**: `syslenz`
- **種別**: `wrap`（既存 HTTP API を薄く包む MCP レイヤーを同プロセスに追加）
- **ポート**: `3009`（prod 既存。catalog に既登録。MCP も同じ axum プロセスで `/mcp` を提供）
- **ホスト**: `192.168.1.50`（prod）
- **runtime**: `systemd`（既存 `syslenz.service` を再利用）

Phase 1 survey の `decision: wrap` を踏襲。既存 `volta catalog` エントリ `syslenz`（prod:3009, dev:3008）に `mcp` 項を追記する形で参加させる。新規ポート 9236（割当表）は使用しない（既存サービスは既存 port/namespace 優先の指示通り）。

## 2. tools 表

全て `min_role: MEMBER`。既存 HTTP API を MCP tool として公開。`/mcp`（Streamable HTTP, JSON-RPC 2.0）で提供。

| name | 目的 | 入力 schema（要点） | 出力の形 | 副作用 | dry-run | job型 | 所要時間 | min_role |
|------|------|---------------------|----------|--------|----------|-------|----------|----------|
| `snapshot` | 現在のシステムスナップショットを取得 | `{}` | `{timestamp, entries: {source: {source, fields: [{name, value, unit, description}]}}, alerts}` | read | — | no | <1s | MEMBER |
| `history` | スナップショット履歴（最大60件）を取得 | `{}` | `[{snapshot}, ...]` | read | — | no | <1s | MEMBER |
| `sources` | 利用可能なデータソース一覧を取得 | `{}` | `string[]`（50+ source names） | read | — | no | <1s | MEMBER |
| `view` | 指定ビューの描画済みデータを取得 | `{view?: string, locale?: "en"\|"ja", source?: string, field?: string, pid?: string}` | `ViewData` JSON | read | — | no | <1s | MEMBER |
| `field_help` | フィールドのヘルプ・説明を取得 | `{source: string, field: string, lang?: "en"\|"ja"}` | `{description: {normal, detailed, extra}, see_also: [...], breadcrumbs: [...], contextual_hint}` | read | — | no | <1s | MEMBER |
| `article` | メトリクスの教育記事を取得 | `{source?: string, field?: string, id?: string, locale?: "en"\|"ja"}` | `Article` JSON（691 metrics × EN+JA） | read | — | no | <1s | MEMBER |
| `diagnostics` | 自動診断（27 checks, 40+ patterns）を実行 | `{fresh?: bool}` | `[{severity, source, title, detail, suggestion, related_metrics, runbook_url}]` | read | — | no | <1s | MEMBER |
| `get_settings` | 現在の設定を取得 | `{}` | `Config` JSON | read | — | no | <1s | MEMBER |
| `set_alerts` | アラートルールを設定（config.toml 書き込み） | `{rules: [{source, field, condition, severity, message}], confirm?: bool}` | `{alert_rules: [...]}` | write | yes（`confirm:false` で差分プレビュー） | no | <1s | MEMBER |

`export` / `import` はファイルシステム書き込みを伴い MCP 経由では安全性が低いため、Phase 2 では非対応（§7 参照）。

### annotations

全 tool に `annotations` を付与:
- `snapshot`, `history`, `sources`, `view`, `field_help`, `article`, `diagnostics`, `get_settings` → `readOnlyHint: true`
- `set_alerts` → `destructiveHint: true`（`confirm` 引数必須）

## 3. resources 表

| uri | 内容 | mime |
|-----|------|------|
| `syslenz://spec` | 能力仕様（JSON, §2.2 形式） | `application/json` |
| `syslenz://guide` | 使い方ガイド（Markdown） | `text/markdown` |
| `syslenz://sources` | データソース一覧と分類 | `application/json` |
| `syslenz://diagnostics` | 診断チェック一覧 | `application/json` |
| `syslenz://metric-kinds` | MetricKind 8種の分類体系 | `application/json` |

`syslenz://spec` と `syslenz://guide` は規約必須。サーバ起動時に tool 定義から `spec` を生成し、`compositions` / `depends_on` だけ手で書く。

## 4. prompts / skills

### prompts
| name | 用途 |
|------|------|
| `monitor-and-diagnose` | スナップショット取得 → 診断実行 → 結果解釈のプロンプトテンプレート |

### skills
| name | 用途 | locality | applies_when | requires |
|------|------|----------|---------------|----------|
| `monitor-and-diagnose` | syslenz でシステム監視・診断する手順 | service | "システム状態の調査・診断が必要なとき" | "syslenz MCP バックエンドが ready" |
| `multi-host-monitoring` | 複数ホストを syslenz で監視する手順 | service | "複数ホストの監視を設定するとき" | "syslenz MCP バックエンドが ready" |

`docs/skills/<name>/SKILL.md` に配置し、resource `skill://<name>` でも返す。

## 5. 組み合わせ例

1. **異常検知フロー**: `syslenz__snapshot` → `syslenz__diagnostics` → `index__agent_send`（診断結果をエージェント端末に通知）
   - snapshot で全メトリクス取得、diagnostics で 27 のチェックを走らせ、Critical/Warning を index に通知
2. **ヘルス突合せ**: `syslenz__snapshot` → `volta__svc_health`（スナップショットの負荷メトリクスと他サービスのヘルスを突き合わせ）
   - snapshot の `loadavg`, `meminfo`, `processes` と volta サービス群の稼働状態を相関分析
3. **アラート自動生成**: `syslenz__snapshot` → LLM で解析 → `syslenz__set_alerts`（異常パターンからアラートルールを自動生成）
   - snapshot の傾向を LLM が分析し、`set_alerts`（`confirm:true`）で config.toml にルールを書き込み

## 6. 依存と協調

| 相手 repo | 方向 | 入口（tool/resource） | 合意したいこと | 状態 |
|-----------|------|---------------------|----------------|------|
| `syslenz4j` | depends_on | syslenz TCP プロトコル（SNAPSHOT/METRICS） | TCP プロトコル仕様は既存確定。MCP 化に影響なし | issue-hub で通知のみ |
| `volta-mcp` | provides_to | `syslenz://spec`, `syslenz__*` tools | ファサード経由で他エージェントが syslenz 能力を呼べる | Phase 2 完了後に自動集約 |

`syslenz4j` は syslenz の TCP プロトコル（`--serve` モード）に依存するが、MCP 化（HTTP `/mcp`）とは独立。issue-hub で `[mcp] syslenz ↔ syslenz4j: TCP protocol unaffected` を登録し、返答を待たず暫定で進める。

## 7. 非対応にした候補と理由

| 候補 | 理由 |
|------|------|
| `export`（ファイル書き込み） | MCP 経由でホストのファイルシステムに書き込むのは安全性が低い。snapshot は `syslenz__snapshot` で JSON 取得可能なので不要 |
| `import`（ファイル読み込み） | 同上。MCP サーバが動くホストのローカルファイルに依存するため |
| rmcp crate 導入 | 既存 axum に JSON-RPC handler を追加する方が依存増加を抑制できる。rmcp は tokio/schemars 版本制約があり Cargo.toml への影響が大きい |

## 8. 参加方法

### manifest（volta.service.json）
```jsonc
{
  "id": "syslenz",
  "name": "syslenz",
  "description": "Wireshark for Linux — explore /proc, /sys, and network as structured, typed data.",
  "type": "rust",
  "hostname": "syslenz.unlaxer.org",
  "port": 3009,
  "host": "192.168.1.50",
  "runtime": "systemd",
  "exec_start": "/home/opa/syslenz/run.sh",
  "user": "opa",
  "auth": "minRole:MEMBER",
  "health_check": "/healthz",
  "tags": ["monitoring", "system", "diagnostics", "rust", "mcp"],
  "repo_url": "https://github.com/opaopa6969/syslenz",
  "mcp": {
    "enabled": true,
    "port": 3009,
    "path": "/mcp",
    "namespace": "syslenz",
    "min_role": "MEMBER",
    "timeoutMs": 110000,
    "description": "システムモニタリング・診断（50+ ソース、27 診断チェック、691 メトリクス教育記事）"
  }
}
```

### ポート
- `3009`（既存 prod。Web UI と MCP を同じ axum プロセスで提供）

### ホスト
- `192.168.1.50`（prod）

### runtime
- `systemd`（既存 `syslenz.service` を再利用。`run.sh` は `--web 3009 --lang en`）

### auth
- `minRole:MEMBER`（システム情報を含むため public にはしない。gateway が認証を付与）

## 9. テスト方針

### e2e テスト（Rust 統合テスト）
1. **サーバ起動**: `cargo test` で `--web 0`（ephemeral port）を起動
2. **healthz**: `GET /healthz` → 200, body `ok`
3. **tools/list**: `POST /mcp`（JSON-RPC `tools/list`）→ `syslenz__*` tools が返る
4. **主要 tool**: `snapshot`, `sources`, `diagnostics` を `tools/call` で呼び出し
5. **dry-run**: `set_alerts` with `confirm:false` → 差分プレビューが返る
6. **spec resource**: `resources/read` `syslenz://spec` → JSON 仕様
7. **content-encoding**: response に `content-encoding: identity` があること

### MCP クライアントテスト
- `curl` で JSON-RPC リクエストを直接送信（`@modelcontextprotocol/sdk` クライアントは別途）
- `volta__svc_health` / `catalog__audit_backend` で本番確認

## 実装構成

```
src/
  web.rs           # 既存 axum Router に /mcp と /api/diagnostics を追加
  mcp.rs           # 新規: MCP JSON-RPC handler（tools/list, tools/call, resources/read, resources/list, prompts/list, prompts/get）
  diagnostics.rs   # DiagnosticFinding に Serialize derive を追加
deploy/
  syslenz.service  # systemd unit
run.sh             # 起動スクリプト（--web $PORT --lang en）
volta.service.json # manifest（root）
docs/skills/
  monitor-and-diagnose/SKILL.md
  multi-host-monitoring/SKILL.md
docs/mcp/
  DESIGN.md        # このファイル
  STATUS.md        # 進捗・未決事項
```
