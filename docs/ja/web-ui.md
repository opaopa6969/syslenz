---
version: v1.1.0
lang: ja
---

# Web UI

[🇬🇧 English](../en/web-ui.md)

[<- 前: リモート監視](remote.md) | [Index](index.md) | [次: プラグイン ->](plugins.md)


## 目次

- [概要](#概要)
- [Web UIの起動](#web-uiの起動)
- [Webインターフェース](#webインターフェース)
- [リアルタイムストリーミング](#リアルタイムストリーミング)
- [APIエンドポイント](#apiエンドポイント)
- [言語サポート](#言語サポート)
- [デプロイメントノート](#デプロイメントノート)

## 概要

syslenz にはAxumベースのブラウザ向けWeb UI（オプション）が含まれています。Webブラウザがあればどのデバイスからでもアクセスできるライブ更新ダッシュボードです。コンパイル時に `web` 機能を有効にする必要があります。

## Web UIの起動

### run-web.sh でクイックスタート

最も簡単な起動方法は、リポジトリルートの `run-web.sh` スクリプトです。必要に応じて `web` 機能付きでビルドし、サーバーを起動します:

```bash
./run-web.sh          # ポート3000、英語
./run-web.sh 8080     # ポート8080、英語
./run-web.sh 3000 ja  # ポート3000、日本語
```

### web機能付きでコンパイル

```bash
cargo build --release --features web
```

### サーバーの起動

```bash
# デフォルトポート (3000)

[🇬🇧 English](../en/web-ui.md)
syslenz --web

# カスタムポート

[🇬🇧 English](../en/web-ui.md)
syslenz --web 8080

# 日本語ロケール

[🇬🇧 English](../en/web-ui.md)
syslenz --web 8080 --lang ja
```

サーバーは `0.0.0.0` にバインドし、URLを表示します:

```
syslenz web UI: http://localhost:3000
```

このURLをブラウザで開いてください。

### web機能がコンパイルされていない場合

`web` 機能なしで `syslenz --web` を実行すると:

```
Web UI support is not compiled in. Rebuild with: cargo build --features web
```

## Webインターフェース

Web UIはTUIに似たダッシュボードを持つシングルページアプリです:

- 1/5/15分ロードアベレージ付きの**システムロード**
- 合計、利用可能、キャッシュ内訳付きの**メモリ使用量**
- パーセンテージ内訳の**CPU使用率**
- インターフェースごとRX/TXの**ネットワークトラフィック**
- ルートファイルシステムの**ディスク使用量**
- 状態別カウント付きの**プロセスサマリー**

ページはServer-Sent Events (SSE) で毎秒自動更新されるため、手動リフレッシュは不要です。

## リアルタイムストリーミング

Web UIはリアルタイム更新にSSE (Server-Sent Events) を使います:

1. サーバーがバックグラウンドタスクで毎秒スナップショットをキャプチャ
2. 各スナップショットをJSONにシリアライズし、接続中の全SSEクライアントにブロードキャスト
3. ブラウザがイベントを受信し、ページ全体のリロードなしで更新

接続維持のため10秒ごとにkeep-alive pingを送信します。

サーバーはチャート用に60スナップショットの履歴リングバッファも保持します。

## APIエンドポイント

Web UIはブラウザインターフェースとは独立して使えるREST APIを公開します:

### `GET /`

HTMLダッシュボードページを返します。

### `GET /api/snapshot`

現在のスナップショットをJSONで返します。

```bash
curl http://localhost:3000/api/snapshot | jq .
```

**レスポンス:** 全エントリとフィールドを含む完全な `Snapshot` オブジェクト。

### `GET /api/history`

履歴バッファ（最大60スナップショット）をJSON配列で返します。

```bash
curl http://localhost:3000/api/history | jq 'length'
```

### `GET /api/sources`

全データソース名のリストをJSON配列で返します。

```bash
curl http://localhost:3000/api/sources | jq .
```

**レスポンス例:**

```json
["buddyinfo", "cgroups", "cmdline", "conntrack", "cpuinfo", "df", ...]
```

### `GET /api/stream`

リアルタイムスナップショットストリーミング用のSSEエンドポイント。各イベントの `data` フィールドに完全なスナップショットがJSON形式で入ります。

```bash
curl -N http://localhost:3000/api/stream
```

### `GET /api/view`

構造化されたビュー表現を返します。クエリパラメータ:

| パラメータ | 値 | デフォルト |
|-----------|-----|---------|
| `view` | `dashboard`, `welcome`, `detail`, `diff`, `table`, `graph`, `diagnostics`, `category` | `dashboard` |
| `locale` | `en`, `ja` | サーバーのロケール |

```bash
curl 'http://localhost:3000/api/view?view=diagnostics&locale=ja'
```

## 言語サポート

Web UIは `--lang` フラグに従います:

```bash
syslenz --web --lang ja
```

個別のAPIリクエストでは `/api/view` エンドポイントの `locale` クエリパラメータで言語を上書きできます。

## デプロイメントノート

### リバースプロキシの背後で実行

Web UIはnginxなどのリバースプロキシの背後で動作します。SSE接続がバッファリングされないよう注意してください:

```nginx
location /api/stream {
    proxy_pass http://localhost:3000;
    proxy_http_version 1.1;
    proxy_set_header Connection '';
    proxy_buffering off;
    proxy_cache off;
}

location / {
    proxy_pass http://localhost:3000;
}
```

### Dockerで実行

```dockerfile
FROM rust:1.85 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --features web

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/syslenz /usr/local/bin/
EXPOSE 3000
CMD ["syslenz", "--web"]
```

```bash
docker build -t syslenz-web .
docker run --rm --pid=host --privileged -p 3000:3000 syslenz-web
```

### セキュリティに関する注意

- Web UIには認証がありません。インターネットに公開する場合は、認証付きリバースプロキシを使ってください。
- APIは詳細なシステム情報を公開します。信頼できるネットワークにアクセスを限定してください。
- APIコンシューマー向けに `tower-http` 経由でCORSが有効になっています。

---

[<- 前: リモート監視](remote.md) | [Index](index.md) | [次: プラグイン ->](plugins.md)
