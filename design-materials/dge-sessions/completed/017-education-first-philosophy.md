# DGE Session 017: Education-First Philosophy — 教育をファーストクラス機能にする

- **Date**: 2026-03-28
- **Theme**: syslenz を「メトリクスを表示するツール」から「メトリクスを教えるツール」へ転換する。教育を補助機能ではなくファーストクラス機能として設計する哲学と実装戦略を策定する
- **Parent Gaps**: G2 (オンボーディング), Session 009 (教育コンテンツと診断パターン), Session 011 (カテゴリ教育)
- **Characters**: 千石 (品質の番人) + ラインハルト (ビジョナリー) + 今泉 (初心者の代弁者) + ハウス (診断の天才) + ヤン (怠惰な簡潔主義者) + 利根川 (ユーザーの現実)
- **Input**: Session 009 で4段階ヘルプ (OFF/NORMAL/DETAILED/EXTRA) と DiagnosticPattern を設計済み。Session 011 でカテゴリ教育を設計済み。だが根本的な問題が残っている — **教育が「あったらいいな」の補助機能に留まっている**。ユーザーの3つの洞察 (知識が思考を広げる / man コマンドの教訓 / Python help() が凄腕に勝った話) を受けて、教育を syslenz の中核アイデンティティとして再定位する。

---

## 背景: ユーザーの3つの洞察

先輩 (ナレーション): この DGE の出発点となった3つの強力な洞察を記録する。

### 洞察 1: 「知っていることで思考が広がる」

監視ツールを使うとき、我々は **自分が知っているパラメータしか見ない**。600 フィールドあっても、初心者は 10 個しか見ない。残りの 590 は「ノイズ」ではなく「まだ知らない答え」だ。エキスパートが問題を調査するとき、初心者が存在すら知らないパラメータを見る。これは **レベルの問題** — 知れば知るほど、見えるものが増える。教育は nice-to-have ではなく、THE differentiator だ。

### 洞察 2: Unix `man` コマンド — 自己文書化システムのゴールドスタンダード

`--help` はオプションを見せるが「理解」は与えない。`man` ページは深さ、コンテキスト、例、相互参照を提供する。これが体験を変える。syslenz はシステムメトリクスの `man` であるべきだ — 値を見せるだけでなく、それが何を意味し、なぜ重要で、何と関連するかを教える。

### 洞察 3: ポートスキャナエピソード — ドキュメントが競争優位になる

あるユーザーと凄腕プログラマがクライアントサイトに行った。ネットワークドキュメントなし、担当者なし、だがシェルアクセスあり。二人ともポートスキャナを書き始めた — Ruby と Python で。**Python が勝った**。Python の対話シェルには組み込みドキュメント (`help()`, 全クラスの docstring) があった。Ruby にはなかった。ドキュメントにアクセスできる人が、より高いスキルを持つがドキュメントなしの人に勝った。

**教訓: Self-disclosing system (自己開示するシステム) は raw skill に勝つ。使いながら教えてくれるツールは、教えてくれないより優れたツールよりもユーザーを有能にする。**

---

## Scene 1: ノイズ vs シグナル — 600フィールド問題

先輩: syslenz が扱う 600 以上のフィールドのうち、ほとんどがユーザーに無視されている問題について議論する。

👤 今泉: 「すみません、根本的な疑問なんですけど... meminfo だけで 50 フィールド、vmstat で 100 以上、全部合わせたら 600 フィールド以上ありますよね。でも僕がいつも見るのは MemAvailable と load_1min と CPU 使用率くらいです。10 個くらい。残りの 590 フィールドって、ノイズじゃないですか？表示する意味あります？」

🏥 ハウス (杖をつきながら): 「典型的な無知の発言だな。nr_dirty_threshold がノイズに見えるのは、お前がそれが何か知らないからだ。ある日、アプリケーションの書き込みレイテンシが突然スパイクする。iostat を見ても IOPS は正常。ディスクは壊れてない。だが dirty pages が書き込みストームを起こしている。nr_dirty_threshold と nr_dirty の比率を見た瞬間、原因がわかる。**ノイズに見えるデータの中に答えがある。問題は、どのデータが重要かを知らないことだ。**」

👤 今泉: 「でも... 600 フィールドを全部覚えるなんて不可能です。」

🏥 ハウス: 「覚える必要はない。**問題が起きた時に、何を見るべきかを教えてくれるシステム** があればいい。それが syslenz の仕事だ。」

🎰 利根川 (腕組みして): 「問題は表示量じゃない。**『いつ、何が重要になるか』を教えないことだ。** 600 フィールドを常時全部見せるのは確かにノイズだ。だが『今、書き込みレイテンシが高い。原因として dirty pages を確認しろ。nr_dirty_threshold を見ろ』と教えてくれるなら、590 フィールドは "まだ必要じゃない答え" に変わる。ノイズからシグナルへの変換。それが教育だ。」

☕ ヤン: 「整理しよう。」

