# DGE Session 007: G10 深掘り — テストがほぼゼロ

- **Date**: 2026-03-28
- **Theme**: テストカバレッジ戦略 (Test coverage strategy)
- **Parent Gap**: G10 — テストがほぼゼロ (High priority)
- **Characters**: 千石 (品質ガーディアン) + リヴァイ (実装エンフォーサー) + 僕 (スコープリデューサー)

---

## 現状の把握

- テスト数: **1 件のみ** (`remote.rs` の `capture_remote_bad_host_returns_error()`)
- パーサー: **43 個** (`src/proc/*.rs`)、全て `pub fn parse() -> anyhow::Result<ProcEntry>`
- コア型: `Snapshot`, `ProcEntry`, `Field`, `FieldValue` (Serialize/Deserialize 実装済み)
- export/import: `export_snapshot`, `import_snapshot`, `export_series`, `import_series`
- diff: `diff_snapshots()` — 2 つの Snapshot を比較して `Vec<DiffItem>` を返す
- App: `new()`, `from_imported()`, `from_remote()`, `refresh()`
- i18n: `t()` 関数、`source_description()` 関数、Locale: En/Ja
- UI: ratatui ベースの描画関数 (テスト困難)

---

## Scene 1: /proc なしで何がテストできるか

先輩 (ナレーション): 43 個のパーサーは全て `/proc/xxx` を `fs::read_to_string` で直接読んでいる。CI 環境 (GitHub Actions の Ubuntu ランナー) では /proc は存在するが、開発者の手元が macOS や WSL の場合もある。まず「/proc に依存しないテスト」の範囲を特定する。

🧪 千石: 「品質に妥協しない。まず事実を整理しよう。/proc を読まなくてもテストできるコードは以下だ。」

- `FieldValue::display()` — 純粋関数。入力と出力が決定的
- `format_bytes()` — `u64` → `String` の変換
- `format_duration()` — `f64` → `String` の変換
- `export_snapshot` / `import_snapshot` — tempfile に書いて読み戻すだけ
- `export_series` / `import_series` — 同上
- `diff_snapshots()` — 2 つの `Snapshot` を手で作って渡せる
- `i18n::t()` — 全キーに対して `"?"` 以外が返ることを確認
- `i18n::source_description()` — 全ソース名に対して "System information source" / "システム情報ソース" 以外が返ることを確認
- `Locale::from_str()`, `Locale::next()`, `Locale::name()` — 純粋関数
- `systemtime_iso8601` の serialize/deserialize round-trip

→ **Gap 発見: /proc に一切触らなくても、export/import, diff, i18n, FieldValue 表示、時刻変換の全てがテスト可能。これらのテストがゼロなのは怠慢。**

⚔️ リヴァイ: 「汚えな。43 個のパーサーに目が行って、手前のユーティリティ関数を放置してる。まずはそこからだ。format_bytes のエッジケース — 0 バイト、1023 バイト、1024 バイト(= 1 KiB 境界)、GiB 越え。全部やれ。」

→ **Spec implication: format_bytes / format_duration のユニットテストは今日中に書ける。パーサーより先にこれをやる。**

😰 僕: 「...全部一気にやるのは...現実的じゃないです...。優先順位つけませんか...?」

🧪 千石: 「正しい。だから Phase を分ける。Phase 1: /proc 不要のテスト。Phase 2: パーサーテスト。Phase 3: インテグレーション。」

---

## Scene 2: パーサーテスト戦略

先輩: 43 個のパーサーは全て同じパターン: `fs::read_to_string("/proc/xxx")` → パース → `ProcEntry`。テスト方法は 2 つある。

### 方法 A: fixture ファイル方式

テスト用の `/proc` 内容をファイルに保存し、パーサーにそれを読ませる。

🧪 千石: 「現状のパーサーはパスがハードコードされている。`fs::read_to_string("/proc/uptime")` を直接呼んでいる。これをテスタブルにするには、パース関数を 2 層に分離する必要がある。」

```rust
// 現状: テスト不可能
pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/uptime")?;
    // ... parse content ...
}

// 改善案: parse_content を分離
pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/uptime")?;
    parse_content(&content)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
    let parts: Vec<&str> = content.trim().split_whitespace().collect();
    let uptime: f64 = parts[0].parse()?;
    let idle: f64 = parts[1].parse()?;
    // ... build ProcEntry ...
}
```

→ **Gap 発見: 43 個のパーサー全てが I/O とロジックを混在させている。`parse_content(&str)` を分離すれば、fixture なしで文字列をそのまま渡してテストできる。**

