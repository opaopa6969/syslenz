# DGE Session 005: G5/G6/G7 深掘り — 時系列 diff・アラート・マルチホスト

- **Date**: 2026-03-28
- **Theme**: 監視ツールとしての進化 — タイムトラベル・閾値通知・複数ホスト
- **Parent Gaps**: G5, G6, G7 (Session 001)
- **Characters**: 今泉 (questioner) + ヤン (lazy strategist) + 千石 (quality guardian) + Red Team (adversarial)

---

## 現状の整理

先輩 (ナレーション): 現在の実装を確認する。

- `App.snapshots: Vec<Snapshot>` — リングバッファ、最大 60 個（60 秒分）
- `App.refresh()` で `snapshots.push(old)` → 60 超で `snapshots.remove(0)`
- Diff ビュー (`View::Diff`) は `diff_snapshots(&old, &self.current)` で **直前 1 つ** との差分のみ
- Graph ビュー (`View::Graph`) は sparkline で 1 つの数値フィールドの時系列を表示
- SSH リモート: `--ssh user@host` で `stream_remote()` が mpsc チャネル経由でスナップショットを送信。ただし **1 ホスト限定**
- `stream_remote()` は `MAX_CONSECUTIVE_FAILURES = 5` で接続断に対応
- アラート / 通知機能: **なし**
- 自動リフレッシュ: 1000ms 間隔

---

## Scene 1: タイムトラベル diff — 過去のスナップショットを選ぶ

先輩: G5 の本質は「リングバッファに 60 個のスナップショットがあるのに、diff は直前としか比較できない」こと。

### 選択方式の議論

👤 今泉: 「5 分前と今を比べたい、って場面を想像してください。60 個のスナップショットが並んでて、どうやって選ぶんですか？リストですか？スライダーですか？」

☕ ヤン: 「キーバインドが一番シンプル。`[` と `]` で比較対象のスナップショットを前後に動かす。ステータスバーに "Comparing: current vs. T-35 (35 seconds ago)" って出す。Diff ビュー内でだけ有効にすれば、既存のキーバインドと衝突しない。」

→ **Gap 発見: スナップショットの選択 UI が存在しない。キーバインド方式なら実装が軽い。**

👤 今泉: 「でも 60 個を `[` 連打で移動するのは辛くないですか？」

☕ ヤン: 「`Shift+[` で 10 個飛ばし。`Home` で最古、`End` で直前。4 キーで十分でしょ。あとは... ステータスバーに "T-0 ... T-59" のミニマップ的な位置表示があればいい。紅茶ください。」

→ **Spec 提案: `App` に `diff_target_index: Option<usize>` を追加。`None` なら直前（従来動作）、`Some(i)` なら `snapshots[i]` と比較。**

### Diff UI の表示

🎋 千石: 「diff の中身は既に `diff_snapshots()` が返す `Vec<DiffItem>` で構造化されてる。比較対象を変えるだけなら、`refresh()` の中の `diff_snapshots(&old, &self.current)` の `old` を差し替えるだけ。UI 変更は最小限。」

→ **Spec 提案: `View::Diff` に入った時、ステータスバーにタイムスタンプと操作ガイドを表示。**

```
[Diff] current vs T-35 (14:23:05)  |  [ ] prev  [ ] next  Shift+[/] skip 10  Home/End
```

### Red Team の攻撃

🔴 Red Team: 「リングバッファが 60 個しかない。1 秒間隔なら 1 分分。"5 分前と比較したい" って今泉が言ったけど、5 分前のデータはもう消えてる。これ、根本的に足りないんじゃないか？」

☕ ヤン: 「バッファサイズを設定可能にすればいい。`--history 300` で 5 分分。メモリは... 1 Snapshot が仮に 100KB なら 300 個で 30MB。現代のサーバーなら問題ない。ただし G9（設定ファイル）と連動させるべき。`config.toml` の `history_size = 300` で永続化。」

→ **Gap 発見: リングバッファサイズが固定 60。設定可能にすべき。**