```
600 フィールドの分類:

[常時重要] ~20 フィールド
  MemAvailable, load_1min, cpu_usage, rx_bytes, tx_bytes, ...
  → 常に表示。初心者にも見える

[状況依存] ~100 フィールド
  nr_dirty, nr_writeback, pgfault, SYN_SENT, ...
  → 普段はノイズ。特定の問題が起きた時だけシグナルになる
  → 「問題が起きた時に見るべきフィールド」として教育で橋渡し

[専門家向け] ~200 フィールド
  nr_zone_inactive_anon, thp_fault_alloc, ...
  → カーネル開発者レベル。一般ユーザーには不要
  → 存在は教えるが、深入りは自己責任

[生データ] ~280 フィールド
  カウンタ値、累積値など
  → 単体では意味がない。差分や比率で意味が出る
  → 「このフィールドは X と組み合わせて見る」と教育
```

☕ ヤン: 「つまり、『全部見せる / 全部隠す』の二択じゃない。**今の状況に関連する知識を優先表示する** デザインパターンだ。」

→ **デザインパターン: Context-Dependent Education (コンテキスト依存教育)**
```
条件: メモリ圧力が高い (MemAvailable < 20% of MemTotal)
表示: 「メモリ関連の重要フィールド」を優先表示
  - nr_dirty / nr_dirty_threshold (dirty page 比率)
  - pgfault / pgmajfault (ページフォルト頻度)
  - SwapFree / SwapTotal (スワップ使用状況)
  - Slab / SReclaimable (カーネルキャッシュ)
教育: 「MemAvailable が低い時、以下を確認してください...」
```

→ **Gap G-EDU-1 発見: フィールドの「重要度コンテキスト」メタデータが未定義。各フィールドが「いつ重要になるか」を記述する仕組みが必要。**

---

## Scene 2: man コマンドの教訓 — 深さのある自己開示

先輩: Unix man ページの構造から syslenz のヘルプ設計を学ぶ。

🦁 ラインハルト: 「man は Unix の最大の発明の一つだ。1971 年から 50 年以上、全てのコマンドが自分自身を説明する。`ls --help` は "what"（何ができるか）を見せる。`man ls` は "why" と "how"（なぜそうするか、どう使うか）を教える。syslenz のメトリクスも同じであるべきだ。MemAvailable の値を見せるだけじゃない。**MemAvailable が何を意味し、なぜ MemFree と違い、どう使うべきかを教える。** syslenz で Linux を学んだエンジニアは、man ページを開かなくても /proc がわかるようになる。それがビジョンだ。」

🎋 千石: 「man の品質基準を分析しよう。50 年間生き残った構造には理由がある。」

```
man ページの構造:
  NAME        — コマンド名と1行説明
  SYNOPSIS    — 使い方の書式
  DESCRIPTION — 詳細な説明
  OPTIONS     — オプション一覧
  EXAMPLES    — 具体的な使用例
  FILES       — 関連ファイル
  SEE ALSO    — 関連コマンドへのクロスリファレンス
  BUGS        — 既知の問題
  AUTHORS     — 著者

syslenz メトリクスヘルプへのマッピング:
  NAME        → フィールド名と1行説明 (MemAvailable — 利用可能メモリ量)
  DESCRIPTION → 詳細な説明 (カーネルがどう計算するか、MemFree との違い)
  VALUES      → 値の読み方 (正常範囲、警告閾値、危険閾値)
  EXAMPLES    → 実例 (「8GB マシンで MemAvailable が 200MB → 危険」)
  SEE ALSO    → 関連メトリクス (MemFree, Cached, SwapFree, vmstat/pgfault)
  DIAGNOSTIC  → この値が異常な時の診断手順
```

🎋 千石: 「**教育コンテンツの品質は妥協できない。間違った知識は無知より有害だ。** 『MemAvailable はアプリケーションが使えるメモリ量です』という説明は正確に見えて不正確だ。正確には『カーネルが新しいプロセスに割り当てられると推定するメモリ量で、MemFree + reclaimable cache + reclaimable slab の近似値』だ。初心者向けの簡略版と上級者向けの正確版、両方必要だ。どちらか一方だけでは教育として不十分。」

☕ ヤン: 「Session 009 で設計した4段階ヘルプ (OFF/NORMAL/DETAILED/EXTRA) はまさにこの構造を実現するためのものだった。だが Session 009 では i18n とヘルプ表示の仕組みに集中して、**コンテンツの構造**は詰められていなかった。今ここで決める。」

→ **Spec 提案: syslenz メトリクスヘルプの man 風構造**

```rust
/// man ページ風のメトリクスヘルプ構造
struct MetricHelp {
    // NAME — 1行説明 (HelpMode::Normal で表示)
    summary: LocalizedText,

    // DESCRIPTION — 詳細説明 (HelpMode::Detailed で表示)
    description: LocalizedText,

    // VALUES — 値の読み方 (HelpMode::Detailed で表示)
    value_guide: ValueGuide,

    // EXAMPLES — 具体例 (HelpMode::Extra で表示)
    examples: Vec<LocalizedText>,

    // SEE ALSO — 関連メトリクスへのリンク (全モードで表示)
    see_also: Vec<MetricRef>,

    // DIAGNOSTIC — 異常時の診断手順 (異常値検出時に表示)
    diagnostic: Option<DiagnosticGuide>,
}

struct ValueGuide {
    unit: &'static str,          // "bytes", "count", "percentage"
    normal_range: &'static str,  // "通常: MemTotal の 20% 以上"
    warning: &'static str,       // "注意: 10-20%"
    critical: &'static str,      // "危険: 10% 未満"
}

struct MetricRef {
    source: &'static str,        // "meminfo"
    field: &'static str,         // "Cached"
    relation: &'static str,      // "MemAvailable の計算に含まれる"
}
```

