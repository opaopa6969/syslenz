# syslenz 技術仕様書

DGE Sessions 001-008 で特定された全変更を統合した技術仕様。

- **対象バージョン**: v0.2.0 (現行 v0.1.0 からの差分)
- **最終更新**: 2026-03-28
- **入力**: DGE Sessions 001-008

---

## 目次

1. [データモデル変更](#1-データモデル変更)
2. [UI 仕様](#2-ui-仕様)
3. [キーバインド一覧](#3-キーバインド一覧)
4. [設定ファイル仕様](#4-設定ファイル仕様)
5. [アラートシステム仕様](#5-アラートシステム仕様)
6. [テスト仕様](#6-テスト仕様)
7. [CI/CD 仕様](#7-cicd-仕様)
8. [README 新構造](#8-readme-新構造)

---

## 1. データモデル変更

### 1.1 View enum 拡張 (Session 003)

既存の 5 バリアントに `Welcome` と `Dashboard` を追加する。

**ファイル**: `src/ui/app.rs`

```rust
pub enum View {
    Welcome,     // キーバインド一覧 + CTA (Session 003: G3)
    Dashboard,   // システムサマリー 5 ソース厳選 (Session 003: G2)
    Overview,    // 既存: サイドバー + 選択ソースの全フィールド
    Detail,      // 既存: 単一ソースのフィールドテーブル
    Diff,        // 既存: 前回スナップショットとの差分表示
    TableView,   // 既存: Table 型フィールドの展開表示
    Graph,       // 既存: sparkline グラフ
}
```

`Welcome` と `Dashboard` はサイドバーを非表示にし、全幅レイアウトで描画する。

### 1.2 App 構造体の追加フィールド

**ファイル**: `src/ui/app.rs`

以下のフィールドを `App` 構造体に追加する。

```rust
pub struct App {
    // === 既存フィールド (変更なし) ===
    pub snapshots: Vec<Snapshot>,
    pub current: Snapshot,
    pub diffs: Vec<DiffItem>,
    pub view: View,
    pub focus: Focus,
    pub selected_source: usize,
    pub source_keys: Vec<String>,
    pub selected_field: usize,
    pub sidebar_scroll: usize,
    pub field_scroll: usize,
    pub table_scroll: usize,
    pub running: bool,
    pub last_refresh: Instant,
    pub auto_refresh: bool,
    pub refresh_interval_ms: u64,
    pub search_query: String,
    pub searching: bool,
    pub filtered_keys: Option<Vec<usize>>,
    pub graph_field: Option<(String, String)>,
    pub status_message: Option<String>,
    pub remote_host: Option<String>,
    pub remote_rx: Option<mpsc::Receiver<Snapshot>>,
    pub locale: Locale,
    pub show_help: bool,

    // === 新規フィールド (Session 003: Dashboard/Welcome) ===
    /// Dashboard 内で選択中のセクション (0=loadavg/uptime, 1=meminfo, 2=stat, 3=net/dev)
    pub selected_dashboard_section: usize,
    /// Detail から go_back() したとき Dashboard に戻るかどうかの判定フラグ
    pub came_from_dashboard: bool,

    // === 新規フィールド (Session 005: タイムトラベル diff) ===
    /// Diff ビューで比較するスナップショットのインデックス。
    /// None = 直前のスナップショット (従来動作)、Some(i) = snapshots[i] と比較。
    pub diff_target_index: Option<usize>,
    /// リングバッファの最大サイズ。デフォルト 60。config.toml の history_size で変更可能。
    pub max_snapshots: usize,

    // === 新規フィールド (Session 005: 接続状態) ===
    /// リモート接続の状態表示用。ローカルモードでは ConnectionStatus::Local。
    pub connection_status: ConnectionStatus,

    // === 新規フィールド (Session 005/006: アラート) ===
    /// config.toml の [[alert]] セクションから読み込んだアラートルール
    pub alert_rules: Vec<AlertRule>,
    /// 現在発火中のアラートイベント
    pub active_alerts: Vec<AlertEvent>,

    // === 新規フィールド (Session 005: マルチホスト) ===
    /// ホストごとの状態。hosts[0] は常に localhost。
    pub hosts: Vec<HostState>,
    /// 現在アクティブなホストのインデックス (タブ切り替え用)
    pub active_host: usize,
}
```

**初期値 (App::new())**:

```rust
selected_dashboard_section: 0,
came_from_dashboard: false,
diff_target_index: None,
max_snapshots: 60,
connection_status: ConnectionStatus::Local,
alert_rules: Vec::new(),
active_alerts: Vec::new(),
hosts: Vec::new(),   // マルチホスト実装時に移行
active_host: 0,
```

**初期ビュー変更**:

| コンストラクタ | 変更前 | 変更後 | 理由 |
|---|---|---|---|
| `App::new()` | `View::Overview` | `View::Dashboard` | ライブデータ前提。Dashboard で即座にシステム状態を把握 |
| `App::from_remote()` | `View::Overview` | `View::Dashboard` | リモートもライブデータ |
| `App::from_imported()` | `View::Overview` | `View::Overview` (変更なし) | 過去データは Dashboard のリアルタイム感と矛盾する |

### 1.3 HostState 構造体 (Session 005: G7)

マルチホスト対応のため、Snapshot 管理をホスト単位に分離する。

**ファイル**: `src/ui/app.rs`

```rust
/// ホスト単位の状態管理。マルチホストタブ方式の基盤。
pub struct HostState {
    /// ホスト識別子。ローカルは "localhost"、リモートは "user@host"。
    pub host: String,

    /// スナップショットのリングバッファ (max_snapshots 個)。
    pub snapshots: Vec<Snapshot>,

    /// 最新のスナップショット。
    pub current: Snapshot,

    /// 直前スナップショットとの差分 (または diff_target_index 指定の差分)。
    pub diffs: Vec<DiffItem>,

    /// リモート接続の状態。ローカルホストは ConnectionStatus::Local。
    pub connection_status: ConnectionStatus,

    /// このホストで発火中のアラートイベント。
    pub alert_events: Vec<AlertEvent>,
}
```

**移行戦略**: Phase 1 で `HostState` を導入し、`App` の既存フィールド (`snapshots`, `current`, `diffs`) を `hosts[active_host]` 経由でアクセスするヘルパーを提供する。

```rust
impl App {
    /// アクティブなホストの状態への参照を返す。
    pub fn active_host_state(&self) -> &HostState {
        &self.hosts[self.active_host]
    }

    /// アクティブなホストの状態への可変参照を返す。
    pub fn active_host_state_mut(&mut self) -> &mut HostState {
        &mut self.hosts[self.active_host]
    }
}
```

### 1.4 AlertRule 構造体 (Session 005: G6)

**ファイル**: `src/alert.rs` (新規)

```rust
use serde::Deserialize;
use std::time::Instant;

/// アラートルール。config.toml の [[alert]] セクションからデシリアライズされる。
#[derive(Debug, Clone, Deserialize)]
pub struct AlertRule {
    /// 対象の /proc ソース名 (例: "meminfo", "loadavg")
    pub source: String,
    /// 対象フィールド名 (例: "MemAvailable", "load_1min")
    pub field: String,
    /// 比較演算子
    pub op: CompareOp,
    /// 閾値 (数値)。FieldValue::Bytes, Integer, Float, Duration を f64 に変換して比較する。
    pub threshold: f64,
    /// 重要度
    pub severity: Severity,
    /// アラート発火時のメッセージ
    pub message: String,
}

/// 比較演算子。TOML の condition フィールド (例: "> 8.0") からパースされる。
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum CompareOp {
    /// `>` — より大きい
    Gt,
    /// `<` — より小さい
    Lt,
    /// `>=` — 以上
    Gte,
    /// `<=` — 以下
    Lte,
    /// `==` — 等しい
    Eq,
    /// `!=` — 等しくない
    Neq,
}

impl CompareOp {
    /// 文字列から CompareOp をパースする。
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim() {
            ">"  => Some(CompareOp::Gt),
            "<"  => Some(CompareOp::Lt),
            ">=" => Some(CompareOp::Gte),
            "<=" => Some(CompareOp::Lte),
            "==" => Some(CompareOp::Eq),
            "!=" => Some(CompareOp::Neq),
            _    => None,
        }
    }

    /// 2 つの f64 値をこの演算子で比較する。
    pub fn evaluate(&self, actual: f64, threshold: f64) -> bool {
        match self {
            CompareOp::Gt  => actual > threshold,
            CompareOp::Lt  => actual < threshold,
            CompareOp::Gte => actual >= threshold,
            CompareOp::Lte => actual <= threshold,
            CompareOp::Eq  => (actual - threshold).abs() < f64::EPSILON,
            CompareOp::Neq => (actual - threshold).abs() >= f64::EPSILON,
        }
    }
}

/// アラートの重要度。
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Warning,
    Critical,
}

/// アラートの状態遷移。
#[derive(Debug, Clone)]
pub enum AlertState {
    /// 正常状態。閾値を超えていない。
    Normal,
    /// 発火中。Instant は発火開始時刻。
    Firing(Instant),
    /// 解除済み。Instant は解除時刻。次の評価サイクルで Normal に遷移する。
    Resolved(Instant),
}

/// 発火したアラートイベント。履歴保持用。
#[derive(Debug, Clone)]
pub struct AlertEvent {
    /// 発火時刻
    pub timestamp: Instant,
    /// 発火したルール
    pub rule: AlertRule,
    /// 発火時の実測値
    pub actual_value: f64,
    /// 対象ホスト名
    pub host: String,
}
```

### 1.5 ConnectionStatus enum (Session 005: G5-2)

**ファイル**: `src/ui/app.rs`

```rust
use std::time::Instant;

/// リモート接続の状態。ステータスバーとタブバーで色表示に使用する。
#[derive(Debug, Clone)]
pub enum ConnectionStatus {
    /// ローカルモード。SSH 接続なし。
    Local,
    /// リモート接続中。last_seen は最後にスナップショットを受信した時刻。
    Connected { last_seen: Instant },
    /// リモート接続断。last_seen は最後の受信時刻、since は切断検知時刻。
    Disconnected { last_seen: Instant, since: Instant },
    /// リモート接続試行中。
    Connecting,
}
```

**表示色マッピング**:

| ConnectionStatus | タブバー色 | ステータスバー表示 |
|---|---|---|
| `Local` | White | (表示なし) |
| `Connected` | Green | `[SSH: user@host] Connected` |
| `Disconnected` | Yellow | `[SSH: user@host] Disconnected (15s ago)` |
| `Connecting` | DarkGray | `[SSH: user@host] Connecting...` |

### 1.6 Config 構造体 (Session 006: G9)

**ファイル**: `src/config.rs` (新規)

```rust
use serde::Deserialize;

/// syslenz 設定ファイルのルート構造体。
/// $XDG_CONFIG_HOME/syslenz/config.toml からデシリアライズされる。
#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub otel: OtelConfig,
    #[serde(default)]
    pub web: WebConfig,
    #[serde(default)]
    pub ssh: SshConfig,
    /// アラートルール (複数指定可能)
    #[serde(default, rename = "alert")]
    pub alerts: Vec<crate::alert::AlertRule>,
}

/// [general] セクション
#[derive(Debug, Deserialize, Default)]
pub struct GeneralConfig {
    /// 表示言語。"en" | "ja"。CLI: --lang
    pub lang: Option<String>,
    /// TUI のリフレッシュ間隔 (ミリ秒)。デフォルト: 1000
    pub interval_ms: Option<u64>,
    /// 表示するソースのフィルタリスト (空 = 全ソース表示)
    pub sources: Option<Vec<String>>,
    /// リングバッファの最大サイズ。デフォルト: 60
    pub history_size: Option<usize>,
}

/// [otel] セクション
#[derive(Debug, Deserialize, Default)]
pub struct OtelConfig {
    /// OTLP gRPC エンドポイント。デフォルト: "http://localhost:4317"
    pub endpoint: Option<String>,
    /// メトリクス push 間隔 (秒)。デフォルト: 5
    pub interval_secs: Option<u64>,
}

/// [web] セクション
#[derive(Debug, Deserialize, Default)]
pub struct WebConfig {
    /// Web UI の listen ポート。デフォルト: 3000
    pub port: Option<u16>,
}

/// [ssh] セクション
#[derive(Debug, Deserialize, Default)]
pub struct SshConfig {
    /// デフォルトの SSH ホスト (省略可)
    pub host: Option<String>,
}

impl Config {
    /// 設定ファイルを読み込む。ファイルが存在しない、または不正な場合はデフォルト値を返す。
    pub fn load() -> Self {
        let path = config_path();
        match std::fs::read_to_string(&path) {
            Ok(content) => match toml::from_str(&content) {
                Ok(cfg) => cfg,
                Err(e) => {
                    eprintln!("Warning: failed to parse config: {}", e);
                    Self::default()
                }
            },
            Err(_) => Self::default(),
        }
    }
}

/// 設定ファイルのパスを決定する。
/// $XDG_CONFIG_HOME/syslenz/config.toml > ~/.config/syslenz/config.toml
fn config_path() -> std::path::PathBuf {
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        std::path::PathBuf::from(xdg).join("syslenz/config.toml")
    } else if let Ok(home) = std::env::var("HOME") {
        std::path::PathBuf::from(home).join(".config/syslenz/config.toml")
    } else {
        std::path::PathBuf::from("config.toml")
    }
}
```

### 1.7 FieldValue への PartialEq derive 追加 (Session 007: G10-3)

**ファイル**: `src/proc/mod.rs`

```rust
// 変更前
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldValue { ... }

// 変更後
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FieldValue { ... }
```

**注意**: `FieldValue::Float(f64)` は `PartialEq` で NaN の比較が `false` になる。テストでは `f64` の比較に epsilon ベースのヘルパーを使用する。

---

## 2. UI 仕様

### 2.1 Dashboard レイアウト (Session 003: G2)

起動時のデフォルトビュー (`App::new()`, `App::from_remote()`)。サイドバーを非表示にし、全幅で以下のレイアウトを描画する。

```
+-----------------------------------------------------+
| Dashboard -- hostname | uptime: 3d 2h | load: 0.5   |  <- 上段 (3行): loadavg + uptime
+-------------------------+---------------------------+
|  Memory                 |  CPU                      |  <- 中段 (9行): meminfo (左) + stat (右)
|  Total:  15.6 GiB       |  User:   12.3%            |
|  Free:    8.2 GiB       |  System:  3.1%            |
|  Avail:  10.1 GiB       |  Idle:   84.6%            |
|  Swap:    2.0 GiB       |  IOWait:  0.0%            |
|  SwapFree: 2.0 GiB      |  IRQ:     0.0%            |
+-------------------------+---------------------------+
|  Network (net/dev)                                  |  <- 下段 (残り): net/dev テーブル
|  eth0    RX: 1.2 GiB   TX: 340 MiB                 |
|  lo      RX: 500 MiB   TX: 500 MiB                 |
+-----------------------------------------------------+
| D:dashboard  ?:help  Enter:browse sources  q:quit   |  <- ステータスバー
+-----------------------------------------------------+
```

**データソース (固定 5 つ)**:

| セクション | ソース | 表示フィールド |
|---|---|---|
| ヘッダー | `uptime` | `uptime` |
| ヘッダー | `loadavg` | `load_1`, `load_5`, `load_15` |
| Memory | `meminfo` | `mem_total`, `mem_free`, `mem_available`, `swap_total`, `swap_free` |
| CPU | `stat` | `cpu_user_pct`, `cpu_system_pct`, `cpu_idle_pct`, `cpu_iowait_pct`, `cpu_irq_pct` |
| Network | `net/dev` | Table フィールド (インタフェースごと RX/TX) |

**セクション選択**: `j`/`k` で `selected_dashboard_section` (0-3) を移動。選択中のセクションはボーダー色が Yellow に変わる。`Enter` で対応ソースの Detail ビューに遷移する。

**描画関数**:

```rust
fn draw_dashboard(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // ヘッダー: hostname + uptime + loadavg
            Constraint::Length(9),   // 中段: meminfo (左) + stat (右)
            Constraint::Min(5),      // 下段: net/dev テーブル
        ])
        .split(area);

    // ヘッダー
    let uptime_str = get_field_display(&app.current, "uptime", "uptime");
    let load1 = get_field_display(&app.current, "loadavg", "load_1");
    let load5 = get_field_display(&app.current, "loadavg", "load_5");
    let load15 = get_field_display(&app.current, "loadavg", "load_15");
    let header_line = Line::from(vec![
        Span::styled(" uptime: ", Style::default().fg(Color::DarkGray)),
        Span::styled(&uptime_str, Style::default().fg(Color::Cyan)),
        Span::styled("  load: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{} {} {}", load1, load5, load15),
            Style::default().fg(Color::Yellow),
        ),
    ]);
    let header_block = Block::default()
        .borders(Borders::ALL)
        .title(" Dashboard ")
        .border_style(if app.selected_dashboard_section == 0 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });
    f.render_widget(Paragraph::new(header_line).block(header_block), chunks[0]);

    // 中段: meminfo + stat の 2 カラム
    let mid = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    draw_dashboard_memory(f, app, mid[0]);
    draw_dashboard_cpu(f, app, mid[1]);

    // 下段: net/dev
    draw_dashboard_network(f, app, chunks[2]);
}

/// ヘルパー: Snapshot からフィールドの display 値を取得
fn get_field_display(snapshot: &Snapshot, source: &str, field_name: &str) -> String {
    snapshot.entries.get(source)
        .and_then(|entry| entry.fields.iter().find(|f| f.name == field_name))
        .map(|f| f.value.display())
        .unwrap_or_else(|| "-".to_string())
}
```

### 2.2 Welcome 画面 (Session 003: G3)

`W` キーでいつでも表示可能。中央寄せの Paragraph ウィジェット 1 つで実装する。ターミナル 80x24 で崩れないこと。

```rust
fn draw_welcome(f: &mut Frame, app: &App, area: Rect) {
    let l = app.locale;
    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "syslenz",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "Wireshark for /proc",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  j/k    ", Style::default().fg(Color::Yellow)),
            Span::raw(i18n::t(l, T::WELCOME_NAV)),
        ]),
        Line::from(vec![
            Span::styled("  Enter  ", Style::default().fg(Color::Yellow)),
            Span::raw(i18n::t(l, T::WELCOME_DRILL)),
        ]),
        Line::from(vec![
            Span::styled("  d      ", Style::default().fg(Color::Yellow)),
            Span::raw(i18n::t(l, T::WELCOME_DIFF)),
        ]),
        Line::from(vec![
            Span::styled("  /      ", Style::default().fg(Color::Yellow)),
            Span::raw(i18n::t(l, T::WELCOME_SEARCH)),
        ]),
        Line::from(vec![
            Span::styled("  g      ", Style::default().fg(Color::Yellow)),
            Span::raw(i18n::t(l, T::WELCOME_GRAPH)),
        ]),
        Line::from(vec![
            Span::styled("  ?      ", Style::default().fg(Color::Yellow)),
            Span::raw(i18n::t(l, T::WELCOME_HELP)),
        ]),
        Line::from(vec![
            Span::styled("  L      ", Style::default().fg(Color::Yellow)),
            Span::raw(i18n::t(l, T::WELCOME_LANG)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            i18n::t(l, T::WELCOME_CTA),
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        )),
    ];
    let p = Paragraph::new(text)
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(p, area);
}
```

**i18n テキスト**:

| キー | EN | JA |
|---|---|---|
| `welcome_nav` | Navigate sources | ソースを選択 |
| `welcome_drill` | Drill into source | ソースの詳細を表示 |
| `welcome_diff` | Show changes since last refresh | 前回からの変更を表示 |
| `welcome_search` | Search sources | ソースを検索 |
| `welcome_graph` | Sparkline graph (numeric fields) | スパークライングラフ (数値フィールド) |
| `welcome_help` | Toggle field descriptions | フィールド説明の表示切替 |
| `welcome_lang` | Toggle EN/JA | 言語切替 EN/JA |
| `welcome_cta` | Press Enter or D for Dashboard | Enter または D でダッシュボードへ |

### 2.3 アラート表示 (Session 005: G6)

アラートは TUI 上の 3 箇所で表示する。

**表示箇所 1: ステータスバー (右端)**

```
[syslenz] 43 sources | T-0..T-59 | [!2 WARN] [!!1 CRIT]
```

**表示箇所 2: サイドバーのソース名着色**

アラートが発火中のソースは、サイドバーのソース名の前景色を severity に応じて変更する。

| Severity | 前景色 | 修飾 |
|---|---|---|
| Info | Cyan | なし |
| Warning | Yellow | なし |
| Critical | Red | Bold |

**表示箇所 3: Detail ビューのフィールド背景色**

閾値を超えているフィールドの行全体の背景色を変更する。

| Severity | 背景色 |
|---|---|
| Info | (変更なし) |
| Warning | DarkGray |
| Critical | DarkRed |

### 2.4 タイムトラベル diff (Session 005: G5)

Diff ビュー (`View::Diff`) 内で、比較対象のスナップショットを `[`/`]` キーで切り替える。

**ステータスバー表示**:

```
[Diff] current vs T-35 (14:23:05)  |  [ prev  ] next  Home oldest  End latest
```

`diff_target_index` が `None` の場合は "current vs T-1 (latest)" と表示し、従来と同じ動作になる。

### 2.5 マルチホストタブバー (Session 005: G7)

ステータスバーの上にタブバーを表示する。各タブにはホスト名と接続状態/アラート状態を表示する。

```
 [localhost] [web1: ok] [web2: !1W] [web3: !!1C]
------------------------------------------------------
 (メインコンテンツエリア)
------------------------------------------------------
 [syslenz] Total: [!1 WARN] [!!1 CRIT] | 43 sources
```

**タブ表示の色分け**:

| 状態 | 色 | 表示例 |
|---|---|---|
| ローカル (アラートなし) | White | `[localhost]` |
| リモート接続中 (アラートなし) | Green | `[web1: ok]` |
| リモート接続断 | Yellow | `[web1: disconnected]` |
| Warning アラートあり | Yellow | `[web2: !1W]` |
| Critical アラートあり | Red | `[web3: !!1C]` |

### 2.6 Help パネル (既存、? キー)

`?` キーで表示/非表示を切り替えるヘルプパネル。既に実装済み。フィールドの `description` を表示する。

### 2.7 render.rs のレイアウト分岐

```rust
pub fn draw(f: &mut Frame, app: &App) {
    match app.view {
        View::Welcome => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(10), Constraint::Length(3)])
                .split(f.area());
            draw_welcome(f, app, chunks[0]);
            draw_status_bar(f, app, chunks[1]);
            return;
        }
        View::Dashboard => {
            let outer = if app.show_help {
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Min(10),
                        Constraint::Length(5),
                        Constraint::Length(3),
                    ])
                    .split(f.area())
            } else {
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(10), Constraint::Length(3)])
                    .split(f.area())
            };
            draw_dashboard(f, app, outer[0]);
            if app.show_help {
                draw_help_panel(f, app, outer[1]);
                draw_status_bar(f, app, outer[2]);
            } else {
                draw_status_bar(f, app, outer[1]);
            }
            return;
        }
        _ => { /* 既存の sidebar + content レイアウト */ }
    }

    // 既存のレイアウトロジック (sidebar 22列 + content)
    // ...
}
```

---

## 3. キーバインド一覧

### 3.1 全キーバインドテーブル

| キー | スコープ | 動作 | 追加元 |
|---|---|---|---|
| `q` / `Esc` | 全画面 | アプリケーション終了 | 既存 |
| `j` / `Down` | 全画面 | カーソルを下に移動 (サイドバー: ソース選択、コンテンツ: フィールド選択、Dashboard: セクション選択) | 既存 |
| `k` / `Up` | 全画面 | カーソルを上に移動 | 既存 |
| `h` / `Left` / `Backspace` | 全画面 | 戻る (Detail -> Overview or Dashboard、TableView -> Detail) | 既存 |
| `l` / `Right` / `Enter` | 全画面 | ドリルイン (サイドバー -> Detail、Dashboard -> Detail、Detail -> TableView) | 既存 |
| `Tab` | 全画面 | フォーカス切り替え (Sidebar <-> Content) | 既存 |
| `/` | 全画面 | 検索モード開始 | 既存 |
| `d` | 全画面 | Diff ビューに切り替え | 既存 |
| `r` | 全画面 | 手動リフレッシュ | 既存 |
| `a` | 全画面 | 自動リフレッシュのトグル | 既存 |
| `g` | Detail | Graph ビュー (数値フィールド選択時) | 既存 |
| `e` | 全画面 | 現在のスナップショットを JSON エクスポート | 既存 |
| `L` | 全画面 | 言語切り替え (EN <-> JA) | 既存 |
| `?` | 全画面 | ヘルプパネルの表示/非表示 | 既存 |
| **`D`** | **全画面** | **Dashboard ビューに切り替え** | **Session 003** |
| **`W`** | **全画面** | **Welcome 画面を表示** | **Session 003** |
| **`[`** | **Diff ビュー** | **比較対象を 1 つ古いスナップショットに移動** | **Session 005** |
| **`]`** | **Diff ビュー** | **比較対象を 1 つ新しいスナップショットに移動 (None = 直前に戻る)** | **Session 005** |
| **`{`** (Shift+`[`) | **Diff ビュー** | **比較対象を 10 個古いスナップショットにジャンプ** | **Session 005** |
| **`}`** (Shift+`]`) | **Diff ビュー** | **比較対象を 10 個新しいスナップショットにジャンプ** | **Session 005** |
| **`Home`** | **Diff ビュー** | **比較対象を最古のスナップショットに移動** | **Session 005** |
| **`End`** | **Diff ビュー** | **比較対象を直前のスナップショット (None) にリセット** | **Session 005** |
| **`Ctrl+1`..`Ctrl+9`** | **全画面** | **ホストタブ切り替え (1=localhost, 2-9=リモートホスト)** | **Session 005** |

### 3.2 新規キーバインドの main.rs 実装

```rust
// Dashboard / Welcome 切り替え
KeyCode::Char('D') => {
    app.view = View::Dashboard;
    app.focus = Focus::Content;
    app.came_from_dashboard = false;
}
KeyCode::Char('W') => {
    app.view = View::Welcome;
}

// タイムトラベル diff (Diff ビュー内のみ)
KeyCode::Char('[') if matches!(app.view, View::Diff) => {
    let max = app.snapshots.len().saturating_sub(1);
    app.diff_target_index = Some(match app.diff_target_index {
        None => max,                          // 直前 -> 最古方向に 1 つ
        Some(i) => i.saturating_sub(1),       // さらに古い方へ
    });
}
KeyCode::Char(']') if matches!(app.view, View::Diff) => {
    if let Some(i) = app.diff_target_index {
        let max = app.snapshots.len().saturating_sub(1);
        if i >= max {
            app.diff_target_index = None;     // 直前に戻る
        } else {
            app.diff_target_index = Some(i + 1);
        }
    }
}
KeyCode::Char('{') if matches!(app.view, View::Diff) => {
    app.diff_target_index = Some(match app.diff_target_index {
        None => app.snapshots.len().saturating_sub(10),
        Some(i) => i.saturating_sub(10),
    });
}
KeyCode::Char('}') if matches!(app.view, View::Diff) => {
    if let Some(i) = app.diff_target_index {
        let max = app.snapshots.len().saturating_sub(1);
        let new_i = (i + 10).min(max);
        if new_i >= max {
            app.diff_target_index = None;
        } else {
            app.diff_target_index = Some(new_i);
        }
    }
}
KeyCode::Home if matches!(app.view, View::Diff) => {
    app.diff_target_index = Some(0);
}
KeyCode::End if matches!(app.view, View::Diff) => {
    app.diff_target_index = None;
}

// マルチホストタブ切り替え
KeyCode::Char(c) if c.is_ascii_digit() && event.modifiers.contains(KeyModifiers::CONTROL) => {
    let idx = (c as usize) - ('1' as usize);
    if idx < app.hosts.len() {
        app.active_host = idx;
    }
}
```

### 3.3 go_back() の拡張

```rust
pub fn go_back(&mut self) {
    match self.focus {
        Focus::Content => {
            match self.view {
                View::TableView | View::Graph => {
                    self.view = View::Detail;
                }
                View::Detail if self.came_from_dashboard => {
                    self.view = View::Dashboard;
                    self.came_from_dashboard = false;
                }
                View::Welcome => {
                    self.view = View::Dashboard;
                }
                _ => {
                    self.focus = Focus::Sidebar;
                    self.view = View::Overview;
                }
            }
        }
        Focus::Sidebar => {}
    }
}
```

---

## 4. 設定ファイル仕様

### 4.1 config.toml 完全な構造

**ファイルパス**:

```
$XDG_CONFIG_HOME/syslenz/config.toml
~/.config/syslenz/config.toml  (XDG_CONFIG_HOME 未設定時のフォールバック)
```

ファイルが存在しなければ全てデフォルト値で動作する。不正な TOML はスキップし、警告をステータスバーに表示する。

**完全な雛形**:

```toml
# syslenz configuration file
# Place at: ~/.config/syslenz/config.toml

[general]
# 表示言語: "en" | "ja"
# 型: String, デフォルト: "en"
# CLI: --lang <en|ja>, 環境変数: SYSLENZ_LANG
lang = "en"

# TUI のリフレッシュ間隔 (ミリ秒)
# 型: u64, デフォルト: 1000
# CLI: --interval <ms>
interval_ms = 1000

# 起動時に表示するソースのフィルタ (省略 = 全ソース表示)
# 型: Vec<String>, デフォルト: なし (全ソース)
# sources = ["meminfo", "loadavg", "stat", "net/dev", "processes", "pressure"]

# リングバッファの最大サイズ (スナップショット数)
# 型: usize, デフォルト: 60
# CLI: --history <N>
# history_size = 300

[otel]
# OTLP gRPC エンドポイント
# 型: String, デフォルト: "http://localhost:4317"
# CLI: --otel [endpoint]
endpoint = "http://localhost:4317"

# メトリクス push 間隔 (秒)
# 型: u64, デフォルト: 5
# CLI: --interval <secs> (--otel モード時)
interval_secs = 5

[web]
# Web UI の listen ポート
# 型: u16, デフォルト: 3000
# CLI: --web [port]
port = 3000

[ssh]
# デフォルトの SSH ホスト (省略可)
# 型: String, デフォルト: なし
# CLI: --ssh <user@host>
# host = "user@192.168.1.100"

# === アラートルール (複数指定可能) ===
# [[alert]]
# source = "meminfo"
# field = "MemAvailable"
# op = "Lt"              # Gt | Lt | Gte | Lte | Eq | Neq
# threshold = 500000000  # 500MB (Bytes)
# severity = "critical"  # info | warning | critical
# message = "Available memory below 500MB"
#
# [[alert]]
# source = "loadavg"
# field = "load_1min"
# op = "Gt"
# threshold = 8.0
# severity = "warning"
# message = "Load average exceeded 8.0"
```

### 4.2 全キー一覧

| セクション | キー | 型 | デフォルト値 | CLI 対応 | 環境変数 |
|---|---|---|---|---|---|
| `[general]` | `lang` | `Option<String>` | `"en"` | `--lang` | `SYSLENZ_LANG` |
| `[general]` | `interval_ms` | `Option<u64>` | `1000` | `--interval` | `SYSLENZ_INTERVAL_MS` |
| `[general]` | `sources` | `Option<Vec<String>>` | 全ソース | - | - |
| `[general]` | `history_size` | `Option<usize>` | `60` | `--history` | `SYSLENZ_HISTORY_SIZE` |
| `[otel]` | `endpoint` | `Option<String>` | `"http://localhost:4317"` | `--otel [endpoint]` | `SYSLENZ_OTEL_ENDPOINT` |
| `[otel]` | `interval_secs` | `Option<u64>` | `5` | `--interval` (otel時) | `SYSLENZ_OTEL_INTERVAL` |
| `[web]` | `port` | `Option<u16>` | `3000` | `--web [port]` | `SYSLENZ_WEB_PORT` |
| `[ssh]` | `host` | `Option<String>` | なし | `--ssh` | `SYSLENZ_SSH_HOST` |
| `[[alert]]` | (後述) | `Vec<AlertRule>` | 空 | - | - |

### 4.3 優先順位

```
CLI 引数  >  環境変数 (SYSLENZ_*)  >  config.toml  >  デフォルト値
```

**main.rs での解決パターン**:

```rust
mod config;

fn main() -> Result<()> {
    let cfg = config::Config::load();
    let args: Vec<String> = std::env::args().collect();

    // 例: lang の解決 (CLI > env > config > default)
    let locale = if let Some(pos) = args.iter().position(|a| a == "--lang") {
        i18n::Locale::from_str(args.get(pos + 1).unwrap())
    } else if let Ok(lang) = std::env::var("SYSLENZ_LANG") {
        i18n::Locale::from_str(&lang)
    } else if let Some(ref lang) = cfg.general.lang {
        i18n::Locale::from_str(lang)
    } else {
        i18n::Locale::En
    };

    // 同様のパターンを interval_ms, otel endpoint, web port 等に適用
    // ...
}
```

---

## 5. アラートシステム仕様

### 5.1 TOML [[alert]] セクション形式

```toml
[[alert]]
source = "meminfo"           # 対象の /proc ソース名
field = "MemAvailable"       # 対象フィールド名
op = "Lt"                    # 比較演算子: Gt | Lt | Gte | Lte | Eq | Neq
threshold = 500000000        # 閾値 (数値)
severity = "critical"        # 重要度: info | warning | critical
message = "Available memory below 500MB"  # アラートメッセージ

[[alert]]
source = "loadavg"
field = "load_1min"
op = "Gt"
threshold = 8.0
severity = "warning"
message = "Load average exceeded 8.0"

[[alert]]
source = "pressure"
field = "cpu_some_avg10"
op = "Gt"
threshold = 50.0
severity = "warning"
message = "CPU pressure > 50%"
```

### 5.2 CompareOp (比較演算子)

| バリアント | TOML 表記 | 意味 | 評価例 (actual=10, threshold=8) |
|---|---|---|---|
| `Gt` | `"Gt"` | より大きい (`>`) | `true` |
| `Lt` | `"Lt"` | より小さい (`<`) | `false` |
| `Gte` | `"Gte"` | 以上 (`>=`) | `true` |
| `Lte` | `"Lte"` | 以下 (`<=`) | `false` |
| `Eq` | `"Eq"` | 等しい (`==`) | `false` |
| `Neq` | `"Neq"` | 等しくない (`!=`) | `true` |

**数値変換**: `FieldValue` を `f64` に変換して比較する。

```rust
fn field_value_to_f64(fv: &FieldValue) -> Option<f64> {
    match fv {
        FieldValue::Bytes(b)    => Some(*b as f64),
        FieldValue::Integer(i)  => Some(*i as f64),
        FieldValue::Float(f)    => Some(*f),
        FieldValue::Duration(d) => Some(*d),
        FieldValue::Text(_)     => None,
        FieldValue::Table(_)    => None,
    }
}
```

### 5.3 Severity (重要度)

| バリアント | ステータスバー表記 | サイドバー前景色 | Detail 背景色 |
|---|---|---|---|
| `Info` | (カウントに含めない) | Cyan | (変更なし) |
| `Warning` | `[!N WARN]` | Yellow | DarkGray |
| `Critical` | `[!!N CRIT]` | Red + Bold | DarkRed |

### 5.4 状態遷移

```
Normal ──(条件成立)──> Firing ──(条件解除)──> Resolved ──(次サイクル)──> Normal
```

| 遷移 | トリガー | 処理 |
|---|---|---|
| Normal -> Firing | `CompareOp::evaluate()` が `true` を返した | `AlertEvent` を `active_alerts` に追加 |
| Firing -> Firing | 条件が引き続き `true` | **何もしない** (デバウンス) |
| Firing -> Resolved | `CompareOp::evaluate()` が `false` を返した | `AlertEvent` を `active_alerts` から削除、`alert_history` に移動 |
| Resolved -> Normal | 次の `refresh()` サイクル | ステートをリセット |

### 5.5 デバウンス仕様

同一ルール (source + field + op + threshold が一致) が `Firing` 状態の間は、新しい `AlertEvent` を生成しない。これにより、閾値を毎秒超え続けても `active_alerts` には 1 件しか追加されない。

**実装**:

```rust
fn evaluate_alerts(&mut self) {
    for rule in &self.alert_rules {
        let actual = self.current.entries.get(&rule.source)
            .and_then(|e| e.fields.iter().find(|f| f.name == rule.field))
            .and_then(|f| field_value_to_f64(&f.value));

        if let Some(actual) = actual {
            let is_firing = rule.op.evaluate(actual, rule.threshold);
            let already_active = self.active_alerts.iter()
                .any(|a| a.rule.source == rule.source && a.rule.field == rule.field);

            if is_firing && !already_active {
                // Normal -> Firing
                self.active_alerts.push(AlertEvent {
                    timestamp: Instant::now(),
                    rule: rule.clone(),
                    actual_value: actual,
                    host: self.hosts.get(self.active_host)
                        .map(|h| h.host.clone())
                        .unwrap_or_else(|| "localhost".to_string()),
                });
            } else if !is_firing && already_active {
                // Firing -> Resolved
                self.active_alerts.retain(|a| {
                    !(a.rule.source == rule.source && a.rule.field == rule.field)
                });
            }
        }
    }
}
```

### 5.6 表示仕様 (3 箇所)

上記 2.3 節を参照。

**設定パースエラーのハンドリング**: 不正なアラートルール (存在しないソース名、不正な op 値など) はスキップし、ステータスバーに "Warning: N alert rules skipped (invalid config)" を表示する。アプリの起動は止めない。

---

## 6. テスト仕様

### 6.1 Phase 1 テスト一覧 (T1-T11): /proc 不要、即実装可能

#### T1: field_value_display_bytes

**ファイル**: `src/proc/mod.rs` (`#[cfg(test)] mod tests`)

```rust
#[test]
fn field_value_display_bytes() {
    assert_eq!(FieldValue::Bytes(0).display(), "0 B");
    assert_eq!(FieldValue::Bytes(1023).display(), "1023 B");
    assert_eq!(FieldValue::Bytes(1024).display(), "1.0 KiB");
    assert_eq!(FieldValue::Bytes(1048576).display(), "1.0 MiB");
    assert_eq!(FieldValue::Bytes(1073741824).display(), "1.00 GiB");
    assert_eq!(FieldValue::Bytes(16 * 1073741824).display(), "16.00 GiB");
}
```

#### T2: field_value_display_duration

```rust
#[test]
fn field_value_display_duration() {
    assert_eq!(FieldValue::Duration(0.5).display(), "0.5s");
    assert_eq!(FieldValue::Duration(90.0).display(), "1m 30s");
    assert_eq!(FieldValue::Duration(7200.0).display(), "2h 0m 0s");
    assert_eq!(FieldValue::Duration(90061.0).display(), "1d 1h 1m 1s");
}
```

#### T3: snapshot_export_import_roundtrip

**ファイル**: `src/export.rs` (`#[cfg(test)] mod tests`)

```rust
#[test]
fn snapshot_export_import_roundtrip() {
    let original = make_test_snapshot();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.json");

    export_snapshot(&original, &path).unwrap();
    let restored = import_snapshot(&path).unwrap();

    assert_eq!(original.entries.len(), restored.entries.len());
    for (key, entry) in &original.entries {
        let restored_entry = restored.entries.get(key)
            .expect(&format!("Missing key: {}", key));
        assert_eq!(entry.fields.len(), restored_entry.fields.len());
        for (f1, f2) in entry.fields.iter().zip(restored_entry.fields.iter()) {
            assert_eq!(f1.name, f2.name);
            assert_eq!(f1.description, f2.description);
            assert_eq!(f1.value.display(), f2.value.display());
        }
    }
}
```

#### T4: series_export_import_roundtrip

```rust
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

#### T5: diff_identical_returns_empty

**ファイル**: `src/proc/mod.rs` (`#[cfg(test)] mod tests`)

```rust
#[test]
fn diff_identical_returns_empty() {
    let snap = make_test_snapshot();
    let diffs = diff_snapshots(&snap, &snap);
    assert!(diffs.is_empty());
}
```

#### T6: diff_detects_change

```rust
#[test]
fn diff_detects_change() {
    let snap1 = make_test_snapshot();
    let mut snap2 = make_test_snapshot();
    if let Some(entry) = snap2.entries.get_mut("meminfo") {
        entry.fields[0].value = FieldValue::Bytes(999999);
    }
    let diffs = diff_snapshots(&snap1, &snap2);
    assert!(!diffs.is_empty());
    assert_eq!(diffs[0].source, "meminfo");
}
```

#### T7: diff_ignores_small_float

```rust
#[test]
fn diff_ignores_small_float() {
    let snap1 = make_test_snapshot();
    let mut snap2 = snap1.clone();
    // loadavg の load_1min を微小変更 (0.50 -> 0.5005, 差 = 0.0005 < 0.001)
    if let Some(entry) = snap2.entries.get_mut("loadavg") {
        entry.fields[0].value = FieldValue::Float(0.5005);
    }
    let diffs = diff_snapshots(&snap1, &snap2);
    let loadavg_diffs: Vec<_> = diffs.iter()
        .filter(|d| d.source == "loadavg")
        .collect();
    assert!(loadavg_diffs.is_empty());
}
```

#### T8: i18n_all_keys_have_translations

**ファイル**: `src/i18n.rs` (`#[cfg(test)] mod tests`)

```rust
#[test]
fn i18n_all_keys_have_translations() {
    let keys = [
        T::SOURCE, T::DRILL_IN, T::BACK, T::DIFF, T::SEARCH,
        T::REFRESH, T::GRAPH, T::AUTO, T::EXPORT, T::QUIT,
        T::HELP, T::LANG, T::AGO, T::SNAPS,
        // 各ビュー名、Welcome テキスト等も全て列挙
    ];
    for key in keys {
        let en = t(Locale::En, key);
        let ja = t(Locale::Ja, key);
        assert_ne!(en, "?", "Missing EN translation for: {}", key);
        assert_ne!(ja, "?", "Missing JA translation for: {}", key);
    }
}
```

#### T9: i18n_source_descriptions_complete

```rust
#[test]
fn i18n_source_descriptions_complete() {
    let sources = [
        "meminfo", "uptime", "loadavg", "version", "mounts",
        "partitions", "cpuinfo", "stat", "net/dev", "diskstats",
        "processes", "swaps", "buddyinfo", "cgroups", "cmdline",
        "consoles", "crypto", "devices", "filesystems", "interrupts",
        "iomem", "ioports", "locks", "modules", "vmstat",
        "zoneinfo", "softirqs", "misc", "pressure", "net/tcp",
        "net/udp", "net/unix", "net/arp", "net/route", "net/sockstat",
        "net/snmp", "net/netstat", "net/wireless", "slabinfo",
        "pagetypeinfo", "schedstat", "dma", "timer_list",
    ];
    for source in sources {
        let en = source_description(Locale::En, source);
        let ja = source_description(Locale::Ja, source);
        assert_ne!(en, "System information source",
            "Missing EN description for: {}", source);
        assert_ne!(ja, "システム情報ソース",
            "Missing JA description for: {}", source);
    }
}
```

#### T10: locale_from_str_variants

```rust
#[test]
fn locale_from_str_variants() {
    assert_eq!(Locale::from_str("ja"), Locale::Ja);
    assert_eq!(Locale::from_str("jp"), Locale::Ja);
    assert_eq!(Locale::from_str("japanese"), Locale::Ja);
    assert_eq!(Locale::from_str("JA"), Locale::Ja);
    assert_eq!(Locale::from_str("en"), Locale::En);
    assert_eq!(Locale::from_str("unknown"), Locale::En);  // フォールバック
    assert_eq!(Locale::from_str(""), Locale::En);
}
```

#### T11: systemtime_iso8601_roundtrip

```rust
#[test]
fn systemtime_iso8601_roundtrip() {
    let snap = make_test_snapshot();
    let json = serde_json::to_string(&snap).unwrap();
    let restored: Snapshot = serde_json::from_str(&json).unwrap();
    assert_eq!(snap.timestamp, restored.timestamp);
}
```

#### テストヘルパー: make_test_snapshot

```rust
#[cfg(test)]
fn make_test_snapshot() -> Snapshot {
    use std::collections::BTreeMap;
    use std::time::SystemTime;

    let mut entries = BTreeMap::new();

    entries.insert("meminfo".into(), ProcEntry {
        source: "/proc/meminfo".into(),
        fields: vec![
            Field {
                name: "MemTotal".into(),
                value: FieldValue::Bytes(16 * 1024 * 1024 * 1024),
                unit: Some("kB".into()),
                description: "Total usable RAM".into(),
            },
            Field {
                name: "MemFree".into(),
                value: FieldValue::Bytes(8 * 1024 * 1024 * 1024),
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

### 6.2 Phase 2 テスト (T12-T16): parse_content 分離

全 43 パーサーに `pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry>` を追加する。既存の `parse()` は `parse_content` を呼ぶラッパーに変更する。破壊的変更なし。

**リファクタリングパターン**:

```rust
// 変更前 (現状の各パーサー)
pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = std::fs::read_to_string("/proc/uptime")?;
    let parts: Vec<&str> = content.trim().split_whitespace().collect();
    // ... パース処理 ...
    Ok(ProcEntry { ... })
}

// 変更後
pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = std::fs::read_to_string("/proc/uptime")?;
    parse_content(&content)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
    let parts: Vec<&str> = content.trim().split_whitespace().collect();
    // ... パース処理 ...
    Ok(ProcEntry { ... })
}
```

**優先 5 パーサー**:

#### T12: parse_uptime_content

**ファイル**: `src/proc/uptime.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_uptime_content_basic() {
        let content = "12345.67 98765.43\n";
        let entry = parse_content(content).unwrap();
        assert_eq!(entry.source, "/proc/uptime");
        // uptime フィールドが Duration(12345.67) であること
        let uptime_field = entry.fields.iter()
            .find(|f| f.name == "uptime")
            .expect("uptime field not found");
        match &uptime_field.value {
            FieldValue::Duration(d) => assert!((d - 12345.67).abs() < 0.01),
            other => panic!("Expected Duration, got {:?}", other),
        }
    }
}
```

#### T13: parse_loadavg_content

**ファイル**: `src/proc/loadavg.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_loadavg_content_basic() {
        let content = "0.50 0.75 1.00 3/150 12345\n";
        let entry = parse_content(content).unwrap();
        // load_1min = 0.50, load_5min = 0.75, load_15min = 1.00
        // running_processes = 3, total_processes = 150
        let load1 = entry.fields.iter().find(|f| f.name.contains("1")).unwrap();
        match &load1.value {
            FieldValue::Float(f) => assert!((f - 0.50).abs() < 0.01),
            other => panic!("Expected Float, got {:?}", other),
        }
    }
}
```

#### T14: parse_meminfo_content

**ファイル**: `src/proc/meminfo.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_meminfo_content_basic() {
        let content = "\
MemTotal:       16384000 kB
MemFree:         8192000 kB
MemAvailable:   12000000 kB
Buffers:          512000 kB
Cached:          2048000 kB
";
        let entry = parse_content(content).unwrap();
        // MemTotal = 16384000 * 1024 = 16777216000 bytes
        let mem_total = entry.fields.iter()
            .find(|f| f.name.to_lowercase().contains("total") ||
                       f.name.contains("mem_total"))
            .expect("MemTotal field not found");
        match &mem_total.value {
            FieldValue::Bytes(b) => {
                assert_eq!(*b, 16384000 * 1024, "kB to Bytes conversion failed");
            }
            other => panic!("Expected Bytes, got {:?}", other),
        }
    }
}
```

#### T15: parse_version_content

**ファイル**: `src/proc/version.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_version_content_basic() {
        let content = "Linux version 6.6.87.2-microsoft-standard-WSL2 \
            (root@machine) (gcc (GCC) 12.2.0, GNU ld 2.38) \
            #1 SMP Mon Mar 10 2025\n";
        let entry = parse_content(content).unwrap();
        assert!(!entry.fields.is_empty());
        // kernel_version フィールドが存在すること
        let version_field = entry.fields.iter()
            .find(|f| f.name.contains("version") || f.name.contains("kernel"));
        assert!(version_field.is_some(), "version field not found");
    }
}
```

#### T16: parse_stat_content

**ファイル**: `src/proc/stat.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_stat_content_basic() {
        let content = "\
cpu  10132153 290696 3084719 46828483 16683 0 25195 0 0 0
cpu0 1393280 32966 572056 13343292 6130 0 17875 0 0 0
intr 30006437 149 9 0 0 0 0 0 0 1 79 0 0 156 0 0 0
ctxt 38014093
btime 1418183276
processes 26442
procs_running 1
procs_blocked 0
";
        let entry = parse_content(content).unwrap();
        assert!(!entry.fields.is_empty());
        // context_switches が 38014093 であること
        let ctxt = entry.fields.iter()
            .find(|f| f.name.contains("context") || f.name.contains("ctxt"));
        assert!(ctxt.is_some(), "context_switches field not found");
    }
}
```

### 6.3 Phase 3 smoke test (T17-T18)

**ファイル**: `tests/smoke.rs` (新規、インテグレーションテスト)

```rust
//! Smoke tests that require a real Linux /proc filesystem.
//! These tests verify that parsers don't panic on real-world /proc data.
//! They do NOT verify specific values (which are environment-dependent).

#[cfg(target_os = "linux")]
mod linux_smoke {
    use syslenz::proc::*;

    /// T17: 全 43 パーサーが panic せずに Ok または Err を返すこと。
    /// 権限不足 (slabinfo) やカーネル設定依存 (pressure) で Err になるのは許容する。
    #[test]
    fn all_parsers_dont_panic() {
        let _ = meminfo::parse();
        let _ = uptime::parse();
        let _ = loadavg::parse();
        let _ = version::parse();
        let _ = mounts::parse();
        let _ = partitions::parse();
        let _ = cpuinfo::parse();
        let _ = stat::parse();
        let _ = net_dev::parse();
        let _ = diskstats::parse();
        let _ = processes::parse();
        let _ = swaps::parse();
        let _ = buddyinfo::parse();
        let _ = cgroups::parse();
        let _ = cmdline::parse();
        let _ = consoles::parse();
        let _ = crypto::parse();
        let _ = devices::parse();
        let _ = filesystems::parse();
        let _ = interrupts::parse();
        let _ = iomem::parse();
        let _ = ioports::parse();
        let _ = locks::parse();
        let _ = modules::parse();
        let _ = vmstat::parse();
        let _ = zoneinfo::parse();
        let _ = softirqs::parse();
        let _ = misc::parse();
        let _ = pressure::parse();
        let _ = net_tcp::parse();
        let _ = net_udp::parse();
        let _ = net_unix::parse();
        let _ = net_arp::parse();
        let _ = net_route::parse();
        let _ = net_sockstat::parse();
        let _ = net_snmp::parse();
        let _ = net_netstat::parse();
        let _ = net_wireless::parse();
        let _ = slabinfo::parse();
        let _ = pagetypeinfo::parse();
        let _ = schedstat::parse();
        let _ = dma::parse();
        let _ = timer_list::parse();
    }

    /// T18: Snapshot::capture() が成功し、最低 10 個の entry を含むこと。
    #[test]
    fn snapshot_capture_returns_entries() {
        let snap = Snapshot::capture().unwrap();
        assert!(
            snap.entries.len() >= 10,
            "Expected at least 10 entries, got {}",
            snap.entries.len()
        );
        // 必ず存在するはずの entry
        assert!(snap.entries.contains_key("meminfo"));
        assert!(snap.entries.contains_key("uptime"));
        assert!(snap.entries.contains_key("loadavg"));
    }
}
```

### 6.4 Cargo.toml 変更

```toml
[dev-dependencies]
tempfile = "3"
```

### 6.5 テストファイル配置

```
src/
  proc/
    mod.rs          <- T1, T2, T5, T6, T7, T11 (#[cfg(test)])
    uptime.rs       <- T12 (parse_content 分離後)
    loadavg.rs      <- T13
    meminfo.rs      <- T14
    version.rs      <- T15
    stat.rs         <- T16
  export.rs         <- T3, T4 (#[cfg(test)])
  i18n.rs           <- T8, T9, T10 (#[cfg(test)])
tests/
  smoke.rs          <- T17, T18 (integration test)
```

---

## 7. CI/CD 仕様

### 7.1 ci.yml ワークフロー

**ファイル**: `.github/workflows/ci.yml`

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always

jobs:
  check:
    name: Check (${{ matrix.features || 'default' }})
    runs-on: ubuntu-latest
    strategy:
      fail-fast: false
      matrix:
        features:
          - ""                      # デフォルト features
          - "--all-features"        # otel + web + x11widget
          - "--no-default-features" # features なし
    steps:
      - uses: actions/checkout@v4

      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy

      - uses: Swatinem/rust-cache@v2

      - name: Check formatting
        run: cargo fmt --check

      - name: Clippy lint
        run: cargo clippy ${{ matrix.features }} -- -D warnings

      - name: Build
        run: cargo build ${{ matrix.features }}

      - name: Test
        run: cargo test ${{ matrix.features }}

  deny:
    name: License audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: EmbarkStudios/cargo-deny-action@v1
        with:
          command: check licenses
```

### 7.2 release.yml ワークフロー

**ファイル**: `.github/workflows/release.yml`

```yaml
name: Release

on:
  push:
    tags: ["v*"]

permissions:
  contents: write

jobs:
  build:
    name: Build (${{ matrix.target }})
    strategy:
      fail-fast: false
      matrix:
        include:
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
          - target: aarch64-unknown-linux-gnu
            os: ubuntu-latest
          - target: x86_64-unknown-linux-musl
            os: ubuntu-latest
          - target: aarch64-unknown-linux-musl
            os: ubuntu-latest
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4

      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Install cross
        run: cargo install cross --locked

      - name: Build release binary
        run: cross build --release --target ${{ matrix.target }}

      - name: Package
        run: |
          mkdir -p dist
          cp target/${{ matrix.target }}/release/syslenz dist/
          cd dist
          tar czf syslenz-${{ matrix.target }}.tar.gz syslenz
          sha256sum syslenz-${{ matrix.target }}.tar.gz > syslenz-${{ matrix.target }}.tar.gz.sha256

      - uses: actions/upload-artifact@v4
        with:
          name: syslenz-${{ matrix.target }}
          path: |
            dist/syslenz-${{ matrix.target }}.tar.gz
            dist/syslenz-${{ matrix.target }}.tar.gz.sha256

  publish:
    name: Publish release
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: actions/download-artifact@v4
        with:
          path: artifacts
          merge-multiple: true

      - name: Verify CHANGELOG entry
        run: |
          VERSION=${GITHUB_REF_NAME#v}
          grep -q "## \[${VERSION}\]" CHANGELOG.md || \
            (echo "ERROR: No CHANGELOG entry for ${VERSION}" && exit 1)

      - name: Create GitHub Release
        uses: softprops/action-gh-release@v2
        with:
          files: |
            artifacts/*.tar.gz
            artifacts/*.sha256
          generate_release_notes: true

      - uses: dtolnay/rust-toolchain@stable

      - name: Publish to crates.io
        run: cargo publish
        env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

### 7.3 クロスビルドターゲット

| ターゲット | アーキテクチャ | libc | ユースケース |
|---|---|---|---|
| `x86_64-unknown-linux-gnu` | x86_64 | glibc | 標準的な Linux サーバー |
| `aarch64-unknown-linux-gnu` | ARM64 | glibc | AWS Graviton, Raspberry Pi |
| `x86_64-unknown-linux-musl` | x86_64 | musl (static) | Alpine, distroless コンテナ |
| `aarch64-unknown-linux-musl` | ARM64 | musl (static) | ARM コンテナ環境 |

### 7.4 deny.toml 許可ライセンスリスト

**ファイル**: `deny.toml`

```toml
[licenses]
unlicensed = "deny"
allow = [
    "MIT",
    "Apache-2.0",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "ISC",
    "Zlib",
    "Unicode-3.0",
    "Unicode-DFS-2016",
]
copyleft = "deny"

[licenses.private]
ignore = false

[bans]
multiple-versions = "warn"
wildcards = "allow"

[sources]
unknown-registry = "deny"
unknown-git = "deny"
```

### 7.5 Cargo.toml メタデータ追加

```toml
[package]
name = "syslenz"
version = "0.1.0"
edition = "2024"
license = "MIT"
description = "Wireshark for /proc - structured, typed Linux system information viewer"
repository = "https://github.com/xxx/syslenz"
homepage = "https://github.com/xxx/syslenz"
keywords = ["linux", "proc", "tui", "system-monitor", "sysadmin"]
categories = ["command-line-utilities", "os::linux-apis"]
```

---

## 8. README 新構造

### 8.1 完全な目次

```markdown
# syslenz

> Wireshark for /proc

![demo](docs/assets/demo.gif)

(価値提案 2-3 行)

## Why syslenz?
## Install
## Features
## Usage
## Keybindings
<details><summary>Supported /proc Sources (43)</summary></details>
## Configuration
## Roadmap
## License
```

### 8.2 各セクションの内容仕様

#### タイトル + タグライン

```markdown
# syslenz

> Wireshark for /proc

[![CI](https://github.com/xxx/syslenz/actions/workflows/ci.yml/badge.svg)](https://github.com/xxx/syslenz/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/syslenz)](https://crates.io/crates/syslenz)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
```

#### ヒーロー GIF

タグラインとバッジの直後に配置。15 秒以内。vhs で生成。

```markdown
![demo](docs/assets/demo.gif)
```

**GIF の内容 (15 秒)**:
1. `syslenz` を起動 (0-2 秒)
2. サイドバーで `meminfo` を選択 (2-4 秒)
3. フィールドをスクロール (4-6 秒)
4. `net/tcp` に移動、Table ビュー (6-9 秒)
5. `d` で diff ビュー (9-12 秒)
6. `g` で graph ビュー (12-15 秒)

#### 価値提案

```markdown
Explore every Linux `/proc` file as structured, typed data --- from memory and CPU
to network sockets, kernel modules, and cgroup pressure.
No config. No daemon. Just run it.
```

#### Why syslenz?

```markdown
## Why syslenz?

| | |
|---|---|
| **Instant deep-dive** | SSH in, run `syslenz`, see everything. No agents, no config, no setup. |
| **Structured export** | Every field is typed (Bytes, Duration, Table...) with full JSON export. Pipe to `jq`, diff between hosts, attach to incident reports. |
| **Learn Linux internals** | Every field includes a human-readable description. Browse `/proc` like a textbook. |
```

#### Install

```markdown
## Install

### From GitHub Releases (recommended)

\```bash
curl -L https://github.com/xxx/syslenz/releases/latest/download/syslenz-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv syslenz /usr/local/bin/
\```

### From crates.io

\```bash
cargo install syslenz
\```

### From source

\```bash
git clone https://github.com/xxx/syslenz.git
cd syslenz
cargo build --release
\```

### Optional features

\```bash
cargo build --release --features otel     # OpenTelemetry export
cargo build --release --features web      # Web UI
cargo build --release --features x11widget # X11 floating widget
\```
```

#### Features

3 枚のスクリーンショットとともに主要機能を紹介。

```markdown
## Features

### 43 /proc Sources

Browse memory, CPU, network, disk, kernel modules, cgroups, pressure stall
information, and more --- all parsed into typed fields with descriptions.

![main view](docs/assets/main-view.png)

### Live Diffing

See what changed since the last refresh. Fields that increased are green,
decreased are red.

![diff view](docs/assets/diff-view.png)

### Time-Series Graphs

Track any numeric field over time with built-in sparkline graphs.

![graph view](docs/assets/graph-view.png)

### Dashboard

One-glance system overview: load average, memory, CPU, and network.
Press `D` from any view.

### Alerts

Define threshold-based alerts in `config.toml`. Fired alerts appear in the
status bar, sidebar, and field highlighting.

### Multi-Host Monitoring

Monitor multiple hosts via SSH tabs. `Ctrl+1..9` to switch.

### JSON Export / Import

\```bash
syslenz --export snapshot.json
syslenz --import snapshot.json
\```

### OpenTelemetry Export

\```bash
syslenz --otel http://localhost:4317 --interval 5
\```

### Web UI

\```bash
syslenz --web 3000
\```
```

#### Usage

```markdown
## Usage

\```
syslenz                          # TUI mode (default)
syslenz --lang ja                # Japanese UI
syslenz --ssh user@host          # Remote monitoring via SSH
syslenz --ssh user@h1 --ssh user@h2  # Multi-host tabs
syslenz --export snapshot.json   # Export current state
syslenz --import snapshot.json   # View saved snapshot
syslenz --otel                   # OpenTelemetry export
syslenz --web 3000               # Web UI on port 3000
syslenz --history 300            # Keep 5 minutes of snapshots
\```
```

#### Keybindings

セクション 3 の全キーバインドテーブルをそのまま記載。

#### Supported /proc Sources

折りたたみ `<details>` タグで囲む。カテゴリ別に整理。

```markdown
<details>
<summary>Supported /proc Sources (43)</summary>

**System**
- `uptime` - System uptime and idle time
- `version` - Kernel version string
- `cmdline` - Kernel boot parameters
- `loadavg` - Load averages and process counts

**Memory**
- `meminfo` - Detailed memory usage
- `buddyinfo` - Memory fragmentation (buddy allocator)
- `slabinfo` - Kernel slab cache statistics
- `vmstat` - Virtual memory statistics (~100 fields)
- `zoneinfo` - Memory zone information
- `pagetypeinfo` - Page type distribution
- `swaps` - Active swap devices

**CPU / Scheduler**
- `stat` - CPU time breakdown per core
- `cpuinfo` - CPU model, features, cache
- `schedstat` - Scheduler statistics
- `softirqs` - Software interrupt counters
- `interrupts` - Hardware interrupt counters
- `pressure` - Pressure Stall Information (PSI)

**Network**
- `net/dev` - Interface RX/TX bytes and packets
- `net/tcp` - TCP connection table
- `net/udp` - UDP socket table
- `net/unix` - Unix domain sockets
- `net/arp` - ARP cache
- `net/route` - Routing table
- `net/sockstat` - Socket statistics
- `net/snmp` - SNMP counters
- `net/netstat` - Extended network statistics
- `net/wireless` - Wireless interface statistics

**Storage**
- `diskstats` - Block device I/O statistics
- `mounts` - Mounted filesystems
- `partitions` - Partition table
- `filesystems` - Supported filesystem types
- `locks` - File locks

**Process**
- `processes` - Process list (top-like)
- `cgroups` - Control group hierarchy

**Hardware / Kernel**
- `modules` - Loaded kernel modules
- `crypto` - Cryptographic algorithms
- `devices` - Character and block devices
- `consoles` - Console devices
- `iomem` - I/O memory map
- `ioports` - I/O port map
- `dma` - DMA channels
- `timer_list` - Kernel timer list
- `misc` - Miscellaneous devices

</details>
```

#### Configuration

```markdown
## Configuration

syslenz reads `~/.config/syslenz/config.toml` (or `$XDG_CONFIG_HOME/syslenz/config.toml`).
All settings are optional. CLI arguments override config file values.

See [Section 4](#4-設定ファイル仕様) of the technical spec for the complete schema.
```

#### Roadmap

```markdown
## Roadmap

- [x] 43 /proc source parsers
- [x] TUI with sidebar, drill-in, search, diff, graph
- [x] JSON export/import
- [x] SSH remote monitoring
- [x] OpenTelemetry export
- [x] Web UI
- [x] Japanese/English i18n
- [ ] Dashboard view (one-glance overview)
- [ ] Welcome screen (onboarding)
- [ ] Threshold alerts (config.toml)
- [ ] Time-travel diff (compare any two snapshots)
- [ ] Multi-host tabs (SSH)
- [ ] config.toml support
- [ ] CI/CD (GitHub Actions)
- [ ] Binary releases (GitHub Releases, crates.io)
```

#### License

```markdown
## License

[MIT](LICENSE)
```

### 8.3 ビジュアルアセット

| ファイル | 生成方法 | サイズ |
|---|---|---|
| `docs/assets/demo.gif` | vhs (`docs/demo.tape`) | 横 1200px, 15 秒以内 |
| `docs/assets/main-view.png` | vhs Screenshot or 手動キャプチャ | 横 800px |
| `docs/assets/diff-view.png` | 同上 | 横 800px |
| `docs/assets/graph-view.png` | 同上 | 横 800px |

**撮影環境の標準設定**:

```
Terminal: 120 columns x 35 rows
Theme: Catppuccin Mocha (暗色)
Font: JetBrains Mono (等幅)
Font size: 14pt
```

**vhs tape ファイル** (`docs/demo.tape`):

```tape
Output docs/assets/demo.gif
Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 750
Set Theme "Catppuccin Mocha"

Type "syslenz"
Enter
Sleep 2s
Down 5
Sleep 500ms
Enter
Sleep 1.5s
Down 3
Sleep 300ms
Down 3
Sleep 300ms
Down 3
Sleep 1s
Escape
Down 15
Sleep 500ms
Enter
Sleep 1.5s
Type "d"
Sleep 2s
Type "g"
Sleep 2s
Type "q"
```

---

## 実装順序 (推奨)

DGE Session 005 で合意された Phase 構成に基づく。

```
Phase 1: 基盤 (法務 + CI + テスト)
  Step 1-1: LICENSE ファイル作成
  Step 1-2: Cargo.toml メタデータ追加
  Step 1-3: .github/workflows/ci.yml 作成
  Step 1-4: deny.toml 作成
  Step 1-5: CHANGELOG.md 作成
  Step 1-6: Phase 1 テスト (T1-T11) 実装
  Step 1-7: FieldValue に PartialEq derive 追加

Phase 2: データモデルリファクタリング
  Step 2-1: HostState 構造体切り出し
  Step 2-2: active_host_state() ヘルパー追加
  Step 2-3: 既存テストが通ることを確認
  Step 2-4: parse_content 分離 (優先 5 パーサー)
  Step 2-5: Phase 2 テスト (T12-T16) 実装

Phase 3: UI 機能追加
  Step 3-1: View::Dashboard + draw_dashboard()
  Step 3-2: View::Welcome + draw_welcome()
  Step 3-3: D/W キーバインド追加
  Step 3-4: i18n キー追加 (en/ja)
  Step 3-5: render.rs レイアウト分岐

Phase 4: 設定 + アラート
  Step 4-1: src/config.rs 新規作成
  Step 4-2: main.rs に Config::load() 統合
  Step 4-3: src/alert.rs 新規作成 (AlertRule, CompareOp, Severity)
  Step 4-4: App::evaluate_alerts() 実装
  Step 4-5: アラート表示 (ステータスバー, サイドバー着色, フィールド背景色)

Phase 5: タイムトラベル diff
  Step 5-1: diff_target_index フィールド追加
  Step 5-2: [/] キーバインド (Diff ビュー内)
  Step 5-3: ステータスバーに T-N 表示
  Step 5-4: max_snapshots の設定対応

Phase 6: マルチホスト
  Step 6-1: --ssh 複数指定対応
  Step 6-2: タブバー描画
  Step 6-3: Ctrl+1..9 キーバインド
  Step 6-4: ホスト別接続状態表示
  Step 6-5: ホスト別アラート集計

Phase 7: リリース準備
  Step 7-1: .github/workflows/release.yml 作成
  Step 7-2: README 刷新 (新構造)
  Step 7-3: スクリーンショット / GIF 撮影
  Step 7-4: docs/otel.md + docker-compose.yml
  Step 7-5: git tag v0.2.0 && git push --tags
```

---

## 新規ファイル一覧

| ファイル | 内容 | Phase |
|---|---|---|
| `src/config.rs` | Config 構造体、load()、config_path() | Phase 4 |
| `src/alert.rs` | AlertRule, CompareOp, Severity, AlertEvent, AlertState | Phase 4 |
| `tests/smoke.rs` | T17, T18 (Linux smoke test) | Phase 1 |
| `.github/workflows/ci.yml` | CI ワークフロー | Phase 1 |
| `.github/workflows/release.yml` | リリースワークフロー | Phase 7 |
| `deny.toml` | cargo-deny ライセンスポリシー | Phase 1 |
| `LICENSE` | MIT License 全文 | Phase 1 |
| `CHANGELOG.md` | Keep a Changelog 形式 | Phase 1 |
| `docs/demo.tape` | vhs GIF 生成スクリプト | Phase 7 |
| `docs/assets/demo.gif` | ヒーロー GIF | Phase 7 |
| `docs/assets/main-view.png` | メインビュースクリーンショット | Phase 7 |
| `docs/assets/diff-view.png` | Diff ビュースクリーンショット | Phase 7 |
| `docs/assets/graph-view.png` | Graph ビュースクリーンショット | Phase 7 |
| `docs/otel.md` | OpenTelemetry ユースケースドキュメント | Phase 7 |
| `docs/otel-quickstart/docker-compose.yml` | OTel + Prometheus + Grafana | Phase 7 |
| `docs/otel-quickstart/otel-collector-config.yaml` | OTel Collector 設定 | Phase 7 |
| `docs/otel-quickstart/prometheus.yml` | Prometheus 設定 | Phase 7 |

---

## 変更ファイル一覧

| ファイル | 変更内容 | Phase |
|---|---|---|
| `Cargo.toml` | license, description, repository, keywords, categories, dev-dependencies | Phase 1 |
| `src/ui/app.rs` | View enum 拡張, App フィールド追加, HostState, ConnectionStatus, go_back() 拡張, move_up/move_down Dashboard 分岐 | Phase 2-6 |
| `src/ui/render.rs` | draw() レイアウト分岐, draw_welcome(), draw_dashboard(), get_field_display(), アラート表示 | Phase 3-5 |
| `src/main.rs` | D/W/[/]/Ctrl+1..9 キーバインド, Config::load() 統合 | Phase 3-6 |
| `src/i18n.rs` | Welcome/Dashboard 用 T 定数 10 個追加, en()/ja() エントリ追加 | Phase 3 |
| `src/proc/mod.rs` | FieldValue に PartialEq derive, テスト追加 (T1, T2, T5, T6, T7, T11) | Phase 1 |
| `src/export.rs` | テスト追加 (T3, T4) | Phase 1 |
| `src/proc/uptime.rs` | parse_content() 分離, テスト (T12) | Phase 2 |
| `src/proc/loadavg.rs` | parse_content() 分離, テスト (T13) | Phase 2 |
| `src/proc/meminfo.rs` | parse_content() 分離, テスト (T14) | Phase 2 |
| `src/proc/version.rs` | parse_content() 分離, テスト (T15) | Phase 2 |
| `src/proc/stat.rs` | parse_content() 分離, テスト (T16) | Phase 2 |
| `README.md` | 全面刷新 (新構造、バッジ、GIF、Why セクション、折りたたみ Sources) | Phase 7 |
