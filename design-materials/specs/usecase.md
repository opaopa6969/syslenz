# syslenz ユースケース仕様書

- **作成日**: 2026-03-28
- **ソース**: DGE Sessions 001-008
- **ステータス**: Draft

---

## 概要

syslenz は Linux の /proc ファイルシステムを構造化・型付きデータとしてパースし、TUI で閲覧・監視・エクスポートする Rust 製ツールである。本文書は DGE セッションで特定された 3 ペルソナと全ユースケースを網羅的に定義する。

### コンセプト

> **"Wireshark for /proc"** --- /proc の全 43 ソースを構造化・型付きデータとしてゼロ設定で可視化する。

### 3 つの強み

1. **ゼロ設定** --- バイナリ 1 つ、設定ファイルゼロ、root 不要。ssh して実行するだけ。
2. **構造化エクスポート** --- 全フィールドが型付き (Bytes, Duration, Table...) で JSON export 可能。
3. **教育性** --- 全フィールドに人間が読める説明文が付属。/proc の辞書として機能する。

---

## ペルソナ定義

DGE Session 002 で確定した 3 ペルソナ:

| ID | ペルソナ | 説明 | 主な利用場面 |
|----|---------|------|-------------|
| **(A)** | **SRE / インフラエンジニア** | ssh して障害切り分け、パフォーマンス調査を行う。Datadog や Prometheus を補完するツールとして、またはそれらを導入できない環境での代替として使用。 | 障害時の深掘り調査、リアルタイム監視、マルチホスト比較 |
| **(B)** | **Linux 学習者 / CS 学生** | /proc の中身を理解したい。OS の授業やセルフスタディで Linux の内部構造を学ぶ。 | /proc ソースの閲覧、フィールド説明の参照、構造化データの学習 |
| **(C)** | **セキュリティ監査員** | システム状態を JSON export して監査レポートに添付する。/proc/net/tcp の全コネクション、/proc/modules のカーネルモジュール一覧等を証跡として保存。 | スナップショット取得、JSON export、ホスト間差分比較 |

---

## ユースケース一覧

### カテゴリ別索引

| カテゴリ | ID 範囲 | 件数 | 概要 |
|---------|---------|------|------|
| 基本操作 | UC-BAS-01 ~ UC-BAS-06 | 6 | TUI の基本的なナビゲーションと操作 |
| ダッシュボード | UC-DSH-01 ~ UC-DSH-03 | 3 | ダッシュボードとウェルカム画面 |
| 監視 | UC-MON-01 ~ UC-MON-07 | 7 | リアルタイム監視、diff、アラート、リモート |
| エクスポート | UC-EXP-01 ~ UC-EXP-04 | 4 | JSON export/import、OpenTelemetry |
| 設定 | UC-CFG-01 ~ UC-CFG-02 | 2 | config.toml とアラートルール |
| Web/Widget | UC-WEB-01 ~ UC-WEB-02 | 2 | Web UI と X11 ウィジェット |

---

## 基本操作 (UC-BAS)

### UC-BAS-01: TUI 起動してソース一覧を閲覧

| 項目 | 内容 |
|------|------|
| **Trigger** | ユーザーがターミナルで `syslenz` を実行する |
| **Actor** | (A) SRE, (B) 学習者, (C) 監査員 --- 全ペルソナ共通 |
| **Input** | なし (ゼロ設定で起動) |
| **Output** | TUI 画面: 左サイドバーに 43 の /proc ソース一覧、右コンテンツエリアに選択中ソースの詳細 |

**メインフロー:**

1. ユーザーがターミナルで `syslenz` を実行する
2. アプリケーションが /proc ファイルシステムから全 43 ソースをパースし、Snapshot を構築する
3. デフォルトビュー (Dashboard) が表示される (UC-DSH-01 参照)
4. ユーザーが `j/k` でサイドバーのソースを上下に移動する
5. 選択中のソースに対応するフィールド一覧がコンテンツエリアに表示される

**代替フロー:**

- (A-1) `--import <file>` 指定時: ファイルから Snapshot を読み込み、View::Overview で起動する (Dashboard ではない、ライブデータがないため)
- (A-2) `--ssh user@host` 指定時: リモートホストの /proc を読み込み、Dashboard で起動する

**関連 Gap:** G2-1 (初期表示が buddyinfo だった問題 --- Dashboard デフォルト化で解消), G3-2 (初回起動時に何をすべきかわからない)

---

### UC-BAS-02: ソースを選択してフィールド詳細を閲覧

| 項目 | 内容 |
|------|------|
| **Trigger** | サイドバーでソースが選択された状態で `Enter` キーを押す |
| **Actor** | (B) 学習者 (フィールドの意味を理解したい), (A) SRE (特定の値を確認したい) |
| **Input** | 選択されたソース名 (例: `meminfo`, `net/tcp`, `loadavg`) |
| **Output** | Detail ビュー: Name / Value / Unit / Description の 4 列テーブル |

**メインフロー:**

1. ユーザーがサイドバーで目的のソースにカーソルを合わせる (`j/k` で移動)
2. `Enter` を押してコンテンツエリアにフォーカスを移動する (View::Detail)
3. フィールドテーブルが表示される --- 各行に名前、現在値、単位、説明文が表示
4. `j/k` でフィールド間を移動し、値と説明を確認する
5. `BS` (Backspace) または `Esc` でサイドバーに戻る

