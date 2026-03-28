# DGE Session 003: G2 + G3 深掘り — ダッシュボードとオンボーディング

- **Date**: 2026-03-28
- **Theme**: 初回起動体験の改善 / ダッシュボードビュー追加
- **Parent Gaps**: G2 (Missing logic, Medium), G3 (UX gap, Medium) from Session 001
- **Characters**: ヤン + リヴァイ + 僕 + 千石
- **Template**: feature-planning

---

## Scene 1: ダッシュボードに何を表示するか

先輩 (ナレーション): 現状の syslenz を起動すると、左サイドバーに 43 の /proc ソースがアルファベット順で並び、右側には先頭ソース (buddyinfo) の Detail が表示される。htop は起動した瞬間に CPU バー、メモリバー、プロセスリストが見える。glances も同様。syslenz は「buddyinfo の Zoneinfo テーブル」が見える。これでは初心者は何を見ているのかわからない。

☕ ヤン: 「htop を起動して最初に見えるのは、CPU 使用率とメモリ残量。それだけで "このマシン大丈夫かな" がわかる。syslenz は buddyinfo... メモリフラグメンテーションの話を最初に見せてる。誰がそれ欲しいの。紅茶ください。」

→ **Gap 発見: 初期表示ソースが BTreeMap のキー順 (alphabetical) で、buddyinfo が先頭になっている。ユーザーにとって最も重要な情報 (CPU, メモリ, ロードアベレージ) が最初に見えない。**

⚔️ リヴァイ: 「ダッシュボードに載せるのは 5 つでいい。loadavg (負荷)、meminfo (メモリ)、stat (CPU)、uptime (稼働時間)、net/dev (ネットワーク)。これだけで "このマシンの状態" は 3 秒で把握できる。余計な装飾はいらない。」

→ **Spec implication: ダッシュボードのデータソースは固定 5 つ。loadavg, meminfo, stat, uptime, net/dev。動的に選ぶ仕組みは MVP では不要。**

☕ ヤン: 「表示形式はどうする？全フィールド出したら Overview と変わらない。meminfo なら MemTotal, MemFree, MemAvailable, SwapTotal, SwapFree の 5 つだけ。loadavg は load_1, load_5, load_15 の 3 つ。stat は cpu_user_pct, cpu_system_pct, cpu_idle_pct。厳選すべきでしょ。」

→ **Spec implication: 各ソースから「ダッシュボード用フィールド」を 3-5 個ハードコードで選定する。全フィールドではなくサマリーを見せる。**

😰 僕: 「...あの、サイドバーとの関係は... ダッシュボードを見ている時、サイドバーは何を表示するんですか... ダッシュボードからソースにドリルインできないと... 意味が...」

→ **Gap 発見: ダッシュボードと既存のサイドバーナビゲーションの関係が未定義。ダッシュボードの各セクションから対応ソースの Detail に飛べる必要がある。**

🎀 千石: 「ダッシュボードに diff 情報も入れたいわ。"前回との変化" が色でわかるとか。数値が増えたら緑、減ったら赤。それだけで "何か起きてる" がわかるでしょ。」

→ **Spec implication: ダッシュボードのフィールド値に diff カラーリングを適用する。snapshots が 2 つ以上あれば、前回比で増加=Green, 減少=Red, 変化なし=White。**

---

## Scene 2: 初回ユーザー体験

先輩: 初めて syslenz を起動するユーザーを想定する。`cargo install` したばかりの学生、あるいは SRE が初めてこのツールを触る場面。現状: ターミナルにいきなり 43 ソースのサイドバーと buddyinfo の詳細が表示される。キーバインドはステータスバーに `j/k source  Enter drill-in  BS back  d diff  / search  ? help  L lang  q quit` と書いてあるが小さい。

😰 僕: 「...初めて起動した人は... j と k が vim のカーソル移動だって知らないかもしれません... ? でヘルプが出るのも... ? を押そうと思わないんじゃ...」

→ **Gap 発見: ヘルプへの導線が弱い。`?` キーの存在を知らないとヘルプにたどり着けない。初回起動時には自動でヒントを表示するか、ウェルカム画面が必要。**

☕ ヤン: 「オンボーディングは 2 段階でいいよ。第 1 段階: 初回起動時にウェルカム画面を出す。"syslenz へようこそ。Enter でダッシュボードへ。? でヘルプ。" それだけ。第 2 段階: ダッシュボード画面の各セクションに "Enter で詳細" のヒントを薄く表示。5 回起動したらヒント消す。」

