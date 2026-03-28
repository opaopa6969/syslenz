# DGE Session 013: 統一UIアーキテクチャ — 3重実装の痛みと ViewData 抽象化レイヤーの設計

- **Date**: 2026-03-28
- **Theme**: TUI / Web / X11 の3重レンダリング実装を統一するための ViewData 抽象化レイヤーの設計と移行計画
- **Parent Gaps**: G-UI-1 (3重実装の保守コスト), G-UI-2 (プラットフォーム間の挙動不一致), G-UI-3 (新ビュー追加のスケーラビリティ)
- **Characters**: ヤン (怠惰な簡潔主義者) + リヴァイ (実装の鬼) + 千石 (品質の番人) + ラインハルト (ビジョナリー) + 今泉 (初心者の代弁者) + 僕 (スコープ削減係)
- **Input**: TUI (`src/ui/render.rs` 808行), Web (`src/web.rs` 1173行, HTML/CSS/JS 埋め込み), X11 (`src/x11_widget.rs` 331行)。合計 2312行のレンダリングコード。9つのビュー (Dashboard, Welcome, Overview, Detail, Diff, TableView, Graph, Diagnostics, CategoryGuide) を最大3プラットフォームに個別実装。Session 012 のプラグインアーキテクチャと Session 011 のカテゴリ教育により、今後さらにビューが増加する見込み。

---

## 現状の整理

先輩 (ナレーション): 3つのレンダラーの現在地を整理する。

### レンダリング実装の比較

| プラットフォーム | ファイル | 行数 | 対応ビュー | 状態管理 |
|---------------|---------|------|-----------|---------|
| TUI (ratatui) | `src/ui/render.rs` | 808 | Dashboard, Welcome, Overview, Detail, Diff, TableView, Graph, Diagnostics | Rust `App` struct |
| Web (axum + 埋め込みHTML/JS) | `src/web.rs` | 1173 | Dashboard, Welcome, Detail, Diff, Diagnostics, Graph | JS `S` object (独立した状態) |
| X11 (x11rb) | `src/x11_widget.rs` | 331 | Dashboard のみ (CPU%, メモリ, Load) | `WidgetMetrics` struct |

### 重複の実態

**Dashboard の例** — 同じ「Load / Memory / CPU / Network」を3回別々に描画:

- **TUI** (`render.rs:485-603`): `draw_dashboard()` — `get_field_value()` で Snapshot からフィールドを取得、ratatui の `Table`, `Paragraph`, `Row`, `Cell` で描画。i18n は `app.locale` で分岐。
- **Web** (`web.rs:278-`): JS の `renderDashboard()` — SSE で受信した JSON Snapshot を JS で解析、DOM 操作で `dash-card` を生成。i18n は JS の `I[S.locale]` で分岐。
- **X11** (`x11_widget.rs:49-`): `WidgetMetrics::from_snapshot()` — Snapshot から手動で値を抽出、x11rb の `draw_text()` でピクセル座標に描画。i18n なし。

**問題の核心**: 3つとも同じデータ (Snapshot の loadavg, meminfo, stat, net/dev) から同じ情報を抽出しているが、抽出ロジックが3箇所に散らばっている。

---

## Scene 1: 問題の可視化 — 3重実装の痛み

先輩: 3つのプラットフォームに同じビューを実装し続けることの問題を議論する。

🎋 千石 (腕を組んで): 「まず事実を見てくれ。Web の Dashboard と TUI の Dashboard で表示項目が微妙に違う。TUI は `selected_dashboard_section` でセクション選択のハイライトがある。Web にはそれがない代わりに Chart.js のリアルタイムグラフがある。X11 は CPU%, メモリ使用率, Load の3つしか出さない。**同じ "Dashboard" という名前の、別のプロダクトが3つある。** これは品質問題だ。」

👤 今泉: 「あの... Session 011 で設計した CategoryGuide ビューって、これから3つに書くんですか？ カテゴリごとに "メモリの全体像" とか "ネットワークの問題切り分け" とか教育コンテンツを表示するビューですよね。あれを TUI と Web と X11 に...」