🔴 Red Team: 「もう一つ。リモートモードで SSH 接続が diff の最中に切れたらどうなる？T-30 を見てるときに新しいスナップショットが来なくなる。ユーザーは "固まった" と思うのか、"最後のデータを見てる" と理解できるのか？」

🎋 千石: 「ステータスバーに接続状態を常時表示すべき。`[SSH: user@host] Connected (last: 14:23:06)` が `[SSH: user@host] Disconnected (last: 14:23:06, 15s ago)` に変わる。色も緑から黄色に。これは G5 だけの問題じゃなく、リモートモード全体の UX。」

→ **Spec 提案: `App` に `last_snapshot_time: Instant` と `connection_status: ConnectionStatus` を追加。ステータスバーで常時表示。**

### 実装設計

👤 今泉: 「具体的にどの構造体を変えるんですか？」

☕ ヤン: 「最小限の変更リスト:」

```rust
// app.rs に追加
pub struct App {
    // ... 既存フィールド ...
    pub diff_target_index: Option<usize>,  // None = 直前, Some(i) = snapshots[i]
    pub max_snapshots: usize,              // デフォルト 60, --history で変更可能
}

// refresh() の変更
// diff_target_index が Some の場合、diff_snapshots の第1引数を snapshots[i] にする

// キーバインド (Diff ビュー内のみ)
// '[' → diff_target_index を1つ前に
// ']' → diff_target_index を1つ後に
// Shift+'[' → 10個前に
// Shift+']' → 10個後に
// Home → snapshots[0] (最古)
// End → None (直前、デフォルトに戻る)
```

→ **Spec 提案: `diff_target_index` ベースの実装。既存の `diff_snapshots()` ロジックは変更不要。**

---

## Scene 2: アラート閾値と通知

先輩: G6 — 「メモリ 90% 超えたら教えてほしい」という要望。現状ゼロ。

### アラートルールの定義

👤 今泉: 「閾値ってどこで定義するんですか？設定ファイル？コマンドライン？TUI 内？」

☕ ヤン: 「3 段階で考えよう。MVP は設定ファイル。TOML でシンプルに。」

```toml
# ~/.config/syslenz/config.toml

[[alert]]
source = "meminfo"
field = "MemAvailable"
condition = "< 500_000_000"   # 500MB 未満
severity = "critical"
message = "Available memory below 500MB"

[[alert]]
source = "loadavg"
field = "load_1min"
condition = "> 8.0"
severity = "warning"
message = "Load average exceeded 8.0"

[[alert]]
source = "pressure"
field = "cpu_some_avg10"
condition = "> 50.0"
severity = "warning"
message = "CPU pressure > 50%"
```

→ **Spec 提案: `AlertRule` 構造体を定義。`source`, `field`, `condition` (比較演算子 + 数値), `severity` (info/warning/critical), `message`。**

🎋 千石: 「condition のパース。`> 8.0`, `< 500000`, `>= 90`, `!= 0` くらいサポートすれば十分。正規表現はいらない。比較対象は数値フィールド (`FieldValue::Bytes`, `Integer`, `Float`, `Duration`) のみ。文字列フィールドの比較は MVP では不要。」

→ **Spec 提案: 比較演算子は `>`, `<`, `>=`, `<=`, `==`, `!=` の 6 種。数値のみ。**

### TUI での表示方式

👤 今泉: 「アラートが発火したら、TUI 上でどう見せるんですか？ポップアップ？色？音？」

☕ ヤン: 「ポップアップは邪魔。ステータスバーの右側にアラートカウンターを常時表示。」

```
[syslenz] 43 sources | T-0..T-59 | [!2 WARN] [!!1 CRIT]
```

🎋 千石: 「加えて、サイドバーのソース名に色をつける。meminfo がアラート中なら赤文字。これで "どのソースが問題か" が一目でわかる。Detail ビューのフィールドも、閾値超えてるフィールドは背景色を変える。」

→ **Spec 提案: アラート表示は 3 箇所。(1) ステータスバーのカウンター、(2) サイドバーのソース名着色、(3) Detail ビューのフィールド背景色。**