**代替フロー:**

- (A-1) `?` キーでヘルプモードをトグルし、フィールド説明の表示/非表示を切り替える

**関連 Gap:** G1-6 (教育ユースケース未明示 --- Description 列が教育機能の中核)

---

### UC-BAS-03: テーブルデータをドリルイン表示

| 項目 | 内容 |
|------|------|
| **Trigger** | Detail ビューで Table 型フィールド (例: net/tcp のコネクション一覧) を選択して `Enter` |
| **Actor** | (A) SRE (コネクション一覧や CPU ごとの統計を見たい), (C) 監査員 (全コネクションを監査) |
| **Input** | Table 型の FieldValue を持つフィールド |
| **Output** | TableView: 行列形式のテーブル表示、ヘッダー付き |

**メインフロー:**

1. Detail ビューで Table 型フィールドにカーソルを合わせる
2. `Enter` で View::TableView に遷移する
3. テーブルがヘッダー行 + データ行で表示される
4. `j/k` で行をスクロール、`h/l` で列をスクロール (幅が広い場合)
5. `BS` で Detail ビューに戻る

**代替フロー:**

- なし

**関連 Gap:** なし (既存機能として実装済み)

---

### UC-BAS-04: ソースを検索 (/ キー)

| 項目 | 内容 |
|------|------|
| **Trigger** | 任意のビューで `/` キーを押す |
| **Actor** | (A) SRE (43 ソースから素早く目的のソースを見つけたい) |
| **Input** | 検索文字列 (例: `mem`, `net`, `cpu`) |
| **Output** | サイドバーのソース一覧がインクリメンタルにフィルタされる |

**メインフロー:**

1. ユーザーが `/` キーを押す
2. ステータスバーに検索入力フィールドが表示される
3. 文字を入力するたびにサイドバーのソース一覧がインクリメンタルにフィルタされる
4. `Enter` で検索結果の先頭ソースを選択し、検索モードを終了する
5. `Esc` で検索をキャンセルし、元のソース一覧に戻る

**代替フロー:**

- (A-1) 検索文字列に一致するソースがない場合、サイドバーは空のまま表示される

**関連 Gap:** なし (既存機能として実装済み)

---

### UC-BAS-05: 言語切り替え (L キー)

| 項目 | 内容 |
|------|------|
| **Trigger** | 任意のビューで `L` キーを押す |
| **Actor** | (B) 学習者 (日本語で /proc の説明を読みたい), 全ペルソナ |
| **Input** | なし |
| **Output** | UI テキスト、ソース説明、フィールド説明が英語/日本語で切り替わる |

**メインフロー:**

1. ユーザーが `L` キーを押す
2. 現在のロケールが `En` なら `Ja` に、`Ja` なら `En` に切り替わる
3. サイドバーのソース説明、Detail ビューのフィールド説明、ステータスバーのテキストが即座に切り替わる

**代替フロー:**

- (A-1) config.toml に `lang = "ja"` が設定されている場合、起動時から日本語で表示される (UC-CFG-01 参照)
- (A-2) `--lang ja` CLI 引数でも起動時の言語を指定可能

**関連 Gap:** G3-3 (Welcome/Dashboard テキストの i18n 未対応), G9-1 (config に入れるべき項目が未整理)

---

### UC-BAS-06: ヘルプ表示トグル (? キー)

| 項目 | 内容 |
|------|------|
| **Trigger** | 任意のビューで `?` キーを押す |
| **Actor** | (B) 学習者 (フィールドの詳細な説明を確認したい) |
| **Input** | なし |
| **Output** | Detail ビューのフィールドテーブルに Description 列が表示/非表示される |

**メインフロー:**

1. ユーザーが `?` キーを押す
2. ヘルプモードがトグルされる (`show_help: bool`)
3. ヘルプモード ON: フィールドの Description 列が表示される
4. ヘルプモード OFF: Description 列が非表示になり、Name/Value/Unit のみのコンパクト表示になる

**代替フロー:**

- なし

**関連 Gap:** G3-1 (ヘルプへの導線が弱い --- `?` の存在を知らないとヘルプにたどり着けない。Welcome 画面で `?` キーの存在を告知)

---

## ダッシュボード (UC-DSH)

### UC-DSH-01: ダッシュボードで主要メトリクスを一覧 (D キー)

| 項目 | 内容 |
|------|------|
| **Trigger** | アプリケーション起動時 (デフォルトビュー)、または任意のビューで `D` キーを押す |
| **Actor** | (A) SRE (サーバーの全体状況を 3 秒で把握したい) |
| **Input** | なし |
| **Output** | Dashboard ビュー: loadavg, uptime, meminfo, stat, net/dev の厳選フィールドをサイドバーなし全幅で表示 |

**メインフロー:**

1. `syslenz` を起動する、または任意のビューで `D` キーを押す
2. View::Dashboard に遷移する (サイドバーは非表示)
3. 以下のレイアウトで主要メトリクスが表示される:
   - **ヘッダー行**: uptime + load_1, load_5, load_15
   - **中段左**: Memory (MemTotal, MemFree, MemAvailable, SwapTotal, SwapFree)
   - **中段右**: CPU (User%, System%, Idle%, IOWait%, IRQ%)
   - **下段**: Network (net/dev のインタフェースごと RX/TX)