⚔️ リヴァイ: 「43 個全部を一度にリファクタリングする必要はない。重要なのは手順だ。(1) `parse_content` を追加、(2) `parse` が `parse_content` を呼ぶように変更、(3) テスト追加。1 パーサーあたり 5 分。やれ。」

→ **Spec implication: 全パーサーに `pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry>` を追加。既存の `parse()` は `parse_content` を呼ぶラッパーに変更。破壊的変更なし。**

### 方法 B: 実機 /proc 直接読み (CI 限定)

🧪 千石: 「CI が Linux なら `/proc` はある。だが問題がある。」

1. `/proc/slabinfo` は root 権限が必要 → CI で失敗する可能性
2. `/proc/pressure` は kernel config 依存 (`CONFIG_PSI=y`) → 古いカーネルでは存在しない
3. `/proc/net/wireless` は WiFi ドライバ依存 → サーバーには存在しないことがある
4. プロセス数、メモリ量、CPU 数は環境ごとに異なる → **値の assertion ができない**

→ **Gap 発見: 実機テストは「パースが成功すること」しか確認できない。値の正しさは確認できない。fixture テスト (方法 A) でないと「"MemTotal: 16384 kB" を 16384 * 1024 Bytes に変換できたか」を検証できない。**

⚔️ リヴァイ: 「両方やれ。方法 A で値の正確性を検証。方法 B (`#[cfg(target_os = "linux")]` 付き) で "本番の /proc でパニックしない" ことを保証。二重の防壁だ。」

😰 僕: 「...43 個全部に fixture 作るの...辛くないですか...」

🧪 千石: 「だから優先順位だ。fixture を最初に作るべきパーサーは以下の 5 つ。」

1. **meminfo** — フィールド数が多く、kB → Bytes 変換ロジックがある
2. **loadavg** — `/` 区切りのパース (running/total) がある
3. **stat** — CPU 時間の行パースが複雑
4. **net_tcp** — 16 進アドレスのデコードがある (バグが出やすい)
5. **cpuinfo** — マルチエントリ (CPU ごと) のパースがある

→ **Spec implication: この 5 つに fixture を作り、parse_content のテストを書く。残りは方法 B (実機 smoke test) で最低限カバー。**

---

## Scene 3: インテグレーションテスト

先輩: 個別パーサーの次は、コンポーネント間の結合テスト。

### 3-1: export/import round-trip

🧪 千石: 「これは最も費用対効果の高いテストだ。serialize して deserialize して元と一致すれば、Snapshot の全フィールドの Serialize/Deserialize 実装が正しいことが保証される。」

```rust
#[test]
fn snapshot_export_import_roundtrip() {
    let original = make_test_snapshot(); // 手動で Snapshot を構築
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.json");

    export_snapshot(&original, &path).unwrap();
    let restored = import_snapshot(&path).unwrap();

    assert_eq!(original.entries.len(), restored.entries.len());
    for (key, entry) in &original.entries {
        let restored_entry = restored.entries.get(key).unwrap();
        assert_eq!(entry.fields.len(), restored_entry.fields.len());
        for (f1, f2) in entry.fields.iter().zip(restored_entry.fields.iter()) {
            assert_eq!(f1.name, f2.name);
            assert_eq!(f1.description, f2.description);
            // FieldValue の比較 (display() ベース)
            assert_eq!(f1.value.display(), f2.value.display());
        }
    }
}

#[test]
fn series_export_import_roundtrip() {
    let s1 = make_test_snapshot();
    let s2 = make_test_snapshot();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("series.json");

    export_series(&[s1.clone(), s2.clone()], &path).unwrap();
    let restored = import_series(&path).unwrap();

    assert_eq!(2, restored.len());
}
```

→ **Gap 発見: FieldValue に PartialEq が derive されていない。display() ベースの比較は浮動小数点の精度で壊れる可能性がある。PartialEq を追加するか、専用の比較ヘルパーが必要。**

→ **Spec implication: `FieldValue` に `#[derive(PartialEq)]` を追加。Float の比較は epsilon ベースで。Table は Vec<Vec<String>> なのでそのまま PartialEq で OK。**

### 3-2: diff_snapshots テスト

⚔️ リヴァイ: 「diff_snapshots は現在テストゼロだ。以下の 3 ケースをカバーしろ。」