```
severity 色マッピング:
- info:     Cyan (前景)
- warning:  Yellow (前景) + DarkGray (背景)
- critical: Red (前景、太字) + DarkRed (背景)
```

### アラート履歴

🎋 千石: 「発火したアラートはログしておくべき。`Vec<AlertEvent>` に timestampとルールと値を記録。`'a'` キーでアラート履歴ビューを開く。`View::Alerts` を追加。」

→ **Spec 提案: `View::Alerts` を追加。アラート履歴のリスト表示。**

```rust
pub struct AlertEvent {
    pub timestamp: Instant,
    pub rule: AlertRule,
    pub actual_value: f64,
}

pub struct App {
    // ... 既存 ...
    pub alert_rules: Vec<AlertRule>,
    pub active_alerts: Vec<AlertEvent>,     // 現在発火中
    pub alert_history: Vec<AlertEvent>,     // 過去のアラート（最大100件）
}
```

### Red Team の攻撃

🔴 Red Team: 「アラートストーム。loadavg が 8.0 を超えて、毎秒アラートが発火する。1 分で 60 件。画面がアラートで埋まらないか？」

☕ ヤン: 「デバウンス。同一ルールは "発火 → 解除 → 再発火" のサイクルでしか新しいイベントを作らない。`active_alerts` にある間は再発火しない。解除されたら `alert_history` に移す。」

→ **Spec 提案: アラートのデバウンス。同一ルールで連続発火しない。状態遷移: `Normal → Firing → (condition解除) → Resolved → Normal`。**

🔴 Red Team: 「条件式が不正な設定を書いたらどうなる？`condition = "hello world"` とか。起動時にクラッシュするのか？」

🎋 千石: 「設定読み込み時にバリデーション。不正なルールはスキップして警告をステータスバーに表示。起動を止めてはいけない。ゼロ設定がコアバリューなんだから、設定ファイルが壊れてても起動できなきゃ本末転倒。」

→ **Spec 提案: 設定パースエラーはログ + ステータスバー警告。アプリ起動は止めない。**

🔴 Red Team: 「TUI 以外への通知は？syslenz をバックグラウンドで走らせて、デスクトップ通知や Slack に飛ばしたい。」

☕ ヤン: 「MVP ではやらない。ただし拡張ポイントは残す。`AlertAction` enum に `Tui` の他に将来 `Command(String)` を追加できるように。`command = "notify-send 'syslenz: {message}'"` みたいな。V2 の話。」

→ **Spec 提案: MVP はTUI表示のみ。将来の `alert.action` フィールドで外部コマンド実行に対応する拡張ポイントを設計に含める。**

---

## Scene 3: マルチホスト監視アーキテクチャ

先輩: G7 — 現在は `--ssh user@host` で 1 ホストのみ。`remote.rs` の `stream_remote()` が 1 つの mpsc チャネルを返す。

### アーキテクチャ選択肢

👤 今泉: 「複数ホストって、どういう画面になるんですか？タブ？分割画面？」

☕ ヤン: 「3 パターン考えられる:」

```
(A) タブ方式: Tab キーでホスト切り替え。1 画面 = 1 ホスト。
    → シンプル、実装コスト低い
    → 同時比較ができない

(B) 分割画面: ホストごとにペイン分割。
    → 同時比較できる
    → 3 ホスト以上で画面が狭すぎる

(C) アグリゲートビュー: 全ホストの同一フィールドを1テーブルに並べる。
    → "全サーバーの load_1min を一覧" ができる
    → 実装コスト高い
```

☕ ヤン: 「MVP は (A) タブ方式。これなら `App` の構造変更が最小限。将来 (C) を追加する。」

→ **Spec 提案: MVP はタブ方式。`Ctrl+1`〜`Ctrl+9` でホスト切り替え。タブバーをステータスバーの上に表示。**

### 内部設計

🎋 千石: 「問題は `App` の設計。今は `App.current: Snapshot` が 1 つ。これを複数ホストに対応させる。」

