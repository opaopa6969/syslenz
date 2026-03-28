# DGE Session 008: G12 + G13 深掘り — ポジショニングとビジュアルアセット

- **Date**: 2026-03-28
- **Theme**: README の第一印象を決定づける「言葉」と「絵」
- **Parent Gaps**: G12 (ポジショニング / タグラインが曖昧, Medium), G13 (README にスクリーンショットがない, High)
- **Characters**: 利根川 (ユーザー真実) + ラインハルト (征服者) + 大和田 (ビジネスリアリスト) + ヤン (簡潔化)
- **Input**: Session 002 成果 — 3 ペルソナ (SRE/学生/監査), 3 強み (ゼロ設定/構造化 export/教育性)

---

## Scene 1: README の構造と第一印象

先輩 (ナレーション): 現在の README は「タグライン → Features 箇条書き → Install → Keybindings → Supported Sources → Roadmap → License」の順。スクリーンショットはゼロ。タグラインは "Wireshark for /proc — a structured, schema-driven system information viewer"。GitHub を開いた瞬間に何が目に入るかを議論する。

🎰 利根川: 「いいか、現実を教えてやる。ユーザーは README の最初の 3 行しか読まない。3 行だ。タイトル、タグライン、そしてスクリーンショットがあればそこに目が行く。なければ Features の最初の 1 項目だけ見て閉じる。GitHub のスター数が伸びるプロジェクトは例外なく "開いた瞬間に何のツールかわかる" ものだ。今の syslenz はどうだ？」

先輩: 今のタグラインは "Wireshark for /proc — a structured, schema-driven system information viewer"。

🎰 利根川: 「"schema-driven system information viewer" — この後半が致命的だ。誰も "schema-driven" で興奮しない。これは実装者の言葉であってユーザーの言葉じゃない。"Wireshark for /proc" は秀逸だ。これだけでいい。後半は殺せ。」

→ **Gap 発見: タグラインの後半が技術実装の説明であり、ユーザー便益を語っていない。"schema-driven" は開発者の自己満足。**

☕ ヤン: 「タグラインのあと、何を書くかだけど。今は "syslenz parses every /proc file into typed, structured data and presents it in a fast TUI with diffing, time-series graphs, and JSON export/import" — 長すぎ。息継ぎなしで 43 文字。読まないよ。」

→ **Gap 発見: リード文が 1 文に詰め込みすぎ。機能の羅列であって価値の提示ではない。**

👑 ラインハルト: 「Session 002 で結論が出ている。3 つの強みだ。ゼロ設定、構造化 export、教育性。これを 3 行で示せ。そしてその直後にスクリーンショットだ。言葉で説明する前に見せろ。百聞は一見に如かず — これはすべての TUI ツールに当てはまる真理だ。」

→ **Spec implication: README 冒頭の構造は「タグライン (1 行) → 価値提案 (3 行) → スクリーンショット → 以降詳細」の順にすべき。**

🦈 大和田: 「ちなみに、今の Features セクションは 6 項目あるが、最初の "43 /proc sources" がいきなり来る。ユーザーからすれば "だから何？" だ。"43" という数字に意味があるのは /proc を知ってる人間だけだ。/proc を知らない人間 — つまり潜在ユーザーの大半 — にとっては "htop が見せない Linux の内部を全部見せる" のほうが刺さる。」

→ **Gap 発見: Features の表現が実装ベース ("43 sources", "schema-driven") でユーザーベネフィットベース ("see what htop can't", "understand your system") ではない。**

---

## Scene 2: ビジュアルアセット戦略

先輩: TUI の現在の画面構成を確認する。左サイドバー (22 列幅) に 43 ソースが並ぶ。右コンテンツエリアにフィールドテーブル (Name / Value / Unit / Description の 4 列)。最下部にステータスバー (キーバインド表示)。diff ビューでは赤/緑のハイライト。graph ビューでは sparkline。

🎰 利根川: 「スクリーンショットがない README は、メニューに写真がないレストランだ。Session 001 でも言った。で、まだない。なぜない？」

先輩: 撮影していないためです。

🎰 利根川: 「言い訳はいい。問題は "何を撮るか" だ。1 枚で全部伝えようとするな。かといって 10 枚貼っても誰もスクロールしない。必要なのは 3 枚 + 1 GIF。以上だ。」

→ **Gap 発見: スクリーンショット戦略が未定義。何を撮るか、何枚必要か、どの順で見せるかの設計がない。**