🦁 ラインハルト: 「SEE ALSO が最も重要だ。man ページで最も価値ある部分は SEE ALSO だ。`man fork` を読んで `SEE ALSO: exec(3), wait(2), clone(2)` を見た瞬間、プロセスの世界が開ける。syslenz でも MemAvailable を見て `SEE ALSO: Cached, SwapFree, pgfault` が見えた瞬間、メモリ管理の全体像が開ける。」

→ **Gap G-EDU-2 発見: MetricHelp 構造体と SEE ALSO メタデータの実装が必要。Session 009 の EducationContent を拡張する形で実現する。**

---

## Scene 3: Python help() の教訓 — 自己開示が速度を生む

先輩: ポートスキャナエピソードから、自己開示システムの設計を学ぶ。

🎰 利根川: 「聞いてくれ。あるユーザーと凄腕プログラマがクライアントサイトに行った。ネットワークドキュメントなし。担当者なし。だがサーバーへのシェルアクセスはあった。二人ともポートスキャナを書き始めた。一人は Ruby、一人は Python。**Python が勝った。**」

🎰 利根川: 「なぜか？ Python の対話シェルで `help(socket)` と打てば、socket モジュールの全メソッド、全引数、全挙動がドキュメントとして読めた。`socket.AF_INET` が何か分からなくても、その場で分かる。Ruby にはそれがなかった。凄腕プログラマは Ruby のリファレンスを Google で検索しようとしたが、クライアントのネットワークは外部接続が制限されていた。**スキルよりも知識へのアクセスが勝った。** ドキュメントがある方が勝つんだ。」

☕ ヤン: 「つまり要点はこうだ。syslenz で MemAvailable を見た時に、**その場で** 以下が分かれば、エキスパートと同じ速度で調査できる:」

```
1. 「これは何か」      → 利用可能メモリの推定量
2. 「何と比べるべきか」 → MemTotal に対する割合で見る
3. 「次に何を見るべきか」→ 低い場合: Cached, Slab, SwapFree を確認
4. 「何が起きているか」 → 10% 未満: OOM Killer 発動リスク
```

☕ ヤン: 「Python の help() の本質は **探索可能性 (discoverability)** だ。知らないものを見つけられる。syslenz でフィールドを選択した時に SEE ALSO が表示されれば、ユーザーは関連フィールドを辿って知識を広げられる。知らなかったフィールドを発見し、その意味を学び、次の調査に活かせる。**探索が学習になる。**」

🏥 ハウス: 「Python の help() には限界もある。構造化されていない。長い。全部読まないと欲しい情報にたどり着けない。syslenz では Python の自己開示性を取り入れつつ、man ページの構造を組み合わせるべきだ。**構造化された自己開示。** それが我々の設計原則だ。」

→ **実装提案: フィールド選択時の SEE ALSO 表示**

```
[MemAvailable を選択した状態]

┌─ Help ──────────────────────────────────────────┐
│ MemAvailable — 利用可能メモリの推定量            │
│                                                   │
│ 現在値: 1.2 GB / 8.0 GB (15%) ⚠ 注意            │
│                                                   │
│ SEE ALSO:                                         │
│   MemFree     — MemAvailable の構成要素の一つ     │
│   Cached      — ページキャッシュ (回収可能)       │
│   SwapFree    — スワップ残量                      │
│   pgfault     — ページフォルト頻度 (vmstat)       │
│   SReclaimable — 回収可能 Slab キャッシュ         │
│                                                   │
│ [Tab] 詳細  [→] SEE ALSO に移動                  │
└───────────────────────────────────────────────────┘
```

→ **Gap G-EDU-3 発見: SEE ALSO ナビゲーション — ヘルプパネル内で関連メトリクスを選択して直接ジャンプする UI 操作の実装。**

---

## Scene 4: コンテキスト依存教育の設計

先輩: 問題が起きた時にだけ表示される教育コンテンツの設計を議論する。

🏥 ハウス: 「600 フィールドを常に教える必要はない。**患者が来た時に、その症状に関連する知識だけ見せればいい。** 診断ビューで『メモリ不足』が検出されたとき、通常のヘルプではなく**『メモリ不足の時に見るべき10のメトリクス』** を表示する。」

🏥 ハウス: 「Session 009 で DiagnosticPattern を設計した。だがあの設計には欠陥がある。症状を検出して警告するだけで、**次のアクション** を教えない。『メモリが少ないです』は誰でも言える。エキスパートが言うのは『メモリが少ない。まず Slab を見ろ。SReclaimable が大きければ `slabtop` で何が食っているか確認しろ。小さければプロセスの RSS を見ろ。Cached が大きいなら echo 3 > /proc/sys/vm/drop_caches で解放できるが本番では慎重にやれ』だ。」

👤 今泉: 「つまり、問題が起きてない時はノイズに見えるフィールドが、問題が起きた瞬間に『これを見ろ』と教えてくれる？それはすごく助かります。」

