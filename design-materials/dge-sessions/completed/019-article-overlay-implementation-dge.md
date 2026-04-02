# DGE Session 019: Article Overlay 実装詳細化 — 仕様凍結前レビュー

- **Date**: 2026-04-02
- **Theme**: Session 018 の構想を実装可能な粒度（状態遷移、データスキーマ、API、受け入れ条件）まで分解する
- **Parent**: Session 018 (Metric Article Overlay System)
- **Characters**: 千石 + ラインハルト + 今泉 + ハウス + ヤン + 利根川

---

## 1. 実装範囲の確定 (MVP)

### In Scope (v1.5)
- TUI: `A` で記事オーバーレイ開閉
- TUI: 記事本文スクロール（`j/k`, `PgUp/PgDn`）
- TUI: SEE ALSO 選択と `Enter` ジャンプ
- Web: `A` + ボタンでオーバーレイ開閉
- Web: SEE ALSO クリックで metric/article ジャンプ
- Article 種別: Metric / Group / Concept（最小セット）

### Out of Scope (v1.5)
- 学習履歴
- 推薦アルゴリズム
- URL共有 (`?article=`)
- Markdown完全レンダラ（MVPは簡易整形）

---

## 2. 状態モデル

☕ ヤン: 「Viewを増やしすぎるより、overlay stateを別管理した方が壊れにくい。」

### 決定

- `App.view` は既存のまま
- 新規状態 `article_overlay: Option<ArticleOverlayState>` を追加
- overlay表示中は入力を優先処理

```rust
pub struct ArticleOverlayState {
    pub article_id: String,
    pub scroll: usize,
    pub selected_link: usize,
}
```

### キー挙動

- `A`: open/close toggle
- `Esc` / `q`: close
- `j/k`: 本文スクロール
- `Tab` / `Shift+Tab`: SEE ALSO 選択移動
- `Enter`: 選択リンク実行

---

## 3. Article スキーマ (MVP)

🎋 千石: 「本体は静的配列でもいい。まず schema を固定しろ。」

```rust
pub enum ArticleKind { Metric, Group, Concept }

pub enum ArticleLink {
    Metric { source: &'static str, field: &'static str, label_en: &'static str, label_ja: &'static str },
    Article { id: &'static str, label_en: &'static str, label_ja: &'static str },
}

pub struct EducationArticle {
    pub id: &'static str,
    pub kind: ArticleKind,
    pub title_en: &'static str,
    pub title_ja: &'static str,
    pub body_en: &'static str,
    pub body_ja: &'static str,
    pub links: &'static [ArticleLink],
}
```

---

## 4. 解決ルール

### 4.1 Metric解決
- key: `{source}.{field}`
- 例: `meminfo.MemAvailable`

### 4.2 Group解決
- `*_min`, `*_max`, `*_count` は同一 stem にまとめる
- key: `{source}.{stem}_distribution`
- 例: `net/netstat.TCPRto_distribution`

### 4.3 Fallback
- 一致なし: `concept.reading-metrics` を表示

---

## 5. ジャンプ仕様

### metric link
1. 対象 source を sidebar index に解決
2. 対象 field index を解決
3. `view = Detail`, `focus = Content`
4. overlay close

### article link
1. `article_id` を差し替え
2. `scroll = 0`
3. `selected_link = 0`

---

## 6. Web API 仕様 (MVP)

`GET /api/article?source=...&field=...&locale=en|ja`

### Response

```json
{
  "found": true,
  "id": "meminfo.MemAvailable",
  "title": "MemAvailable",
  "kind": "Metric",
  "body": "...",
  "links": [
    {"type":"metric","label":"SwapFree","source":"meminfo","field":"SwapFree"},
    {"type":"article","label":"PSI","id":"concept.pressure-stall"}
  ]
}
```

---

## 7. 受け入れ条件 (MVP)

1. TUIで `A` を押すと、現在選択メトリクスに応じた記事が開く
2. TUIで `Enter` により SEE ALSO の metric ジャンプが機能する
3. Webで `A` またはボタンから同等の記事オーバーレイが開く
4. `*_min/*_max/*_count` 系で group 記事が優先表示される
5. 未定義メトリクスでも fallback 記事が開く（空画面にしない）

---

## 8. リスクと対策

- リスク: 手動記事増加で整合性崩壊
  - 対策: id 命名規約 + 簡易整合テスト
- リスク: Web/TUI の挙動差
  - 対策: API と resolver を共通化
- リスク: 表示崩れ（長文）
  - 対策: overlay高さ固定 + 縦スクロール

---

## 9. セッション結論

- Session 018 を実装可能粒度に凍結
- v1.5 は「overlay体験の成立」を優先
- 記事数より先に「読む体験の完成度」を確保