🎋 千石: 「X11 に教育コンテンツを出す意味はないだろう。だが TUI と Web には必要だ。マークダウン的な構造化テキスト + ハイライトされたメトリクス値 + ソースへのリンク。TUI では ratatui の `Paragraph` と `Span` でスタイリング、Web では HTML/CSS。同じコンテンツを2つの言語で書く — いや、マークアップ言語まで含めれば **4つの言語** で書くことになる。Rust のスタイリングコード、HTML、CSS、JavaScript。」

👤 今泉: 「Session 012 のプラグインの話も... プラグインが独自のビューを提供するとき、3つのレンダラー用のコードを全部書かないといけないんですか？」

⚔️ リヴァイ: 「現実を見ろ。今の実装は **動いている**。Dashboard も Detail も Diff も、3つのプラットフォームで動いている。品質問題があるのは事実だが、ユーザーの前でクラッシュしたことはない。ここで大規模リファクタを始めて、3つのレンダラーが全部壊れるリスクを取るのか？」

🎋 千石: 「リスクの話をするなら、新ビューを追加するたびにバグが3倍になるリスクも考えろ。CategoryGuide を3つに書いたら、そのうち1つは必ず壊れる。修正するときも3箇所直す。直し忘れる。**今の方式はスケールしない。**」

**数字で見る痛み:**

```
現在 (9ビュー x 最大3プラットフォーム):
  TUI: 8ビュー実装済み = 808行
  Web: 6ビュー実装済み = 1173行 (HTML/CSS/JS含む)
  X11: 1ビュー実装済み = 331行
  合計: 2312行

Session 011 + 012 の計画を実装した場合 (推定):
  CategoryGuide (5カテゴリ x 2言語)
  PluginView (プラグインごと)
  EducationOverlay (フィールド詳細)
  → +3ビュー x 2プラットフォーム (TUI + Web) = +6実装
  → 推定 +1500行

保守コスト:
  ビュー修正1件 → 最大3箇所修正 → テスト3回 → レビュー3回
```

---

## Scene 2: 各アプローチの議論

先輩: 5つのアプローチについて各 character が議論する。

### Approach A: ViewModel Pattern (共通 Component Tree)

```rust
enum Component {
    Table { headers: Vec<String>, rows: Vec<Vec<String>> },
    KeyValue { items: Vec<(String, String)> },
    BarChart { label: String, value: f64, max: f64 },
    Text { content: String, style: TextStyle },
    Layout { direction: Direction, children: Vec<Component> },
    Sparkline { data: Vec<f64> },
}

trait Renderer {
    fn render(&self, component: &Component, area: Rect);
}
```

🦁 ラインハルト: 「Component tree は正しい方向だ。UI をデータとして表現する。React の仮想DOM、Flutter の Widget tree、同じ原理だ。定義は1回、レンダリングは N 回。」

🧠 ヤン (椅子に深く座りながら): 「ラインハルト、理想は分かる。だが React でも Vue でも、仮想DOM を自作して失敗したプロジェクトは山ほどある。汎用 Component tree を作ると、必ず "この Table にだけ特殊なスタイルが要る" "この Layout だけ条件分岐が要る" という例外が出てくる。結果、Component enum に特殊ケースが増殖して、抽象化レイヤーが **2番目のシステム** になる。UI フレームワークを作る仕事をしたいのか？ syslenz を作る仕事をしたいのか？」

⚔️ リヴァイ: 「同意だ。汎用 Component tree は罠。Dashboard の Load セクションと Detail の Field テーブルは、データ構造が全然違う。無理に同じ `Component` に押し込むと、各 Renderer が `Component` の中身を解釈するために大量の match 文を書くことになる。**抽象化のための抽象化は汚い。**」

### Approach B: Server-Rendered HTML (TUI = Web)

🧠 ヤン: 「ターミナルで HTML をレンダリングする？ `carbonyl` や `browsh` みたいな？ 正気か。依存関係が Chromium になるぞ。syslenz のバイナリが 100MB になる。」

⚔️ リヴァイ: 「却下。」

### Approach C: Shared Data Model, Platform-Specific Views