🏥 ハウス: 「正確にはこうだ。」

```
通常時:
  nr_dirty           → [ヘルプ] "書き込み待ちの dirty ページ数"  (ただの説明)
  nr_dirty_threshold → [ヘルプ] "dirty ページの閾値"            (ただの説明)

異常検出時 (書き込みレイテンシ高):
  nr_dirty           → [!] 書き込みストーム検出
  nr_dirty_threshold → [!] dirty 比率: 87% — 閾値を大幅超過

  ┌─ Diagnostic Guide ───────────────────────────────┐
  │ 書き込みストーム (Write Storm) が検出されました     │
  │                                                     │
  │ 原因の候補:                                         │
  │   1. バッファリングされた書き込みがフラッシュ中      │
  │   2. vm.dirty_ratio の設定が高すぎる                │
  │   3. ストレージデバイスが遅い                       │
  │                                                     │
  │ 確認すべきメトリクス:                                │
  │   → nr_dirty / nr_dirty_threshold (現在: 87%)       │
  │   → nr_writeback (現在のフラッシュ量)               │
  │   → diskstats: await (I/O レイテンシ)               │
  │                                                     │
  │ 対処コマンド:                                        │
  │   $ sysctl vm.dirty_ratio                           │
  │   $ sysctl vm.dirty_background_ratio                │
  │   $ iostat -x 1                                     │
  └─────────────────────────────────────────────────────┘
```

🎰 利根川: 「これだ。これが Python の help() を超えるものだ。Python の help() は聞かれたら答える。syslenz は **聞かれる前に、必要な時に教える。** プロアクティブな教育だ。」

→ **実装提案: DiagnosticFinding の拡張**

```rust
/// Session 009 の DiagnosticPattern を拡張
struct DiagnosticFinding {
    // 既存 (Session 009)
    severity: Severity,
    symptom: LocalizedText,
    causes: Vec<LocalizedText>,
    checks: Vec<&'static str>,

    // 新規: 関連メトリクスへのジャンプ
    related_metrics: Vec<MetricRef>,

    // 新規: コンテキスト依存の教育コンテンツ
    contextual_education: Vec<LocalizedText>,

    // 新規: 「次に何を見るべきか」のガイド
    next_steps: Vec<NextStep>,
}

struct NextStep {
    description: LocalizedText,        // "Slab の使用状況を確認"
    target: Option<MetricRef>,         // Some(("meminfo", "Slab"))
    command: Option<&'static str>,     // Some("slabtop")
}
```

→ **Gap G-EDU-4 発見: DiagnosticFinding に related_metrics と next_steps を追加する実装。Session 009 の DIAGNOSTIC_PATTERNS の拡張。**

---

## Scene 5: プログレッシブ・ディスクロージャー — レベルに合わせた開示

先輩: ユーザーの知識レベルに応じた段階的な情報開示を設計する。

☕ ヤン: 「全部教えようとするな。ユーザーの **『今の問題』に関連する知識だけ見せろ。** そして、情報の深さはレベルに合わせる。初心者には MemAvailable の1行説明。中級者にはキャッシュとの関係。上級者にはカーネルのページ回収アルゴリズム。全部一度に見せるのは教育じゃない。情報の洪水だ。」

🎋 千石: 「Session 009 で設計した4段階ヘルプ (OFF/NORMAL/DETAILED/EXTRA) はまさにこのプログレッシブ・ディスクロージャーだ。だが **まだ足りない**。段階的に深く見せることはできるが、**横に広げる** 仕組みがない。」

🎋 千石: 「足りないものを3つ挙げる。」

```
足りないもの 1: SEE ALSO (関連メトリクス相互リンク)
  現状: MemAvailable を見ている → MemAvailable の説明しか見えない
  あるべき姿: MemAvailable → Cached → SReclaimable → Slab → ... と辿れる
  man ページでは当たり前。syslenz にはまだない。

足りないもの 2: "Why this matters NOW" (現在値に基づく動的コンテキスト)
  現状: 「MemAvailable は利用可能メモリ量です」(静的)
  あるべき姿: 「MemAvailable は 1.2GB (15%)。20% を切っているため要注意。
              Cached が 3.2GB あるため、実質的にはもう少し余裕がある可能性。」(動的)
  ヘルプが現在値を参照して、文脈に応じたアドバイスを生成する。

足りないもの 3: Learning Breadcrumbs (学習パンくず)
  現状: ヘルプを読んでも「次に何を学ぶべきか」が分からない
  あるべき姿: 「MemAvailable を理解しましたか？
              次は vmstat の pgfault を見てみましょう。
              ページフォルトを理解すると、メモリ不足の初期兆候を読めるようになります。」
  ユーザーの学習パスを設計する。
```

🦁 ラインハルト: 「Learning Breadcrumbs は美しい。syslenz を使い続けるだけで、ユーザーの Linux 知識が自然と広がる。ある日気づいたら、/proc の主要パラメータを全部理解している。**syslenz で Linux を学んだエンジニアは、man ページを開かなくても /proc がわかるようになる。**」

☕ ヤン: 「理想は分かった。実装コストを考えよう。」

