---
version: v1.1.0
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

syslenz には3つのオプションのコンパイル時機能があります:

```bash
# Web UI (バイナリサイズ約3MB増、tokio + axum が必要)

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

コンテナがホストの `/proc` を読み取れるよう、`--pid=host` と `--privileged` フラグが必要です。

リポジトリには最小コンテナイメージをビルドする `Dockerfile` と、クイックセットアップ用の `docker-compose.yml` が含まれています:

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

Web UI をワンステップでビルド・起動する便利スクリプト `run-web.sh` も用意されています:

```bash
./run-web.sh          # ポート3000、英語
./run-web.sh 8080     # ポート8080、英語
./run-web.sh 3000 ja  # ポート3000、日本語
```

Docker Compose の詳しい設定は[リモート監視](remote.md)ページ、ブラウザからのアクセスは[Web UI](web-ui.md)ページを参照してください。

### バイナリダウンロード

`x86_64-unknown-linux-gnu` 向けのビルド済みバイナリは [GitHub Releases](https://github.com/opaopa6969/syslenz/releases) ページから入手できます。

```bash
curl -L https://github.com/opaopa6969/syslenz/releases/latest/download/syslenz-linux-amd64 -o syslenz
chmod +x syslenz
sudo mv syslenz /usr/local/bin/
```

## 初回起動

次のコマンドを実行するだけです:

```bash
syslenz
```

**ダッシュボード**が開きます。システムの状態を一画面にまとめた全画面ビューです:

- ロードアベレージ (1, 5, 15 分)
- メモリ使用量 (合計、利用可能、キャッシュ)
- CPU 使用率の内訳
- ネットワークインターフェースのトラフィック
- ディスク使用量
- プロセスの概要

ダッシュボードはデフォルトで毎秒自動リフレッシュします。

### インポートモード (読み取り専用)

以前キャプチャしたスナップショットを閲覧できます:

```bash
syslenz --import snapshot.json
```

自動リフレッシュ無効のクラシック（Overview）モードで開き、別のマシンや過去の時点のスナップショットを確認できます。

## 基本操作

syslenz は完全にキーボードで操作します。主要なナビゲーションキーは全ビュー共通です:

| キー | アクション |
|------|----------|
| `j` / `k` または 矢印キー上下 | 選択を上下に移動 |
| `Enter` または 矢印キー右 | 選択項目にドリルイン |
| `Backspace` または 矢印キー左 | 前のビューに戻る |
| `Tab` | サイドバーとコンテンツのフォーカス切り替え |
| `PageUp` / `PageDown` | ページ単位でスクロール |
| `q` または `Esc` | syslenz を終了 |

### ダッシュボードでの操作

- `j`/`k` でダッシュボードセクション（ロード、メモリ、CPU、ネットワーク等）を選択
- `Enter` でそのセクションの詳細をクラシックモードで表示

### クラシックモードでの操作

- **サイドバー**に全データソースを一覧表示（例: `meminfo`、`loadavg`、`net/tcp`）
- `j`/`k` でソースを選択、`Enter` で詳細表示
- `Tab` でサイドバーと詳細パネルのフォーカスを切り替え
- 詳細パネルでは `j`/`k` でフィールドをスクロール

## ビューの切り替え

syslenz には複数のビューがあり、キー1つで切り替えられます:

| キー | ビュー | 説明 |
|------|--------|------|
| `D` | ダッシュボード | システムの概要（デフォルト） |
| `O` | Overview（クラシック） | サイドバー＋詳細パネル |
| `W` | ウェルカム | クイックスタート情報 |
| `X` | 自動診断 | 自動診断の結果 |
| `C` | カテゴリガイド | Linux内部の教育ガイド |

いつでもビューを切り替えられます。元のビューは記憶されており、`Backspace` で戻れます。

## ヘルプの使い方

syslenz には複数レベルのヘルプシステムが組み込まれています:

| キー | アクション |
|------|----------|
| `?` | ヘルプレベルを切り替え: OFF -> NORMAL -> DETAILED -> EXTRA -> OFF |
| `L` | 言語切り替え (英語 <-> 日本語) |

ヘルプ有効時は画面下部にパネルが表示され、現在のビューと選択項目に応じた情報が表示されます。ヘルプレベルが高いほど、フィールドの説明や使い方のヒントなど、より詳しい情報を確認できます。

## CLIフラグリファレンス

| フラグ | 引数 | 説明 |
|--------|------|------|
| `--export` | `<file.json>` | スナップショットをJSONにエクスポートして終了 |
| `--import` | `<file.json>` | スナップショットを読み込んでTUIを開く |
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
| `--lang` | `<en\|ja>` | 言語を設定（設定ファイルより優先） |
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
