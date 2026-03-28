---
version: v1.0.0
lang: ja
---

# プラグインシステム

[<- 前: Web UI](web-ui.md) | [Index](index.md) | [次: 設定 ->](config.md)

[🇬🇧 English](../en/plugins.md)

## 目次

- [概要](#概要)
- [プラグインプロトコル](#プラグインプロトコル)
- [プラグインディレクトリ](#プラグインディレクトリ)
- [プラグインの作成手順](#プラグインの作成手順)
- [例: JVMメモリプラグイン](#例-jvmメモリプラグイン)
- [例: Docker統計プラグイン](#例-docker統計プラグイン)
- [プラグインの検出と実行](#プラグインの検出と実行)
- [プラグインのデバッグ](#プラグインのデバッグ)

## 概要

syslenz は実行可能プラグインを介したカスタムデータソースをサポートしています。プラグインは、`ProcEntry` JSON オブジェクトをstdoutに出力するプラグインディレクトリ内の任意の実行可能ファイルです。プラグインはサイドバーに `plugin/` プレフィックス付きで表示され、全てのビュー（ダッシュボード、クラシック、自動診断など）とシームレスに統合されます。

## プラグインプロトコル

プラグインは以下を満たす必要があります:

1. 実行可能ファイルであること（言語は問わない: シェル、Python、Rust、Go など）
2. 引数を受け取らないこと（stdinは `/dev/null`）
3. `ProcEntry` スキーマに準拠する単一のJSONオブジェクトをstdoutに出力
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

`value` フィールドは以下のタグ付きenumバリアントのいずれかでなければなりません:

| バリアント | JSONフォーマット | 例 |
|-----------|----------------|-----|
| Bytes | `{"Bytes": 1024}` | メモリ、ディスクサイズ |
| Integer | `{"Integer": 42}` | カウンター、数量 |
| Float | `{"Float": 3.14}` | パーセンテージ、比率 |
| Text | `{"Text": "hello"}` | 文字列、ステータス値 |
| Duration | `{"Duration": 86400.0}` | 秒単位の時間 |
| Table | `{"Table": [["col1","col2"],["a","b"]]}` | 表形式データ |

### タイムアウト

プラグインは**5秒以内**に完了する必要があります。期限を超えるとプラグインはkillされ、エラーメッセージがstderrに出力されスキップされます。

### 終了コード

- 終了コード0: 成功。stdoutがJSONとしてパースされる。
- 非ゼロ終了: プラグインはスキップされる。エラーがstderrに出力される。

## プラグインディレクトリ

プラグインは以下に保存されます:

```
~/.config/syslenz/plugins/
```

または、`$XDG_CONFIG_HOME` が設定されている場合:

```
$XDG_CONFIG_HOME/syslenz/plugins/
```

syslenz はこのディレクトリが存在しない場合、自動的に作成します。

**重要:** Unixシステムでは、プラグインファイルに実行権限ビットが設定されている必要があります（`chmod +x`）。実行権限のないファイルはサイレントにスキップされます。

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

出力が ProcEntry スキーマに一致する有効なJSONであることを確認します。

### ステップ4: syslenzを実行

```bash
syslenz
```

プラグインはサイドバーに `plugin/my-plugin` として表示されます。

## 例: JVMメモリプラグイン

`jcmd` を介してJVMメトリクスを読み取るPythonプラグイン:

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

if ! command -v docker &>/dev/null; then
    echo '{"source":"/custom/docker-stats","fields":[{"name":"status","value":{"Text":"docker が見つかりません"},"unit":null,"description":"プラグインステータス"}]}'
    exit 0
fi

# コンテナ統計を取得 (no-stream で単一スナップショット)
stats=$(docker stats --no-stream --format '{{.Name}}\t{{.CPUPerc}}\t{{.MemUsage}}\t{{.NetIO}}' 2>/dev/null)

if [ -z "$stats" ]; then
    echo '{"source":"/custom/docker-stats","fields":[{"name":"status","value":{"Text":"実行中のコンテナなし"},"unit":null,"description":"プラグインステータス"}]}'
    exit 0
fi

# テーブル行を構築
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

syslenz が起動時（および各リフレッシュ時）に行うこと:

1. `~/.config/syslenz/plugins/` 内の全ファイルを読み取り
2. ファイル以外（ディレクトリ、ディレクトリへのシンボリックリンク）をスキップ
3. 実行権限ビットのないファイルをスキップ（Unixの場合）
4. 残りの各ファイルを実行:
   - stdin: `/dev/null`
   - stdout: パイプ（キャプチャ）
   - stderr: パイプ（キャプチャ、失敗時に表示）
   - タイムアウト: 5秒
5. stdoutを `ProcEntry` JSONとしてパース
6. エントリを `plugin/<拡張子なしファイル名>` として挿入

プラグインはメインの `/proc` パースと並行して実行されます。失敗したプラグインはstderrにエラーメッセージを出力してサイレントにスキップされます。

## プラグインのデバッグ

### 手動テスト

```bash
# プラグインを直接実行
~/.config/syslenz/plugins/my-plugin

# JSONを検証
~/.config/syslenz/plugins/my-plugin | python3 -m json.tool

# 終了コードを確認
~/.config/syslenz/plugins/my-plugin; echo "Exit: $?"
```

### 権限の確認

```bash
ls -la ~/.config/syslenz/plugins/
# プラグインに +x が設定されていることを確認
```

### syslenzのstderrを確認

ターミナルでsyslenzを実行し、プラグインエラーのstderrを監視:

```bash
syslenz 2>/tmp/syslenz-errors.log
# 終了後:
cat /tmp/syslenz-errors.log
```

エラーメッセージの例:

```
[syslenz] plugin "my-plugin" skipped: exited with status 1
[syslenz] plugin "slow-plugin" skipped: plugin timed out after 5s
```

### よくある問題

| 問題 | 原因 | 修正方法 |
|------|------|---------|
| プラグインが表示されない | ファイルが実行不可 | `chmod +x plugin-file` |
| プラグインが表示されない | プラグインディレクトリにない | `~/.config/syslenz/plugins/` に移動 |
| "exited with status 1" | プラグインスクリプトにエラー | プラグインを手動で実行してエラーを確認 |
| "plugin timed out" | プラグインが5秒超 | 最適化するかキャッシュ |
| "failed to parse" | 無効なJSON出力 | JSON出力を手動で検証 |
| サイドバーのフィールドが空 | JSONスキーマの不一致 | フィールドの型がFieldValueバリアントに一致するか確認 |

---

[<- 前: Web UI](web-ui.md) | [Index](index.md) | [次: 設定 ->](config.md)