→ **Spec implication: 初回起動時に Welcome 画面 (View::Welcome) を表示。Enter でダッシュボードに遷移。2 回目以降は直接ダッシュボードに遷移。**

⚔️ リヴァイ: 「"5 回起動したら消す" とかいう曖昧な仕組みはいらない。`~/.config/syslenz/` にファイルを作る？それもいらない。もっと単純にしろ。初回起動かどうかは判定しない。`W` キーでいつでも Welcome を出す。ダッシュボードは `D` キー。以上。」

→ **Spec implication: 起動時のデフォルトビューを View::Dashboard にする。View::Welcome は `W` キーでトグル。初回判定ロジックは MVP では不要。状態管理を最小にする。**

🎀 千石: 「ウェルカム画面のデザインは大事よ。syslenz のロゴ... はいらないか。でも少なくとも: (1) ツール名とバージョン、(2) "Wireshark for /proc" のタグライン、(3) キーバインド一覧 (7-8 行)、(4) "Press Enter to start / D for Dashboard" の CTA。これが全部ターミナル幅 80 文字に収まること。」

→ **Spec implication: Welcome 画面は Paragraph ウィジェット 1 つで実装。中央寄せ。背景色なし。ターミナル 80x24 で崩れないレイアウト。**

😰 僕: 「...あの、日本語のときは... ウェルカム画面も日本語ですよね... i18n 対応...」

→ **Gap 発見: Welcome 画面とダッシュボード画面のテキストに i18n キーが必要。既存の T 構造体に追加する。**

---

## Scene 3: 実装アプローチ

先輩: 現在の App 構造体は `view: View` で画面状態を管理している。View enum は `Overview, Detail, Diff, TableView, Graph` の 5 つ。ここに Dashboard と Welcome を追加する方針で議論する。

⚔️ リヴァイ: 「View に 2 つ追加するだけだ。render.rs の match app.view に分岐を足す。Dashboard は sidebar を表示しない、全幅レイアウト。Welcome も全幅。Detail/Diff 系は今まで通り sidebar 付き。」

→ **Spec implication: Dashboard と Welcome では sidebar を非表示にする。render.rs の `draw()` で view に応じてレイアウトを切り替える。**

```
// レイアウト分岐の概念
match app.view {
    View::Welcome | View::Dashboard => {
        // sidebar なし、全幅で content を描画
        draw_welcome(f, app, chunks[0]);  // or draw_dashboard
    }
    _ => {
        // 既存: sidebar 22 + content
    }
}
```

☕ ヤン: 「Dashboard のレイアウトは 3 行構成でいいよ。上段: loadavg + uptime (1 行サマリー)。中段: meminfo + stat (2 カラム、各フィールド 3-5 行)。下段: net/dev (テーブル、インタフェースごと RX/TX)。全部 Paragraph か Table ウィジェットで済む。新しい依存は不要。」

→ **Spec implication: ダッシュボードのレイアウト構成:**

```
┌─────────────────────────────────────────────────────┐
│ Dashboard — hostname | uptime: 3d 2h | load: 0.5 0.3 0.2 │
├─────────────────────────┬───────────────────────────┤
│  Memory                 │  CPU                      │
│  Total:  15.6 GiB       │  User:   12.3%            │
│  Free:    8.2 GiB       │  System:  3.1%            │
│  Avail:  10.1 GiB       │  Idle:   84.6%            │
│  Swap:    2.0 GiB       │  IOWait:  0.0%            │
│  SwapFree: 2.0 GiB      │  IRQ:     0.0%            │
├─────────────────────────┴───────────────────────────┤
│  Network (net/dev)                                  │
│  eth0    RX: 1.2 GiB   TX: 340 MiB   RX/s: 12 KiB │
│  lo      RX: 500 MiB   TX: 500 MiB   RX/s:  0 B   │
├─────────────────────────────────────────────────────┤
│ D:dashboard  ?:help  Enter:browse sources  q:quit   │
└─────────────────────────────────────────────────────┘
```

😰 僕: 「...net/dev の RX/s って... 前回スナップショットとの差分を時間で割らないと出ませんよね... それは今回のスコープに入れるんですか...」

→ **Gap 発見: レート計算 (bytes/sec) は diff_snapshots の拡張が必要。MVP では raw 値 (累計 RX/TX bytes) のみ表示し、レート表示は将来対応とする。**

