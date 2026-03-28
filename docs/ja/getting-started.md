---
version: v1.0.0
lang: ja
---

# はじめに

[🇬🇧 English](../en/getting-started.md)

[<- 前: Index](index.md) | [Index](index.md) | [次: ダッシュボード ->](dashboard.md)


## 目次

- [インストール](#インストール)
- [初回起動](#初回起動)
- [基本操作](#基本操作)
- [ビューの切り替え](#ビューの切り替え)
- [ヘルプの使い方](#ヘルプの使い方)
- [CLIフラグリファレンス](#cliフラグリファレンス)

## インストール

### ソースから (Cargo)

Rust 2024 エディション (1.85+) が必要です。

```bash
git clone https://github.com/opaopa6969/syslenz.git
cd syslenz
cargo build --release
sudo cp target/release/syslenz /usr/local/bin/
```

または直接インストール:

```bash
cargo install --path .
```

### オプション機能

syslenz には3つのオプションコンパイル時機能があります:

```bash
# Web UI (バイナリサイズ約3MB増加、tokio + axum が必要)

[🇬🇧 English](../en/getting-started.md)
cargo build --release --features web

# OpenTelemetry メトリクスエクスポート (tokio + OTLP クレートが必要)

[🇬🇧 English](../en/getting-started.md)
cargo build --release --features otel

# X11 フローティングウィジェット

[🇬🇧 English](../en/getting-started.md)
cargo build --release --features x11widget

# 全機能

[🇬🇧 English](../en/getting-started.md)
cargo build --release --features "web,otel,x11widget"
```

### Docker

```bash
# 対話的に実行

[🇬🇧 English](../en/getting-started.md)
docker run --rm -it --pid=host --privileged syslenz/syslenz

# スナップショットをエクスポート

[🇬🇧 English](../en/getting-started.md)
docker run --rm --pid=host --privileged syslenz/syslenz --export /dev/stdout > snapshot.json
```

コンテナがホストの `/proc` ファイルシステムを読み取れるように、`--pid=host` と `--privileged` フラグが必要です。

リポジトリには最小コンテナイメージをビルドするための `Dockerfile` と、クイックセットアップ用の `docker-compose.yml` が含まれています:

```bash
# Docker Compose でビルド・実行 (TCPサーバーモード)

[🇬🇧 English](../en/getting-started.md)
docker compose up -d
syslenz --connect localhost:9100

# Web UI プロファイル

[🇬🇧 English](../en/getting-started.md)
docker compose --profile web up -d
# http://localhost:3000 を開く

[🇬🇧 English](../en/getting-started.md)
```

Web UI をワンステップでビルド・起動する便利スクリプト `run-web.sh` も提供されています:

```bash
./run-web.sh          # ポート3000、英語
./run-web.sh 8080     # ポート8080、英語
./run-web.sh 3000 ja  # ポート3000、日本語
```

詳細な Docker Compose 設定は[リモート監視](remote.md)ページ、ブラウザベースアクセスは[Web UI](web-ui.md)ページを参照してください。

### バイナリダウンロード

`x86_64-unknown-linux-gnu` 向けのビルド済みバイナリは [GitHub Releases](https://github.com/opaopa6969/syslenz/releases) ページから入手できます。

```bash
curl -L https://github.com/opaopa6969/syslenz/releases/latest/download/syslenz-linux-amd64 -o syslenz
chmod +x syslenz
sudo mv syslenz /usr/local/bin/
```

## 初回起動

以下のコマンドを実行するだけです:

```bash
syslenz
```

**ダッシュボード**表示が開きます。システムの健全性の全体像を表示する全画面ビューです:

- システムロードアベレージ (1, 5, 15 分)
- メモリ使用量 (合計、利用可能、キャッシュ)
- CPU 使用率の内訳
- ネットワークインターフェーストラフィック
- ディスク使用量
- アクティブプロセスの概要

ダッシュボードはデフォルトで毎秒自動リフレッシュされます。

### インポートモード (読み取り専用)

以前にキャプチャしたスナップショットを閲覧することもできます:

```bash
syslenz --import snapshot.json
```

自動リフレッシュが無効のクラシック（Overview）モードで開き、別のマシンや過去の時点のスナップショットを検査できます。

## 基本操作

syslenz は完全にキーボード駆動です。コアナビゲーションキーは全ビューで動作します:

| キー | アクション |
|------|----------|
| `j` / `k` または 矢印キー上下 | 選択を上下に移動 |
| `Enter` または 矢印キー右 | 選択項目にドリルイン |
| `Backspace` または 矢印キー左 | 前のビューに戻る |
| `Tab` | サイドバーとコンテンツのフォーカスを切り替え |
| `PageUp` / `PageDown` | ページ単位でスクロール |
| `q` または `Esc` | syslenz を終了 |

### ダッシュボードでの操作

- `j`/`k` でダッシュボードセクション（ロード、メモリ、CPU、ネットワーク等）を選択
- `Enter` でそのセクションの詳細ビューにクラシックモードでドリルイン

### クラシックモードでの操作

- **サイドバー**に全データソースが一覧表示（例: `meminfo`、`loadavg`、`net/tcp`）
- `j`/`k` でソースを選択、`Enter` で詳細表示
- `Tab` でサイドバーと詳細パネルのフォーカスを切り替え
- 詳細パネルでは `j`/`k` でフィールドをスクロール

## ビューの切り替え

syslenz には単一キーショートカットでアクセスできる複数のビューがあります:

| キー | ビュー | 説明 |
|------|--------|------|
| `D` | ダッシュボード | システム健全性の概要（デフォルト） |
| `O` | Overview（クラシック） | サイドバー＋詳細パネル |
| `W` | ウェルカム | クイックスタート情報のウェルカム画面 |
| `X` | 自動診断 | 自動診断結果 |
| `C` | カテゴリガイド | Linux内部の教育ガイド |

いつでもビューを切り替えられます。元のビューは記憶されており、`Backspace` で戻れます。

## ヘルプの使い方

syslenz には複数レベルの組み込みヘルプシステムがあります:

| キー | アクション |
|------|----------|
| `?` | ヘルプレベルを切り替え: OFF -> NORMAL -> DETAILED -> EXTRA -> OFF |
| `L` | 言語切り替え (英語 <-> 日本語) |

ヘルプが有効な場合、画面下部にパネルが表示され、現在のビューと選択項目に関する文脈情報が表示されます。高いヘルプレベルほど、フィールドの説明や使用法のヒントなど、より詳細な情報が提供されます。

## CLIフラグリファレンス

| フラグ | 引数 | 説明 |
|--------|------|------|
| `--export` | `<file.json>` | スナップショットをJSONにエクスポートして終了 |
| `--import` | `<file.json>` | インポートしたスナップショットでTUIを開く |
| `--export-series` | `<dir>` | 時系列スナップショットをディレクトリにエクスポート |
| `--interval` | `<seconds>` | `--export-series` と `--otel` のインターバル |
| `--count` | `<n>` | `--export-series` のスナップショット数 |
| `--ssh` | `<user@host>` | SSH経由でリモートホストを監視 |
| `--docker` | `<container>` | Docker exec経由でコンテナを監視 |
| `--connect` | `<host:port>` | syslenz TCPサーバーに接続 |
| `--serve` | `[bind_addr]` | TCPサーバーを起動（デフォルト: `0.0.0.0:9100`） |
| `--web` | `[port]` | Web UIを起動（デフォルトポート: `3000`、`web`機能が必要） |
| `--otel` | `[endpoint]` | OTLP経由でメトリクスをエクスポート（デフォルト: `http://localhost:4317`、`otel`機能が必要） |
| `--widget` | | X11フローティングウィジェットを起動（`x11widget`機能が必要） |
| `--lang` | `<en\|ja>` | 言語を設定（設定ファイルを上書き） |
| `--classic` | | ダッシュボードの代わりにクラシックモードで起動 |

### 使用例

```bash
# 基本的な対話的使用

[🇬🇧 English](../en/getting-started.md)
syslenz

# スナップショットのエクスポート

[🇬🇧 English](../en/getting-started.md)
syslenz --export snapshot.json

# 60スナップショットを1秒間隔でキャプチャ

[🇬🇧 English](../en/getting-started.md)
syslenz --export-series ./data --interval 1 --count 60

# リモートサーバーの監視

[🇬🇧 English](../en/getting-started.md)
syslenz --ssh admin@192.168.1.100

# Dockerコンテナの監視

[🇬🇧 English](../en/getting-started.md)
syslenz --docker my-app-container

# コンテナ内でTCPサーバーを起動

[🇬🇧 English](../en/getting-started.md)
syslenz --serve 0.0.0.0:9100

# リモートTCPサーバーに接続

[🇬🇧 English](../en/getting-started.md)
syslenz --connect 192.168.1.100:9100

# Web UIをポート8080で起動

[🇬🇧 English](../en/getting-started.md)
syslenz --web 8080

# OTLPコレクターにメトリクスをエクスポート

[🇬🇧 English](../en/getting-started.md)
syslenz --otel http://otel-collector:4317 --interval 10

# 日本語インターフェース

[🇬🇧 English](../en/getting-started.md)
syslenz --lang ja
```

---

[<- 前: Index](index.md) | [Index](index.md) | [次: ダッシュボード ->](dashboard.md)