```
実装の3層:

Layer 1: SEE ALSO (低コスト、高効果)
  - MetricHelp に see_also: Vec<MetricRef> を追加するだけ
  - データは静的。一度定義すれば変わらない
  - 主要 50 フィールドの関連メトリクスを定義 → 約 200 個の MetricRef
  - 工数: 2-3 日

Layer 2: "Why this matters NOW" (中コスト、高効果)
  - 現在値を参照して動的にメッセージを生成
  - 条件分岐が必要 (値の範囲 × メッセージ)
  - 主要 20 フィールドに動的コンテキストを実装
  - 工数: 5-7 日

Layer 3: Learning Breadcrumbs (高コスト、長期効果)
  - 学習パスの設計が必要 (メモリ系 → CPU 系 → I/O 系 → ネットワーク系)
  - ユーザーの「何を見たか」の履歴追跡が必要
  - 工数: 10-14 日
  - v2.0 以降でよい
```

→ **Gap G-EDU-5 発見: プログレッシブ・ディスクロージャーの3層実装ロードマップ。Layer 1 (SEE ALSO) は v1.3 で、Layer 2 (動的コンテキスト) は v1.4 で、Layer 3 (Learning Breadcrumbs) は v2.0 で。**

---

## Scene 6: 実装ロードマップ — Education as First Class

先輩: ここまでの議論をまとめ、具体的な実装ロードマップを策定する。

🦁 ラインハルト: 「ここまでの議論で明確になった。syslenz の教育機能は補助機能ではない。**ファーストクラス機能**だ。他の監視ツールとの最大の差別化ポイントだ。htop は数値を見せる。Grafana はグラフを見せる。syslenz は **理解を見せる**。」

🎰 利根川: 「Python の help() の話を思い出せ。凄腕プログラマが Python に負けた。同じことがサーバー管理でも起きる。htop を使い慣れた凄腕エンジニアが、syslenz の教育機能を持つ初心者に負ける。知識へのアクセスが速いほうが勝つ。**それが我々の勝ち筋だ。**」

🎋 千石: 「品質基準を決めよう。教育コンテンツは適当に書くな。」

```
教育コンテンツの品質基準:

1. 正確性: カーネルソースまたは公式ドキュメントに基づくこと
   - /proc/meminfo の説明は Documentation/filesystems/proc.txt を参照
   - 曖昧な表現 ("たぶん" "おそらく") は禁止

2. 段階性: NORMAL/DETAILED/EXTRA の各レベルで異なる深さ
   - NORMAL: 1行。「何か」がわかる
   - DETAILED: 3-5行。「なぜ重要か」「何と関連するか」がわかる
   - EXTRA: 10行以上。カーネルの仕組みまで踏み込む

3. 実用性: 「だから何？」に答えること
   - NG: "MemAvailable はカーネルが推定する利用可能メモリ量です"
   - OK: "MemAvailable はアプリに割り当て可能なメモリの推定量。
          MemTotal の 20% を切ったら要注意。10% 未満は OOM 危険域"

4. 関連性: SEE ALSO で必ず関連メトリクスを示すこと
   - 孤立した知識は使えない。ネットワーク化された知識が力になる

5. 二言語: 英語と日本語の両方を提供すること
   - 日本語が先 (我々のユーザーは日本語話者が多い)
   - 英語は国際展開に必須
```

☕ ヤン: 「具体的な実装をまとめよう。6つの機能を優先順位付きで。」

```
教育ファーストクラス機能 — 実装ロードマップ:

━━━ v1.3 (次のリリース) ━━━

1. SEE ALSO — 関連メトリクスリンク [優先度: 最高]
   MetricHelp に see_also フィールドを追加。
   主要 50 フィールドの関連メトリクスを定義。
   ヘルプパネルに SEE ALSO セクションを表示。
   キー操作で関連メトリクスにジャンプ。
   工数: 3 日

2. man 風ヘルプ構造 [優先度: 最高]
   MetricHelp 構造体を実装 (summary, description, value_guide, examples, see_also)。
   Session 009 の EducationContent を MetricHelp に進化。
   主要 20 フィールドの man 風ヘルプを執筆。
   工数: 5 日

3. Diagnostic Deep-Dive — 診断結果からの関連メトリクスジャンプ [優先度: 高]
   DiagnosticFinding に related_metrics と next_steps を追加。
   診断結果パネルから関連メトリクスに直接ジャンプ。
   Session 009 の 5 パターンを拡張。
   工数: 3 日

━━━ v1.4 ━━━

4. Contextual Hints — 値が異常な時だけ表示される教育コンテンツ [優先度: 中]
   現在値を参照して動的にアドバイスを生成。
   「Why this matters NOW」メッセージ。
   主要 20 フィールドに対応。
   工数: 7 日

5. "Did you know?" Tips — ランダムに表示される豆知識 [優先度: 低]
   起動時またはアイドル時に豆知識を表示。
   「知ってましたか？ vmstat の pgsteal は...」
   ユーザーの知識範囲を徐々に広げる。
   工数: 2 日

━━━ v2.0 ━━━

6. Learning Breadcrumbs — 「次に見るべき」提案 [優先度: 低]
   学習パスの設計 (メモリ → CPU → I/O → ネットワーク)。
   閲覧履歴に基づく「次はこれを見よう」提案。
   工数: 14 日

━━━ v3.0 (将来) ━━━

7. Interactive Tutorial Mode — --tutorial でガイド付きツアー [優先度: 将来]
   syslenz --tutorial で対話型チュートリアルを起動。
   「メモリを理解する」「CPU 負荷を読む」「ネットワーク問題を診断する」
   ステップバイステップのガイド付き操作。
   工数: 21 日
```