⚔️ リヴァイ: 「Dashboard から Detail への遷移方法を決めろ。提案: Dashboard で j/k でセクション (Memory, CPU, Network) を選択、Enter で対応ソースの Detail に飛ぶ。`selected_dashboard_section: usize` を App に追加。」

→ **Spec implication: App に `selected_dashboard_section: usize` を追加。Dashboard のセクション数は固定 (0=loadavg/uptime, 1=meminfo, 2=stat, 3=net/dev)。Enter で対応ソースの selected_source を設定し View::Detail に遷移。**

🎀 千石: 「Esc か BS で Dashboard に戻れることも重要ね。go_back() のロジックで、Detail から戻った先が Overview じゃなくて Dashboard になるようにする。でも... 既存ユーザーが困らないように、サイドバーから入った Detail は Overview に戻る、Dashboard から入った Detail は Dashboard に戻る、って区別が必要よ。」

→ **Spec implication: App に `came_from_dashboard: bool` を追加。Dashboard から Detail に入った場合は go_back() で Dashboard に戻る。サイドバーから入った場合は Overview に戻る（既存動作を維持）。**

---

## Scene 4: MVP スコープ

先輩: 実装量を見積もる。現在の全コードは app.rs (333 行), render.rs (428 行), main.rs (~250 行), i18n.rs (~200 行)。追加する変更量を最小限にしたい。

☕ ヤン: 「最小スコープは 3 つ。(1) View::Dashboard 追加 + draw_dashboard 関数。(2) View::Welcome 追加 + draw_welcome 関数。(3) キーバインド D と W を main.rs に追加。以上。i18n は後回しでもいい。ハードコードの英語で動かして、次のイテレーションで ja を足す。」

→ **Spec implication: MVP では i18n キーを追加するが、英語テキストのハードコードで先に動作確認。ja 翻訳は同一 PR 内で対応可能な量。**

⚔️ リヴァイ: 「ダッシュボードのデータ取得は既存の `app.current.entries` から引くだけだ。新しいデータ構造はいらない。draw_dashboard で `app.current.entries.get("meminfo")` して必要なフィールドを name で検索する。パフォーマンスの心配もない。」

→ **Spec implication: Dashboard 用の新しいデータ構造は追加しない。既存の Snapshot.entries から直接フィールドを引く。ヘルパー関数 `fn get_field_value(snapshot: &Snapshot, source: &str, field_name: &str) -> Option<String>` を追加。**

🎀 千石: 「テストは？ draw_dashboard の出力を ratatui の TestBackend で検証できるわよ。最低限、(1) Dashboard にメモリ値が表示される、(2) Welcome 画面にキーバインドが表示される、の 2 つは書くべき。」

→ **Spec implication: テストは別 PR。この PR では動作する UI を優先。**

😰 僕: 「...起動時のデフォルトビューを Dashboard にするということは... `App::new()` の `view: View::Overview` を `view: View::Dashboard` に変えるんですよね... 既存の `--import` とか `--ssh` のときもダッシュボードでいいんですか...」

→ **Gap 発見: リモートモードやインポートモードでもダッシュボードがデフォルトで良いか要検討。リモートは OK (同じ /proc データ)。インポートは過去データなので Dashboard のリアルタイム感と矛盾する可能性がある。**

☕ ヤン: 「インポートモードのときは Overview をデフォルトにしたままでいい。Dashboard はライブデータ前提。`App::new()` と `App::from_remote()` は Dashboard、`App::from_imported()` は Overview。単純。」

→ **Spec implication: `App::new()` と `App::from_remote()` の初期 view を `View::Dashboard` に変更。`App::from_imported()` は `View::Overview` のまま。**

---

## Gap Summary (Session 003)

