<!-- ⚠ DGE AUTO-GENERATED — review before acting -->
---
status: draft
source: DGE-020
---

# P-A3: CLI --query モード

## 変更ファイル
- `src/main.rs` — --query フラグハンドリング + `run_query()` 関数

## 使い方
```
syslenz --query                              → 全ソース名を改行区切り
syslenz --query meminfo                      → 全フィールドを "field_name\tvalue" (TSV)
syslenz --query meminfo.MemAvailable         → "2048000" (値のみ)
syslenz --query meminfo.MemAvailable --json  → {"source":"meminfo","field":"MemAvailable","value":{...},...}
```

## エラー処理
- ソース未発見: stderr に "Source 'X' not found" + exit 1
- フィールド未発見: stderr に "Field 'X' not found in 'Y'" + exit 1

## 実装位置
- main() の --export-json と --install-service の間に早期リターン
- TUI/raw mode に入らない

## テスト (tests/smoke.rs)
- --query → exit 0 + stdout にソース名
- --query meminfo.MemTotal → exit 0 + 数値
- --query nonexist → exit 1