🧠 ヤン: 「現状の延長だろ。"データモデルは共通、ビューは各プラットフォームで書く" — 今やってることだ。何が変わる？」

🎋 千石: 「今の問題は、データモデルが共通でも **データ抽出ロジック** が共通じゃないこと。TUI の `draw_dashboard()` が Snapshot から `loadavg` の `load1` を取り出すコードと、Web の JS が `snapshot.entries.loadavg.fields` から同じ値を取り出すコードが別々に存在する。"何を表示するか" の決定が各レンダラーに散らばっている。」

### Approach D: Web-First, TUI as Text Client

🧠 ヤン: 「Web を一次にして TUI をクライアント化？ TUI が `curl localhost:8080` してテキストに変換する？ レイテンシが... いや、localhost なら問題ないか。だが TUI ユーザーは Web サーバーを起動したくない人だ。」

👤 今泉: 「SSH 先のサーバーで syslenz を使うとき、Web サーバーが必要になるんですか？ それは使いにくくないですか？」

🧠 ヤン: 「却下の理由としては十分だ。」

### Approach E: DSL/Template

🦁 ラインハルト: 「テンプレートは教育コンテンツには良い。だが Dashboard のようなインタラクティブなビューには向かない。`selected_dashboard_section` のハイライトをテンプレートで表現するのは無理がある。」

⚔️ リヴァイ: 「TOML でビュー定義？ TOML パーサーのバグを直す仕事が増えるだけだ。」

---

### 議論の収束

⚔️ リヴァイ: 「整理する。B は論外。D も却下。E は教育コンテンツだけなら使えるが、汎用解ではない。残るは A と C だ。で、A の純粋な Component tree は過度な抽象化の罠がある。C の "現状維持 + 整理" は問題を先送りにするだけ。**どっちの罠がマシか。**」

🦁 ラインハルト: 「A と C のハイブリッドだ。汎用 Component tree ではなく、**ビューごとの ViewData 構造体** を定義する。Dashboard には `DashboardData`、Detail には `DetailData`。各プラットフォームは ViewData を受け取って自由に描画する。抽象化は軽い。ビュー定義は1箇所。」

🧠 ヤン (少し目を開けて): 「...それなら許容範囲だ。汎用 Component を作らない。ビューの数だけ構造体がある。新ビューを追加するときは構造体を1つ定義して、各レンダラーに1つずつ `render_*` を書く。**抽象化レイヤーがない** から壊れようがない。ただのデータ構造だ。」

---

## Scene 3: 現実的な統一レイヤーの設計

先輩: Approach C + A のハイブリッド — ViewData パターンの具体設計を議論する。

### アーキテクチャ概要

```
            App (state)
                |
        ┌───────┴───────┐
        │  ViewData 層  │  ← "何を表示するか" を決定 (Rust, 共通)
        │  (純粋関数)    │
        └───────┬───────┘
                |
    ┌───────────┼───────────┐
    │           │           │
TUI Renderer  Web API    X11 Renderer
(ratatui)     (JSON→JS)  (x11rb, 簡易版)
    │           │           │
 "どう描くか"  "どう描くか"  "どう描くか"
```

### ViewData 定義