🏥 ハウス: 「1 と 3 が最も ROI が高い。SEE ALSO があればユーザーは自分で探索できる。Diagnostic Deep-Dive があれば問題発生時に迷わない。この2つだけで、syslenz は "教えるツール" になる。」

🎰 利根川: 「同意だ。Python の help() に勝つのは 1 (SEE ALSO) + 2 (man 風構造) だ。凄腕に勝つのは 3 (Diagnostic Deep-Dive) + 4 (Contextual Hints) だ。」

---

## Gap Summary

### 発見された Gap 一覧

| Gap ID | 内容 | 優先度 | 対応バージョン |
|--------|------|--------|---------------|
| G-EDU-1 | フィールドの「重要度コンテキスト」メタデータ — 各フィールドが「いつ重要になるか」の定義 | 高 | v1.3 |
| G-EDU-2 | MetricHelp 構造体と SEE ALSO メタデータの実装 | 最高 | v1.3 |
| G-EDU-3 | SEE ALSO ナビゲーション — ヘルプパネル内から関連メトリクスへのジャンプ UI | 最高 | v1.3 |
| G-EDU-4 | DiagnosticFinding の拡張 — related_metrics, next_steps の追加 | 高 | v1.3 |
| G-EDU-5 | プログレッシブ・ディスクロージャー3層ロードマップの実装 | 中 | v1.3-v2.0 |

### 既存 Gap との関係

| 既存 Gap | 本 Session での進展 |
|----------|---------------------|
| G2 (オンボーディング) | 教育ファーストクラスにより、オンボーディング自体が不要になる方向。ツールを使うこと自体が学習 |
| Session 009 の EducationContent | MetricHelp に拡張。SEE ALSO, ValueGuide, DiagnosticGuide を追加 |
| Session 009 の DiagnosticPattern | DiagnosticFinding に拡張。related_metrics, next_steps を追加 |
| Session 011 のカテゴリ教育 | カテゴリレベルの教育 + フィールドレベルの教育 = 完全な教育体系 |

---

## 具体的な実装仕様

### S1: MetricHelp 構造体 (man ページ風ヘルプ)

```rust
use crate::i18n::{Locale, LocalizedText};

/// man ページ風の構造化ヘルプ
pub struct MetricHelp {
    /// NAME — 1行説明
    pub summary: LocalizedText,
    /// DESCRIPTION — 詳細説明
    pub description: LocalizedText,
    /// VALUES — 値の読み方ガイド
    pub value_guide: Option<ValueGuide>,
    /// EXAMPLES — 具体的な使用例
    pub examples: Vec<LocalizedText>,
    /// SEE ALSO — 関連メトリクスへのクロスリファレンス
    pub see_also: Vec<MetricRef>,
    /// DIAGNOSTIC — 異常時の診断ガイド
    pub diagnostic: Option<DiagnosticGuide>,
}

pub struct LocalizedText {
    pub en: &'static str,
    pub ja: &'static str,
}

pub struct ValueGuide {
    pub unit: &'static str,
    pub normal_range_en: &'static str,
    pub normal_range_ja: &'static str,
    pub warning_en: &'static str,
    pub warning_ja: &'static str,
    pub critical_en: &'static str,
    pub critical_ja: &'static str,
}

pub struct MetricRef {
    pub source: &'static str,
    pub field: &'static str,
    pub relation_en: &'static str,
    pub relation_ja: &'static str,
}

pub struct DiagnosticGuide {
    pub condition: &'static str,           // "MemAvailable < 10% of MemTotal"
    pub causes: Vec<LocalizedText>,
    pub next_steps: Vec<NextStep>,
}

pub struct NextStep {
    pub description: LocalizedText,
    pub target: Option<MetricRef>,
    pub command: Option<&'static str>,
}
```

### S2: SEE ALSO データの例 (主要メトリクス)

