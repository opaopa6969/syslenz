---
version: v1.4.0
lang: ja
---

# はじめに

[English](../en/getting-started.md)

[← Index](index.md) | [次: ダッシュボード →](dashboard.md)

---

## 目次

- [動作要件](#動作要件)
- [インストール](#インストール)
  - [ワンライナーインストーラー](#ワンライナーインストーラー)
  - [cargo install (crates.io)](#cargo-install-cratesio)
  - [ソースから](#ソースから)
  - [オプション機能](#オプション機能)
  - [Docker](#docker)
  - [ビルド済みバイナリ](#ビルド済みバイナリ)
- [初回起動](#初回起動)
- [基本操作](#基本操作)
- [ビューの切り替え](#ビューの切り替え)
- [ヘルプの使い方](#ヘルプの使い方)
- [メトリクス配信 (--serve)](#メトリクス配信---serve)
- [Web UI (--web)](#web-ui---web)
- [CLI フラグリファレンス](#cli-フラグリファレンス)

---

## 動作要件

- **Linux**（主要サポート）: `/proc` と `/sys` を持つカーネル 3.10 以上
- **macOS** / **Windows**: プラットフォームアダプター経由でサポート（各 24 ソース）
- **Rust**: edition 2024、rustc 1.85 以上（ソースビルドの場合）

---

## インストール

### ワンライナーインストーラー

```bash
curl -sSf https://raw.githubusercontent.com/opaopa6969/syslenz/main/scripts/install.sh | sh
```

プラットフォーム向けの最新ビルド済みバイナリをダウンロードし `/usr/local/bin/` に配置します。

### cargo install (crates.io)

```bash
cargo install syslenz
```

デフォルトフィーチャーセット（`web` 有効）でインストール。Rust 1.85+ が必要。

### ソースから

```bash
git clone https://github.com/opaopa6969/syslenz.git
cd syslenz
cargo build --release
sudo cp target/release/syslenz /usr/local/bin/
```

またはクローンしたディレクトリから直接インストール:

```bash
cargo install --path .
```

### オプション機能

syslenz には 3 つのオプションコンパイル時フィーチャーがあります:

```bash
# Web UI（HTTP サーバー、設定 GUI — デフォルトビルドに含まれる）
cargo build --release --features web

# OpenTelemetry メトリクスエクスポート
cargo build --release --features otel

# X11 フローティングウィジェット
cargo build --release --features x11widget

# 全フィーチャー
cargo build --release --features "web,otel,x11widget"
```

### Docker

```bash
# インタラクティブ実行（コンテナ内 TUI）
docker run --rm -it --pid=host --privileged ghcr.io/opaopa6969/syslenz

# スナップショットを stdout にエクスポート
docker run --rm --pid=host --privileged ghcr.io/opaopa6969/syslenz --export /dev/stdout > snapshot.json

# TCP サーバーモード — ポート 9100 でリッスン（認証なし。信頼できるネットワークのみ）
docker run --rm -p 9100:9100 --pid=host ghcr.io/opaopa6969/syslenz --serve

# Web UI
docker compose --profile web up -d
# http://localhost:3000 を開く

# Grafana + Prometheus + syslenz
docker compose --profile grafana up -d
# http://localhost:3001 (Grafana)、http://localhost:9090 (Prometheus)
```

`--pid=host` と `--privileged` フラグにより、コンテナがホストの `/proc` ファイルシステムを読み取れるようになります。

### ビルド済みバイナリ

Linux (x86_64, aarch64, musl)、macOS (x86_64, aarch64)、Windows のビルド済みバイナリが [GitHub Releases](https://github.com/opaopa6969/syslenz/releases) ページにあります。

```bash
# Linux x86_64
curl -L https://github.com/opaopa6969/syslenz/releases/latest/download/syslenz-linux-x86_64.tar.gz | tar xz
sudo mv syslenz /usr/local/bin/

# チェックサム検証
sha256sum -c syslenz-linux-x86_64.tar.gz.sha256
```

---

## 初回起動

```bash
syslenz
```

**ダッシュボード**ビューが表示されます — 以下を含むシステムの全幅概要:

- ロードアベレージ（1、5、15 分）とスパークライン履歴
- メモリ使用量（合計、利用可能、キャッシュ）とバーグラフ
- CPU 使用率の内訳
- ネットワークインターフェーストラフィック
- ディスク使用量
- アクティブプロセスサマリー

ダッシュボードはデフォルトで毎秒自動更新されます。

### インポートモード（読み取り専用リプレイ）

```bash
syslenz --import snapshot.json
```

自動更新を無効にしてクラシックモードで開きます。別マシンまたは過去の時点でキャプチャしたスナップショットを検査するのに使用。

---

## 基本操作

syslenz は完全にキーボード駆動です。これらのキーは全ビューで機能します:

| キー | アクション |
|------|-----------|
| `j` / `k` または 下/上矢印 | 選択を下/上に移動 |
| `Enter` または 右矢印 | 選択項目にドリルイン |
| `Backspace` または 左矢印 | 戻る |
| `Tab` | サイドバーとコンテンツのフォーカス切替 |
| `PageUp` / `PageDown` | ページ単位でスクロール |
| `q` または `Esc` | 終了 |

### ダッシュボードで

- `j`/`k` でセクション（ロード、メモリ、CPU、ネットワーク等）を選択
- `Enter` でそのセクションの詳細ビューにドリルイン（クラシックモード）
- `Backspace` でダッシュボードに戻る

### クラシックモードで

- **サイドバー**が全データソース（`meminfo`、`loadavg`、`net/tcp` 等）を一覧表示
- `j`/`k` でソースを移動、`Enter` でフィールドを表示
- `Tab` でサイドバーと詳細パネルのフォーカスを切替
- 詳細パネルで `j`/`k` でフィールドをスクロール

---

## ビューの切り替え

| キー | ビュー | 説明 |
|------|--------|------|
| `D` | ダッシュボード | システムヘルス概要（起動時のデフォルト） |
| `O` | クラシック | サイドバー + 詳細パネル |
| `W` | ウェルカム | キーバインド一覧とヒント |
| `X` | 診断 | 自動検出された問題と推奨アクション |
| `C` | カテゴリガイド | Linux 内部構造の教育ガイド |

任意のビューから `d` を押すと **Diff** ビューに入ります（現在のスナップショットと過去のものを比較）。

---

## ヘルプの使い方

syslenz には組み込みの多段階ヘルプシステムがあります:

| キー | アクション |
|------|-----------|
| `?` | ヘルプレベルをサイクル: OFF → NORMAL → DETAILED → EXTRA → OFF |
| `L` | 言語切替（英語 ↔ 日本語） |

ヘルプが有効な場合、画面下部に現在のフィールドに関するコンテキスト情報のパネルが表示されます。EXTRA レベルでは SEE ALSO クロスリファレンスと学習ブレッドクラムが表示されます。

---

## メトリクス配信 (--serve)

`--serve` は `SNAPSHOT` リクエストに JSON で応答する軽量 TCP サーバーを起動します:

```bash
# 全インターフェースでサーバーを起動（デフォルトポート 9100）
syslenz --serve

# ループバックに限定 — 共有ホストで推奨
syslenz --serve 127.0.0.1:9100

# 別ターミナルやリモートマシンから接続
syslenz --connect localhost:9100
```

> **セキュリティ**: `--serve` には認証がありません。共有やインターネット公開ホストでは `127.0.0.1` にバインドするかファイアウォールで制限してください。SDK（`syslenz4j`、`syslenz4py`、`syslenz4node`）はこのエンドポイントに接続します。

---

## Web UI (--web)

`--web` はブラウザダッシュボードを持つ HTTP サーバーを起動します（`web` feature が必要、デフォルトで有効）:

```bash
# ポート 3000 で起動
syslenz --web 3000

# ブラウザで開く
# http://localhost:3000          — ライブダッシュボード
# http://localhost:3000/settings — 設定 GUI（アラートルールエディタ）
```

> **セキュリティ**: 現リリースの Web サーバーには認証機能がありません。ループバックのみで使用するか、ネットワーク公開時は TLS と認証を持つリバースプロキシを前段に置いてください。
>
> **計画中（未実装）**: Fleet View（`/fleet`）と認証（Basic Auth / Token）。

### 設定 GUI

`http://localhost:3000/settings` を開くとブラウザでアラートルールを編集できます。変更は `~/.config/syslenz/config.toml` に保存され、syslenz を再起動せずに即時反映されます。

---

## CLI フラグリファレンス

| フラグ | 引数 | 説明 |
|-------|------|------|
| `--export` | `<file.json>` | スナップショットを JSON にエクスポートして終了 |
| `--import` | `<file.json>` | インポートしたスナップショットで TUI を開く |
| `--export-series` | `<dir>` | タイムシリーズスナップショットをディレクトリにエクスポート |
| `--interval` | `<seconds>` | `--export-series` と `--otel` の間隔 |
| `--count` | `<n>` | `--export-series` のスナップショット数 |
| `--ssh` | `<user@host>` | SSH 経由リモートホスト監視（複数指定可） |
| `--docker` | `<container>` | Docker コンテナ監視 |
| `--connect` | `<host:port>` | syslenz TCP サーバーに接続 |
| `--serve` | `[bind_addr]` | TCP サーバー起動（デフォルト: `0.0.0.0:9100`） |
| `--web` | `[port]` | Web UI 起動（デフォルト: 3000、`web` 必要） |
| `--otel` | `[endpoint]` | OTLP エクスポート（デフォルト: `http://localhost:4317`、`otel` 必要） |
| `--prometheus` | `[port]` | Prometheus `/metrics` エンドポイント（デフォルト: 9101、`otel` 必要） |
| `--provider` | `<name>` | プロバイダーを名前で有効化（複数指定可） |
| `--widget` | — | X11 フローティングウィジェット（`x11widget` 必要） |
| `--lang` | `<en\|ja>` | UI 言語設定（設定ファイルを上書き） |
| `--classic` | — | ダッシュボードではなくクラシックモードで起動 |
| `--tutorial` | — | インタラクティブ 8 ステップチュートリアルを起動 |

### 使用例

```bash
# 基本的なインタラクティブ使用
syslenz

# 日本語インターフェース
syslenz --lang ja

# スナップショットをエクスポート
syslenz --export snapshot.json

# 60 スナップショットを 1 秒ごとにキャプチャ
syslenz --export-series ./data --interval 1 --count 60

# リモートサーバーを監視
syslenz --ssh admin@192.168.1.100

# 2 ホストを同時監視（F1/F2 でタブ切替）
syslenz --ssh admin@host1 --ssh admin@host2

# Docker コンテナを監視
syslenz --docker my-app-container

# ループバックのみで TCP サーバーを起動（共有ホストで安全）
syslenz --serve 127.0.0.1:9100

# リモート TCP サーバーに接続
syslenz --connect 192.168.1.100:9100

# ポート 8080 で Web UI を起動
syslenz --web 8080

# OTLP コレクターにメトリクスをエクスポート
syslenz --otel http://otel-collector:4317 --interval 10

# Prometheus メトリクスエンドポイント
syslenz --prometheus

# カスタムポートと MySQL プロバイダーで Prometheus
syslenz --prometheus 9102 --provider mysql

# チュートリアルモードを起動
syslenz --tutorial
```

---

[← Index](index.md) | [次: ダッシュボード →](dashboard.md)