4. 自動リフレッシュ (1 秒間隔) でダッシュボードの値がリアルタイム更新される
5. `j/k` でダッシュボードのセクション (loadavg, meminfo, stat, net/dev) を選択できる

**代替フロー:**

- (A-1) `--import` モードでは起動時のデフォルトが View::Overview (ライブデータでないため Dashboard は不適切)
- (A-2) スナップショットが 2 つ以上蓄積された場合、フィールド値に diff カラーリングを適用 (増加=Green, 減少=Red) --- MVP 後の機能

**関連 Gap:** G2-1 (初期表示が BTreeMap alphabetical 順だった), G2-2 (重要ソースのサマリーが一目で見えない), G2-5 (レート計算 bytes/sec がない --- MVP では累計値のみ), G3-4 (インポートモードでの Dashboard の不適切さ)

---

### UC-DSH-02: ダッシュボードから Detail へドリルイン

| 項目 | 内容 |
|------|------|
| **Trigger** | Dashboard ビューでセクションを選択して `Enter` を押す |
| **Actor** | (A) SRE (ダッシュボードで異常を見つけ、詳細を確認したい) |
| **Input** | 選択中の Dashboard セクション (0=loadavg/uptime, 1=meminfo, 2=stat, 3=net/dev) |
| **Output** | 対応ソースの Detail ビュー (全フィールド表示) |

**メインフロー:**

1. Dashboard ビューで `j/k` を使いセクションを選択する (選択中セクションはボーダー色がハイライト)
2. `Enter` を押す
3. `selected_dashboard_section` に対応するソース (例: 1 → meminfo) の Detail ビューに遷移する
4. `came_from_dashboard = true` がセットされる
5. `BS` または `Esc` で Dashboard に戻る (came_from_dashboard の判定による)

**代替フロー:**

- (A-1) サイドバーから Detail に入った場合は `BS` で Overview に戻る (既存動作を維持)

**関連 Gap:** G2-3 (Dashboard から Detail へのドリルインがない)

---

### UC-DSH-03: Welcome 画面でキーバインド確認 (W キー)

| 項目 | 内容 |
|------|------|
| **Trigger** | 任意のビューで `W` キーを押す |
| **Actor** | (B) 学習者 (初めて syslenz を触る、操作方法がわからない) |
| **Input** | なし |
| **Output** | Welcome 画面: ツール名、タグライン、主要キーバインド一覧、CTA |

**メインフロー:**

1. ユーザーが `W` キーを押す
2. View::Welcome に遷移する (サイドバーは非表示、全幅中央寄せ)
3. 以下が表示される:
   - ツール名 `syslenz` (Cyan, Bold)
   - タグライン `Wireshark for /proc` (DarkGray)
   - キーバインド一覧 (j/k, Enter, d, /, g, ?, L の 7 操作)
   - CTA: "Press Enter or D for Dashboard" (Green, Bold)
4. `Enter` で Dashboard に遷移する
5. `BS` で直前のビューに戻る

**代替フロー:**

- (A-1) 日本語ロケールの場合、キーバインド説明と CTA が日本語で表示される

**関連 Gap:** G3-1 (ヘルプへの導線が弱い), G3-2 (初回起動時に何をすべきかわからない), G3-3 (Welcome テキストの i18n)

---

## 監視 (UC-MON)

### UC-MON-01: 自動リフレッシュで変化を監視

| 項目 | 内容 |
|------|------|
| **Trigger** | アプリケーション起動後、自動的に定期リフレッシュが開始される |
| **Actor** | (A) SRE (サーバーの状態をリアルタイムで監視したい) |
| **Input** | リフレッシュ間隔 (デフォルト: 1000ms、`--interval` または config.toml で変更可能) |
| **Output** | 各ビューの値がリフレッシュ間隔ごとに最新の /proc データに更新される |

**メインフロー:**

1. アプリケーションが起動すると、設定されたリフレッシュ間隔で `App.refresh()` が呼ばれる
2. /proc から全ソースを再パースし、新しい Snapshot を生成する
3. 直前の Snapshot がリングバッファ (`snapshots: Vec<Snapshot>`) に追加される (最大 60 個、`--history` で変更可能)
4. 現在表示中のビュー (Dashboard, Detail, Diff, Graph) の値が更新される
5. Diff ビューの場合、差分情報も再計算される

**代替フロー:**

- (A-1) SSH リモートモードでは、`stream_remote()` が mpsc チャネル経由でスナップショットを送信する。接続断が `MAX_CONSECUTIVE_FAILURES = 5` 回で検知される
- (A-2) リングバッファサイズは `config.toml` の `history_size` で変更可能 (G5-1)

**関連 Gap:** G5-1 (リングバッファサイズが固定 60)

---

### UC-MON-02: Diff ビューで変更箇所を確認

| 項目 | 内容 |
|------|------|
| **Trigger** | 任意のビューで `d` キーを押す |
| **Actor** | (A) SRE (前回リフレッシュからの変化を一目で把握したい) |
| **Input** | 現在の Snapshot と比較対象の Snapshot |
| **Output** | Diff ビュー: 変化したフィールドのみ表示、増加=Green, 減少=Red のカラーリング |

**メインフロー:**

