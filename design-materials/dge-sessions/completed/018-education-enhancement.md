# DGE Session 018: 教育機能強化 — Learning Breadcrumbs, Tutorial Mode, Tips

- **Date**: 2026-03-29
- **Theme**: DGE 017 で設計した教育機能の具体実装を詰める。4つの未実装アイテム (Learning Breadcrumbs, Interactive Tutorial, Did you know? Tips, 診断→メトリクスジャンプUI) の Gap を洗い出す
- **Parent Gaps**: DGE 017 (教育ファーストクラス), Session 009 (教育コンテンツと診断パターン), Session 011 (カテゴリ教育)
- **Characters**: 今泉 (初心者代弁) + ハウス (診断の天才) + ヤン (簡潔主義) + 利根川 (ユーザーの現実) + 千石 (品質の番人)
- **Input**: EDUCATION-PHILOSOPHY.md の7原則 + 現在の実装 (4段階ヘルプ, SEE ALSO 105リンク, コンテキストヒント10, カテゴリガイド6)

---

## Scene 1: Learning Breadcrumbs — 「次に見るべき」の設計

先輩 (ナレーション): Learning Breadcrumbs は「このフィールドを理解したら、次は○○を見てみよう」というナビゲーション。SEE ALSO は「関連」を示すが、Breadcrumbs は「学習順序」を示す。この違いをどう設計するか。

👤 今泉: 「あの、SEE ALSO と Learning Breadcrumbs って何が違うんですか？MemAvailable の SEE ALSO に Cached や SwapFree があるのは分かるんですけど、Breadcrumbs も同じフィールドを指すなら、ダブりません？」

🏥 ハウス: 「違う。SEE ALSO は **『一緒に見るべきもの』** — 水平方向のリンクだ。Breadcrumbs は **『理解を深めるための次のステップ』** — 垂直方向の導線だ。MemAvailable を見ているやつに SEE ALSO は『Cached も見ろ』と言う。Breadcrumbs は『MemAvailable を理解したなら、次は "なぜ MemAvailable と MemFree が違うのか" を理解しろ。それが分かったら Slab の回収可能性を学べ』と言う。**順序がある。**」

🎰 利根川: 「待て。ユーザーの現実を考えろ。障害対応中に『次は Slab を学ぼう』なんて表示されたら邪魔だ。Breadcrumbs が出るタイミングの問題がある。」

☕ ヤン: 「整理しよう。」

```
SEE ALSO: 水平リンク (同時に見るべき関連メトリクス)
  MemAvailable → [Cached, SwapFree, pgfault]
  表示: DETAILED/EXTRA ヘルプで常に表示

Breadcrumbs: 垂直リンク (学習の深化パス)
  MemAvailable → MemFree との違い → Slab/SReclaimable → vm.min_free_kbytes
  表示: EXTRA ヘルプでのみ表示。障害対応時は邪魔にならない
```

→ **Gap 発見: Breadcrumbs は EXTRA レベル限定で表示すべき。DETAILED では SEE ALSO のみ。**

👤 今泉: 「あと、Breadcrumbs のデータはどこに持つんですか？SEE ALSO は i18n.rs に入ってますよね。同じ場所？」

☕ ヤン: 「i18n.rs に追加するのは勘弁してくれ。あのファイルもう巨大だろ。Breadcrumbs は education.rs の Category 構造を拡張すればいい。カテゴリごとに学習パスを定義する方が自然だ。」

🏥 ハウス: 「いや、フィールドレベルの Breadcrumbs も要る。カテゴリレベルだけだと『Memory カテゴリの学習パス』は定義できても、『MemAvailable を見た後に次はどこへ』が定義できない。」

→ **Gap 発見: Breadcrumbs は2層構造 — カテゴリレベル (education.rs) + フィールドレベル (i18n.rs or 新構造) が必要**

千石: 「品質の問題を指摘させてください。Learning Breadcrumbs を追加するなら、**全フィールドに breadcrumb を定義する必要はない。** 重要な 30-50 フィールドに絞るべきです。600 全部に breadcrumb を書くのは保守不可能です。」

→ **Gap 発見: Breadcrumbs 対象フィールドの選定基準が未定義。SEE ALSO の31フィールドをベースに拡張するのが現実的。**

---

## Scene 2: Interactive Tutorial Mode (--tutorial)