```rust
pub struct HostState {
    pub host: String,                          // "localhost" or "user@host"
    pub snapshots: Vec<Snapshot>,              // リングバッファ
    pub current: Snapshot,
    pub diffs: Vec<DiffItem>,
    pub remote_rx: Option<mpsc::Receiver<Snapshot>>,
    pub connection_status: ConnectionStatus,
    pub alert_events: Vec<AlertEvent>,
}

pub enum ConnectionStatus {
    Local,
    Connected { last_seen: Instant },
    Disconnected { last_seen: Instant, since: Instant },
    Connecting,
}

pub struct App {
    pub hosts: Vec<HostState>,
    pub active_host: usize,                    // タブのインデックス
    // ... UI state (view, focus, etc.) は共通 ...
    pub alert_rules: Vec<AlertRule>,           // ルールは全ホスト共通
}
```

☕ ヤン: 「コマンドラインは `--ssh user@host1 --ssh user@host2` の複数指定。ローカルホストは常に hosts[0]。」

→ **Spec 提案: `--ssh` フラグの複数指定。`HostState` 構造体に Snapshot 管理を分離。**

### Red Team の攻撃

🔴 Red Team: 「5 ホスト接続して、3 番目のホストの SSH が切れた。diff ビューで T-30 を見てる最中に。何が起きる？」

🎋 千石: 「ホストごとに独立した `ConnectionStatus` を持つ。接続断はそのホストのタブにだけ影響。タブバーで色を変える — 緑=接続中、黄=接続断、灰=再接続中。他のホストの操作は影響を受けない。」

→ **Spec 提案: ホストごとに独立した接続状態管理。タブバーにステータス色表示。**

🔴 Red Team: 「10 ホスト接続。各 1 秒間隔。毎秒 10 回の SSH コマンド実行。ネットワーク帯域とリモートホストの負荷は？」

☕ ヤン: 「各 `stream_remote()` は独立スレッド。SSH のオーバーヘッドは確かに大きい。長期的には SSH のパーシステント接続 (`ControlMaster`) を使うか、リモート側で `syslenz --stream` を実装して 1 接続で連続 JSON 出力にすべき。MVP では注意書きだけ。README に "10+ hosts may cause significant SSH overhead" と書く。」

→ **Gap 発見: 各スナップショットで SSH 接続を毎回張り直している。`ControlMaster` またはパーシステント接続の最適化が将来必要。**

🔴 Red Team: 「マルチホスト + アラート。ホスト A で critical、ホスト B で warning。アラートカウンターはホスト別？全体？」

🎋 千石: 「両方。ステータスバーには全体のカウント。タブバーにはホスト別のインジケータ。」

```
[host1: ok] [host2: !1W] [host3: !!1C] [host4: ok]
─────────────────────────────────────────────────
[syslenz] Total: [!1 WARN] [!!1 CRIT] | 43 sources
```

→ **Spec 提案: アラートカウンターはタブバー（ホスト別）+ ステータスバー（全体集計）の 2 層。**

---

## Scene 4: 各ギャップの MVP 定義

先輩: 3 つのギャップそれぞれの MVP を定義する。

### G5 MVP: タイムトラベル diff

☕ ヤン: 「最小構成:」

1. `App.diff_target_index: Option<usize>` を追加
2. `View::Diff` 内で `[` / `]` キーで比較対象を移動
3. ステータスバーに比較対象のタイムスタンプ（正確には "T-N"）を表示
4. `diff_snapshots()` の呼び出し元を変更（`old` を `snapshots[i]` にする）

🎋 千石: 「テスト:」
- `diff_target_index` が `None` のとき従来通り動くこと
- `snapshots` が空のとき panic しないこと
- `diff_target_index` が `snapshots.len()` 以上にならないバウンドチェック

→ **Spec 提案: G5 MVP は 4 項目。実装量は app.rs のキーバインド追加 + ステータスバー表示変更。diff ロジック本体の変更は不要。**

### G6 MVP: アラート

☕ ヤン: 「最小構成:」

