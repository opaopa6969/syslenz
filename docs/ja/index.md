---
version: v1.4.0
lang: ja
---

# syslenz ドキュメント

[🇬🇧 English](../en/index.md)


> **Wireshark for /proc** -- スキーマ駆動のシステム情報ビューア

syslenz は Linux の `/proc`、`/sys`、ネットワーク設定ファイルを型付きの構造化データとしてパースし、差分表示、時系列グラフ、自動診断、教育ガイド、JSON エクスポート/インポートを備えた高速 TUI で表示します。深夜の本番障害対応にも、Linux の仕組みの学習にも、syslenz はシステムの状態を構造化して即座に把握する手段を提供します。

![demo](../assets/demo.gif)

## 目次

- [クイックインストール](#クイックインストール)
- [主な機能](#主な機能)
- [ドキュメント一覧](#ドキュメント一覧)

## クイックインストール

**ソースから (Cargo):**

```bash
cargo install --path .
```

**オプション機能付き:**

```bash
# Web UI サポート

[🇬🇧 English](../en/index.md)
cargo install --path . --features web

# OpenTelemetry エクスポート

[🇬🇧 English](../en/index.md)
cargo install --path . --features otel

# 全機能

[🇬🇧 English](../en/index.md)
cargo install --path . --features "web,otel,x11widget"
```

**Docker:**

```bash
docker run --rm -it --pid=host --privileged syslenz/syslenz
```

**バイナリダウンロード:**

[GitHub Releases](https://github.com/opaopa6969/syslenz/releases) ページから最新リリースをダウンロードし、`PATH` の通ったディレクトリに配置してください。

## 主な機能

- **55以上のデータソース** -- `/proc`、`/sys`、ネットワーク設定、温度、ディスク、conntrack、DNS など
- **スキーマ駆動パース** -- 全フィールドに型（Bytes、Integer、Float、Duration、Text、Table）、単位、説明を付与
- **ダッシュボード** -- ロード、メモリ、CPU、ネットワーク、ディスクをひと目で確認
- **自動診断** -- メモリ圧迫、CPU過負荷、スワップ枯渇、ゾンビプロセス、ソケットリーク、ディスク満杯、サーマルスロットリング、FD枯渇、DNS設定ミス、conntrackオーバーフローを自動検出
- **教育カテゴリガイド** -- メモリ、CPU、ネットワーク、ストレージ、プロセスをストーリー仕立てで解説し、Linux の内部構造を学べる
- **スナップショット差分** -- リフレッシュごとの変化をカラーでハイライト
- **時系列グラフ** -- 数値フィールドのスパークライン表示（60スナップショットのリングバッファ）
- **JSONエクスポート/インポート** -- システム状態をキャプチャ・共有し、後から再生
- **リモート監視** -- SSH、Docker exec、TCP サーバー/クライアントモード
- **Web UI** -- SSE リアルタイムストリーミング対応のブラウザダッシュボード
- **プラグインシステム** -- 実行可能プラグインでカスタムデータソースを追加
- **OpenTelemetry エクスポート** -- 全数値メトリクスを OTLP 対応バックエンドにプッシュ
- **バイリンガル** -- 英語・日本語完全対応（`L`キーまたは`--lang`で切替）
- **キーボード操作の TUI** -- サイドバーナビゲーション、ドリルイン、検索、クリップボードコピー
- **診断ジャンプ** -- 診断結果から関連メトリクスへ直接ジャンプ
- **Learning Breadcrumbs** -- EXTRA ヘルプレベルで「次のステップ」ヒントを表示（18フィールド）
- **"Did you know?" Tips** -- Welcome 画面にランダムな学習 Tips を表示
- **チュートリアルモード** -- `--tutorial` で実データを使った8ステップのガイド付きウォークスルー
- **SDK 3種** -- Java (syslenz4j)、Python (syslenz4py)、Node.js (syslenz4node) で syslenz にプログラムからアクセス
- **Grafana 連携** -- `docker compose --profile grafana` でダッシュボード自動プロビジョニング

## v1.4.0 の新機能

- **診断ジャンプ** -- 診断結果から関連メトリクスソースへ直接ナビゲーション（view_history スタック + ピッカー UI）
- **"Did you know?" Tips** -- Welcome ビューに動的なランダム教育 Tips を表示
- **Learning Breadcrumbs** -- EXTRA ヘルプレベルで18フィールド（EN/JA）に「次のステップ」ヒントを表示し、深い探索を促す
- **インタラクティブチュートリアル** -- `--tutorial` で実データを埋め込んだ8ステップのガイド付きウォークスルーを起動
- **SEE ALSO 相互参照** -- 31フィールドに105のクロスリファレンスを追加
- **Python SDK (syslenz4py)** -- Python から syslenz に接続（`sdk/python/`）
- **Node.js SDK (syslenz4node)** -- Node.js から syslenz に接続（`sdk/node/`）
- **OTEL ブリッジ改善** -- リソース属性、i18n 記述、カウンター検出
- **Provider 貢献ガイド** -- 新規 Provider 作成のステップバイステップガイド（JA/EN、テンプレート付き）
- **Grafana ダッシュボードプロビジョニング** -- `docker compose --profile grafana` で Prometheus + Grafana + ビルトインダッシュボードを自動セットアップ
- **MetricKind / CommonMetric enum** -- 型付きメトリクス分類（8 variants）とクロスプラットフォームメトリクス（15種）
- **診断パターン拡張** -- 27チェック関数、40以上のパターン

## v1.1.0 の新機能

- **タイムトラベル差分** -- 差分ビューで `[` / `]` キーを使い、過去・未来のスナップショットと比較可能。ステータスバーの `T-N` インジケータで比較対象を表示
- **アラートシステム** -- `[[alert]]` 設定ルールでカスタムアラートを定義。ステータスバーへのアラート表示、サイドバーの色分け、フィールドマーカーで閾値超過を即座に把握
- **AAバーグラフ** -- ダッシュボードの RAM / Swap / CPU セクションに ASCIIアートバーグラフ (`████░░░░`) を表示し、使用率を視覚的に確認
- **スパークライン** -- ダッシュボードにロードとメモリ履歴のスパークライン (`▁▂▃▄▅▆▇█`) を表示し、直近のトレンドをひと目で把握
- **詳細ビュー自動スパークライン** -- 数値フィールドを選択すると、フィールド下部にスパークライングラフを自動表示
- **検索バーの可視化** -- `/` でステータスバーにカーソル付き検索入力欄が表示され、入力状態が明確に
- **[Enter to expand] インジケータ** -- テーブル型フィールドに `[Enter to expand]` を表示し、ドリルインできることを明示
- **ViewData 統一 UI** -- TUI と Web UI が同じ ViewData 構造体を共有し、一貫したデータ表示を実現
- **Web UI 改善** -- 数値フィールドの自動グラフ表示、PgUp/PgDn キーサポート、サーバーからのカテゴリガイド配信
- **i18n 拡張** -- フィールド翻訳カバレッジが 390/600 に拡大（前バージョンから大幅増）

## v1.3.0 の新機能

- **GPU 監視 (nvidia-smi)** -- NVIDIA GPU の温度、使用率、VRAM、ファン速度、消費電力をリアルタイムで監視。`nvidia-smi` が存在する環境で自動的に有効化
- **systemd サービス監視** -- systemd の全体状態、実行中/失敗サービス数、失敗サービスの一覧を表示。`degraded` 状態を即座に把握
- **Prometheus エクスポート (`--prometheus`)** -- OTLP Collector なしで `/metrics` エンドポイントを直接公開。Prometheus から直接スクレイプ可能
- **診断パターン 10 種追加** -- メモリリーク検出、スワップ活動監視、OOM Kill 検出、TCP 再送/UDP エラー、最近の再起動通知、負荷スパイク/回復、高メモリプロセス警告、孤立 TCP ソケット、IP 転送検出、カーネル汚染チェック
- **Provider システム** -- データベースやミドルウェア向けの標準化された Provider テンプレート。MySQL、PostgreSQL、Redis、nginx の公式 Provider を同梱
- **ダッシュボード GPU セクション** -- NVIDIA GPU が利用可能な環境で、GPU 温度・使用率・VRAM をダッシュボードに表示

## ドキュメント一覧

| ドキュメント | 説明 |
|-------------|------|
| [はじめに](getting-started.md) | インストール、初回起動、基本操作、CLIフラグ |
| [ダッシュボード](dashboard.md) | ダッシュボード：メトリクス、セクション、ナビゲーション |
| [クラシックモード](classic-mode.md) | サイドバー＋詳細表示、検索、差分、グラフ |
| [自動診断](diagnostics.md) | 自動診断エンジン、重要度レベル、全チェック項目 |
| [教育機能](education.md) | カテゴリガイド、ヘルプレベル、Linux内部の学習 |
| [リモート監視](remote.md) | SSH、Docker、TCP サーバー/クライアントモード |
| [Web UI](web-ui.md) | ブラウザダッシュボードのセットアップと使い方 |
| [プラグイン](plugins.md) | プラグインシステム、カスタムプラグインの作成 |
| [設定](config.md) | config.toml リファレンス、全オプション |
| [キーバインド](keybindings.md) | ビュー別キーバインド一覧 |
| [OpenTelemetry](otel.md) | OTLP メトリクスエクスポート、Prometheus/Grafana セットアップ |
| [データソース](sources.md) | 55以上の全データソース：読み取り内容と主要フィールド |
| [Provider 貢献ガイド](provider-contribution-guide.md) | 新規 Provider の作り方、テンプレート、テスト手順 |

---

[Index](index.md) | [次: はじめに ->](getting-started.md)
