---
version: v1.0.0
lang: ja
---

# Web UI

[<- 前: リモート監視](remote.md) | [Index](index.md) | [次: プラグイン ->](plugins.md)

[🇬🇧 English](../en/web-ui.md)

## 目次

- [概要](#概要)
- [Web UIの起動](#web-uiの起動)
- [Webインターフェース](#webインターフェース)
- [リアルタイムストリーミング](#リアルタイムストリーミング)
- [APIエンドポイント](#apiエンドポイント)
- [言語サポート](#言語サポート)
- [デプロイメントノート](#デプロイメントノート)

## 概要

syslenz にはAxumベースのオプションブラウザベースWeb UIが含まれています。Webブラウザを持つ任意のデバイスからアクセス可能なライブ更新ダッシュボードを提供します。Web UIはコンパイル時に `web` 機能を有効にする必要があります。

## Web UIの起動

### run-web.sh でクイックスタート

Web UI を最も簡単に起動する方法は、リポジトリルートにある `run-web.sh` 便利スクリプトです。必要に応じて `web` 機能付きでビルドし、サーバーを起動します:

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
syslenz --web

# カスタムポート
syslenz --web 8080

# 日本語ロケール
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

Web UIはTUIに似たダッシュボードを持つシングルページアプリケーションを提供します:

- 1/5/15分ロードアベレージ付きの**システムロード**
- 合計、利用可能、キャッシュ内訳付きの**メモリ使用量**
- パーセンテージ内訳の**CPU使用率**
- インターフェースごとのRX/TXの**ネットワークインターフェーストラフィック**
- ルートファイルシステムの**ディスク使用量**
- 状態別カウント付きの**プロセスサマリー**

ページはServer-Sent Events (SSE) を介して毎秒自動更新されます -- 手動リフレッシュは不要です。

## リアルタイムストリーミング

Web UIはリアルタイム更新にSSE (Server-Sent Events) を使用します:

1. サーバーがバックグラウンドタスクで毎秒スナップショットをキャプチャ
2. 各スナップショットはJSONにシリアライズされ、接続中の全SSEクライアントにブロードキャスト
3. ブラウザはイベントを受信し、ページ全体のリロードなしで更新

接続を維持するために10秒ごとにkeep-alive pingが送信されます。

サーバーはチャート用に60スナップショットのヒストリーリングバッファも維持します。

## APIエンドポイント

Web UIはブラウザインターフェースとは独立して使用できるREST APIを公開します:

### `GET /`

HTMLダッシュボードページを返します。

### `GET /api/snapshot`

現在のスナップショットをJSONとして返します。

```bash
curl http://localhost:3000/api/snapshot | jq .
```

**レスポンス:** 全エントリとフィールドを含む完全な `Snapshot` オブジェクト。

### `GET /api/history`

ヒストリーバッファ（最大60スナップショット）をJSON配列として返します。

```bash
curl http://localhost:3000/api/history | jq 'length'
```

### `GET /api/sources`

全データソース名のリストをJSON配列として返します。

```bash
curl http://localhost:3000/api/sources | jq .
```

**レスポンス例:**

```json
["buddyinfo", "cgroups", "cmdline", "conntrack", "cpuinfo", "df", ...]
```

### `GET /api/stream`

リアルタイムスナップショットストリーミング用のSSEエンドポイント。各イベントはJSONとして完全なスナップショットを `data` フィールドに含みます。

```bash
curl -N http://localhost:3000/api/stream
```

### `GET /api/view`

構造化されたビュー表現を返します。クエリパラメータを受け付けます:

| パラメータ | 値 | デフォルト |
|-----------|-----|---------|
| `view` | `dashboard`, `welcome`, `detail`, `diff`, `table`, `graph`, `diagnostics`, `category` | `dashboard` |
| `locale` | `en`, `ja` | サーバーのロケール |

```bash
curl 'http://localhost:3000/api/view?view=diagnostics&locale=ja'
```

## 言語サポート

Web UIは `--lang` フラグを尊重します:

```bash
syslenz --web --lang ja
```

個別のAPIリクエストは `/api/view` エンドポイントの `locale` クエリパラメータで言語をオーバーライドすることもできます。

## デプロイメントノート

### リバースプロキシの背後での実行

Web UIはnginxなどのリバースプロキシの背後で動作します。SSE接続がバッファリングされないようにしてください:

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

### Dockerでの実行

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

### セキュリティに関する考慮事項

- Web UIには認証がありません。インターネットに公開する場合は、認証付きリバースプロキシを使用してください。
- APIは詳細なシステム情報を公開します。信頼できるネットワークにアクセスを制限してください。
- APIコンシューマー向けに `tower-http` を介してCORSが有効化されています。

---

[<- 前: リモート監視](remote.md) | [Index](index.md) | [次: プラグイン ->](plugins.md)