1. ユーザーが `d` キーを押す
2. View::Diff に遷移する
3. `diff_snapshots(&old, &current)` が実行され、`Vec<DiffItem>` が生成される
4. 変化したフィールドのみがリスト表示される (変化なしフィールドは非表示)
5. 各行に old_value と new_value が並び、増加は Green、減少は Red で色付けされる

**代替フロー:**

- (A-1) スナップショットが 1 つ以下の場合、Diff ビューは空 ("No changes yet" 表示)
- (A-2) Float 値の微小変化 (差が 0.001 以下) は diff として検出されない

**関連 Gap:** G5 (任意時点との比較ができない --- UC-MON-03 で解消), G10-5 (diff_snapshots のフィールド数不一致が無視される)

---

### UC-MON-03: 時系列 diff で任意時点と比較 ([ ] キー)

| 項目 | 内容 |
|------|------|
| **Trigger** | Diff ビュー (View::Diff) で `[` または `]` キーを押す |
| **Actor** | (A) SRE (5 分前と今の状態を比較して障害原因を切り分けたい) |
| **Input** | リングバッファ内のスナップショットインデックス |
| **Output** | 選択した過去スナップショットと現在の Snapshot との diff |

**メインフロー:**

1. ユーザーが Diff ビューに遷移する (`d` キー)
2. デフォルトでは直前のスナップショットとの diff が表示される (`diff_target_index: None`)
3. `[` キーを押すと比較対象が 1 つ前 (より古い) のスナップショットに移動する
4. ステータスバーに "Comparing: current vs T-35 (35 seconds ago)" のように表示される
5. `]` キーで 1 つ後 (より新しい) のスナップショットに移動する

**代替フロー:**

- (A-1) `{` (Shift+[) で 10 個前に、`}` (Shift+]) で 10 個後にジャンプする
- (A-2) `Home` で最古のスナップショット (snapshots[0])、`End` でデフォルト (直前) に戻る
- (A-3) スナップショット数がゼロの場合、`[` `]` は何もしない

**関連 Gap:** G5 (時系列 diff がない --- 本 UC で解消), G5-1 (リングバッファサイズ固定 --- `--history` と config.toml で設定可能に)

---

### UC-MON-04: グラフビューで数値フィールドの推移確認

| 項目 | 内容 |
|------|------|
| **Trigger** | Detail ビューで数値フィールドを選択して `g` キーを押す |
| **Actor** | (A) SRE (CPU 使用率やメモリ推移をビジュアルで確認したい) |
| **Input** | 選択中のフィールド (Bytes, Integer, Float, Duration 型のみ対象) |
| **Output** | Graph ビュー: sparkline グラフで時系列推移を表示 |

**メインフロー:**

1. Detail ビューで数値型フィールド (Bytes, Integer, Float, Duration) にカーソルを合わせる
2. `g` キーを押す
3. View::Graph に遷移する
4. リングバッファ内の過去スナップショットから該当フィールドの値を抽出し、sparkline グラフとして描画する
5. 自動リフレッシュのたびにグラフの右端に新しい値が追加され、グラフがスクロールする

**代替フロー:**

- (A-1) 文字列型や Table 型フィールドでは `g` キーは無効 (グラフ化できない)
- (A-2) スナップショット数が 1 以下の場合、グラフは 1 点のみ表示される

**関連 Gap:** なし (既存機能として実装済み)

---

### UC-MON-05: 閾値アラートで異常検知

| 項目 | 内容 |
|------|------|
| **Trigger** | 自動リフレッシュ時に、アラートルールの条件が成立した場合 |
| **Actor** | (A) SRE (メモリ枯渇や高負荷を能動的に検知したい) |
| **Input** | config.toml の `[[alert]]` セクションで定義されたアラートルール |
| **Output** | TUI 上の 3 箇所にアラート表示: (1) ステータスバーのカウンター、(2) サイドバーのソース名着色、(3) Detail ビューのフィールド背景色 |

**メインフロー:**

1. ユーザーが config.toml にアラートルールを定義する (UC-CFG-02 参照)
2. アプリケーション起動時にルールが読み込まれる (不正なルールはスキップ + 警告表示)
3. 各リフレッシュ時に `App.refresh()` の末尾でルール評価が実行される
4. 条件が成立したルールは `active_alerts` に追加される (状態: Normal -> Firing)
5. ステータスバーに `[!2 WARN] [!!1 CRIT]` のようなカウンターが表示される
6. サイドバーで該当ソース名が着色される (warning=Yellow, critical=Red)
7. 条件が解除されると状態が Firing -> Resolved -> Normal に遷移する (デバウンス)

**代替フロー:**

- (A-1) config.toml が存在しない場合、アラート機能は無効 (ゼロ設定の原則を維持)
- (A-2) 同一ルールの連続発火はデバウンスされる (Firing 中は再発火しない)
- (A-3) 不正な condition (パースエラー) はスキップされ、アプリケーション起動は止めない
- (A-4) `a` キーでアラート履歴ビュー (View::Alerts) を表示可能 (将来機能)

**関連 Gap:** G6 (閾値アラートがない --- 本 UC で解消), G6-1 (アラートストーム対策), G6-2 (不正設定でアプリが起動不能になるリスク), G6-3 (TUI 外への通知手段 --- MVP では TUI 表示のみ)

---