| # | Gap | Category | Severity | 対応方針 |
|---|-----|----------|----------|----------|
| G2-1 | 初期表示が buddyinfo (BTreeMap alphabetical 順) | UX gap | High | Dashboard をデフォルトビューにする |
| G2-2 | 重要ソースのサマリーが一目で見えない | Missing logic | High | Dashboard に loadavg/meminfo/stat/uptime/net_dev の厳選フィールドを表示 |
| G2-3 | Dashboard から Detail へのドリルインがない | UX gap | Medium | j/k でセクション選択、Enter で Detail 遷移 |
| G2-4 | diff カラーリングがダッシュボードにない | UX gap | Low | 前回比の増減で Green/Red 着色 (MVP 後) |
| G2-5 | レート計算 (bytes/sec) がない | Missing logic | Low | MVP では累計値のみ、レートは将来対応 |
| G3-1 | ヘルプへの導線が弱い (? を知らないと辿れない) | UX gap | High | Welcome 画面でキーバインド一覧を表示 |
| G3-2 | 初回起動時に何をすべきかわからない | UX gap | High | Welcome 画面 (W キー) + Dashboard デフォルト化 |
| G3-3 | Welcome/Dashboard テキストの i18n 未対応 | i18n gap | Medium | T 構造体に新キー追加、en/ja 両方実装 |
| G3-4 | インポートモードで Dashboard が不適切な可能性 | UX gap | Low | from_imported のみ View::Overview 維持 |

---

## Spec 提案

### 1. View enum の拡張 (`src/ui/app.rs`)

```rust
pub enum View {
    Welcome,     // 新規: キーバインド一覧 + CTA
    Dashboard,   // 新規: システムサマリー (5 ソース厳選)
    Overview,    // 既存: サイドバー + 選択ソースの全フィールド
    Detail,      // 既存
    Diff,        // 既存
    TableView,   // 既存
    Graph,       // 既存
}
```

### 2. App 構造体の追加フィールド (`src/ui/app.rs`)

```rust
pub struct App {
    // ... 既存フィールド ...
    pub selected_dashboard_section: usize,  // 新規: 0-3 (loadavg, meminfo, stat, net/dev)
    pub came_from_dashboard: bool,          // 新規: Detail からの戻り先判定
}
```

初期値: `selected_dashboard_section: 0`, `came_from_dashboard: false`

### 3. App::new() / from_remote() の初期ビュー変更

```rust
// App::new() と App::from_remote():
view: View::Dashboard,   // 変更: Overview → Dashboard

// App::from_imported(): 変更なし
view: View::Overview,
```

### 4. キーバインド追加 (`src/main.rs`)

```rust
KeyCode::Char('D') => {
    app.view = View::Dashboard;
    app.focus = Focus::Content;
    app.came_from_dashboard = false;
}
KeyCode::Char('W') => {
    app.view = View::Welcome;
}
```

### 5. go_back() の拡張 (`src/ui/app.rs`)

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

### 6. Dashboard セクション → Detail 遷移 (`src/ui/app.rs`)

```rust
// Dashboard での Enter 処理 (enter_selected 内に追加)
View::Dashboard => {
    let target_source = match self.selected_dashboard_section {
        0 => "loadavg",
        1 => "meminfo",
        2 => "stat",
        3 => "net/dev",
        _ => "loadavg",
    };
    if let Some(idx) = self.source_keys.iter().position(|k| k == target_source) {
        self.selected_source = idx;
        self.selected_field = 0;
        self.field_scroll = 0;
        self.view = View::Detail;
        self.focus = Focus::Content;
        self.came_from_dashboard = true;
    }
}
```

### 7. Dashboard での j/k ナビゲーション (`src/ui/app.rs`)

move_up / move_down 内に Dashboard 分岐を追加:

```rust
// move_up 内
Focus::Content => match self.view {
    View::Dashboard => {
        self.selected_dashboard_section =
            self.selected_dashboard_section.saturating_sub(1);
    }
    // ... 既存の分岐 ...
}

// move_down 内
Focus::Content => match self.view {
    View::Dashboard => {
        if self.selected_dashboard_section < 3 {
            self.selected_dashboard_section += 1;
        }
    }
    // ... 既存の分岐 ...
}
```

### 8. render.rs のレイアウト分岐

```rust
pub fn draw(f: &mut Frame, app: &App) {
    match app.view {
        View::Welcome => {
            // 全幅レイアウト (sidebar なし)
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(10), Constraint::Length(3)])
                .split(f.area());
            draw_welcome(f, app, chunks[0]);
            draw_status_bar(f, app, chunks[1]);
            return;
        }
        View::Dashboard => {
            // 全幅レイアウト (sidebar なし)
            let chunks = if app.show_help {
                // main + help + status
            } else {
                // main + status
            };
            draw_dashboard(f, app, main_area);
            // help, status は既存ロジック
            return;
        }
        _ => { /* 既存の sidebar + content レイアウト */ }
    }
}
```

### 9. draw_welcome 関数 (`src/ui/render.rs`)