先輩: `--tutorial` フラグで起動する対話的チュートリアル。ユーザーがステップバイステップでシステムメトリクスを学ぶモード。

👤 今泉: 「チュートリアルって、具体的にどういう体験なんですか？起動して何が表示されるんですか？」

🏥 ハウス: 「俺ならこう設計する。実際のシステムデータを使って教える。**模擬データじゃなく、いま動いてるマシンの値** を使う。『お前のマシンの MemAvailable は 3.2GB だ。MemTotal の 40% だな。一般的に 20% 以下になると問題が起き始める。今は健全だ。次のステップに進め。』」

🎰 利根川: 「そもそも --tutorial の対象ユーザーは誰だ？Linux 初心者？インフラエンジニア志望？syslenz 初心者？対象によって内容が全く変わる。」

→ **Gap 発見: チュートリアルのペルソナが未定義。**

☕ ヤン: 「3つ全部は無理だ。最初の1つを選べ。」

```
ペルソナ候補:
A) Linux 初心者 → CPU/Memory/Disk/Network の基礎から
B) インフラエンジニア → パフォーマンスチューニング寄り
C) syslenz 初心者 → TUI の操作方法 + データの見方

現実的な選択: C → A の順。
Cは10分で終わるチュートリアル。Aは30分級の学習コース。
Bはv2以降。
```

→ **Gap 発見: Phase 1 は syslenz 操作チュートリアル (C)、Phase 2 で Linux 基礎 (A) の2段階リリースが必要。**

千石: 「チュートリアル中に別の View に飛べるんですか？飛べるなら、戻ってこれるんですか？状態管理が複雑になります。」

→ **Gap 発見: Tutorial モードの UI 状態管理 — 通常モードとの共存方法が未設計。App 構造体に tutorial_state を持つか、別 View にするか。**

☕ ヤン: 「Tutorial を別 View にすると、通常のデータが見えなくなる。OverlayView パターンのほうがいい。通常の Dashboard の上にチュートリアルのステップをオーバーレイで表示。実データが見えたまま、ガイドが被さる。」

🏥 ハウス: 「ゲームのチュートリアルと同じだな。プレイ中に矢印と吹き出しが出る。中断もできる。」

→ **Gap 発見: Overlay パターン vs 独立 View パターンの選定が必要。Overlay なら実データと一緒に学べるが、render.rs の複雑度が上がる。**

---

## Scene 3: "Did you know?" ランダム Tips

先輩: 起動時やアイドル時に表示するランダムな豆知識。

☕ ヤン: 「一番簡単なやつだ。起動時に1行表示するだけ。Tips の配列を持って、ランダムに1つ選ぶ。5行で実装できる。」

🎰 利根川: 「**邪魔にならないか？** 毎回起動時に出たらウザいぞ。」

☕ ヤン: 「config で `tips = false` にすればいい。デフォルトは on。」

千石: 「Tips の内容の品質が問題です。『MemAvailable はメモリの空き容量です』レベルの Tips は価値がない。**ユーザーが驚くような、知らなかった関連性** を教えるべきです。」

🏥 ハウス: 「例えば:」

```
💡 Did you know?
"load average の数字が CPU コア数を超えると、プロセスが CPU 待ちに
なっていることを意味します。あなたのマシンは 4 コアなので、
load_1min > 4.0 が注意信号です。"
```

→ **Gap 発見: Tips はユーザーのマシン情報を含む動的コンテンツにすべき。静的な文字列配列では不十分。**

👤 今泉: 「どこに表示するんですか？Dashboard の上？下？Welcome 画面？」

☕ ヤン: 「Welcome 画面が一番自然だろ。Dashboard の footer でもいい。」

→ **Gap 発見: Tips の表示場所 — Welcome View の下部 or Dashboard footer。両方試してフィードバックで決める。**

千石: 「Tips の数は最初いくつ必要ですか？10個？50個？」

☕ ヤン: 「20個で十分。毎日使っても3週間はかぶらない。足りなくなったら足せばいい。」

→ **Gap 発見: 初期 Tips 数は20。カテゴリ別 (Memory 5, CPU 5, Network 3, Storage 3, Process 2, General 2) で偏りなく。**

---

## Scene 4: 診断結果からメトリクスジャンプ UI

先輩: diagnostics.rs の診断結果から、関連メトリクスの Detail View に直接ジャンプする機能。