```rust
// src/ui/view_data.rs

use crate::diagnostics::DiagnosticFinding;

/// 全ビューの ViewData を統合する enum。
/// 各 variant はそのビューが「何を表示するか」を完全に記述する。
/// レンダリング方法 (色、レイアウト、ウィジェット) には一切言及しない。
#[derive(Debug, Clone, serde::Serialize)]
pub enum ViewData {
    Dashboard(DashboardData),
    Welcome(WelcomeData),
    Detail(DetailData),
    Diff(DiffData),
    TableView(TableViewData),
    Graph(GraphData),
    Diagnostics(DiagnosticsData),
    CategoryGuide(CategoryGuideData),
}

// ---- Dashboard ----

#[derive(Debug, Clone, serde::Serialize)]
pub struct DashboardData {
    pub load: LoadSection,
    pub memory: MemorySection,
    pub cpu: CpuSection,
    pub network: NetworkSection,
    pub selected_section: usize,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LoadSection {
    pub load1: String,
    pub load5: String,
    pub load15: String,
    pub uptime: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MemorySection {
    pub fields: Vec<MetricItem>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CpuSection {
    pub fields: Vec<MetricItem>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct NetworkSection {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MetricItem {
    pub name: String,
    pub value: String,
    pub severity: MetricSeverity,
}

#[derive(Debug, Clone, serde::Serialize)]
pub enum MetricSeverity {
    Normal,
    Warning,
    Critical,
    Muted,
}

// ---- Welcome ----

#[derive(Debug, Clone, serde::Serialize)]
pub struct WelcomeData {
    pub title: String,
    pub subtitle: String,
    pub keybindings: Vec<KeyBinding>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct KeyBinding {
    pub key: String,
    pub description: String,
}

// ---- Detail ----

#[derive(Debug, Clone, serde::Serialize)]
pub struct DetailData {
    pub source_name: String,
    pub fields: Vec<FieldItem>,
    pub selected_field: usize,
    pub scroll_offset: usize,
    pub total_fields: usize,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct FieldItem {
    pub name: String,
    pub value: String,
    pub unit: String,
    pub description: String,
    pub value_type: String,  // "bytes", "int", "float", "text", "table", "duration"
}

// ---- Diff ----

#[derive(Debug, Clone, serde::Serialize)]
pub struct DiffData {
    pub changes: Vec<DiffChange>,
    pub total_changes: usize,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DiffChange {
    pub source: String,
    pub field: String,
    pub old_value: String,
    pub new_value: String,
}

// ---- TableView ----

#[derive(Debug, Clone, serde::Serialize)]
pub struct TableViewData {
    pub source_name: String,
    pub field_name: String,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub scroll_offset: usize,
}

// ---- Graph ----

#[derive(Debug, Clone, serde::Serialize)]
pub struct GraphData {
    pub source_name: String,
    pub field_name: String,
    pub data_points: Vec<f64>,
    pub min: f64,
    pub max: f64,
}

// ---- Diagnostics ----

#[derive(Debug, Clone, serde::Serialize)]
pub struct DiagnosticsData {
    pub findings: Vec<DiagnosticItem>,
    pub total_findings: usize,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DiagnosticItem {
    pub severity: String,     // "INFO", "WARN", "CRIT"
    pub source: String,
    pub title: String,
    pub detail: String,
    pub suggestion: String,
}

// ---- CategoryGuide ----

#[derive(Debug, Clone, serde::Serialize)]
pub struct CategoryGuideData {
    pub category_name: String,
    pub title: String,
    pub overview: String,
    pub related_sources: Vec<RelatedSource>,
    pub diagnostic_flow: Vec<DiagnosticStep>,
    pub key_metrics: Vec<MetricItem>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct RelatedSource {
    pub source_name: String,
    pub relevance: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DiagnosticStep {
    pub step: usize,
    pub instruction: String,
    pub source: String,
    pub field: String,
}
```

⚔️ リヴァイ: 「いいな。この構造体を見れば、Dashboard が何を表示するか一目で分かる。LoadSection に `load1`, `load5`, `load15`, `uptime`。MemorySection に `MetricItem` のリスト。ネットワークはヘッダー + 行。**仕様がコードに埋め込まれている。**」

🧠 ヤン: 「注目すべきは `MetricSeverity` だ。値の "意味" をデータ層で決定している。MemAvailable が 10% 以下なら `Critical`、50% 以上なら `Normal`。この判断は **1箇所で行われる**。TUI は `Critical` を赤で塗り、Web は `.sev-crit` クラスを付け、X11 は `RED_COLOR` で描く。判断ロジックの重複がない。」

🎋 千石: 「`serde::Serialize` を derive しているのが重要だ。ViewData を JSON にシリアライズすれば、そのまま Web API のレスポンスになる。Web の JS は ViewData の JSON を受け取って描画するだけ。Snapshot の生データを JS で解析する必要がなくなる。」

### ViewData 生成関数