### UC-MON-06: SSH リモートホスト監視

| 項目 | 内容 |
|------|------|
| **Trigger** | `syslenz --ssh user@host` でアプリケーションを起動する |
| **Actor** | (A) SRE (リモートサーバーにログインせずに /proc を監視したい) |
| **Input** | SSH 接続先 (user@host 形式) |
| **Output** | リモートホストの /proc データが TUI に表示される (ローカルと同じ UI) |

**メインフロー:**

1. ユーザーが `syslenz --ssh user@host` を実行する
2. `stream_remote()` がバックグラウンドスレッドで SSH 接続を確立する
3. リモートホストで `/proc` ファイルの読み取りコマンドが実行され、結果が mpsc チャネル経由でメインスレッドに送信される
4. Dashboard ビューで表示される (リモートでも同じ /proc データ構造)
5. 接続状態がステータスバーに表示される (`[SSH: user@host] Connected (last: 14:23:06)`)

**代替フロー:**

- (A-1) SSH 接続失敗時: `MAX_CONSECUTIVE_FAILURES = 5` 回連続失敗でエラー表示。ステータスバーが `[SSH: user@host] Disconnected` に変わる
- (A-2) 接続断からの復帰: 自動リトライが行われる
- (A-3) SSH 認証は ssh-agent またはシステムの SSH 設定 (~/.ssh/config) に依存する

**関連 Gap:** G5-2 (リモート接続状態がユーザーに見えない), G7-1 (SSH 接続が毎回張り直し --- 将来の ControlMaster 最適化)

---

### UC-MON-07: マルチホストタブ切り替え監視

| 項目 | 内容 |
|------|------|
| **Trigger** | `syslenz --ssh user@host1 --ssh user@host2` で複数ホスト指定して起動する |
| **Actor** | (A) SRE (複数サーバーの状態を 1 つの TUI で監視・比較したい) |
| **Input** | 複数の SSH 接続先 |
| **Output** | タブ方式でホスト間を切り替え可能。各ホストが独立した Snapshot 管理・接続状態を持つ |

**メインフロー:**

1. ユーザーが `syslenz --ssh user@host1 --ssh user@host2 --ssh user@host3` を実行する
2. ローカルホスト (hosts[0]) + 各リモートホストが `HostState` として初期化される
3. ステータスバーの上にタブバーが表示される: `[localhost] [host1: ok] [host2: ok] [host3: ok]`
4. `Ctrl+1`〜`Ctrl+9` (または `F1`〜`F9`) でホスト間を切り替える
5. 選択中ホストの Snapshot データが Dashboard / Detail / Diff 等の全ビューに反映される

**代替フロー:**

- (A-1) 特定ホストの SSH 接続が切れた場合: そのタブのインジケータ色が緑→黄に変わる。他のホストへの影響はなし
- (A-2) アラートルールは全ホスト共通。アラートイベントはホストごとに管理。タブバーにホスト別アラートカウンター、ステータスバーに全体集計を表示
- (A-3) diff_target_index は HostState ごとに独立 (ホスト A で T-30 を見ていても、ホスト B に切り替えたらホスト B の diff_target_index が使われる)

**関連 Gap:** G7 (マルチホスト監視が未対応 --- 本 UC で解消), G7-1 (SSH パーシステント接続の最適化), G7-2 (アラート集計表示), G7-3 (G5/G6/G7 の実装順序依存 --- HostState 切り出しが先)

---

## エクスポート (UC-EXP)

### UC-EXP-01: 現在のスナップショットを JSON export

| 項目 | 内容 |
|------|------|
| **Trigger** | `syslenz --export <file>` でバッチモード実行、または将来の TUI 内 export キー |
| **Actor** | (C) 監査員 (システム状態のスナップショットを証跡として保存したい) |
| **Input** | 出力先ファイルパス |
| **Output** | JSON ファイル: Snapshot 構造体のシリアライズ (timestamp, entries: { source: { fields: [...] } }) |

**メインフロー:**

1. ユーザーが `syslenz --export /path/to/snapshot.json` を実行する
2. /proc から全 43 ソースをパースし、Snapshot を構築する
3. Snapshot を `serde_json` で JSON にシリアライズする
4. 指定パスにファイルを書き出す
5. アプリケーションが正常終了する (TUI は起動しない)

**代替フロー:**

- (A-1) 書き込み先ディレクトリが存在しない場合: エラーメッセージを表示して終了
- (A-2) `--ssh user@host --export snapshot.json` でリモートホストのスナップショットも export 可能

**関連 Gap:** G1-7 (監査ワークフロー未文書化 --- export はあるが使い方の例がない)

---

### UC-EXP-02: 時系列スナップショットをシリーズ export

| 項目 | 内容 |
|------|------|
| **Trigger** | `syslenz --export-series <file> --count N --interval M` でバッチモード実行 |
| **Actor** | (A) SRE (障害発生中の時系列データをキャプチャしたい), (C) 監査員 (一定期間のシステム挙動を記録) |
| **Input** | 出力先ファイルパス、キャプチャ回数 (count)、間隔 (interval) |
| **Output** | JSON ファイル: Snapshot の配列 (`Vec<Snapshot>` のシリアライズ) |

**メインフロー:**