```rust
#[test]
fn diff_identical_snapshots_returns_empty() {
    let snap = make_test_snapshot();
    let diffs = diff_snapshots(&snap, &snap);
    assert!(diffs.is_empty());
}

#[test]
fn diff_detects_integer_change() {
    let mut snap1 = make_test_snapshot();
    let mut snap2 = make_test_snapshot();
    // snap2 の "meminfo" の MemTotal を変更
    if let Some(entry) = snap2.entries.get_mut("meminfo") {
        entry.fields[0].value = FieldValue::Bytes(999999);
    }
    let diffs = diff_snapshots(&snap1, &snap2);
    assert!(!diffs.is_empty());
    assert_eq!(diffs[0].source, "meminfo");
}

#[test]
fn diff_ignores_small_float_difference() {
    // Float の差が 0.001 以下なら changed = false
    let mut snap1 = make_test_snapshot();
    let mut snap2 = snap1.clone();
    if let Some(entry) = snap2.entries.get_mut("loadavg") {
        entry.fields[0].value = FieldValue::Float(0.5005); // 元が 0.50 なら差 0.0005
    }
    let diffs = diff_snapshots(&snap1, &snap2);
    // 0.0005 < 0.001 なので diff に含まれないはず
    let loadavg_diffs: Vec<_> = diffs.iter()
        .filter(|d| d.source == "loadavg")
        .collect();
    assert!(loadavg_diffs.is_empty());
}
```

→ **Gap 発見: diff_snapshots は zip を使っているため、フィールド数が異なる場合 (パーサーのバージョン違いなど) に余分なフィールドが無視される。これはバグの可能性がある。**

→ **Spec implication: フィールド数不一致のケースのテストも追加。将来的に zip → zip_longest へ変更を検討。**

### 3-3: systemtime_iso8601 round-trip

🧪 千石: 「Snapshot の timestamp は `systemtime_iso8601` というカスタム serde モジュールで処理される。これは自前実装の日付パーサーだ。自前実装 = バグの温床。必ずテストを書け。」

```rust
#[test]
fn systemtime_roundtrip_via_snapshot_json() {
    let snap = make_test_snapshot();
    let json = serde_json::to_string(&snap).unwrap();
    let restored: Snapshot = serde_json::from_str(&json).unwrap();
    // timestamp が一致すること (ナノ秒精度)
    assert_eq!(snap.timestamp, restored.timestamp);
}
```

→ **Spec implication: systemtime_iso8601 のエッジケース (エポック直後、2038 年問題周辺、閏年の 2/29) もテストすべき。**

---

## Scene 4: テスト MVP — 最低限どこまでやるか

先輩: 全てを一度にはできない。「今あるバグを最も効率よく捕まえる」テストセットは何か。

😰 僕: 「...最小限でお願いします...。リリースを遅らせたくない...。」

🧪 千石: 「最小限でも妥協しない。以下が "テスト MVP" だ。これ以下は品質として成立しない。」

### MVP テスト一覧 (Phase 1: /proc 不要、即実装可能)

| # | テスト名 | 対象ファイル | テスト対象 | 意義 |
|---|---------|-------------|-----------|------|
| T1 | `field_value_display_bytes` | `src/proc/mod.rs` | `format_bytes(0)`, `format_bytes(1024)`, `format_bytes(1048576)`, `format_bytes(1073741824)` | 全ての単位境界 |
| T2 | `field_value_display_duration` | `src/proc/mod.rs` | `format_duration(0.5)`, `format_duration(90.0)`, `format_duration(7200.0)`, `format_duration(90061.0)` | 秒/分/時/日 |
| T3 | `snapshot_export_import_roundtrip` | `src/export.rs` | serialize → file → deserialize → 比較 | Snapshot 全体の Serde 正当性 |
| T4 | `series_export_import_roundtrip` | `src/export.rs` | 複数 Snapshot の round-trip | Series 機能の正当性 |
| T5 | `diff_identical_returns_empty` | `src/proc/mod.rs` | 同一 Snapshot の diff | diff の基本正当性 |
| T6 | `diff_detects_change` | `src/proc/mod.rs` | 値を変えた Snapshot の diff | diff がちゃんと差分を検出 |
| T7 | `diff_ignores_small_float` | `src/proc/mod.rs` | Float の微小差 | 閾値ロジックの正しさ |
| T8 | `i18n_all_keys_have_translations` | `src/i18n.rs` | 全 T::* キーで t(En, key) != "?" && t(Ja, key) != "?" | 翻訳漏れの検出 |
| T9 | `i18n_source_descriptions_complete` | `src/i18n.rs` | 全 43 ソース名で source_description がデフォルトでない | ソース説明の漏れ検出 |
| T10 | `locale_from_str_variants` | `src/i18n.rs` | "ja", "jp", "japanese", "en", "unknown" | Locale パース |
| T11 | `systemtime_iso8601_roundtrip` | `src/proc/mod.rs` | timestamp の serialize/deserialize | 自前日付パーサーの信頼性 |

