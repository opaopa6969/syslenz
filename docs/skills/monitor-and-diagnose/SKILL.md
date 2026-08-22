---
name: monitor-and-diagnose
description: syslenz でシステム状態の監視・診断を行う手順
volta:
  version: 1
  namespace: syslenz
  locality: service
  applies_when: システム状態の調査・診断が必要なとき
  requires: syslenz MCP バックエンドが ready
  min_role: MEMBER
  export: true
---

# syslenz でシステム監視・診断をする

## 概要

syslenz MCP の `snapshot` で現在のシステム状態を取得し、`diagnostics` で 27 の自動診断チェックを実行して、Critical/Warning の結果を解釈する手順。

## 手順

1. **スナップショット取得**: `syslenz__snapshot` を呼び出し、50+ ソースの構造化 JSON を取得する。
2. **診断実行**: `syslenz__diagnostics` を呼び出す。`fresh: true` で最新キャプチャ、`fresh: false`（既定）でキャッシュから。
3. **結果の解釈**:
   - `severity: Critical` → 即時対応。`suggestion` に従いアクション。
   - `severity: Warning` → 調査推奨。`related_metrics` で関連メトリクスを確認。
   - `severity: Info` → 参考情報（リスニングポート、IP 転送など）。
4. **詳細確認**: `syslenz__field_help` で特定フィールドの説明を取得し、`syslenz__article` で教育記事を読む。

## 出力の形

```json
{
  "severity": "Warning",
  "source": "systemd",
  "title": "systemd: system state degraded",
  "detail": "systemd reports the system as degraded.",
  "suggestion": "Run: systemctl --failed for details.",
  "related_metrics": [],
  "runbook_url": null
}
```

## 組み合わせ

- `syslenz__snapshot` → `syslenz__diagnostics` → `index__agent_send`（エージェント端末に通知）
- `syslenz__diagnostics` → `syslenz__field_help`（診断結果のフィールドを深掘り）
