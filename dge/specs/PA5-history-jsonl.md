<!-- ⚠ DGE AUTO-GENERATED — review before acting -->
---
status: draft
source: DGE-020
---

# P-A5: History JSONL 保存 (書き込みのみ)

## 新規ファイル
- `src/history.rs`

## 変更ファイル
- `src/config.rs` — [history] セクション追加
- `src/main.rs` — run() ループ内で write_if_due() 呼び出し、mod history 追加
- `src/proc/mod.rs` — Snapshot 系 struct に Deserialize derive 追加

## Config
```toml
[history]
enabled = true
interval_secs = 60
retention_days = 7
# path = "~/.local/share/syslenz/history"  # default
```

## データ構造
```rust
pub struct HistoryWriter {
    dir: PathBuf,
    interval: Duration,
    retention_days: u32,
    last_write: Option<Instant>,
}
```

## メソッド
- `new(config) → Self`
- `write_if_due(&mut self, snapshot) → Result<()>` — 間隔チェック + JSONL 追記
- `cleanup(&self) → Result<()>` — retention_days 超のファイル削除

## ファイル形式
- パス: `~/.local/share/syslenz/history/YYYY-MM-DD.jsonl`
- 各行: `{"timestamp":"...","entries":{...}}` (Snapshot の JSON)
- 容量: ~50KB/snapshot × 1440/日 ≈ 70MB/日

## テスト
- write_if_due() が interval 内では書き込まないこと
- write_if_due() が interval 経過後に JSONL に1行追記すること
- cleanup() が古いファイルを削除すること
