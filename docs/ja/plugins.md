---
version: v1.3.0
lang: ja
---

# プラグインシステム

[🇬🇧 English](../en/plugins.md)

[<- 前: Web UI](web-ui.md) | [Index](index.md) | [次: 設定 ->](config.md)


## 目次

- [概要](#概要)
- [プラグインプロトコル](#プラグインプロトコル)
- [プラグインディレクトリ](#プラグインディレクトリ)
- [プラグインの作成手順](#プラグインの作成手順)
- [例: JVMメモリプラグイン](#例-jvmメモリプラグイン)
- [例: Docker統計プラグイン](#例-docker統計プラグイン)
- [プラグインの検出と実行](#プラグインの検出と実行)
- [プラグインのデバッグ](#プラグインのデバッグ)
- [Provider (v1.3.0)](#provider-v130)
  - [Provider テンプレート](#provider-テンプレート)
  - [MySQL provider](#mysql-provider)
  - [PostgreSQL provider](#postgresql-provider)
  - [Redis provider](#redis-provider)
  - [nginx provider](#nginx-provider)
  - [5分で Provider を書く方法](#5分で-provider-を書く方法)

## 概要

syslenz は実行可能プラグインでカスタムデータソースを追加できます。プラグインは `ProcEntry` JSON オブジェクトをstdoutに出力する実行可能ファイルで、プラグインディレクトリに配置します。サイドバーに `plugin/` プレフィックス付きで表示され、全ビュー（ダッシュボード、クラシック、自動診断など）にシームレスに統合されます。

## プラグインプロトコル

プラグインは以下を満たす必要があります:

1. 実行可能ファイルであること（言語は問わない: シェル、Python、Rust、Go など）
2. 引数を受け取らないこと（stdinは `/dev/null`）
3. `ProcEntry` スキーマに準拠した単一のJSONオブジェクトをstdoutに出力
4. 成功時にステータス0で終了

### ProcEntry JSONスキーマ

```json
{
  "source": "/custom/jvm-memory",
  "fields": [
    {
      "name": "HeapUsed",
      "value": { "Bytes": 536870912 },
      "unit": "bytes",
      "description": "現在のJVMヒープメモリ使用量"
    },
    {
      "name": "HeapMax",
      "value": { "Bytes": 1073741824 },
      "unit": "bytes",
      "description": "JVMヒープ最大サイズ"
    },
    {
      "name": "GCCount",
      "value": { "Integer": 42 },
      "unit": null,
      "description": "ガベージコレクション合計回数"
    }
  ]
}
```

### FieldValue型

`value` フィールドは以下のタグ付きenumバリアントのいずれかです:

| バリアント | JSONフォーマット | 例 |
|-----------|----------------|-----|
| Bytes | `{"Bytes": 1024}` | メモリ、ディスクサイズ |
| Integer | `{"Integer": 42}` | カウンター、数量 |
| Float | `{"Float": 3.14}` | パーセンテージ、比率 |
| Text | `{"Text": "hello"}` | 文字列、ステータス値 |
| Duration | `{"Duration": 86400.0}` | 秒単位の時間 |
| Table | `{"Table": [["col1","col2"],["a","b"]]}` | 表形式データ |

### タイムアウト

プラグインは**5秒以内**に完了する必要があります。超過するとプラグインはkillされ、エラーメッセージがstderrに出力されてスキップされます。

### 終了コード

- 終了コード0: 成功。stdoutをJSONとしてパース。
- 非ゼロ終了: スキップ。エラーをstderrに出力。

## プラグインディレクトリ

プラグインは以下に配置します:

```
~/.config/syslenz/plugins/
```

`$XDG_CONFIG_HOME` が設定されている場合:

```
$XDG_CONFIG_HOME/syslenz/plugins/
```

syslenz はこのディレクトリが存在しなければ自動作成します。

**重要:** Unixシステムでは、プラグインファイルに実行権限が必要です（`chmod +x`）。実行権限のないファイルはスキップされます。

## プラグインの作成手順

### ステップ1: プラグインファイルを作成

```bash
mkdir -p ~/.config/syslenz/plugins
touch ~/.config/syslenz/plugins/my-plugin
chmod +x ~/.config/syslenz/plugins/my-plugin
```

### ステップ2: プラグインロジックを記述

最もシンプルなプラグイン (bash):

```bash
#!/bin/bash
cat <<'EOF'
{
  "source": "/custom/my-plugin",
  "fields": [
    {
      "name": "example_field",
      "value": {"Integer": 42},
      "unit": null,
      "description": "サンプルフィールド"
    }
  ]
}
EOF
```

### ステップ3: プラグインをテスト

```bash
~/.config/syslenz/plugins/my-plugin | python3 -m json.tool
```

出力が ProcEntry スキーマに合致する有効なJSONであることを確認します。

### ステップ4: syslenzを実行

```bash
syslenz
```

プラグインはサイドバーに `plugin/my-plugin` として表示されます。

## 例: JVMメモリプラグイン

`jcmd` でJVMメトリクスを読み取るPythonプラグイン:

```python
#!/usr/bin/env python3
"""syslenz プラグイン: 最初の実行中Javaプロセスのヒープメモリ。"""
import json
import subprocess
import sys

def main():
    # 最初のJava PIDを検索
    result = subprocess.run(['pgrep', '-f', 'java'], capture_output=True, text=True)
    if result.returncode != 0:
        print(json.dumps({
            "source": "/custom/jvm-memory",
            "fields": [
                {"name": "status", "value": {"Text": "Javaプロセスが見つかりません"},
                 "unit": None, "description": "プラグインステータス"}
            ]
        }))
        return

    pid = result.stdout.strip().split('\n')[0]

    # jcmd でヒープ情報を取得
    result = subprocess.run(
        ['jcmd', pid, 'GC.heap_info'],
        capture_output=True, text=True, timeout=3
    )

    fields = []
    for line in result.stdout.split('\n'):
        if 'used' in line.lower():
            parts = line.split()
            for i, p in enumerate(parts):
                if p == 'used' and i + 1 < len(parts):
                    try:
                        used = int(parts[i + 1].rstrip(','))
                        fields.append({
                            "name": "HeapUsed",
                            "value": {"Bytes": used},
                            "unit": "bytes",
                            "description": "現在のJVMヒープメモリ使用量"
                        })
                    except ValueError:
                        pass

    if not fields:
        fields = [{"name": "status", "value": {"Text": "ヒープ情報をパースできませんでした"},
                   "unit": None, "description": "プラグインステータス"}]

    print(json.dumps({"source": "/custom/jvm-memory", "fields": fields}))

if __name__ == '__main__':
    main()
```

`~/.config/syslenz/plugins/jvm-memory` として保存し、`chmod +x` を実行。

## 例: Docker統計プラグイン

Dockerコンテナのリソース使用量を収集するシェルプラグイン:

```bash
#!/bin/bash
# syslenz プラグイン: Dockerコンテナ統計サマリー

[🇬🇧 English](../en/plugins.md)

if ! command -v docker &>/dev/null; then
    echo '{"source":"/custom/docker-stats","fields":[{"name":"status","value":{"Text":"docker が見つかりません"},"unit":null,"description":"プラグインステータス"}]}'
    exit 0
fi

# コンテナ統計を取得 (no-stream で単一スナップショット)

[🇬🇧 English](../en/plugins.md)
stats=$(docker stats --no-stream --format '{{.Name}}\t{{.CPUPerc}}\t{{.MemUsage}}\t{{.NetIO}}' 2>/dev/null)

if [ -z "$stats" ]; then
    echo '{"source":"/custom/docker-stats","fields":[{"name":"status","value":{"Text":"実行中のコンテナなし"},"unit":null,"description":"プラグインステータス"}]}'
    exit 0
fi

# テーブル行を構築

[🇬🇧 English](../en/plugins.md)
rows="["
first=true
while IFS=$'\t' read -r name cpu mem net; do
    if [ "$first" = true ]; then
        first=false
    else
        rows+=","
    fi
    rows+="[\"$name\",\"$cpu\",\"$mem\",\"$net\"]"
done <<< "$stats"
rows+="]"

count=$(echo "$stats" | wc -l)

cat <<EOF
{
  "source": "/custom/docker-stats",
  "fields": [
    {
      "name": "container_count",
      "value": {"Integer": $count},
      "unit": null,
      "description": "実行中のDockerコンテナ数"
    },
    {
      "name": "containers",
      "value": {"Table": $rows},
      "unit": null,
      "description": "コンテナリソース使用量 (名前, CPU%, メモリ, ネットI/O)"
    }
  ]
}
EOF
```

`~/.config/syslenz/plugins/docker-stats` として保存し、`chmod +x` を実行。

## プラグインの検出と実行

syslenz は起動時（および各リフレッシュ時）に以下を行います:

1. `~/.config/syslenz/plugins/` 内の全ファイルを読み取り
2. ファイル以外（ディレクトリ、ディレクトリへのシンボリックリンク）をスキップ
3. 実行権限のないファイルをスキップ（Unixの場合）
4. 残りの各ファイルを実行:
   - stdin: `/dev/null`
   - stdout: パイプ（キャプチャ）
   - stderr: パイプ（キャプチャ、失敗時に表示）
   - タイムアウト: 5秒
5. stdoutを `ProcEntry` JSONとしてパース
6. エントリを `plugin/<拡張子なしファイル名>` として挿入

プラグインはメインの `/proc` パースと並行して実行されます。失敗したプラグインはstderrにエラーを出力してスキップされます。

## プラグインのデバッグ

### 手動テスト

```bash
# プラグインを直接実行

[🇬🇧 English](../en/plugins.md)
~/.config/syslenz/plugins/my-plugin

# JSONを検証

[🇬🇧 English](../en/plugins.md)
~/.config/syslenz/plugins/my-plugin | python3 -m json.tool

# 終了コードを確認

[🇬🇧 English](../en/plugins.md)
~/.config/syslenz/plugins/my-plugin; echo "Exit: $?"
```

### 権限の確認

```bash
ls -la ~/.config/syslenz/plugins/
# プラグインに +x が設定されていることを確認

[🇬🇧 English](../en/plugins.md)
```

### syslenzのstderrを確認

ターミナルで syslenz を実行し、プラグインエラーのstderrを監視:

```bash
syslenz 2>/tmp/syslenz-errors.log
# 終了後:

[🇬🇧 English](../en/plugins.md)
cat /tmp/syslenz-errors.log
```

エラーメッセージの例:

```
[syslenz] plugin "my-plugin" skipped: exited with status 1
[syslenz] plugin "slow-plugin" skipped: plugin timed out after 5s
```

### よくある問題

| 問題 | 原因 | 対処 |
|------|------|------|
| プラグインが表示されない | ファイルが実行不可 | `chmod +x plugin-file` |
| プラグインが表示されない | プラグインディレクトリにない | `~/.config/syslenz/plugins/` に移動 |
| "exited with status 1" | プラグインスクリプトにエラー | 手動で実行してエラーを確認 |
| "plugin timed out" | 5秒超過 | 最適化またはキャッシュ |
| "failed to parse" | 無効なJSON出力 | JSON出力を手動で検証 |
| サイドバーのフィールドが空 | JSONスキーマの不一致 | フィールドの型がFieldValueバリアントに合っているか確認 |

## Provider (v1.3.0)

v1.3.0 から、よく使われる外部サービス向けの実行可能プラグインを Provider として配布しています。Provider は別の実行基盤ではなく、通常のプラグインローダーと ProcEntry JSON プロトコルを使います。`providers/template/` に自作用テンプレートがあります。

### Provider テンプレート

リポジトリの `providers/template/` ディレクトリに、Provider の雛形が含まれています。このテンプレートをコピーして独自の Provider を作成できます:

```bash
cp providers/template/syslenz-provider-template syslenz-provider-my-service
# collect_metrics 関数を編集
chmod +x syslenz-provider-my-service
```

テンプレートの構造:

```
providers/template/
├── syslenz-provider-template  # 実行可能スクリプト (ProcEntry JSON を出力)
└── README.md                  # 使い方と設定の説明
```

Provider は通常のプラグインと同じ ProcEntry JSON プロトコルに準拠します。違いは、接続情報を環境変数で受け取る規約と、`providers/` ディレクトリで管理される点です。

### MySQL provider

MySQL/MariaDB のグローバルステータスとプロセスリストを収集します。

**環境変数:**

| 変数 | デフォルト | 説明 |
|------|-----------|------|
| `MYSQL_HOST` | `localhost` | 接続先ホスト |
| `MYSQL_PORT` | `3306` | 接続先ポート |
| `MYSQL_USER` | `root` | ユーザー名 |
| `MYSQL_PASS` | (なし) | パスワード |

**収集メトリクス:** threads_connected, threads_running, questions, slow_queries, innodb_buffer_pool_reads, innodb_buffer_pool_read_requests, buffer_pool_hit_rate, uptime

**使い方:**

```bash
# 環境変数を設定してインストール
mkdir -p ~/.config/syslenz/plugins
cp providers/mysql/syslenz-provider-mysql ~/.config/syslenz/plugins/
chmod +x ~/.config/syslenz/plugins/syslenz-provider-mysql
export MYSQL_HOST=127.0.0.1
export MYSQL_PASS=secret

# syslenz を起動すると plugin/mysql として表示される
syslenz
```

### PostgreSQL provider

PostgreSQL の接続数、データベースサイズ、キャッシュヒット率を収集します。

**環境変数:**

| 変数 | デフォルト | 説明 |
|------|-----------|------|
| `PGHOST` | `localhost` | 接続先ホスト |
| `PGPORT` | `5432` | 接続先ポート |
| `PGUSER` | `postgres` | ユーザー名 |
| `PGPASSWORD` | (なし) | パスワード |
| `PGDATABASE` | `postgres` | データベース名 |

**収集メトリクス:** active_connections, idle_connections, database_size, cache_hit_ratio, xact_commit, xact_rollback, deadlocks

**使い方:**

```bash
mkdir -p ~/.config/syslenz/plugins
cp providers/postgres/syslenz-provider-postgres ~/.config/syslenz/plugins/
chmod +x ~/.config/syslenz/plugins/syslenz-provider-postgres
export PGPASSWORD=secret
syslenz
```

### Redis provider

Redis のメモリ使用量、接続数、キースペース情報を収集します。

**環境変数:**

| 変数 | デフォルト | 説明 |
|------|-----------|------|
| `REDIS_HOST` | `localhost` | 接続先ホスト |
| `REDIS_PORT` | `6379` | 接続先ポート |
| `REDIS_PASS` | (なし) | パスワード |
| `REDIS_URL` | (なし) | host/port/password より優先する接続 URL |

**収集メトリクス:** used_memory, used_memory_rss, connected_clients, blocked_clients, keyspace_hits, keyspace_misses, hit_rate, instantaneous_ops_per_sec

**使い方:**

```bash
mkdir -p ~/.config/syslenz/plugins
cp providers/redis/syslenz-provider-redis ~/.config/syslenz/plugins/
chmod +x ~/.config/syslenz/plugins/syslenz-provider-redis
syslenz
```

### nginx provider

nginx のスタブステータスモジュールから接続数とリクエスト統計を収集します。

**環境変数:**

| 変数 | デフォルト | 説明 |
|------|-----------|------|
| `NGINX_STATUS_URL` | `http://localhost/nginx_status` | stub_status エンドポイントの URL |

**前提条件:** nginx の `ngx_http_stub_status_module` が有効で、ステータスエンドポイントが設定されていること。

**収集メトリクス:** active_connections, accepts, handled, requests, reading, writing, waiting

**使い方:**

```bash
mkdir -p ~/.config/syslenz/plugins
cp providers/nginx/syslenz-provider-nginx ~/.config/syslenz/plugins/
chmod +x ~/.config/syslenz/plugins/syslenz-provider-nginx
export NGINX_STATUS_URL=http://localhost:8080/nginx_status
syslenz
```

### 5分で Provider を書く方法

独自の Provider は以下の手順で簡単に作成できます:

**1. テンプレートをコピー**

```bash
cp providers/template/syslenz-provider-template syslenz-provider-my-app
```

**2. スクリプトを編集**

`syslenz-provider-my-app` の `collect_metrics` 関数を編集し、接続情報を環境変数から取得して `add_field` でメトリクスを追加します。最後の `emit_entry` が ProcEntry JSON を出力します。

**3. インストールして確認**

```bash
PLUGIN_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/syslenz/plugins"
mkdir -p "$PLUGIN_DIR"
cp syslenz-provider-my-app "$PLUGIN_DIR/"
chmod +x "$PLUGIN_DIR/syslenz-provider-my-app"
"$PLUGIN_DIR/syslenz-provider-my-app" | python3 -m json.tool
syslenz  # plugin/syslenz-provider-my-app として表示される
```

Provider はプラグインと同じ仕組みで動作するため、既存のプラグインデバッグ手法がそのまま使えます。

---

[<- 前: Web UI](web-ui.md) | [Index](index.md) | [次: 設定 ->](config.md)