```rust
// src/ui/view_data.rs (続き)

impl ViewData {
    /// App の現在の状態から ViewData を生成する。
    /// これが "何を表示するか" の唯一の決定ポイント。
    pub fn from_app(app: &App) -> Self {
        match app.view {
            View::Dashboard => ViewData::Dashboard(build_dashboard_data(app)),
            View::Welcome => ViewData::Welcome(build_welcome_data(app)),
            View::Detail | View::Overview => ViewData::Detail(build_detail_data(app)),
            View::Diff => ViewData::Diff(build_diff_data(app)),
            View::TableView => ViewData::TableView(build_table_view_data(app)),
            View::Graph => ViewData::Graph(build_graph_data(app)),
            View::Diagnostics => ViewData::Diagnostics(build_diagnostics_data(app)),
        }
    }
}

fn build_dashboard_data(app: &App) -> DashboardData {
    let l = app.locale;
    DashboardData {
        load: LoadSection {
            load1: get_field_value(app, "loadavg", "load1"),
            load5: get_field_value(app, "loadavg", "load5"),
            load15: get_field_value(app, "loadavg", "load15"),
            uptime: get_field_value(app, "uptime", "uptime"),
        },
        memory: MemorySection {
            fields: ["MemTotal", "MemFree", "MemAvailable", "Buffers",
                     "Cached", "SwapTotal", "SwapFree"]
                .iter()
                .filter_map(|name| extract_metric(app, "meminfo", name))
                .collect(),
        },
        cpu: CpuSection {
            fields: ["cpu_user", "cpu_system", "cpu_idle", "cpu_iowait",
                     "context_switches", "processes_running"]
                .iter()
                .filter_map(|name| extract_metric(app, "stat", name))
                .collect(),
        },
        network: build_network_section(app),
        selected_section: app.selected_dashboard_section,
    }
}
```

🦁 ラインハルト: 「`build_dashboard_data()` を見れば、Dashboard が何を表示するか完全に分かる。loadavg から3つの値、meminfo から7つのフィールド、stat から6つのフィールド、net/dev のテーブル。**これが仕様書だ。** TUI の `draw_dashboard()` が 120行かけてやっていたデータ抽出が、30行の純粋関数になっている。」

---

## Scene 4: Web との同期問題

先輩: Web UI の状態管理をどう統一するかを議論する。

👤 今泉: 「あの... 根本的な疑問なんですが。TUI の状態は Rust の `App` struct にありますよね。Web の状態は JavaScript の `S` object にある。この2つが別々に存在しているのは問題じゃないですか？ Web で `S.selectedSource` を変えても、サーバー側の `App` は知らない。つまり ViewData を生成する `App` と、Web の `S` が別の状態を持っている。」

🎋 千石: 「いい指摘だ。現在の Web 実装を見ろ:」

```javascript
// 現在の web.rs 内の JS
const S = {
  snapshot: null,        // ← サーバーの Snapshot のコピー
  view: 'dashboard',     // ← ローカル状態 (サーバーと無関係)
  selectedSource: 0,     // ← ローカル状態
  selectedField: 0,      // ← ローカル状態
  helpLevel: 0,          // ← ローカル状態
  // ...
};
```

🎋 千石: 「`snapshot` はサーバーから SSE で受信する。だが `view`, `selectedSource` 等はブラウザ側のローカル状態だ。サーバーの `App` がどのビューを表示しているかは Web 側に関係ない。これは意図的な設計だ — Web ユーザーと TUI ユーザーが独立して操作できる。」

🧠 ヤン: 「つまり ViewData の生成は2箇所で行われる。TUI 用は `ViewData::from_app(&app)` で Rust 側。Web 用は... サーバーが ViewData を生成するのか？ クライアントが生成するのか？」

⚔️ リヴァイ: 「シンプルにしろ。2つの選択肢だ:」

**選択肢 1: サーバーが ViewData を生成して JSON で送る**

```
SSE: { "type": "dashboard", "data": { "load": {...}, "memory": {...}, ... } }
→ JS は受け取った ViewData をそのまま DOM に描画
→ JS の責務: UI操作 (クリック、キー) → サーバーにアクション送信 → ViewData 受信 → 描画
```

