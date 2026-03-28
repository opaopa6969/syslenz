---
version: v1.0.0
lang: ja
---

# リモート監視

[<- 前: 教育機能](education.md) | [Index](index.md) | [次: Web UI ->](web-ui.md)

[🇬🇧 English](../en/remote.md)

## 目次

- [概要](#概要)
- [SSHモード](#sshモード)
- [Dockerモード](#dockerモード)
- [TCPサーバー/クライアントモード](#tcpサーバークライアントモード)
- [Docker Composeセットアップ](#docker-composeセットアップ)
- [トラブルシューティング](#トラブルシューティング)

## 概要

syslenz は3つのリモート監視方法をサポートしています:

| 方法 | フラグ | ユースケース |
|------|--------|------------|
| SSH | `--ssh user@host` | SSHアクセスのあるリモートサーバーを監視 |
| Docker | `--docker container` | `docker exec` 経由でコンテナを監視 |
| TCP | `--serve` / `--connect` | SSHのないコンテナ用の軽量エージェント |

3つの方法全てが、リモートターゲットでsyslenzを実行し、JSONスナップショットをローカルTUIにストリーミングして動作します。ローカルTUIはリモートシステムのデータを完全なインタラクティビティ（ダッシュボード、クラシック、自動診断など）で表示します。

## SSHモード

```bash
syslenz --ssh user@host
```

**動作の仕組み:**
1. syslenz が `ssh -T -o BatchMode=yes -o ConnectTimeout=10 user@host syslenz --export -` を起動
2. リモートの syslenz がスナップショットをキャプチャし、JSONをstdoutに出力
3. ローカルの syslenz がJSONをデシリアライズしてTUIに表示
4. これを毎秒繰り返す（設定の `interval_ms` で変更可能）

**要件:**
- リモートホストに syslenz がインストールされ、`PATH` に含まれていること
- SSH鍵認証が設定されていること（BatchModeはパスワードプロンプトを無効化）
- リモートホストに到達可能であること

**特徴:**
- ローカルのSSHエージェントと設定（`~/.ssh/config`）を継承
- 一時的な障害に耐性: 5回連続のSSH失敗まではサイレントにスキップ、その後諦める
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

**動作の仕組み:**
1. syslenz が `docker exec container_name syslenz --export -` を起動
2. コンテナ内の syslenz がコンテナの `/proc` からスナップショットをキャプチャ
3. JSONがローカルTUIにストリーミング

**要件:**
- コンテナ内に syslenz がインストールされていること
- ローカルの `PATH` に `docker` があること
- コンテナが実行中であること

**注意:** スナップショットはコンテナから見た `/proc` を反映し、ホストとは異なる場合があります。ホストPID名前空間（`--pid=host`）のコンテナはホストレベルのデータを参照します。

**Dockerfileへのsyslenz追加例:**

```dockerfile
FROM ubuntu:24.04
# ... アプリのセットアップ ...
COPY --from=syslenz/syslenz:latest /usr/local/bin/syslenz /usr/local/bin/syslenz
```

## TCPサーバー/クライアントモード

SSHが利用できない環境（最小コンテナ、Kubernetesポッドなど）のために、syslenzには軽量TCPプロトコルが含まれています。

### サーバー側

```bash
syslenz --serve [bind_addr]
```

デフォルトバインドアドレス: `0.0.0.0:9100`

サーバーはTCP接続をリッスンします。クライアントが `SNAPSHOT\n` を送信すると、サーバーはスナップショットをキャプチャし、JSONとしてシリアライズして返送します。1接続につき1リクエスト（シンプルプロトコル）。

サーバーは同一スレッドで1接続ずつ処理するため、非常に軽量です（ランタイム依存関係なし、非同期処理なし）。

### クライアント側

```bash
syslenz --connect host:port
```

クライアントは毎秒TCPサーバーに接続し、`SNAPSHOT\n` を送信、JSONレスポンスを読み取り、ローカルTUIに表示します。

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

異なるリモートターゲットに対して複数のsyslenzインスタンスを実行できます。syslenzにはマルチホストビューはありませんが、複数のターミナルペインを使用できます:

```bash
# ターミナル1
syslenz --ssh web-server-1

# ターミナル2
syslenz --ssh web-server-2

# ターミナル3
syslenz --connect db-server:9100
```

## トラブルシューティング

### SSH: "syslenz not found"

リモートホストに syslenz がインストールされていないか、PATHに含まれていません。

**修正:** リモートホストにsyslenzをインストール:

```bash
ssh user@host 'curl -L https://github.com/opaopa6969/syslenz/releases/latest/download/syslenz-linux-amd64 -o /usr/local/bin/syslenz && chmod +x /usr/local/bin/syslenz'
```

### SSH: "Permission denied"

SSH鍵認証が設定されていません。

**修正:** SSH鍵認証を設定:

```bash
ssh-keygen -t ed25519  # 鍵がない場合
ssh-copy-id user@host
```

### SSH: 接続タイムアウト

リモートホストに到達できないか、ファイアウォールがSSHをブロックしています。

**修正:** 手動で接続をテスト:

```bash
ssh -o ConnectTimeout=10 user@host echo ok
```

### Docker: "No such container"

コンテナ名が間違っているか、コンテナが実行されていません。

**修正:** 実行中のコンテナを確認:

```bash
docker ps --format '{{.Names}}'
```

### Docker: コンテナ内で "syslenz not found"

コンテナイメージにsyslenzがインストールされていません。

**修正:** Dockerfileにsyslenzを追加するか、実行時にインストール:

```bash
docker exec container_name sh -c 'curl -L <release-url> -o /usr/local/bin/syslenz && chmod +x /usr/local/bin/syslenz'
```

### TCP: "Connection refused"

syslenzサーバーが実行されていない、ポートが間違っている、またはファイアウォールが接続をブロックしています。

**修正:** サーバーがリッスンしていることを確認:

```bash
ss -tlnp | grep 9100
```

### リモートストリームが5回の失敗後に停止

3つのリモート方法全てに5回連続失敗のリトライ制限があります。5回失敗後、ストリームは停止し、TUIは最後に受信したスナップショットで凍結します。

**修正:** ネットワーク接続を確認、リモートのsyslenzプロセスを再起動、またはローカルのsyslenzを再起動。

---

[<- 前: 教育機能](education.md) | [Index](index.md) | [次: Web UI ->](web-ui.md)