```rust
// meminfo/MemAvailable の SEE ALSO
static MEM_AVAILABLE_SEE_ALSO: &[MetricRef] = &[
    MetricRef {
        source: "meminfo", field: "MemFree",
        relation_en: "Component of MemAvailable calculation",
        relation_ja: "MemAvailable 計算の構成要素",
    },
    MetricRef {
        source: "meminfo", field: "Cached",
        relation_en: "Page cache, reclaimable under memory pressure",
        relation_ja: "ページキャッシュ。メモリ圧力下で回収可能",
    },
    MetricRef {
        source: "meminfo", field: "SwapFree",
        relation_en: "When MemAvailable is low, check if swap is being used",
        relation_ja: "MemAvailable が低い時、スワップ使用状況を確認",
    },
    MetricRef {
        source: "meminfo", field: "SReclaimable",
        relation_en: "Reclaimable slab cache, included in MemAvailable estimate",
        relation_ja: "回収可能 Slab キャッシュ。MemAvailable の推定に含まれる",
    },
    MetricRef {
        source: "vmstat", field: "pgfault",
        relation_en: "Page fault rate — rises when memory is tight",
        relation_ja: "ページフォルト頻度 — メモリ逼迫時に上昇",
    },
];

// loadavg/load_1min の SEE ALSO
static LOAD_1MIN_SEE_ALSO: &[MetricRef] = &[
    MetricRef {
        source: "loadavg", field: "load_5min",
        relation_en: "Compare 1min vs 5min to see if load is rising or falling",
        relation_ja: "1分と5分を比較して負荷が上昇中か下降中か判断",
    },
    MetricRef {
        source: "stat", field: "cpu_iowait",
        relation_en: "If load is high but CPU usage is low, check iowait",
        relation_ja: "load が高いのに CPU 使用率が低い場合、iowait を確認",
    },
    MetricRef {
        source: "stat", field: "cpu_user",
        relation_en: "CPU-bound load indicator",
        relation_ja: "CPU バウンド負荷の指標",
    },
    MetricRef {
        source: "processes", field: "state",
        relation_en: "Check for D-state (uninterruptible sleep) processes inflating load",
        relation_ja: "D state (割り込み不可スリープ) のプロセスが load を膨張させていないか確認",
    },
    MetricRef {
        source: "pressure", field: "cpu_some_avg10",
        relation_en: "PSI CPU pressure — more accurate than load average",
        relation_ja: "PSI CPU 圧力 — load average より正確",
    },
];
```

### S3: コンテキスト依存教育のインターフェース

```rust
/// 現在値に基づく動的教育コンテンツを生成
pub fn contextual_hint(
    locale: Locale,
    source: &str,
    field: &str,
    current_value: f64,
    reference_values: &HashMap<String, f64>,  // 関連フィールドの現在値
) -> Option<String> {
    // 例: MemAvailable の場合
    // current_value = 1_200_000_000 (1.2 GB)
    // reference_values = { "MemTotal": 8_000_000_000 }
    // → ratio = 15%
    // → "MemAvailable は MemTotal の 15%。20% 以下のため要注意。
    //    Cached (3.2GB) が回収可能なため、実質的にはもう少し余裕がある可能性。"
    None // v1.4 で実装
}
```

### S4: ヘルプパネルの UI 設計

```
━━━ HelpMode::Normal (デフォルト) ━━━

┌─ ? Help ─────────────────────────────┐
│ MemAvailable                          │
│ 利用可能メモリの推定量                │
│                                       │
│ SEE ALSO: MemFree, Cached, SwapFree   │
│ [Tab] 詳細  [?] OFF                  │
└───────────────────────────────────────┘

━━━ HelpMode::Detailed (Tab で切替) ━━━

┌─ ? Help ─────────────────────────────────────────┐
│ MemAvailable — 利用可能メモリの推定量              │
│                                                     │
│ カーネルが新しいプロセスに割り当てられると推定する   │
│ メモリ量。MemFree + 回収可能キャッシュの近似値。    │
│ MemFree より実用的な指標。                          │
│                                                     │
│ 値の読み方:                                         │
│   正常: MemTotal の 20% 以上                        │
│   注意: 10-20% ⚠                                   │
│   危険: 10% 未満 🔴                                 │
│                                                     │
│ SEE ALSO:                                           │
│   → MemFree      MemAvailable 計算の構成要素        │
│   → Cached       ページキャッシュ (回収可能)         │
│   → SwapFree     スワップ残量                       │
│   → pgfault      ページフォルト頻度 (vmstat)        │
│   → SReclaimable 回収可能 Slab キャッシュ            │
│                                                     │
│ [Tab] さらに詳細  [→] SEE ALSO 選択  [?] OFF      │
└───────────────────────────────────────────────────┘

━━━ HelpMode::Extra (Tab で切替) ━━━

┌─ ? Help ─────────────────────────────────────────────┐
│ MemAvailable — 利用可能メモリの推定量                  │
│                                                         │
│ [DESCRIPTION]                                           │
│ Linux 3.14 以降で /proc/meminfo に追加。カーネルが       │
│ MemAvailable を計算するアルゴリズム:                     │
│   MemAvailable ≈ MemFree                                │
│                   - min(MemFree/2, low watermark)       │
│                   + inactive file pages                  │
│                   + SReclaimable                         │
│                   - min(above / 2, all wmarks)           │
│                                                         │
│ [EXAMPLES]                                              │
│ 8GB マシンの例:                                          │
│   MemTotal: 8.0GB, MemAvailable: 1.2GB (15%) → ⚠ 注意  │
│   MemTotal: 8.0GB, MemAvailable: 0.5GB (6%)  → 🔴 危険  │
│                                                         │
│ [DIAGNOSTIC]                                            │
│ MemAvailable < 10% の場合:                               │
│   1. ps aux --sort=-rss | head でメモリ消費プロセス確認  │
│   2. Slab/SReclaimable を確認 (slabtop)                  │
│   3. /proc/<pid>/smaps で詳細メモリマップ確認            │
│                                                         │
│ SEE ALSO: MemFree, Cached, SwapFree, pgfault             │
│ [Tab] 通常表示  [→] SEE ALSO 選択  [?] OFF             │
└─────────────────────────────────────────────────────────┘
```