```rust
fn draw_welcome(f: &mut Frame, app: &App, area: Rect) {
    let l = app.locale;
    // 中央寄せのため vertical + horizontal Layout で Flex::Center 相当を実現
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
            Span::styled("  j/k  ", Style::default().fg(Color::Yellow)),
            Span::raw(i18n::t(l, T::WELCOME_NAV)),       // "Navigate"
        ]),
        Line::from(vec![
            Span::styled("  Enter", Style::default().fg(Color::Yellow)),
            Span::raw(i18n::t(l, T::WELCOME_DRILL)),     // "Drill into source"
        ]),
        Line::from(vec![
            Span::styled("  d    ", Style::default().fg(Color::Yellow)),
            Span::raw(i18n::t(l, T::WELCOME_DIFF)),      // "Show changes"
        ]),
        Line::from(vec![
            Span::styled("  /    ", Style::default().fg(Color::Yellow)),
            Span::raw(i18n::t(l, T::WELCOME_SEARCH)),    // "Search sources"
        ]),
        Line::from(vec![
            Span::styled("  g    ", Style::default().fg(Color::Yellow)),
            Span::raw(i18n::t(l, T::WELCOME_GRAPH)),     // "Sparkline graph"
        ]),
        Line::from(vec![
            Span::styled("  ?    ", Style::default().fg(Color::Yellow)),
            Span::raw(i18n::t(l, T::WELCOME_HELP)),      // "Field descriptions"
        ]),
        Line::from(vec![
            Span::styled("  L    ", Style::default().fg(Color::Yellow)),
            Span::raw(i18n::t(l, T::WELCOME_LANG)),      // "Toggle EN/JA"
        ]),
        Line::from(""),
        Line::from(Span::styled(
            i18n::t(l, T::WELCOME_CTA),                  // "Press Enter or D for Dashboard"
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        )),
    ];
    let p = Paragraph::new(text)
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(p, area);
}
```

### 10. draw_dashboard 関数 (`src/ui/render.rs`)

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

    // --- ヘッダー行 ---
    // uptime の uptime フィールド + loadavg の load_1, load_5, load_15
    let uptime_str = get_field_display(&app.current, "uptime", "uptime");
    let load1 = get_field_display(&app.current, "loadavg", "load_1");
    let load5 = get_field_display(&app.current, "loadavg", "load_5");
    let load15 = get_field_display(&app.current, "loadavg", "load_15");
    let header_line = Line::from(vec![
        Span::styled(" uptime: ", Style::default().fg(Color::DarkGray)),
        Span::styled(&uptime_str, Style::default().fg(Color::Cyan)),
        Span::styled("  load: ", Style::default().fg(Color::DarkGray)),
        Span::styled(format!("{} {} {}", load1, load5, load15),
            Style::default().fg(Color::Yellow)),
    ]);
    let is_selected_0 = app.selected_dashboard_section == 0;
    let header_block = Block::default().borders(Borders::ALL)
        .title(" Dashboard ")
        .border_style(if is_selected_0 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });
    f.render_widget(Paragraph::new(header_line).block(header_block), chunks[0]);

    // --- 中段: meminfo + stat ---
    let mid = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // meminfo パネル
    let mem_fields = [
        ("MemTotal", "mem_total"),
        ("MemFree", "mem_free"),
        ("MemAvailable", "mem_available"),
        ("SwapTotal", "swap_total"),
        ("SwapFree", "swap_free"),
    ];
    let mem_lines: Vec<Line> = mem_fields.iter().map(|(label, field_name)| {
        let val = get_field_display(&app.current, "meminfo", field_name);
        Line::from(vec![
            Span::styled(format!("  {:<14}", label), Style::default().fg(Color::DarkGray)),
            Span::styled(val, Style::default().fg(Color::Green)),
        ])
    }).collect();
    let is_selected_1 = app.selected_dashboard_section == 1;
    let mem_block = Block::default().borders(Borders::ALL)
        .title(" Memory ")
        .border_style(if is_selected_1 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });
    f.render_widget(Paragraph::new(mem_lines).block(mem_block), mid[0]);

    // stat パネル (CPU)
    let cpu_fields = [
        ("User", "cpu_user_pct"),
        ("System", "cpu_system_pct"),
        ("Idle", "cpu_idle_pct"),
        ("IOWait", "cpu_iowait_pct"),
        ("IRQ", "cpu_irq_pct"),
    ];
    let cpu_lines: Vec<Line> = cpu_fields.iter().map(|(label, field_name)| {
        let val = get_field_display(&app.current, "stat", field_name);
        Line::from(vec![
            Span::styled(format!("  {:<14}", label), Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{}%", val), Style::default().fg(Color::Magenta)),
        ])
    }).collect();
    let is_selected_2 = app.selected_dashboard_section == 2;
    let cpu_block = Block::default().borders(Borders::ALL)
        .title(" CPU ")
        .border_style(if is_selected_2 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });
    f.render_widget(Paragraph::new(cpu_lines).block(cpu_block), mid[1]);

    // --- 下段: net/dev ---
    // net/dev は Table 型フィールドなので、テーブルウィジェットで表示
    let is_selected_3 = app.selected_dashboard_section == 3;
    let net_block = Block::default().borders(Borders::ALL)
        .title(" Network (net/dev) ")
        .border_style(if is_selected_3 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });
    // net/dev の Table フィールドからデータを引く
    // ... (既存の draw_table_view と同様のロジック)
    f.render_widget(Paragraph::new(" (net/dev table)").block(net_block), chunks[2]);
}

