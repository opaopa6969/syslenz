# DGE Session 018: Metric Article Overlay System — syslenz を「読む監視ツール」にする

- **Date**: 2026-04-02
- **Theme**: syslenz の教育機能を「ヘルプ文」から「構造化された実用読み物」へ進化させる。volta の glossary レベルの文書体験を、数百メトリクス（単体/グループ）へ適用する。
- **Parent Gaps**: Session 009 (教育/診断), Session 011 (カテゴリ教育), Session 017 (教育ファーストクラス)
- **Characters**: 千石 (品質) + ラインハルト (ビジョン) + 今泉 (初学者) + ハウス (診断) + ヤン (簡潔) + 利根川 (現実)
- **Input**: 現在は OFF/NORMAL/DETAILED/EXTRA のヘルプレベルとカテゴリガイドはあるが、`volta-auth-proxy` の glossary のような「深く読める、リンクで辿れる、実務に効く記事体験」が不足している。

---

## 問題定義

先輩: 既存ヘルプは「その場で助かる」が、学習資産としては薄い。

- 現状の強み: 速い、文脈に沿ったヒント、診断導線がある
- 現状の弱み: 長文の体系知識が不足、関連概念ジャンプが弱い、再利用可能な記事資産になっていない
- ユーザー要求: 
  - volta glossary 級の読み物
  - メトリクス単体だけでなく `*_min`, `*_max`, `*_count` のようなグループ記事
  - ショートカットで「記事オーバーレイ」表示
  - 関連項目ジャンプ
  - TUI と Web 両対応

→ **結論**: ヘルプパネルとは別に「Article Overlay」という第2教育面を追加する。

---

## Scene 1: 「ヘルプ」と「記事」を分離する

👤 今泉: 「今の DETAILED/EXTRA でも説明は読めますよね。何が足りないんですか？」

🦁 ラインハルト: 「ヘルプは『現在の値を理解する』ための短距離走。記事は『概念を自分の武器にする』ための長距離走だ。両方必要だ。」

☕ ヤン: 「役割を分けよう。」

```
Help Panel (既存):
  - 3〜20行
  - 今見ている値の解釈
  - 即時判断向け

Article Overlay (新規):
  - 数十〜数百行
  - 背景、誤解、診断パターン、失敗例、SEE ALSO
  - 学習・定着向け
```

🎋 千石: 「品質基準も分ける。ヘルプは即応性、記事は正確性・再利用性。」

→ **Gap G-EDU-18-1**: 教育UIの二層構造（Help vs Article）が未定義。

---

## Scene 2: メトリクス単体記事とグループ記事

🏥 ハウス: 「`TCPRtoMin`, `TCPRtoMax`, `TCPRtoCount` を個別に説明しても、初心者には意味が繋がらない。**分布の物語**として読ませろ。」

🎰 利根川: 「実務では『count だけ見る』『max だけ見る』は事故る。min/max/count をセットで読む習慣を記事で作るべきだ。」

### 設計決定: Article 種別

1. **Metric Article**
- 例: `meminfo.MemAvailable`
- 特定フィールドの定義、読み方、しきい値、誤解

2. **Group Article**
- 例: `netstat.tcp_rto_distribution` (`*_min`, `*_max`, `*_count`)
- 複数フィールドをセットで解説

3. **Concept Article**
- 例: `pressure_stall_information`, `cache_reclaim`, `context_switch_storm`
- ソース横断の概念

### グループ解決ルール（初期案）

- 命名ベース: `*_min/*_max/*_count`, `*_some/*_full`, `rx_*/tx_*` など
- 手動マッピング優先: 誤分類を避けるため canonical index を用意

→ **Gap G-EDU-18-2**: メトリクス/グループ/概念の article taxonomy 未実装。

---

## Scene 3: Overlay UX（TUI）

👤 今泉: 「記事はどこで開くんですか？」

☕ ヤン: 「キー 1 つ。覚えやすく。」

### TUI 提案仕様

- `A`: 現在選択中メトリクス/グループの Article Overlay を開く
- 表示: 中央オーバーレイ（80-90% 幅、高さ可変）
- スクロール: `j/k`, `PgUp/PgDn`, `g/G`
- 閉じる: `Esc` / `q`
- リンク移動: `Tab` で SEE ALSO セクションへ、`Enter` でジャンプ
- ジャンプ結果:
  - metric link: 該当フィールドへフォーカス遷移
  - article link: 別記事に遷移（overlay内 push/pop）
- パンくず: `Article: meminfo.MemAvailable > pressure_stall_information`

🎋 千石: 「重要: overlay中でも現在値（最新スナップショット）参照は保持する。記事が古典化しないよう、ヘッダに live context を残す。」

→ **Gap G-EDU-18-3**: TUI overlay view / navigation stack / deep-link 遷移が未実装。

---

## Scene 4: Overlay UX（Web）

🦁 ラインハルト: 「教育体験は TUI だけに閉じない。Web も同じ情報設計で揃える。」

### Web 提案仕様

- `A` キー + UI ボタン「Article」
- モーダルではなく **drawer + full-screen 切替可能**
- Markdown レンダリング（見出しナビ + SEE ALSO chips）
- リンククリックで:
  - metric: 現在画面の該当フィールドへスクロール & ハイライト
  - article: drawer 内部遷移
