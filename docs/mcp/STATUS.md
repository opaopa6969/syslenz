# syslenz MCP 化 Phase 2 ステータス

## 完了状況

| 項目 | 状態 | 備考 |
|------|------|------|
| Phase 1 survey | ✓ | `docs/mcp/survey.json` 既存 |
| DESIGN.md | ✓ | `docs/mcp/DESIGN.md` |
| MCP サーバ実装 | ✓ | `src/mcp.rs` (JSON-RPC 2.0 over HTTP), `src/web.rs` に `/mcp` と `/api/diagnostics` ルート追加 |
| テスト | ✓ | e2e: healthz/tools/list/tools/call/resources/list/resources/read/diagnostics/dry-run 全て動作確認 |
| volta.service.json | ✓ | `mcp.enabled: true, namespace: syslenz, port: 3009` |
| deploy unit | ✓ | `deploy/syslenz.service` (systemd), `run.sh` |
| skill | ✓ | `docs/skills/monitor-and-diagnose/SKILL.md`, `docs/skills/multi-host-monitoring/SKILL.md` |
| commit & push | ✓ | PR #25 (MCP実装), #26 (mcp-session-id), #27 (HTTP 202 for notifications) → 全て main に merge |
| volta 登録 (svc_add) | ✓ | confirm:true で実行。catalog の mcp 項が有効化 |
| gateway ルート | — | syslenz.unlaxer.org は既存ルート（変更なし）。`gateway_routes_diff` に他 repo 2 件新規が含まれたため `gateway_routes_apply` は実施せず（指示書の停止条件） |
| prod デプロイ | ✓ | prod (192.168.1.50) で `git pull main` + `cargo build --release --features web` + `sudo systemctl restart syslenz` |
| healthz 200 | ✓ | `https://syslenz.unlaxer.org/healthz` → 200 |
| catalog ready | ✓ | `catalog__backend_status` で `syslenz` namespace が `ready` |
| audit ng=0 | ✓ | `catalog__audit_backend(syslenz)` → 9 ok / 0 ng / 2 skip / 1 unknown |

## gateway_routes_diff について

`volta__gateway_routes_diff()` の結果、syslenz のルートは既存（変更なし）だったが、**他リポジトリの Phase 2 並行実行による 2 件の新規ルート**（fude-engine, henshu-engine）が含まれた。指示書の停止条件「自分の 1 件以外を含む」に該当するため、`gateway_routes_apply` は実施せず、この場で止めた。syslenz 自体のルートは既存のため影響なし。

## issue-hub

- issue #285: `[mcp] syslenz ↔ syslenz4j: TCP protocol unaffected by MCP` — 返答を待たず暫定仕様で進めた。TCP プロトコルは MCP 化の影響を受けない。

## 未決事項

1. **content-encoding: identity の監査 unknown**: `/mcp` の SSE 応答で identity を付けているかは healthz からは判定不能。実際には付与済み（curl で確認）。
2. **rmcp crate 導入の検討**: 今回は手書き JSON-RPC で実装したが、将来的に rmcp（Rust MCP SDK）への移行を検討可能。トランスポート・セッション管理・elicitation 等が標準サポートされる。
3. **認証**: syslenz 本体に認証なし。gateway が `minRole:MEMBER` で保護。システム情報を含むため public にはしない。
4. **export/import tool の非対応**: ファイルシステム書き込みを伴うため Phase 2 では非対応。必要なら `confirm` 付きで追加可能。

## 成果物

- `docs/mcp/DESIGN.md` — Phase 2 設計
- `docs/mcp/SURVEY.md` — Phase 1 調査
- `docs/mcp/survey.json` — Phase 1 調査データ
- `docs/mcp/STATUS.md` — このファイル
- `src/mcp.rs` — MCP JSON-RPC handler (9 tools, 5 resources, 1 prompt)
- `src/web.rs` — `/mcp`, `/api/diagnostics` ルート追加, AppState pub(crate)
- `src/diagnostics.rs` — DiagnosticFinding/Severity に Serialize derive
- `volta.service.json` — manifest (mcp.enabled)
- `deploy/syslenz.service` — systemd unit
- `run.sh` — 起動スクリプト
- `docs/skills/monitor-and-diagnose/SKILL.md`
- `docs/skills/multi-host-monitoring/SKILL.md`
- `README-ja.md` — MCP 節追加