1. ユーザーが `syslenz --export-series /path/to/series.json --count 60 --interval 1000` を実行する
2. 指定間隔 (1000ms) ごとに /proc をパースし、Snapshot を生成する
3. 指定回数 (60 回) 分のスナップショットを収集する
4. 全スナップショットを JSON 配列としてファイルに書き出す
5. アプリケーションが正常終了する

**代替フロー:**

- (A-1) キャプチャ中に Ctrl+C が押された場合: その時点までのスナップショットを書き出して終了

**関連 Gap:** なし (既存機能として実装済み)

---

### UC-EXP-03: JSON インポートしてリプレイ

| 項目 | 内容 |
|------|------|
| **Trigger** | `syslenz --import <file>` で TUI を起動する |
| **Actor** | (A) SRE (過去に取得したスナップショットをオフラインで分析したい), (C) 監査員 (エクスポートされた証跡を別マシンで閲覧) |
| **Input** | JSON ファイル (UC-EXP-01 または UC-EXP-02 で生成されたもの) |
| **Output** | TUI 画面: インポートされたスナップショットデータを表示。View::Overview がデフォルト |

**メインフロー:**

1. ユーザーが `syslenz --import /path/to/snapshot.json` を実行する
2. JSON ファイルを読み込み、Snapshot にデシリアライズする
3. `App::from_imported()` で TUI が起動する (デフォルトビューは View::Overview、Dashboard ではない)
4. インポートされたデータに対して通常通りの操作 (ソース選択、Detail 閲覧、TableView) が可能
5. 自動リフレッシュは無効 (ライブデータではないため)

**代替フロー:**

- (A-1) シリーズ JSON をインポートした場合: 複数スナップショットがリングバッファに展開され、Diff ビューやグラフビューで時系列分析が可能
- (A-2) JSON ファイルの形式が不正な場合: エラーメッセージを表示して終了

**関連 Gap:** G3-4 (インポートモードで Dashboard が不適切 --- View::Overview をデフォルトにすることで解消)

---

### UC-EXP-04: OpenTelemetry メトリクス export

| 項目 | 内容 |
|------|------|
| **Trigger** | `syslenz --otel [endpoint]` でヘッドレスモード起動 |
| **Actor** | (A) SRE (障害調査中に /proc の詳細メトリクスを Grafana で可視化したい --- 一時的デバッグ用途) |
| **Input** | OTLP gRPC エンドポイント (デフォルト: `http://localhost:4317`)、push 間隔 (デフォルト: 5 秒) |
| **Output** | 全 43 ソースの数値フィールド (Bytes, Integer, Float, Duration) が OTLP gauge メトリクスとして gRPC push される |

**メインフロー:**

1. ユーザーが `syslenz --otel http://localhost:4317 --interval 5` を実行する
2. TUI は起動しない (ヘッドレスモード)
3. 指定間隔 (5 秒) ごとに /proc をパースし、全数値フィールドを `syslenz.{source}.{field}` 命名規則で gauge メトリクスとして export する
4. OTel Collector がメトリクスを受信し、Prometheus に転送する
5. ユーザーが Grafana でメトリクスを閲覧する
6. `Ctrl+C` で停止する

**代替フロー:**

- (A-1) OTel Collector に接続できない場合: エラーメッセージを表示してリトライを継続する
- (A-2) config.toml の `[otel]` セクションで endpoint と interval のデフォルトを設定可能 (UC-CFG-01 参照)

**関連 Gap:** G8-1 (node_exporter との差別化未言語化 --- syslenz は 43 ソース全体をカバー、buddyinfo/slabinfo/pressure 等は node_exporter にない), G8-2 (メトリクスに「層」がない --- 全フラット export), G8-3 (OTEL Quick Start がない --- docs/otel.md + docker-compose.yml が必要), G8-4 (メトリクス名の静的一覧がない)

---

## 設定 (UC-CFG)

### UC-CFG-01: config.toml でデフォルト設定

| 項目 | 内容 |
|------|------|
| **Trigger** | アプリケーション起動時に `~/.config/syslenz/config.toml` が存在する場合 |
| **Actor** | (A) SRE (毎回同じ CLI 引数を打ちたくない), (B) 学習者 (日本語をデフォルトにしたい) |
| **Input** | TOML 形式の設定ファイル |
| **Output** | 設定値がアプリケーションのデフォルトに反映される |

**メインフロー:**

1. アプリケーション起動時に `Config::load()` が呼ばれる
2. `$XDG_CONFIG_HOME/syslenz/config.toml` を探す (fallback: `~/.config/syslenz/config.toml`)
3. ファイルが存在すれば TOML としてパースし、Config 構造体にデシリアライズする
4. CLI 引数と config 値をマージする (優先順位: CLI 引数 > 環境変数 > config.toml > デフォルト値)
5. マージされた設定でアプリケーションが起動する

**設定可能項目:**

| セクション | キー | 型 | デフォルト | 説明 |
|-----------|-----|-----|----------|------|
| `[general]` | `lang` | `String` | `"en"` | 表示言語 (`"en"` or `"ja"`) |
| `[general]` | `interval_ms` | `u64` | `1000` | リフレッシュ間隔 (ミリ秒) |
| `[general]` | `sources` | `Vec<String>` | 全ソース | 表示するソースのフィルタ |
| `[otel]` | `endpoint` | `String` | `"http://localhost:4317"` | OTLP gRPC エンドポイント |
| `[otel]` | `interval_secs` | `u64` | `5` | メトリクス push 間隔 (秒) |
| `[web]` | `port` | `u16` | `3000` | Web UI の listen ポート |
| `[ssh]` | `host` | `String` | なし | デフォルト SSH ホスト |