☕ ヤン: 「3 枚の内訳を考えよう。(1) メインビュー — サイドバーと meminfo のフィールドテーブル。これで "何のツールか" がわかる。(2) Diff ビュー — 赤/緑のハイライト。"変化が見える" ことがわかる。(3) Graph ビュー — sparkline。"時系列で追える" ことがわかる。」

→ **Spec implication: スクリーンショット 3 枚の構成:**
1. **メインビュー (meminfo)** — サイドバー + フィールドテーブル。Name/Value/Unit/Description が見える状態
2. **Diff ビュー** — 変化したフィールドが赤/緑でハイライトされている状態
3. **Graph ビュー** — 任意の数値フィールドの sparkline が表示されている状態

👑 ラインハルト: 「GIF は最も重要だ。README の最上部、タグラインの直後に置く。15 秒以内で "起動 → サイドバー移動 → meminfo 選択 → フィールド閲覧 → diff 表示 → graph 表示" を流す。これだけで 10 秒以内にユーザーは理解する。スクリーンショットは GIF の下の Features セクションで補助的に使え。」

→ **Spec implication: GIF (15 秒以内) を README 最上部に配置。スクリーンショットは各 Feature の横に配置。**

🦈 大和田: 「GIF の撮り方だが、3 つ選択肢がある。(1) asciinema でターミナル録画して gif に変換、(2) vhs (charmbracelet/vhs) でスクリプトから自動生成、(3) 手動で画面録画。推奨は vhs だ。再現可能で、CI に組み込める。ターミナルのフォントやサイズが毎回同じになる。」

🎰 利根川: 「vhs 一択だ。理由は簡単だ。README のスクリーンショットは "メンテナンスコスト" がかかる。UI を変更するたびに撮り直しが必要だ。手動撮影ではこれが地獄になる。vhs なら `.tape` ファイルを更新して `vhs record` するだけだ。asciinema は閲覧に JS embed が必要で、GitHub の README では inline 再生できない。vhs は GIF を直接吐く。」

→ **Spec implication: vhs (charmbracelet/vhs) を採用。.tape ファイルをリポジトリに含め、GIF を docs/assets/ に出力。**

☕ ヤン: 「スクリーンショットのサイズとターミナルの設定も決めておこう。GitHub の README のコンテンツ幅は約 830px。GIF の横幅は 800px、高さは 500px くらいがちょうどいい。ターミナルは 120x35 あれば左サイドバー + 右コンテンツが綺麗に収まるはず。フォントは等幅。テーマは暗色系がいい — TUI は暗い背景のほうが映える。」

→ **Spec implication: 撮影環境を標準化。ターミナル 120x35、暗色テーマ、フォントサイズは GIF 横幅 800px に収まるよう調整。**

---

## Scene 3: ポジショニングステートメント

先輩: Session 002 の成果をベースに、README の冒頭コピーを確定させる。現在のタグラインは "Wireshark for /proc — a structured, schema-driven system information viewer"。提案されている 3 ペルソナ: SRE、学生、監査員。3 強み: ゼロ設定、構造化 export、教育性。

👑 ラインハルト: 「ポジショニングは 1 行で決まる。"Wireshark for /proc" — これ以上の説明は要らない。Wireshark を知ってる人間なら "ああ、パケットキャプチャの /proc 版か" と 1 秒で理解する。知らない人間には別の説明が要る。だが、ターゲットユーザー — SRE、学生、セキュリティ — は全員 Wireshark を知っている。このタグラインは変えるな。」

🎰 利根川: 「タグラインはいい。問題はその下の文だ。今の "parses every /proc file into typed, structured data" は実装の説明だ。ユーザーは "何ができるか" を知りたいのであって "どう実装されているか" を知りたいのではない。"See everything Linux knows about itself" — これくらい大胆に言え。」

→ **Gap 発見: リード文が "what it does technically" であって "what you can do with it" ではない。**

☕ ヤン: 「3 行で書くなら、こうじゃない？」

```
syslenz — Wireshark for /proc

See everything Linux knows about itself.
Zero config. One binary. Every /proc file, structured and searchable.
```

🦈 大和田: 「悪くない。だが "See everything Linux knows about itself" は抽象的すぎる。具体性がないとスターは押されない。"htop shows processes. syslenz shows everything else." — 比較で入ったほうが早い。」