### S5: 「教育はファーストクラス機能」設計原則

```
Design Principle: Education as First-Class Feature

1. Self-Disclosing System (自己開示するシステム)
   syslenz は自分自身を説明する。全てのメトリクス、全てのソース、
   全ての診断パターンが「私は何者で、なぜ重要で、何と関連するか」を
   ユーザーに開示する。

2. Knowledge Expands Vision (知識が視野を広げる)
   ユーザーは知っているパラメータしか見ない。syslenz の教育機能は
   「知らなかったパラメータの存在」を教えることで、ユーザーの視野を広げる。
   600 フィールドの全てが潜在的な答えである。

3. Documentation Beats Skill (ドキュメントがスキルに勝つ)
   Python help() の教訓。知識へのアクセスが速い者が、スキルの高い者に勝つ。
   syslenz は使いながら学べるツールであることで、ユーザーを最も有能にする。

4. Context-Dependent Education (コンテキスト依存教育)
   600 フィールドを常に教えるのではなく、「今の状況に関連する知識」を
   優先的に提示する。問題が起きた時に「これを見ろ」と教える。
   ノイズをシグナルに変換するのが教育の役割。

5. Progressive Disclosure (段階的開示)
   初心者には1行説明。中級者には関連性と文脈。上級者にはカーネルの仕組み。
   全部一度に見せるのは教育ではなく情報の洪水。

6. Structured Self-Disclosure (構造化された自己開示)
   Python の help() の自己開示性と、man ページの構造を組み合わせる。
   SUMMARY → DESCRIPTION → VALUES → EXAMPLES → SEE ALSO → DIAGNOSTIC。
   50年生き残った構造には理由がある。

7. Exploration is Learning (探索が学習になる)
   SEE ALSO で関連メトリクスを辿る行為自体が学習。
   ユーザーが syslenz を使い続けるだけで、Linux の知識が自然に広がる。
   syslenz で Linux を学んだエンジニアは man ページを開かなくても
   /proc がわかるようになる。
```

### S6: Python help() に学ぶインタラクティブ探索の設計

```
Python help() の分析:

優れている点:
  - 即座にアクセスできる (help(socket) で全情報)
  - 全メソッド、全引数、全挙動が読める
  - インターネット不要 (オフライン完結)
  - 作業の流れを中断しない

不十分な点:
  - 構造化されていない (長い文字列の羅列)
  - 優先順位がない (重要なメソッドもマイナーなメソッドも同じ扱い)
  - コンテキスト依存でない (現在の状況に関係なく同じ情報)
  - SEE ALSO が弱い (モジュール間の関連が見えにくい)

syslenz が Python help() を超える設計:
  1. 即座のアクセス           — help() と同等。フィールド選択でヘルプ表示
  2. 構造化                   — man 風の NAME/DESCRIPTION/VALUES/SEE ALSO
  3. 優先順位付き             — 重要度コンテキストで表示順制御
  4. コンテキスト依存         — 現在値に基づく動的アドバイス
  5. 相互参照ナビゲーション   — SEE ALSO でフィールド間をジャンプ
  6. 診断統合                 — 異常値検出時に自動で関連情報提示

syslenz のインタラクティブ探索フロー:
  MemAvailable (15%) を見ている
    → [?] ヘルプ表示: "利用可能メモリの推定量。20% 以下で要注意"
    → [Tab] 詳細表示: 計算方法、値の読み方
    → SEE ALSO に Cached が見える
    → [→] Cached にジャンプ: "ページキャッシュ。回収可能なメモリ"
    → SEE ALSO に SReclaimable が見える
    → [→] SReclaimable にジャンプ: "回収可能 Slab キャッシュ"
    → 「なるほど、MemAvailable は MemFree + Cached + SReclaimable の近似か」
    → 学習完了。次回からメモリ問題の調査で 3 フィールドを確認するようになる
```

---

## Next Actions

- [ ] G-EDU-2: MetricHelp 構造体を src/i18n.rs (または src/i18n/education.rs) に実装
- [ ] G-EDU-2: 主要 20 フィールドの MetricHelp データを記述 (MemAvailable, MemFree, Cached, SwapFree, load_1min, load_5min, cpu_user, cpu_system, cpu_iowait, rx_bytes, tx_bytes, pgfault, pgmajfault, nr_dirty, pressure cpu/memory/io, tcp established, processes running)
- [ ] G-EDU-3: SEE ALSO ナビゲーションの UI 実装 (ヘルプパネル内で矢印キーで選択、Enter でジャンプ)
- [ ] G-EDU-4: DiagnosticFinding に related_metrics と next_steps を追加し、Session 009 の 5 パターンを拡張
- [ ] G-EDU-1: フィールド重要度コンテキストの定義 (常時重要 / 状況依存 / 専門家向け / 生データ)
- [ ] G-EDU-5 Layer 2 (v1.4): contextual_hint() の実装 — 現在値に基づく動的教育コンテンツ
- [ ] G-EDU-5 Layer 3 (v2.0): Learning Breadcrumbs — 学習パスと閲覧履歴追跡
- [ ] 設計原則ドキュメント: 「Education as First-Class Feature」の 7 原則を design-materials に追記
