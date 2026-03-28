---
version: v1.0.0
lang: ja
---

# syslenz ドキュメント

[🇬🇧 English](../en/index.md)

> **Wireshark for /proc** -- 構造化されたスキーマ駆動型システム情報ビューア

syslenz は Linux の `/proc`、`/sys`、ネットワーク設定ファイルを型付きの構造化データにパースし、差分表示、時系列グラフ、自動診断、教育ガイド、JSON エクスポート/インポート機能を備えた高速 TUI で表示します。深夜3時の本番障害対応でも、Linux の仕組みの学習でも、syslenz はシステムに対する即座の構造化されたインサイトを提供します。

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
cargo install --path . --features web

# OpenTelemetry エクスポート
cargo install --path . --features otel

# 全機能
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
- **ダッシュボード表示** -- ロード、メモリ、CPU、ネットワーク、ディスクを一目で確認
- **自動診断** -- メモリ圧力、CPU過負荷、スワップ枯渇、ゾンビプロセス、ソケットリーク、ディスク満杯、サーマルスロットリング、FD枯渇、DNS設定ミス、conntrackオーバーフローを自動検出
- **教育カテゴリガイド** -- メモリ、CPU、ネットワーク、ストレージ、プロセスの構造化ストーリーでLinux内部を学習
- **スナップショット差分** -- リフレッシュ間の変化をカラーでハイライト
- **時系列グラフ** -- 任意の数値フィールドのスパークライン表示（60スナップショットリングバッファ）
- **JSONエクスポート/インポート** -- システム状態をキャプチャ、共有、後で再生
- **リモート監視** -- SSH、Docker exec、TCP サーバー/クライアントモード
- **Web UI** -- SSE リアルタイムストリーミングのブラウザベースダッシュボード
- **プラグインシステム** -- 実行可能プラグインでカスタムデータソースを追加
- **OpenTelemetry エクスポート** -- 全数値メトリクスを OTLP 対応バックエンドにプッシュ
- **バイリンガル** -- 英語・日本語完全対応（`L`キーまたは`--lang`で切替）
- **キーボード駆動 TUI** -- サイドバーナビゲーション、ドリルイン、検索、クリップボードコピー

## ドキュメント一覧

| ドキュメント | 説明 |
|-------------|------|
| [はじめに](getting-started.md) | インストール、初回起動、基本操作、CLIフラグ |
| [ダッシュボード](dashboard.md) | ダッシュボード表示：メトリクス、セクション、ナビゲーション |
| [クラシックモード](classic-mode.md) | サイドバー＋詳細表示、検索、差分、グラフ |
| [自動診断](diagnostics.md) | 自動診断エンジン、重要度レベル、全チェック項目 |
| [教育機能](education.md) | カテゴリガイド、ヘルプレベル、Linux内部の学習 |
| [リモート監視](remote.md) | SSH、Docker、TCP サーバー/クライアントモード |
| [Web UI](web-ui.md) | ブラウザベースダッシュボードのセットアップと使用方法 |
| [プラグイン](plugins.md) | プラグインシステム、カスタムプラグインの作成 |
| [設定](config.md) | config.toml リファレンス、全オプション |
| [キーバインド](keybindings.md) | ビュー別キーバインド完全リファレンス |
| [OpenTelemetry](otel.md) | OTLP メトリクスエクスポート、Prometheus/Grafana セットアップ |
| [データソース](sources.md) | 55以上の全データソース：読み取り内容と主要フィールド |

---

[Index](index.md) | [次: はじめに ->](getting-started.md)