1. `AlertRule` 構造体 + TOML からのデシリアライズ
2. `App.refresh()` の末尾でルール評価 → `active_alerts` を更新
3. ステータスバーにアラートカウンター表示
4. サイドバーのソース名に色付け（warning=黄, critical=赤）
5. デバウンス（同一ルールの連続発火防止）

🎋 千石: 「テスト:」
- `condition = "> 8.0"` のパースが正しいこと
- 閾値超え → 発火 → 閾値回復 → 解除 のサイクル
- 不正な condition がスキップされること

🔴 Red Team: 「G9（設定ファイル）がまだ実装されてない。アラートの設定はどこに書く？」

☕ ヤン: 「G6 と G9 は同時に実装すべき。`config.toml` の読み込みを先にやって、その中に `[[alert]]` セクションを含める。G9 の MVP = config.toml の読み込み + アラートルール。他の設定項目（言語、リフレッシュ間隔等）は後でいい。」

→ **Spec 提案: G6 は G9 の部分実装を前提とする。`~/.config/syslenz/config.toml` の `[[alert]]` セクション読み込みを先行実装。**

### G7 MVP: マルチホスト

☕ ヤン: 「最小構成:」

1. `HostState` 構造体に Snapshot 管理を切り出し
2. `App.hosts: Vec<HostState>` + `App.active_host: usize`
3. `--ssh` の複数指定対応（clap の `Vec<String>` 化）
4. タブバー表示（ホスト名 + 接続状態色）
5. `Ctrl+1`〜`Ctrl+9` または `F1`〜`F9` でタブ切り替え

🎋 千石: 「リファクタリングの影響が大きい。`App` のほぼ全メソッドが `self.current` を参照してる。これを `self.hosts[self.active_host].current` にする必要がある。`active_host_state(&self) -> &HostState` ヘルパーを作って段階的に移行すべき。」

🔴 Red Team: 「G7 のリファクタリングは G5, G6 にも影響する。diff_target_index はホストごとに持つのか？アラートのアクティブ状態はホストごとか？」

🎋 千石: 「正しい。`diff_target_index` は `HostState` に含めるべき。アラートルールは `App` 共通、アラートイベントは `HostState` ごと。つまり G7 を先に設計しないと G5, G6 の構造体設計がやり直しになる。」

→ **Spec 提案: 実装順序は G7 の構造体設計（HostState 切り出し）→ G5 → G6。ただし G7 の UI（タブ等）は最後でもいい。**

### 推奨実装順序

```
Phase 1: 構造体リファクタリング
  - HostState 切り出し (G7 の基盤)
  - active_host_state() ヘルパー
  - 既存テストが通ることを確認

Phase 2: G5 タイムトラベル diff
  - HostState.diff_target_index
  - キーバインド ([, ], Shift+[, Shift+], Home, End)
  - ステータスバー表示

Phase 3: G9 (部分) + G6 アラート
  - config.toml 読み込み
  - AlertRule パーサー
  - ルール評価 + デバウンス
  - TUI 表示 (ステータスバー、サイドバー着色)

Phase 4: G7 マルチホスト UI
  - --ssh 複数指定
  - タブバー + 切り替えキーバインド
  - ホスト別接続状態表示
```

---

## Gap Summary (Session 005)