### MVP テスト一覧 (Phase 2: parse_content 分離後)

| # | テスト名 | 対象ファイル | fixture 内容 | 意義 |
|---|---------|-------------|-------------|------|
| T12 | `parse_uptime_content` | `src/proc/uptime.rs` | `"12345.67 98765.43\n"` | 最もシンプルなパーサー |
| T13 | `parse_loadavg_content` | `src/proc/loadavg.rs` | `"0.50 0.75 1.00 3/150 12345\n"` | `/` 区切りパース |
| T14 | `parse_meminfo_content` | `src/proc/meminfo.rs` | 代表的な 5 行の meminfo テキスト | kB → Bytes 変換 |
| T15 | `parse_version_content` | `src/proc/version.rs` | 実際の /proc/version テキスト | 正規表現的抽出 |
| T16 | `parse_stat_content` | `src/proc/stat.rs` | CPU 行 + その他行 | 複雑なマルチ行パース |

### MVP テスト一覧 (Phase 3: CI 実機 smoke test)

| # | テスト名 | 条件 | 意義 |
|---|---------|------|------|
| T17 | `all_parsers_smoke_test` | `#[cfg(target_os = "linux")]` | 43 パーサー全てが panic せずに Ok or Err を返す |
| T18 | `snapshot_capture_smoke` | `#[cfg(target_os = "linux")]` | `Snapshot::capture()` が成功し entries が空でない |

⚔️ リヴァイ: 「Phase 1 の T1-T11 は今すぐ書ける。parse_content の分離すら不要だ。言い訳は聞かない。」

🧪 千石: 「CI の話をしておく。GitHub Actions の `ubuntu-latest` には /proc がある。だが注意点がある。」

1. **`/proc/slabinfo` は root 権限が必要** → `parse()` が `Err` を返す → smoke test では `is_ok()` ではなく `is_ok() || is_err()` (= panic しないこと) を確認
2. **`/proc/pressure` は CONFIG_PSI 依存** → 同上
3. **プロセス一覧は環境依存** → 値の assertion は不可、構造の assertion のみ
4. **WSL2 の /proc は本物の Linux /proc** → WSL2 での開発中もテスト可能

→ **Spec implication: CI の smoke test は以下のパターン:**

```rust
#[cfg(target_os = "linux")]
#[test]
fn all_parsers_dont_panic() {
    // 各パーサーを呼び出し、panic しないことだけを確認
    // Ok でも Err でもよい (権限不足やカーネル設定依存のため)
    let _ = meminfo::parse();
    let _ = uptime::parse();
    let _ = loadavg::parse();
    // ... 全 43 パーサー ...
}

#[cfg(target_os = "linux")]
#[test]
fn snapshot_capture_returns_entries() {
    let snap = Snapshot::capture().unwrap();
    // 最低限いくつかの entry が存在すること
    assert!(snap.entries.len() >= 10,
        "Expected at least 10 entries, got {}", snap.entries.len());
    // 必ず存在するはずの entry
    assert!(snap.entries.contains_key("meminfo"));
    assert!(snap.entries.contains_key("uptime"));
    assert!(snap.entries.contains_key("loadavg"));
}
```

---

## テストヘルパー: make_test_snapshot

全 Phase で共有するテスト用 Snapshot を作るヘルパー関数:

```rust
// tests/helpers.rs or src/proc/mod.rs の #[cfg(test)] 内
fn make_test_snapshot() -> Snapshot {
    use std::collections::BTreeMap;
    use std::time::SystemTime;

    let mut entries = BTreeMap::new();

    entries.insert("meminfo".into(), ProcEntry {
        source: "/proc/meminfo".into(),
        fields: vec![
            Field {
                name: "MemTotal".into(),
                value: FieldValue::Bytes(16 * 1024 * 1024 * 1024), // 16 GiB
                unit: Some("kB".into()),
                description: "Total usable RAM".into(),
            },
            Field {
                name: "MemFree".into(),
                value: FieldValue::Bytes(8 * 1024 * 1024 * 1024), // 8 GiB
                unit: Some("kB".into()),
                description: "Free memory".into(),
            },
        ],
    });

    entries.insert("loadavg".into(), ProcEntry {
        source: "/proc/loadavg".into(),
        fields: vec![
            Field {
                name: "load_1min".into(),
                value: FieldValue::Float(0.50),
                unit: None,
                description: "1-minute load average".into(),
            },
        ],
    });

    entries.insert("uptime".into(), ProcEntry {
        source: "/proc/uptime".into(),
        fields: vec![
            Field {
                name: "uptime".into(),
                value: FieldValue::Duration(86400.0),
                unit: Some("seconds".into()),
                description: "System uptime".into(),
            },
        ],
    });

    Snapshot {
        timestamp: SystemTime::now(),
        entries,
    }
}
```

