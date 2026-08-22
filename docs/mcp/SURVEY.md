# syslenz — MCP 化調査（Phase 1）

## 概要

**syslenz** は Rust 製のシステムモニタリング・教育ツール（"Wireshark for /proc"）。`/proc`・`/sys`・ネットワークから 50+ ソースを構造化・型付き JSON として取得し、TUI・Web UI・TCP サーバ・Docker で提供する。

主な機能:
- 50+ データソース（/proc 43, /sys 3, ネットワーク 5, プラグイン無制限）
- HTTP API（axum）: `/api/snapshot`, `/api/history`, `/api/sources`, `/api/stream`（SSE）, `/api/view`, `/api/field-help`, `/api/article`, `/api/v1/settings`, `/healthz`
- TCP サーバモード（`--serve`）: `SNAPSHOT` / `METRICS` / `QUIT` プロトコル
- 27 の自動診断チェック（40+ パターン）
- 691 メトリクス × EN+JA の教育記事
- Prometheus / OpenTelemetry export
- JSON export/import（スナップショットの保存・再生）
- クロスプラットフォーム（Linux 51+, macOS 24, Windows 24）
- SDK（Java syslenz4j 公開済み, Python/Node planned）

volta catalog に `syslenz`（dev:3008, prod:3009）および `syslenz4j` として既に登録済み。

## 判定と理由

**判定: `wrap`** — 既存 HTTP API を薄く包む MCP レイヤーを追加する。

根拠:
1. **HTTP API が既に充実**: axum で Streamable HTTP を実装済み。`/healthz` が 200、`0.0.0.0` bind、PORT 指定可能。
2. **JSON API が揃っている**: snapshot, history, sources, view, field-help, article, settings が全て JSON で取得可能。
3. **volta catalog に既登録**: syslenz, syslenz4j として dev/prod 環境が設定済み。
4. **不足分は小さい**: `/mcp`（Streamable HTTP エンドポイント）の追加と、diagnostics の HTTP API 化のみ。

## 公開候補

### tools

| name | io | 副作用 | 長時間 | maps_to |
|------|-----|--------|--------|---------|
| `snapshot` | → Snapshot JSON | read | no | `GET /api/snapshot` |
| `history` | → Snapshot[] (最大60件) | read | no | `GET /api/history` |
| `sources` | → string[] | read | no | `GET /api/sources` |
| `view` | { view?, locale?, source?, field?, pid? } → ViewData | read | no | `GET /api/view` |
| `field_help` | { source, field, lang? } → FieldHelp | read | no | `GET /api/field-help` |
| `article` | { source?, field?, id?, locale? } → Article | read | no | `GET /api/article` |
| `diagnostics` | → DiagnosticFinding[] | read | no | `diagnostics.rs:analyze()` ※HTTP API 未実装、Phase 2 で追加 |
| `export` | { path? } → JSON | write | no | `export.rs:export_snapshot` |
| `import` | { path } → Snapshot | read | no | `export.rs:import_snapshot` |
| `set_alerts` | AlertRule[] → void | write | no | `POST /api/v1/settings/alerts` |
| `get_settings` | → Config | read | no | `GET /api/v1/settings` |

### resources

| name | uri |
|------|-----|
| `spec` | `syslenz://spec` |
| `guide` | `syslenz://guide` |
| `sources` | `syslenz://sources` |
| `diagnostics_catalog` | `syslenz://diagnostics` |
| `metric_kinds` | `syslenz://metric-kinds` |

### skills

| name | locality |
|------|----------|
| `monitor-and-diagnose` | service |
| `multi-host-monitoring` | service |

## 組み合わせ例

1. `syslenz__snapshot` → `syslenz__diagnostics` → `index__agent_send`（診断結果をエージェント端末に通知）
2. `syslenz__export` → `volta__svc_health`（スナップショット取得後、他サービスのヘルスと突き合わせ）
3. `syslenz__snapshot` → LLM 解析 → `syslenz__set_alerts`（異常パターンからアラートルールを自動生成）

## 依存と協調

| 相手 repo | 方向 | 能力 | exists_now | 備考 |
|-----------|------|------|------------|------|
| syslenz4j | depends_on | JVM メトリクス収集（TCP プロトコル経由） | yes | syslenz4j が syslenz の TCP プロトコルに依存 |
| volta-mcp | provides_to | syslenz namespace の MCP tools/resources | no | Phase 2 で `/mcp` 追加後、catalog の mcp 項を有効化 |

## ライブラリのサーバ化

該当しない。syslenz は既に常駐サーバ（Web/TCP）として動作する。

## リスク

- **認証なし**: HTTP サーバに認証がない（README に明記）。MCP 経由で公開する場合、syslenz 側（Roadmap に planned）またはファサード側で認証を付ける必要がある。
- **ホスト情報の露出**: snapshot はホストのシステム情報（プロセス、ネットワーク接続、カーネルモジュール等）を含む。公開範囲の考慮が必要。
- **Linux 依存**: `Snapshot::capture()` は `/proc`・`/sys` を読むため Linux ホストが必要。Docker では `--pid=host` が必要。
- **設定ファイル書き込み**: `set_alerts` は `config.toml` に書き込む（副作用あり）。MCP tool 化時は `confirm: bool=false` で dry-run を実装すべき。

## 持ち主への質問

1. `/mcp`（Streamable HTTP エンドポイント）は syslenz 本体（Rust）に追加するか、薄いラッパプロセスで立てるか？Rust MCP SDK の対応状況を確認が必要。
2. volt catalog の syslenz エントリに mcp 項が未設定。Phase 2 で `/mcp` 実装後に追記する。
3. 認証は syslenz 側（Roadmap の Basic Auth / Token）を実装してから MCP 化するか、ファサード側で遮断するか？
4. `diagnostics` は現在 TUI 内部 API のみ。HTTP エンドポイント `/api/diagnostics` を新設する必要がある。