**選択肢 2: サーバーは Snapshot を送り、JS が ViewData 相当のロジックを実行**

```
SSE: { "entries": {...} }  (現状のまま)
→ JS が snapshot から表示データを抽出 (= build_dashboard_data の JS 版)
→ JS の責務: データ抽出 + UI操作 + 描画
```

🧠 ヤン: 「選択肢 2 は現状維持だ。ViewData の意味がない。選択肢 1 にしろ。」

⚔️ リヴァイ: 「待て。選択肢 1 だと、Web ユーザーがサイドバーのソースをクリックするたびにサーバーにリクエストを送って ViewData を再生成する。レイテンシが入る。」

🧠 ヤン: 「localhost だろ。1ms 以下だ。」

⚔️ リヴァイ: 「...まあそうだ。リモートの場合は？ `--remote` で他のホストに接続してる場合。」

🧠 ヤン: 「リモートの場合も syslenz の Web サーバーはローカルで動いている。Snapshot の取得がリモートなだけで、ViewData の生成はローカルだ。問題ない。」

### 結論: SSE で ViewData を JSON として送信

```
現在:
  SSE → Snapshot (JSON) → JS が解析 → JS が状態管理 → JS が描画

移行後:
  SSE → Snapshot (JSON) → サーバーがキャッシュ
  Web操作 → POST /api/action { "action": "select_source", "index": 3 }
          → サーバーが Web 用 App 状態を更新
          → ViewData 生成
          → SSE で ViewData (JSON) を送信
          → JS が描画のみ

  または (より簡易):
  SSE → Snapshot (JSON) → JS がローカルで ViewData を構築 (build 関数を共有)
  ※ ビューロジック (何を表示するか) は Rust の ViewData 構造体が正規定義
  ※ JS 版は TypeScript 型定義から自動生成か、手動で合わせる
```

🧠 ヤン: 「完璧を目指すな。Phase 1 では Web は現状維持でいい。ViewData の恩恵を最初に受けるのは TUI だ。TUI の `draw_dashboard()` が 120行から 30行になる。それだけで十分な勝利だ。Web の統一は Phase 2 でやれ。」

僕: 「...賛成...」

---

## Scene 5: MVP と移行計画

先輩: 現実的な移行計画を策定する。全面リファクタではなく、段階的な移行。

🦁 ラインハルト: 「理想は一気に全ビューを ViewData 化することだ。だが——」

🧠 ヤン (遮って): 「やめろ。一気にやるな。」

⚔️ リヴァイ: 「同意だ。段階的にやる。**1つのビューを ViewData 化して、3つのレンダラーで動くことを確認してから次に進む。**」

僕: 「...Phase 1 だけでも十分では...」

### Phase 1: ViewData 構造体の定義 + TUI の分離 (最優先)

**目標**: TUI の `draw_*()` を「データ生成」と「描画」に分離する。

**手順**:
1. `src/ui/view_data.rs` を作成、ViewData enum と各構造体を定義
2. `build_dashboard_data(&App) -> DashboardData` を実装
3. TUI の `draw_dashboard(f, app, area)` を `draw_dashboard(f, &DashboardData, area, locale)` に変更
4. テスト: ViewData の生成が正しいことを単体テストで確認 (UI テストではなく **データテスト**)
5. 同様に Welcome, Diagnostics を ViewData 化 (全画面ビュー = サイドバーなし = 単純)
6. Detail, Diff, TableView を ViewData 化

**成果物**:
- `src/ui/view_data.rs`: ViewData 定義 + `build_*()` 関数群
- `src/ui/render.rs`: `draw_*()` が ViewData を受け取る形に変更 (行数は変わらないか微増)
- テスト: `build_dashboard_data()` に対する単体テスト

**影響範囲**: TUI のみ。Web と X11 は変更なし。

### Phase 2: Web API が ViewData を JSON で返す

**目標**: Web の JS がサーバーからの ViewData JSON をそのまま描画する。