---

## Gap Summary (Session 007)

| # | Gap | Category | Observe / Suggest / Act |
|---|-----|----------|------------------------|
| G10-1 | /proc 不要のテストがゼロ | Quality gap | Observe: format_bytes, diff, export/import 全てテストなし → Suggest: Phase 1 で T1-T11 を即実装 → Act: `src/proc/mod.rs` と `src/export.rs` と `src/i18n.rs` に `#[cfg(test)] mod tests` 追加 |
| G10-2 | パーサーが I/O とロジックを混在 | Architecture gap | Observe: `parse()` が直接 `fs::read_to_string` → Suggest: `parse_content(&str)` を分離 → Act: 優先 5 パーサー (meminfo, loadavg, stat, net_tcp, cpuinfo) から開始 |
| G10-3 | FieldValue に PartialEq がない | Type gap | Observe: テストで値の比較ができない → Suggest: `#[derive(PartialEq)]` 追加 → Act: `FieldValue` に derive 追加 (Float は手動 impl 検討) |
| G10-4 | i18n の翻訳漏れ検出手段がない | Quality gap | Observe: 新キー追加時に ja 側を忘れても気づけない → Suggest: 全キー網羅テスト → Act: T::* の全定数を列挙して t() が "?" を返さないことを確認 |
| G10-5 | diff_snapshots のフィールド数不一致が無視される | Logic gap | Observe: `zip` のため短い方に切り詰め → Suggest: テストで検出 → Act: 将来的に zip_longest 化 |
| G10-6 | systemtime_iso8601 が自前実装でテストなし | Quality gap | Observe: 日付パース/フォーマットが手書き → Suggest: round-trip テスト + エッジケース → Act: T11 で対応 |
| G10-7 | CI の /proc 依存テストの権限問題 | CI gap | Observe: slabinfo 等は root 必要 → Suggest: smoke test は "panic しない" のみ確認 → Act: `let _ = xxx::parse();` パターン |

---

## Next Actions (優先順)

1. **[即実行] Phase 1: T1-T11 を実装** — /proc 不要、リファクタリング不要
   - `src/proc/mod.rs` に `#[cfg(test)] mod tests` を追加 (T1, T2, T5, T6, T7, T11)
   - `src/export.rs` に `#[cfg(test)] mod tests` を追加 (T3, T4) — `tempfile` crate を dev-dependencies に追加
   - `src/i18n.rs` に `#[cfg(test)] mod tests` を追加 (T8, T9, T10)
2. **[Phase 2] parse_content 分離** — 優先 5 パーサーから
   - `uptime.rs`, `loadavg.rs`, `meminfo.rs`, `version.rs`, `stat.rs`
   - 各ファイルに fixture ベースのテスト T12-T16 を追加
3. **[Phase 3] CI smoke test** — T17, T18
   - `.github/workflows/test.yml` に `cargo test` を追加
   - `#[cfg(target_os = "linux")]` 付きの smoke test
4. **[Phase 2 後] 残り 38 パーサーの parse_content 分離** — 段階的に
5. **[将来] FieldValue に PartialEq を追加** (G10-3)
6. **[将来] diff_snapshots の zip_longest 化** (G10-5)

---

## Cargo.toml 変更 (dev-dependencies)

```toml
[dev-dependencies]
tempfile = "3"
```

---

## 参考: テストファイル配置

```
src/
  proc/
    mod.rs          ← T1, T2, T5, T6, T7, T11 を #[cfg(test)] 内に
    uptime.rs       ← T12 を #[cfg(test)] 内に (parse_content 分離後)
    loadavg.rs      ← T13
    meminfo.rs      ← T14
    version.rs      ← T15
    stat.rs         ← T16
  export.rs         ← T3, T4 を #[cfg(test)] 内に
  i18n.rs           ← T8, T9, T10 を #[cfg(test)] 内に
tests/
  smoke.rs          ← T17, T18 (integration test, #[cfg(target_os = "linux")])
```