- URL 同期（任意）:
  - `?article=meminfo.MemAvailable`
  - 共有可能な学習リンク

→ **Gap G-EDU-18-4**: Web overlay parity（TUI 同等ナビ/遷移）の設計・実装が未着手。

---

## Scene 5: 記事コンテンツモデル（volta glossary 互換の強さ）

🎰 利根川: 「読ませるならテンプレートを固定しろ。品質がブレると教育は壊れる。」

### Article テンプレート（v1）

1. `これは何か`（1段落）
2. `なぜ重要か`（障害/運用への影響）
3. `どう読むか`（値の解釈）
4. `よくある誤解`（誤読パターン）
5. `診断フロー`（手順）
6. `失敗例`（現場の地雷）
7. `SEE ALSO`（関連メトリクス/記事）

### データモデル（Rust）

```rust
pub enum ArticleKind {
    Metric,
    Group,
    Concept,
}

pub struct EducationArticle {
    pub id: &'static str,           // e.g. "meminfo.MemAvailable"
    pub kind: ArticleKind,
    pub title_en: &'static str,
    pub title_ja: &'static str,
    pub markdown_en: &'static str,
    pub markdown_ja: &'static str,
    pub related_metrics: &'static [MetricRef],
    pub related_articles: &'static [&'static str],
    pub tags: &'static [&'static str],
}

pub struct MetricRef {
    pub source: &'static str,
    pub field: &'static str,
}
```

### 配置案

- `docs/articles/en/*.md`
- `docs/articles/ja/*.md`
- `src/education/articles_index.rs`（生成 or 手書き）

→ **Gap G-EDU-18-5**: 記事の canonical schema・保存場所・ロード戦略が未定義。

---

## Scene 6: 生成と運用（数百パラメータに耐える仕組み）

☕ ヤン: 「手書きで 600 本は破綻する。優先度駆動にする。」

### 運用戦略

- Phase A: 上位 50 メトリクス + 15 グループ + 10 概念
- Phase B: 上位 150 + 40 + 25
- Phase C: Long tail を段階追加

### 優先度算出

- 診断で登場する頻度
- ダッシュボード露出
- 誤読事故の多さ
- 初学者のつまずき率

### 品質ゲート

- 技術正確性レビュー（千石ゲート）
- 実務有用性レビュー（ハウス/利根川ゲート）
- 可読性レビュー（今泉ゲート）

→ **Gap G-EDU-18-6**: 「記事量産の品質保証プロセス」が未定義。

---

## 決定事項（Architecture Decisions）

1. Help と Article は分離し、両方残す。
2. Article は `Metric / Group / Concept` の3種を持つ。
3. `A` キーで overlay を開く（TUI/Web共通）。
4. SEE ALSO は「表示」だけでなく「ジャンプ可能」にする。
5. グループ記事は命名規則 + 手動indexで解決する。
6. 初期は高頻度メトリクス優先で段階展開する。

---

## 実装ロードマップ

### v1.5 (MVP)

- TUI:
  - `View::ArticleOverlay` 追加
  - `A` で開閉
  - Markdown プレーン描画（最低限）
  - SEE ALSO の metric jump
- Content:
  - 20 metric + 8 group + 5 concept 記事
  - EN/JA 両対応

### v1.6

- TUI:
  - article-to-article 遷移
  - overlay 内検索 (`/`)
- Web:
  - Article drawer 実装
  - SEE ALSO jump parity

### v1.7

- Web:
  - full-screen article mode
  - URL 共有対応 (`?article=`)
- Content:
  - 100+ 記事

### v2.0

- 学習履歴（Learning Breadcrumbs）
- 「次に読むべき記事」推薦
- 診断結果から記事自動提案

---

## Backlog Candidate (新規)

- **BL-080**: Article schema + loader 実装（EN/JA）
- **BL-081**: TUI Article Overlay (`A` shortcut, scroll, close)
- **BL-082**: SEE ALSO jump (metric/article)
- **BL-083**: Group resolver (`*_min/max/count` 等)
- **BL-084**: Web Article Drawer parity
- **BL-085**: Article authoring guide + quality gate
- **BL-086**: Top 50 metrics article pack

---

## セッション結論

🦁 ラインハルト: 「syslenz は“見る監視”から“読む監視”へ進む。教育をファーストクラスにすると決めたなら、読み物を資産化しなければならない。」

🎰 利根川: 「数字を見せるだけのツールは代替される。理解を渡せるツールだけが残る。」

🎋 千石: 「品質を落とした記事は害になる。テンプレートとレビューゲートを先に作る。」

☕ ヤン: 「まず 50 本。価値を出してから広げる。それが現実的で速い。」

👤 今泉: 「`A` を押せば『今見てる値の教科書』が開く。これなら学べます。」

🏥 ハウス: 「最終目標はこれだ。異常が出た瞬間、ユーザーが“何を見るべきか”を自分で辿れること。」

**Outcome**: G-EDU-18-1〜6 を確定。Article Overlay System を次期教育基盤として採択。