**手順**:
1. `/api/view?view=dashboard` エンドポイントを追加 — ViewData を JSON で返す
2. JS の `renderDashboard()` を ViewData JSON ベースに書き換え
3. SSE で Snapshot の代わりに (または加えて) ViewData を送信
4. JS 側のデータ抽出ロジック (`snapshot.entries.meminfo.fields.find(...)`) を削除
5. 同様に全ビューを移行

**成果物**:
- `/api/view` エンドポイント
- JS の `render*()` が ViewData JSON を受け取る形に変更
- JS の行数が大幅に削減 (データ抽出ロジックがなくなる)

**注意**: Web 側のナビゲーション状態 (selectedSource 等) は引き続き JS 側で管理。ただし「何を表示するか」の決定はサーバー側に移行。

### Phase 3: X11 の ViewData 対応 (簡易版)

**目標**: X11 ウィジェットが `DashboardData` の一部を描画する。

**手順**:
1. `WidgetMetrics` を `DashboardData` から生成するコンバーターを書く
2. 将来的に `WidgetMetrics` を廃止し、`DashboardData` を直接使う

**成果物**: X11 のデータ抽出ロジック (`WidgetMetrics::from_snapshot()`) が `DashboardData` 経由に変更。

👤 今泉: 「そもそも X11 ウィジェットって使う人いるんですか？」

⚔️ リヴァイ: 「...」

🧠 ヤン: 「X11 は Dashboard しか表示しない。Phase 1 で DashboardData ができれば、X11 の対応は `from_snapshot()` を `from_dashboard_data()` に変えるだけだ。20分で終わる。優先度は低いが、コストも低い。」

僕: 「...Phase 1 だけでも十分では...」

🧠 ヤン: 「Phase 1 だけでも十分だ。Phase 2 と 3 は必要になったときにやればいい。」

---

## Gap Summary

| Gap ID | 説明 | 重要度 | Phase |
|--------|------|--------|-------|
| G-UI-1 | 3重実装の保守コスト: 新ビュー追加に3箇所の変更が必要 | 高 | Phase 1 で緩和 |
| G-UI-2 | プラットフォーム間の挙動不一致: Dashboard の表示項目が TUI/Web/X11 で異なる | 中 | Phase 1 で ViewData が正規定義に |
| G-UI-3 | 新ビュー追加のスケーラビリティ: CategoryGuide, PluginView の追加コスト | 高 | Phase 1 で構造体定義が仕様書に |
| G-UI-4 | Web のデータ抽出ロジック重複: JS が Snapshot を直接解析 | 中 | Phase 2 で解消 |
| G-UI-5 | テスト不可能な UI ロジック: draw_*() 内にデータ抽出とレンダリングが混在 | 高 | Phase 1 で分離 |
| G-UI-6 | X11 の限定的ビュー: Dashboard のみ | 低 | Phase 3 (低優先) |

---

## Concrete ViewData Struct Definitions

上記 Scene 3 に完全な定義あり。要約:

| ViewData Variant | 主要フィールド | 用途 |
|-----------------|---------------|------|
| `DashboardData` | `LoadSection`, `MemorySection`, `CpuSection`, `NetworkSection`, `selected_section` | システム概要 |
| `WelcomeData` | `title`, `subtitle`, `keybindings: Vec<KeyBinding>` | 初回表示 / ヘルプ |
| `DetailData` | `source_name`, `fields: Vec<FieldItem>`, `selected_field`, `scroll_offset` | ソース詳細 |
| `DiffData` | `changes: Vec<DiffChange>`, `total_changes` | スナップショット差分 |
| `TableViewData` | `source_name`, `field_name`, `headers`, `rows`, `scroll_offset` | テーブルデータ展開 |
| `GraphData` | `source_name`, `field_name`, `data_points`, `min`, `max` | スパークライン |
| `DiagnosticsData` | `findings: Vec<DiagnosticItem>`, `total_findings` | 自動診断結果 |
| `CategoryGuideData` | `category_name`, `overview`, `related_sources`, `diagnostic_flow`, `key_metrics` | カテゴリ教育 |

---

## Migration Plan for Existing Code

### Step-by-Step: Dashboard の ViewData 化 (Phase 1 の最初のタスク)

