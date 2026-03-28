---
version: v1.1.0
lang: ja
---

# リモート監視

[🇬🇧 English](../en/remote.md)

[<- 前: 教育機能](education.md) | [Index](index.md) | [次: Web UI ->](web-ui.md)


## 目次

- [概要](#概要)
- [SSHモード](#sshモード)
- [Dockerモード](#dockerモード)
- [TCPサーバー/クライアントモード](#tcpサーバークライアントモード)
- [Docker Composeセットアップ](#docker-composeセットアップ)
- [トラブルシューティング](#トラブルシューティング)

## 概要

syslenz は3つのリモート監視方法に対応しています:

| 方法 | フラグ | ユースケース |
|------|--------|------------|
| SSH | `--ssh user@host` | SSHアクセスできるリモートサーバーの監視 |
| Docker | `--docker container` | `docker exec` 経由でのコンテナ監視 |
| TCP | `--serve` / `--connect` | SSHのないコンテナ用の軽量エージェント |

いずれもリモート側で syslenz を実行し、JSONスナップショットをローカルTUIにストリーミングする仕組みです。ローカルTUIはリモートシステムのデータを、ダッシュボード、クラシック、自動診断などフル機能で表示します。

## SSHモード

```bash
syslenz --ssh user@host
```

**仕組み:**
1. syslenz が `ssh -T -o BatchMode=yes -o ConnectTimeout=10 user@host syslenz --export -` を起動
2. リモートの syslenz がスナップショットをキャプチャし、JSONをstdoutに出力
3. ローカルの syslenz がJSONをデシリアライズしてTUIに表示
4. これを毎秒繰り返す（設定の `interval_ms` で変更可能）

**要件:**
- リモートホストに syslenz がインストールされ、`PATH` に含まれていること
- SSH鍵認証が設定されていること（BatchModeではパスワードプロンプトが無効）
- リモートホストに到達できること

**特徴:**
- ローカルのSSHエージェントと設定（`~/.ssh/config`）を引き継ぐ
- 一時的な障害に耐性あり: SSH失敗が5回連続するまではスキップし、それを超えると停止
- TUIタイトルバーにリモートホスト名を表示

**踏み台ホスト経由の例:**

`~/.ssh/config` を設定:

```
Host prod-server
    HostName 10.0.1.50
    User admin
    ProxyJump bastion.example.com
```

実行:

```bash
syslenz --ssh prod-server
```

## Dockerモード

```bash
syslenz --docker container_name
```

**仕組み:**
1. syslenz が `docker exec container_name syslenz --export -` を起動
2. コンテナ内の syslenz がコンテナの `/proc` からスナップショットをキャプチャ
3. JSONがローカルTUIにストリーミングされる

**要件:**
- コンテナ内に syslenz がインストールされていること
- ローカルの `PATH` に `docker` があること
- コンテナが実行中であること

**注意:** スナップショットはコンテナから見た `/proc` を反映するため、ホストとは異なる場合があります。ホストPID名前空間（`--pid=host`）のコンテナはホストレベルのデータを参照します。

**Dockerfileへのsyslenz追加例:**

```dockerfile
FROM ubuntu:24.04
# ... アプリのセットアップ ...

[🇬🇧 English](../en/remote.md)
COPY --from=syslenz/syslenz:latest /usr/local/bin/syslenz /usr/local/bin/syslenz
```

## TCPサーバー/クライアントモード

SSHが使えない環境（最小コンテナ、Kubernetesポッドなど）向けに、syslenzには軽量TCPプロトコルがあります。

### サーバー側

```bash
syslenz --serve [bind_addr]
```

デフォルトバインドアドレス: `0.0.0.0:9100`

サーバーはTCP接続をリッスンし、クライアントから `SNAPSHOT\n` を受信するとスナップショットをキャプチャしてJSONで返します。1接続につき1リクエストのシンプルなプロトコルです。

サーバーは同一スレッドで1接続ずつ処理するため、非常に軽量です（ランタイム依存なし、非同期処理なし）。

### クライアント側

```bash
syslenz --connect host:port
```

クライアントは毎秒TCPサーバーに接続して `SNAPSHOT\n` を送信し、JSONレスポンスを読み取ってローカルTUIに表示します。

**例:**

リモートマシンまたはコンテナで:

```bash
syslenz --serve 0.0.0.0:9100
```

ローカルマシンで:

```bash
syslenz --connect 192.168.1.100:9100
```

## Docker Composeセットアップ

アプリケーションコンテナの監視用の典型的なセットアップ:

```yaml
version: "3.8"

services:
  app:
    image: myapp:latest
    # ... アプリの設定 ...

  syslenz-agent:
    image: syslenz/syslenz:latest
    command: ["syslenz", "--serve", "0.0.0.0:9100"]
    pid: "host"
    privileged: true
    ports:
      - "9100:9100"
```

ワークステーションから:

```bash
syslenz --connect your-docker-host:9100
```

### 複数ホストの監視

異なるリモート対象に対して複数の syslenz インスタンスを実行できます。マルチホストビューはありませんが、複数のターミナルペインを使えます:

```bash
# ターミナル1

[🇬🇧 English](../en/remote.md)
syslenz --ssh web-server-1

# ターミナル2

[🇬🇧 English](../en/remote.md)
syslenz --ssh web-server-2

# ターミナル3

[🇬🇧 English](../en/remote.md)
syslenz --connect db-server:9100
```

## トラブルシューティング

### SSH: "syslenz not found"

リモートホストに syslenz がインストールされていないか、PATHに含まれていません。

**対処:** リモートホストに syslenz をインストール:

```bash
ssh user@host 'curl -L https://github.com/opaopa6969/syslenz/releases/latest/download/syslenz-linux-amd64 -o /usr/local/bin/syslenz && chmod +x /usr/local/bin/syslenz'
```

### SSH: "Permission denied"

SSH鍵認証が設定されていません。

**対処:** SSH鍵認証を設定:

```bash
ssh-keygen -t ed25519  # 鍵がない場合
ssh-copy-id user@host
```

### SSH: 接続タイムアウト

リモートホストに到達できないか、ファイアウォールがSSHをブロックしています。

**対処:** 手動で接続をテスト:

```bash
ssh -o ConnectTimeout=10 user@host echo ok
```

### Docker: "No such container"

コンテナ名が間違っているか、コンテナが実行されていません。

**対処:** 実行中のコンテナを確認:

```bash
docker ps --format '{{.Names}}'
```

### Docker: コンテナ内で "syslenz not found"

コンテナイメージに syslenz がインストールされていません。

**対処:** Dockerfileに syslenz を追加するか、実行時にインストール:

```bash
docker exec container_name sh -c 'curl -L <release-url> -o /usr/local/bin/syslenz && chmod +x /usr/local/bin/syslenz'
```

### TCP: "Connection refused"

syslenz サーバーが起動していない、ポートが間違っている、またはファイアウォールがブロックしています。

**対処:** サーバーがリッスンしていることを確認:

```bash
ss -tlnp | grep 9100
```

### リモートストリームが5回の失敗後に停止

3つのリモート方法全てに5回連続失敗のリトライ上限があります。5回失敗後、ストリームは停止し、TUIは最後に受信したスナップショットのまま止まります。

**対処:** ネットワーク接続を確認し、リモートの syslenz プロセスを再起動するか、ローカルの syslenz を再起動してください。

---

[<- 前: 教育機能](education.md) | [Index](index.md) | [次: Web UI ->](web-ui.md)