🎰 利根川: 「いいぞ大和田。だがもう一歩だ。"htop shows processes" は htop の宣伝になる。syslenz が主語であるべきだ。こうだ:」

```
syslenz — Wireshark for /proc

Explore 43 Linux /proc sources as structured, typed data — from memory and CPU
to network sockets, kernel modules, and cgroup pressure.
No config. No daemon. Just run it.
```

👑 ラインハルト: 「"Just run it" — これだ。syslenz の本質はこの 3 語に集約される。ssh して、バイナリを実行して、全部見える。Grafana は 30 分かかる。Datadog はクレジットカードが要る。syslenz は "just run it" だ。」

→ **Spec implication: 新タグライン構成:**
- **L1**: `# syslenz` (プロジェクト名)
- **L2**: `> Wireshark for /proc` (タグライン — 変更なし)
- **L3**: GIF (ヒーローイメージ)
- **L4-L5**: 具体的な価値提案 (43 sources, structured/typed, no config)
- **L6 以降**: ペルソナ別ユースケース

☕ ヤン: 「ペルソナ別セクションは Session 002 の提案を洗練しよう。3 つのユースケースを "Why syslenz?" セクションにまとめる:」

```markdown
## Why syslenz?

**Instant deep-dive** — SSH into a server, run `syslenz`, see everything.
No agents, no config files, no dashboards to set up.

**Structured export** — Every field is typed (Bytes, Duration, Table...) with
JSON export. Pipe to jq, attach to incident reports, diff between hosts.

**Learn Linux internals** — Every field has a human-readable description.
Browse /proc like a textbook, not a hex dump.
```

🎰 利根川: 「最後に、README の構造全体を決めろ。今の README は Features から始まるが、ユーザーが知りたい順番は "これは何？ → どう見える？ → どうインストールする？ → 何ができる？" だ。今の順番は "これは何 → 何ができる → インストール → キーバインド → ソース一覧 → ロードマップ"。"どう見える" がない。致命的だ。」

→ **Spec implication: README の新しい構造 (上から順):**
1. タイトル + タグライン
2. GIF (ヒーロー)
3. 価値提案 (2-3 行)
4. Why syslenz? (3 ペルソナ別ベネフィット)
5. Install
6. Screenshots (メインビュー / Diff / Graph) + 各機能説明
7. Usage (CLI オプション)
8. Keybindings
9. Supported /proc Sources (折りたたみ `<details>`)
10. Roadmap
11. License

🦈 大和田: 「Supported Sources は 43 個もあるから `<details>` タグで折りたため。開いた瞬間に大量のテキストが見えると離脱する。」

→ **Spec implication: Supported Sources セクションを `<details><summary>` で折りたたみにする。**

---

## Gap Summary (Session 008)

| # | Gap | Category | Severity | 発見元 |
|---|-----|----------|----------|--------|
| G12-1 | タグライン後半 "schema-driven system information viewer" が実装者言語 | Message gap | Medium | Scene 1: 利根川 |
| G12-2 | リード文が 1 文に機能を詰め込みすぎ | Message gap | Medium | Scene 1: ヤン |
| G12-3 | Features の表現が実装ベースでベネフィットベースでない | Message gap | Medium | Scene 1: 大和田 |
| G12-4 | リード文が "what it does" であって "what you can do" ではない | Message gap | Medium | Scene 3: 利根川 |
| G13-1 | スクリーンショット戦略 (何を/何枚/どの順) が未定義 | Message gap | High | Scene 2: 利根川 |
| G13-2 | GIF 録画ツールとワークフローが未選定 | Ops gap | High | Scene 2: 大和田 |
| G13-3 | 撮影環境 (ターミナルサイズ/テーマ) が未標準化 | Ops gap | Medium | Scene 2: ヤン |
| G13-4 | README の情報順序が "ユーザーが知りたい順" でない | Message gap | High | Scene 3: 利根川 |
| G13-5 | Supported Sources が折りたたみでなく README が長すぎる | UX gap | Low | Scene 3: 大和田 |

---

## Spec: README 刷新仕様

### 1. タグライン (G12-1 解消)

**Before:**
```
> Wireshark for /proc — a structured, schema-driven system information viewer
```

**After:**
```
> Wireshark for /proc
```

後半を削除。"Wireshark for /proc" の 4 語で十分。

### 2. ヒーロー GIF (G13-1, G13-2 解消)

タグラインの直後に配置。