```
Before:
  draw_dashboard(f, app, area)
    → app.current.entries.get("loadavg") → field.value → display
    → ratatui Table/Paragraph で描画

After:
  let view_data = ViewData::from_app(app);  // or build_dashboard_data(app)
  draw_dashboard(f, &dashboard_data, area, app.locale)
    → dashboard_data.load.load1 を Span に変換
    → ratatui Table/Paragraph で描画
```

**変更対象ファイル**:
- 新規: `src/ui/view_data.rs` (ViewData 定義 + build 関数)
- 変更: `src/ui/mod.rs` (`pub mod view_data;` 追加)
- 変更: `src/ui/render.rs` (`draw()` が ViewData を生成して各 `draw_*()` に渡す)
- 変更なし: `src/web.rs`, `src/x11_widget.rs` (Phase 1 では触らない)

### Sidebar と共通 UI の扱い

`draw_sidebar()`, `draw_status_bar()`, `draw_help_panel()` は ViewData 化の対象外にする。これらは TUI 固有の UI 要素であり、Web と X11 には対応する概念が異なる (Web はサイドバーの HTML、X11 にはサイドバーがない)。ViewData はビューのコンテンツ領域のみをカバーする。

---

## File Structure Proposal

```
src/
  ui/
    mod.rs              ← pub mod view_data; 追加
    view_data.rs        ← NEW: ViewData enum + 構造体 + build_*() 関数
    render.rs           ← draw_*() が ViewData を受け取る形に変更
    app.rs              ← 変更なし (App struct, View enum)
    graph.rs            ← draw_graph() が GraphData を受け取る形に変更
  web.rs                ← Phase 2: /api/view エンドポイント追加
  x11_widget.rs         ← Phase 3: DashboardData 経由に変更
  diagnostics.rs        ← 変更なし (analyze() は DiagnosticFinding を返す)
```

**view_data.rs の構成**:

```
view_data.rs
  ├── ViewData enum
  ├── 構造体定義 (DashboardData, DetailData, ...)
  ├── ViewData::from_app() → ViewData
  ├── build_dashboard_data() → DashboardData
  ├── build_welcome_data() → WelcomeData
  ├── build_detail_data() → DetailData
  ├── build_diff_data() → DiffData
  ├── build_table_view_data() → TableViewData
  ├── build_graph_data() → GraphData
  ├── build_diagnostics_data() → DiagnosticsData
  └── helper: get_field_value(), extract_metric(), ...
```

---

## Decision: Approach C+A Hybrid (ViewData Pattern)

**選択**: Approach C (Shared Data Model) + Approach A (ViewModel Pattern) のハイブリッド

**理由**:
1. **汎用 Component tree は作らない** — ビューごとの具体的な構造体。過度な抽象化の罠を回避。
2. **ViewData が仕様書になる** — `DashboardData` を見れば Dashboard が何を表示するか分かる。
3. **テスト可能になる** — `build_dashboard_data()` は純粋関数。Snapshot を入力して ViewData を検証。UI フレームワーク不要。
4. **段階的移行が可能** — 1ビューずつ ViewData 化できる。Big Bang リファクタ不要。
5. **Web との統合パスが明確** — `serde::Serialize` により、ViewData をそのまま JSON API にできる。
6. **既存コードの破壊が最小限** — `draw_*()` のシグネチャが変わるだけ。レンダリングロジックはそのまま。

**却下した選択肢**:
- **A (純粋 Component tree)**: 汎用すぎる。UI フレームワーク開発になる。
- **B (Server-Rendered HTML)**: 依存関係が巨大。TUI の利点を殺す。
- **C (現状維持)**: スケールしない。新ビュー追加のコストが線形に増加。
- **D (Web-First)**: SSH 環境で不便。TUI ユーザーの UX を犠牲にする。
- **E (DSL/Template)**: インタラクティブビューに不向き。教育コンテンツのみなら部分的に有効。

**最初の一歩**: `src/ui/view_data.rs` を作成し、`DashboardData` と `build_dashboard_data()` を実装する。TUI の `draw_dashboard()` を ViewData ベースに変更する。これだけで Phase 1 の価値の半分が得られる。