| # | Gap | Category | Severity | Observe / Suggest / Act |
|---|-----|----------|----------|------------------------|
| G5 | 時系列 diff (任意時点の比較) がない | Missing logic | Medium | Observe: リングバッファ 60 個あるのに直前 1 個としか比較できない → Suggest: `diff_target_index` + `[`/`]` キーバインド → Act: Diff ビュー内にスナップショット選択 UI |
| G5-1 | リングバッファサイズが固定 60 | Missing logic | Low | Observe: 1 分以上前のデータが消える → Suggest: `--history N` + config.toml の `history_size` → Act: `max_snapshots` フィールド追加 |
| G5-2 | リモート接続状態がユーザーに見えない | UX gap | Medium | Observe: SSH 切断時に UI がフリーズしたように見える → Suggest: `ConnectionStatus` + ステータスバー表示 → Act: 接続状態の常時表示 |
| G6 | 閾値アラート / 通知がない | Missing logic | Medium | Observe: 監視ツールなのに能動的通知がゼロ → Suggest: TOML ベースの `AlertRule` + TUI 3 箇所表示 → Act: ルール評価エンジン + 表示 |
| G6-1 | アラートストーム対策がない | Missing logic | Medium | Observe: 毎秒発火するルールで画面が埋まる → Suggest: デバウンス (Normal→Firing→Resolved サイクル) → Act: 状態遷移の実装 |
| G6-2 | 不正な設定でアプリが起動不能になるリスク | UX gap | Medium | Observe: condition パースエラーで panic の可能性 → Suggest: エラースキップ + 警告表示 → Act: バリデーション + graceful degradation |
| G6-3 | TUI 外への通知手段がない | Missing logic | Low | Observe: バックグラウンド運用で通知が見えない → Suggest: 将来の `alert.action = "command"` 拡張ポイント → Act: MVP では設計のみ、実装は V2 |
| G7 | マルチホスト監視が未対応 | Missing logic | Low | Observe: `--ssh` が 1 ホスト限定 → Suggest: `HostState` 分離 + タブ方式 → Act: 段階的リファクタリング |
| G7-1 | SSH 接続が毎回張り直し | Performance | Low | Observe: 10 ホスト × 毎秒 = 毎秒 10 SSH 接続 → Suggest: ControlMaster or persistent connection → Act: MVP では注意書き、V2 で最適化 |
| G7-2 | アラートの集計表示が未設計 | UX gap | Low | Observe: マルチホスト時のアラート表示場所が不明 → Suggest: タブバー (ホスト別) + ステータスバー (全体) の 2 層 → Act: G6 と G7 の統合設計 |
| G7-3 | G5/G6/G7 の実装順序依存 | Architecture | Medium | Observe: 構造体設計が G7 に依存 → Suggest: HostState 切り出しを最初に → Act: Phase 1 で構造体リファクタリング |

## Spec Proposals (具体)

### Spec S5: タイムトラベル diff

```rust
// app.rs
pub struct HostState {
    pub diff_target_index: Option<usize>,  // None=直前, Some(i)=snapshots[i]
    pub max_snapshots: usize,              // default 60
    // ...
}

// キーバインド (View::Diff 内のみ)
// '['  → diff_target_index を 1 つ古い方へ
// ']'  → diff_target_index を 1 つ新しい方へ (None まで戻る)
// '{'  (Shift+[) → 10 個古い方へ
// '}'  (Shift+]) → 10 個新しい方へ
// Home → Some(0) (最古)
// End  → None (直前)
```

### Spec S6: アラートシステム

```toml
# ~/.config/syslenz/config.toml
[[alert]]
source = "meminfo"
field = "MemAvailable"
condition = "< 500000000"
severity = "critical"          # info | warning | critical
message = "Memory critically low"
```

```rust
pub struct AlertRule {
    pub source: String,
    pub field: String,
    pub op: CompareOp,          // Gt, Lt, Gte, Lte, Eq, Neq
    pub threshold: f64,
    pub severity: Severity,     // Info, Warning, Critical
    pub message: String,
}

pub enum AlertState { Normal, Firing(Instant), Resolved(Instant) }
```

### Spec S7: マルチホストタブ

```
コマンドライン: syslenz --ssh user@web1 --ssh user@web2 --ssh user@db1

タブバー表示:
 [localhost] [web1: ok] [web2: !1W] [db1: !!1C]

切り替え: Ctrl+1..9 or F1..F9
```

## Next Actions

- [ ] Phase 1: `HostState` 構造体切り出しリファクタリング
- [ ] Phase 2: G5 タイムトラベル diff 実装
- [ ] Phase 3: G9 (config.toml) 部分実装 + G6 アラート実装
- [ ] Phase 4: G7 マルチホスト UI 実装
- [ ] 各 Phase で Red Team 指摘事項のテストケースを追加