/// ヘルパー: Snapshot からフィールドの display 値を取得
fn get_field_display(snapshot: &crate::proc::Snapshot, source: &str, field_name: &str) -> String {
    snapshot.entries.get(source)
        .and_then(|entry| entry.fields.iter().find(|f| f.name == field_name))
        .map(|f| f.value.display())
        .unwrap_or_else(|| "-".to_string())
}
```

### 11. i18n キーの追加 (`src/i18n.rs`)

T 構造体に追加:

```rust
// Welcome 画面
pub const WELCOME_NAV: &str = "welcome_nav";
pub const WELCOME_DRILL: &str = "welcome_drill";
pub const WELCOME_DIFF: &str = "welcome_diff";
pub const WELCOME_SEARCH: &str = "welcome_search";
pub const WELCOME_GRAPH: &str = "welcome_graph";
pub const WELCOME_HELP: &str = "welcome_help";
pub const WELCOME_LANG: &str = "welcome_lang";
pub const WELCOME_CTA: &str = "welcome_cta";

// Dashboard
pub const VIEW_DASHBOARD: &str = "view_dashboard";
pub const VIEW_WELCOME: &str = "view_welcome";
```

翻訳テーブル:

| キー | EN | JA |
|------|----|----|
| welcome_nav | Navigate sources | ソースを選択 |
| welcome_drill | Drill into source | ソースの詳細を表示 |
| welcome_diff | Show changes since last refresh | 前回からの変更を表示 |
| welcome_search | Search sources | ソースを検索 |
| welcome_graph | Sparkline graph (numeric fields) | スパークライングラフ (数値フィールド) |
| welcome_help | Toggle field descriptions | フィールド説明の表示切替 |
| welcome_lang | Toggle EN/JA | 言語切替 EN/JA |
| welcome_cta | Press Enter or D for Dashboard | Enter または D でダッシュボードへ |
| view_dashboard | Dashboard | ダッシュボード |
| view_welcome | Welcome | ようこそ |

### 12. ステータスバーの Dashboard/Welcome 対応

draw_status_bar の view_name マッチに追加:

```rust
View::Dashboard => i18n::t(l, T::VIEW_DASHBOARD),
View::Welcome => i18n::t(l, T::VIEW_WELCOME),
```

ステータスバーのキーバインド表示に `D` と `W` を追加:

```rust
Span::styled("D ", Style::default().fg(Color::Yellow)),
Span::raw(format!("{}  ", i18n::t(l, T::VIEW_DASHBOARD))),
```

---

## 変更ファイル一覧

| ファイル | 変更内容 | 推定行数 |
|---------|---------|---------|
| `src/ui/app.rs` | View enum に 2 追加、App に 2 フィールド追加、move_up/move_down/enter_selected/go_back の Dashboard 分岐 | +60 行 |
| `src/ui/render.rs` | draw() のレイアウト分岐、draw_welcome()、draw_dashboard()、get_field_display() | +120 行 |
| `src/main.rs` | D/W キーバインド追加、Welcome での Enter 処理 | +15 行 |
| `src/i18n.rs` | T 定数 10 追加、en()/ja() に 10 エントリ追加 | +30 行 |
| **合計** | | **+225 行** |
