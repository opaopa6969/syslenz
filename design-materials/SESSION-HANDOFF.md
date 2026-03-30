# syslenz セッション引き継ぎ

## プロジェクト概要

syslenz — Wireshark for Linux。システム情報を構造化データとして可視化する TUI/Web ツール。
教育コンテンツ付きで「これがあればコンピュータがわかる」を目指す。

**GitHub**: https://github.com/opaopa6969/syslenz
**Java SDK**: https://github.com/opaopa6969/syslenz4j (Maven Central: org.unlaxer.infra:syslenz4j:1.1.0)

## 現在のバージョン

v1.3.0 "See More, Learn More" (+ 未タグのコミットあり)

## アーキテクチャ

```
syslenz/
├── Cargo.toml + src/        ← Rust 本体 (core)
├── sdk/java/                ← syslenz4j (Maven)
├── providers/               ← MySQL, PostgreSQL, Redis, nginx, JVM, Docker
├── docs/en/ + docs/ja/      ← ユーザードキュメント (13 each, v1.3.0)
├── docs/features/           ← 機能仕様 (completed/in-progress/planned)
├── design-materials/        ← DGE セッション 17回 + specs
├── DGE/                     ← DGE ツールキット
├── tests/                   ← Rust + Playwright
└── PROJECT.md               ← multi-project 構成説明
```

## 数値

| 項目 | 数値 |
|------|------|
| Linux ソース | 53+ (proc 43 + sys 5 + net 5 + GPU + systemd) |
| macOS ソース | 24 |
| Windows ソース | 24 |
| フィールド数 | 600+ (全て型付き + description) |
| i18n (EN/JA 3レベル) | 584/600 (97%) |
| TUI ビュー | 9 (Dashboard, Classic, Welcome, Detail, Diff, Table, Graph, Diagnostics, CategoryGuide) |
| 診断パターン | 27関数 40+ パターン |
| SEE ALSO 相互参照 | 31フィールド 105リンク |
| コンテキストヒント | 10フィールド |
| Learning Breadcrumbs | 18フィールド (EN/JA) |
| テスト | 161 Rust + 48 Playwright = 209 |
| DGE セッション | 18回 |
| Provider | 6 (JVM, Docker, MySQL, PostgreSQL, Redis, nginx) |
| SDK | 3 (Java, Python, Node.js) |
| MetricKind | 8 variants |
| CommonMetric | 15 cross-platform metrics |
| リリース | v1.0.0, v1.1.0, v1.2.0, v1.3.0 |

## 主要ファイルと役割

| ファイル | 役割 |
|---------|------|
| src/proc/mod.rs | コア型 (Snapshot, ProcEntry, Field, FieldValue) + 43パーサー |
| src/ui/app.rs | App 状態管理 (View, Focus, HelpLevel, alerts, multi-host) |
| src/ui/render.rs | TUI 描画 (全ビュー + SEE ALSO + contextual hints) |
| src/ui/view_data.rs | ViewData 統一 UI 層 (構造定義 → compile → 実体) |
| src/i18n.rs | 国際化 (584 エントリ + see_also + source_description) |
| src/diagnostics.rs | 自動診断 (27 check関数 + related_metrics) |
| src/education.rs | カテゴリガイド (6カテゴリ + 学習パス) |
| src/alert.rs | アラートシステム ([[alert]] config) |
| src/config.rs | 設定ファイル (~/.config/syslenz/config.toml) |
| src/web.rs | Web UI (Axum + SSE + Chart.js) |
| src/prometheus.rs | Prometheus /metrics エンドポイント |
| src/plugin/mod.rs | プラグインローダー |
| src/remote.rs | SSH/Docker/TCP リモート |
| src/serve.rs | TCP サーバーモード (--serve) |

## 設計原則

### DGE (Design-Gap Exploration) 駆動開発
- 全ての設計判断は DGE セッションで駆動
- キャラクター: 今泉(前提), 千石(品質), ヤン(簡素化), ハウス(診断), リヴァイ(実装), ラインハルト(ビジョン), 大和田(ビジネス), 利根川(ユーザー), 僕(スコープ縮小)
- 17セッション、100+ Gap 発見

### 教育はファーストクラス機能 (DGE 017)
- 詳細は EDUCATION-PHILOSOPHY.md を参照
- 「自己開示するシステムは、しないシステムに勝つ」

### UI 設計方針
- ViewData パターン: 構造定義 → compile → 実体 (TUI/Web 統一)
- Classic UI (高速パパッ) は O キーで常に利用可能。デフォルトは Dashboard
- 教育コンテンツの長さを UI の都合で切らない
- 日本語ドキュメントを先に書いて英語に翻訳

## 完了バックログ (2026-03-30 セッション)

### 教育機能強化 (DGE 018)
- [x] 診断結果からのメトリクスジャンプ UI (view_history スタック + ピッカーUI)
- [x] "Did you know?" ランダム Tips (動的コンテンツ、Welcome View)
- [x] Learning breadcrumbs (EXTRA レベル、18フィールド EN/JA)
- [x] Interactive tutorial mode (--tutorial、8ステップ、実データ埋め込み)

### v1.4.0 "Web Scale" (DGE 016)
- [x] OTEL bridge 改善 (resource attrs, i18n descriptions, counter detection)
- [x] Provider contribution guide (JA/EN、テンプレート付き)
- [x] syslenz4py (Python SDK、sdk/python/)
- [x] syslenz4node (Node.js SDK、sdk/node/)

### v2.0.0 "Platform"
- [x] Grafana ダッシュボード (provisioning + docker-compose --profile grafana)
- [x] MetricField enum 自動生成 (MetricKind 8 variants + CommonMetric 15 metrics)
- [x] カスタムダッシュボード → Grafana provisioning で対応
- [x] 長期保存 → Prometheus/OTEL 経由で外部 TSDB に委譲

## 未完了バックログ (次回の作業)

### v2.1.0+ 候補
- [ ] syslenz4py / syslenz4node の PyPI / npm パブリッシュ
- [ ] Grafana data source plugin (専用、Prometheus で不十分な場合)
- [ ] Web UI の独立パッケージ化
- [ ] Java SDK の CommonMetric enum 自動生成

### やらないことリスト (DGE 015)
- ML anomaly detection
- ログ収集
- APM (トレーシング)
- Kubernetes native
- SaaS 化
- カスタムクエリ言語
- モバイルアプリ

## ユーザーのフィードバック (memory/)

| 方針 | 内容 |
|------|------|
| Classic UI 保持 | 高速 UI は O キーで常にアクセス可能 |
| コンテンツ制限なし | 教育コンテンツは本1章分でもスクロール対応 |
| 日本語先行 | ドキュメントは ja → en の順で書く |
| MetricField enum | ✅ 完了 (metric_kind.rs + common_metric.rs) |

## CI/CD

- GitHub Actions: Linux + macOS + Windows + Java SDK
- Release: タグで 6 プラットフォームバイナリ自動ビルド
- Playwright: Web UI 48テスト + 動画録画