🏥 ハウス: 「これは **俺の機能** だ。診断が『メモリ不足の可能性あり』と言った時、ユーザーは次に何をする？手動で meminfo を探してスクロールして MemAvailable を見つける。**面倒だ。** 診断結果から Enter キー1つで関連フィールドに飛ぶべきだ。」

👤 今泉: 「related_metrics はもう実装されてますよね？」

🏥 ハウス: 「データはある。UIが足りない。diagnostics.rs に `related_metrics` フィールドがあって、`["meminfo.MemAvailable", "meminfo.SwapFree"]` みたいな値が入ってる。だが **Diagnostics View から Detail View へのジャンプ UI がない。**」

→ **Gap 発見: Diagnostics View で related_metrics を選択 → Detail View にジャンプするキーバインド (Enter or J) が未実装。**

🎰 利根川: 「ジャンプした後、戻れるか？Diagnostics に戻るキーは？」

→ **Gap 発見: ジャンプ後の戻りナビゲーション。Backspace or Esc で前の View に戻るスタック構造が必要。App に view_history: Vec<ViewState> を持つか。**

千石: 「related_metrics のフォーマットは `source.field_name` ですか？Detail View で特定のフィールドにフォーカスを合わせるには、ソース名とフィールド名の両方が必要です。」

→ **Gap 発見: related_metrics → Detail View のフォーカス解決ロジック。source 名からソースインデックスを逆引きし、field 名からフィールドインデックスを解決する関数が必要。**

☕ ヤン: 「複数の related_metrics がある場合は？リストで表示して選ばせるのか？」

🏥 ハウス: 「そうだ。診断結果を選んで Enter → related_metrics のリストが出る → 1つ選んで Enter → Detail View にジャンプ。2段階。」

→ **Gap 発見: related_metrics が複数の場合のピッカーUI。Diagnostics View 内のサブリスト表示が必要。**

---

## Gap 抽出

| # | Gap | カテゴリ | 重要度 |
|---|-----|---------|--------|
| G18-1 | Breadcrumbs は EXTRA レベル限定。DETAILED は SEE ALSO のみ | UI設計 | P1 |
| G18-2 | Breadcrumbs は2層構造: カテゴリレベル + フィールドレベル | データ設計 | P1 |
| G18-3 | Breadcrumbs 対象フィールドの選定 (SEE ALSO 31フィールドベース) | スコープ | P1 |
| G18-4 | Tutorial のペルソナ: Phase 1 = syslenz操作, Phase 2 = Linux基礎 | スコープ | P0 |
| G18-5 | Tutorial の UI パターン: Overlay vs 独立 View | UI設計 | P1 |
| G18-6 | Tutorial の状態管理 (App との共存) | 実装設計 | P1 |
| G18-7 | Tips は動的コンテンツ (ユーザーのマシン情報を含む) | データ設計 | P1 |
| G18-8 | Tips の表示場所: Welcome 下部 or Dashboard footer | UI設計 | P2 |
| G18-9 | 初期 Tips 数20、カテゴリ偏りなし | スコープ | P2 |
| G18-10 | Diagnostics → Detail View ジャンプキーバインド | 実装 | P0 |
| G18-11 | View 戻りナビゲーション (view_history スタック) | 実装設計 | P0 |
| G18-12 | related_metrics → Detail View のフォーカス解決ロジック | 実装 | P0 |
| G18-13 | related_metrics 複数時のピッカーUI | UI設計 | P1 |

---

## 優先度による実装順序

```
=== Phase A: 診断→メトリクスジャンプ (最もインパクト大、既存データ活用) ===
G18-10: ジャンプキーバインド
G18-11: view_history スタック
G18-12: フォーカス解決ロジック
G18-13: ピッカーUI

=== Phase B: Did you know? Tips (最も軽量) ===
G18-7: 動的 Tips エンジン
G18-8: 表示場所
G18-9: 初期コンテンツ20個

=== Phase C: Learning Breadcrumbs (SEE ALSO の拡張) ===
G18-1: EXTRA 限定表示
G18-2: 2層構造設計
G18-3: 対象フィールド選定

=== Phase D: Interactive Tutorial (最も大規模) ===
G18-4: ペルソナ確定
G18-5: UI パターン選定
G18-6: 状態管理設計
```