**代替フロー:**

- (A-1) config.toml が存在しない場合: 全てデフォルト値で動作する。エラーは発生しない (ゼロ設定の原則)
- (A-2) config.toml の TOML 構文が不正な場合: デフォルト値にフォールバック。パースエラーはアプリケーション起動を止めない
- (A-3) `--lang en` のように CLI 引数で明示指定された場合、config.toml の `lang = "ja"` は上書きされる

**関連 Gap:** G9 (設定ファイルがない --- 本 UC で解消), G9-1 (config に入れるべき項目が未整理), G9-2 (優先順位ルールが未定義), G9-3 (XDG_CONFIG_HOME 未対応)

---

### UC-CFG-02: アラートルール設定

| 項目 | 内容 |
|------|------|
| **Trigger** | config.toml に `[[alert]]` セクションが定義されている場合 |
| **Actor** | (A) SRE (カスタム閾値で異常を検知したい) |
| **Input** | TOML 形式のアラートルール定義 |
| **Output** | アラートルールがアプリケーションに読み込まれ、UC-MON-05 に従ってリアルタイム評価される |

**メインフロー:**

1. ユーザーが config.toml に `[[alert]]` セクションを記述する:
   ```toml
   [[alert]]
   source = "meminfo"
   field = "MemAvailable"
   condition = "< 500000000"
   severity = "critical"
   message = "Available memory below 500MB"
   ```
2. アプリケーション起動時にルールが `Vec<AlertRule>` としてパースされる
3. 各ルールの condition がバリデートされる (比較演算子: `>`, `<`, `>=`, `<=`, `==`, `!=` + 数値)
4. 有効なルールが `App.alert_rules` に格納される
5. リフレッシュごとに UC-MON-05 のフローでルール評価が実行される

**代替フロー:**

- (A-1) 不正な condition (例: `condition = "hello world"`) はスキップされ、ステータスバーに警告が表示される。他の有効なルールは正常に機能する
- (A-2) `[[alert]]` セクションが 0 個の場合: アラート機能は無効。TUI 上にアラート関連の表示はされない
- (A-3) 比較対象は数値フィールド (Bytes, Integer, Float, Duration) のみ。文字列フィールドへのアラートは MVP では未対応

**関連 Gap:** G6 (閾値アラートがない), G6-1 (アラートストーム対策 --- デバウンスで解消), G6-2 (不正設定でのクラッシュ防止 --- graceful degradation)

---

## Web/Widget (UC-WEB)

### UC-WEB-01: Web UI でブラウザからシステム監視

| 項目 | 内容 |
|------|------|
| **Trigger** | `syslenz --web [port]` でアプリケーションを起動する (feature gate: `web`) |
| **Actor** | (A) SRE (TUI を起動できない環境でブラウザから監視したい), (C) 監査員 (チームで共有閲覧したい) |
| **Input** | Web UI の listen ポート (デフォルト: 3000、config.toml で変更可能) |
| **Output** | ブラウザで `http://localhost:3000` にアクセスすると、/proc データが JSON API + SSE で提供される |

**メインフロー:**

1. ユーザーが `syslenz --web 8080` を実行する (feature flag `web` が有効なビルドが必要)
2. axum HTTP サーバーが指定ポートで起動する
3. JSON API エンドポイントが `/proc` データを提供する
4. SSE (Server-Sent Events) で リアルタイム更新がプッシュされる
5. ブラウザでダッシュボード的な画面が表示される

**代替フロー:**

- (A-1) `--web` と TUI は同時に動作する (TUI がフォアグラウンド、Web がバックグラウンド)
- (A-2) feature flag `web` なしでビルドされた場合、`--web` オプションは利用不可

**関連 Gap:** G12 (Web UI の差別化 --- Grafana との違いは「ゼロ設定 + /proc 全構造化」)

---

### UC-WEB-02: X11 ウィジェットで常時表示

| 項目 | 内容 |
|------|------|
| **Trigger** | `syslenz --widget` でウィジェットモードを起動する (feature gate: `x11widget`) |
| **Actor** | (A) SRE (デスクトップ上にシステム情報を常時表示したい) |
| **Input** | なし |
| **Output** | X11 のフローティングウィンドウにシステムメトリクスが常時表示される |

**メインフロー:**

1. ユーザーが `syslenz --widget` を実行する (feature flag `x11widget` が有効なビルドが必要)
2. x11rb を使用して透明なフローティングウィンドウが作成される
3. 主要メトリクス (CPU, メモリ, ロードアベレージ) がウィジェット内に表示される
4. 定期的に /proc を再読み込みし、表示を更新する
5. ウィンドウを閉じるか `Ctrl+C` で終了する

**代替フロー:**

- (A-1) X11 が利用できない環境 (Wayland only, SSH session) ではエラーメッセージを表示して終了
- (A-2) feature flag `x11widget` なしでビルドされた場合、`--widget` オプションは利用不可

**関連 Gap:** なし (実装済み機能。ただし G12 のフィーチャー優先順位議論の対象 --- コアバリューとの線引きが必要)