**内容 (15 秒以内):**
1. `syslenz` を起動 (0-2 秒)
2. サイドバーで `meminfo` を選択 (2-4 秒) — フィールドテーブルが表示される
3. `j/k` でフィールドをスクロール — Description 列が見える (4-6 秒)
4. 別のソース (`net/tcp`) に移動 — Table ビューが表示される (6-9 秒)
5. `d` で diff ビュー — 赤/緑ハイライト (9-12 秒)
6. `g` で graph ビュー — sparkline 表示 (12-15 秒)

**撮影ツール:** `charmbracelet/vhs`

**`.tape` ファイル (リポジトリに含める):**
```
# docs/demo.tape
Output docs/assets/demo.gif
Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 750
Set Theme "Catppuccin Mocha"

Type "syslenz"
Enter
Sleep 2s
# Navigate to meminfo
Down 5
Sleep 500ms
Enter
Sleep 1.5s
# Scroll fields
Down 3
Sleep 300ms
Down 3
Sleep 300ms
Down 3
Sleep 1s
# Switch to net/tcp
Escape
Down 15
Sleep 500ms
Enter
Sleep 1.5s
# Diff view
Type "d"
Sleep 2s
# Graph view
Type "g"
Sleep 2s
Type "q"
```

### 3. 価値提案 (G12-2, G12-4 解消)

GIF の直後:

```markdown
Explore every Linux `/proc` file as structured, typed data — memory, CPU,
network sockets, kernel modules, cgroups, and 37 more sources.
No config. No daemon. Just run it.
```

### 4. Why syslenz? セクション (G12-3 解消)

```markdown
## Why syslenz?

| | |
|---|---|
| **Instant deep-dive** | SSH in, run `syslenz`, see everything. No agents, no config, no setup. |
| **Structured export** | Every field is typed (Bytes, Duration, Table...) with full JSON export. Pipe to `jq`, diff between hosts, attach to incident reports. |
| **Learn Linux internals** | Every field includes a human-readable description. Browse `/proc` like a textbook. |
```

### 5. スクリーンショット 3 枚 (G13-1 解消)

Features セクション内で各機能の横に配置。

| # | 名前 | 表示内容 | 用途 |
|---|------|----------|------|
| 1 | `main-view.png` | サイドバー + meminfo フィールドテーブル (Name/Value/Unit/Description) | "何のツールか" を示す |
| 2 | `diff-view.png` | 同じソースの diff 表示。変化フィールドが赤/緑ハイライト | "変化を追える" ことを示す |
| 3 | `graph-view.png` | 数値フィールドの sparkline グラフ | "時系列データ" を示す |

**保存先:** `docs/assets/`
**撮影方法:** vhs の `Screenshot` コマンド、または `Set` で環境固定後にスクリーンキャプチャ
**サイズ:** 横幅 800px 以内 (GitHub README のコンテンツ幅に合わせる)

### 6. 撮影環境の標準化 (G13-3 解消)

```
Terminal size: 120 columns x 35 rows
Theme: Catppuccin Mocha (暗色)
Font: 等幅 (JetBrains Mono or similar)
Font size: 14pt
GIF width: 1200px → GitHub が自動縮小して ~800px 表示
Screenshot width: 800px
```

### 7. README 構造 (G13-4, G13-5 解消)

```markdown
# syslenz

> Wireshark for /proc

![demo](docs/assets/demo.gif)

Explore every Linux `/proc` file as structured, typed data — (中略)
No config. No daemon. Just run it.

## Why syslenz?
(3 ペルソナ別テーブル)

## Install
(cargo install + バイナリリリースリンク)

## Features
(各機能 + スクリーンショット)

## Usage
(CLI examples)

## Keybindings
(テーブル)

<details>
<summary>Supported /proc Sources (43)</summary>
(カテゴリ別リスト)
</details>

## Roadmap
(箇条書き)

## License
MIT
```

---

## Next Actions

- [ ] `docs/assets/` ディレクトリを作成
- [ ] `docs/demo.tape` を作成し `vhs` で GIF 生成
- [ ] スクリーンショット 3 枚を撮影 (main-view, diff-view, graph-view)
- [ ] README.md を新構造に書き換え (タグライン短縮、GIF 挿入、Why セクション追加、Sources 折りたたみ)
- [ ] vhs を CI (GitHub Actions) に組み込み、UI 変更時に GIF を自動再生成する仕組みを検討