---

## 実装優先順位

DGE Sessions の議論に基づく推奨実装順序:

### Phase 1: 基盤整備 (UC-CFG, UC-BAS)

| 優先度 | ユースケース | 理由 |
|--------|------------|------|
| Critical | UC-CFG-01 | config.toml の読み込みが G6 (アラート) と G9 の前提条件 |
| High | UC-DSH-01, UC-DSH-02, UC-DSH-03 | デフォルトビューの改善。初回体験の全面刷新 |
| High | UC-BAS-01 ~ UC-BAS-06 | 既存機能の安定化・i18n 拡張 |

### Phase 2: 監視機能強化 (UC-MON)

| 優先度 | ユースケース | 理由 |
|--------|------------|------|
| High | UC-MON-03 | 時系列 diff (HostState 切り出しと同時に実装) |
| High | UC-MON-05, UC-CFG-02 | アラート (config.toml 必須 --- Phase 1 完了後) |
| Medium | UC-MON-07 | マルチホスト (HostState 構造体の大規模リファクタリング) |

### Phase 3: エクスポート・エコシステム (UC-EXP, UC-WEB)

| 優先度 | ユースケース | 理由 |
|--------|------------|------|
| Medium | UC-EXP-04 | OTEL ドキュメント整備 (コード変更なし) |
| Low | UC-WEB-01, UC-WEB-02 | コアバリューとの線引き要検討 |

---

## ペルソナ別ユースケースマッピング

### (A) SRE / インフラエンジニア

| 場面 | 使用するユースケース |
|------|-------------------|
| サーバーに ssh して全体状況を把握 | UC-BAS-01 -> UC-DSH-01 |
| メモリの詳細を確認 | UC-DSH-02 -> UC-BAS-02 |
| 5 分前との差分を確認 | UC-MON-02 -> UC-MON-03 |
| CPU 使用率の推移をグラフで確認 | UC-BAS-02 -> UC-MON-04 |
| メモリ枯渇の自動検知 | UC-CFG-02 -> UC-MON-05 |
| 複数サーバーを一画面で監視 | UC-MON-07 |
| 障害レポート用にスナップショット取得 | UC-EXP-01 |
| Grafana で /proc メトリクスを深掘り | UC-EXP-04 |

### (B) Linux 学習者 / CS 学生

| 場面 | 使用するユースケース |
|------|-------------------|
| /proc の全体像を把握 | UC-BAS-01 -> UC-DSH-01 |
| 各 /proc ファイルの意味を学ぶ | UC-BAS-02 -> UC-BAS-06 |
| 日本語でフィールド説明を読む | UC-BAS-05 |
| 操作方法を確認 | UC-DSH-03 |
| 特定のソースを探す | UC-BAS-04 |
| 値がリアルタイムで変化する様子を観察 | UC-MON-01 -> UC-MON-02 |

### (C) セキュリティ監査員

| 場面 | 使用するユースケース |
|------|-------------------|
| システム状態の証跡を取得 | UC-EXP-01 |
| 一定期間の挙動を記録 | UC-EXP-02 |
| オフラインで証跡を分析 | UC-EXP-03 |
| net/tcp の全コネクションを確認 | UC-BAS-02 -> UC-BAS-03 |
| カーネルモジュール一覧を確認 | UC-BAS-02 (modules ソース) |
| 複数ホストの状態を比較監査 | UC-MON-07 -> UC-EXP-01 |

---

## DGE Gap トレーサビリティ

全 UC が DGE Sessions で発見された Gap とどのように対応するかの一覧:

| Gap | Session | ユースケース | ステータス |
|-----|---------|------------|-----------|
| G1 (ターゲットユーザー未定義) | 001 | ペルソナ定義 (A)(B)(C) | Session 002 で解消 |
| G2 (ダッシュボードがない) | 001 | UC-DSH-01, UC-DSH-02 | Spec 提案済み (Session 003) |
| G3 (オンボーディングがない) | 001 | UC-DSH-03 | Spec 提案済み (Session 003) |
| G4 (バイナリ配布がない) | 001 | (本文書スコープ外 --- Ops) | Spec 提案済み (Session 004) |
| G5 (時系列 diff がない) | 001 | UC-MON-03 | Spec 提案済み (Session 005) |
| G6 (閾値アラートがない) | 001 | UC-MON-05, UC-CFG-02 | Spec 提案済み (Session 005) |
| G7 (マルチホスト未対応) | 001 | UC-MON-07 | Spec 提案済み (Session 005) |
| G8 (OTEL ユースケース整理) | 001 | UC-EXP-04 | Spec 提案済み (Session 006) |
| G9 (設定ファイルがない) | 001 | UC-CFG-01, UC-CFG-02 | Spec 提案済み (Session 006) |
| G10 (テストがほぼゼロ) | 001 | (本文書スコープ外 --- QA) | Spec 提案済み (Session 007) |
| G11 (OSS インフラ未整備) | 001 | (本文書スコープ外 --- Ops) | Spec 提案済み (Session 004) |
| G12 (ポジショニング曖昧) | 001 | ペルソナ定義 + 概要 | Spec 提案済み (Session 008) |
| G13 (スクリーンショットなし) | 001 | (本文書スコープ外 --- Docs) | Spec 提案済み (Session 008) |
