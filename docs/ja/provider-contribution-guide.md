---
version: v1.3.0
lang: ja
---

# Provider コントリビューションガイド

[🇬🇧 English](../en/provider-contribution-guide.md)

[<- プラグイン](plugins.md) | [Index](index.md)


## 目次

- [Provider とは](#provider-とは)
- [ディレクトリ構造](#ディレクトリ構造)
- [必要なファイル](#必要なファイル)
- [ProcEntry プロトコル](#procentry-プロトコル)
- [FieldValue 型](#fieldvalue-型)
- [Provider の実装手順](#provider-の実装手順)
  - [ステップ 1: テンプレートをコピー](#ステップ-1-テンプレートをコピー)
  - [ステップ 2: Preflight チェック](#ステップ-2-preflight-チェック)
  - [ステップ 3: メトリクスを収集](#ステップ-3-メトリクスを収集)
  - [ステップ 4: フィールドを構築して出力](#ステップ-4-フィールドを構築して出力)
  - [ステップ 5: README を作成](#ステップ-5-readme-を作成)
- [実装のベストプラクティス](#実装のベストプラクティス)
- [テスト方法](#テスト方法)
- [新しい Provider のスケルトン](#新しい-provider-のスケルトン)
- [既存 Provider の参考例](#既存-provider-の参考例)
- [提出手順](#提出手順)


## Provider とは

Provider は、データベースやミドルウェアなどの外部サービスからメトリクスを収集する実行可能スクリプト（またはバイナリ）です。プラグインシステムの上位概念として v1.3.0 で導入されました。

通常のプラグインと同じ ProcEntry JSON プロトコルに準拠しますが、以下の点で異なります:

- **リポジトリの `providers/` ディレクトリ**で管理される（`~/.config/syslenz/plugins/` にインストールして使用）
- **環境変数**による接続設定が規約化されている
- **README.md** に監視対象、環境変数、インストール手順、テスト方法が記載されている
- **命名規則**: ファイル名は `syslenz-provider-<サービス名>` 形式

Provider を `~/.config/syslenz/plugins/` にコピーすると、syslenz は起動時に自動検出し、サイドバーに `plugin/syslenz-provider-<名前>` として表示します。


## ディレクトリ構造

リポジトリ内の Provider は以下の構造で配置します:

```
providers/
├── template/                          # テンプレート（新規作成のベース）
│   ├── syslenz-provider-template      # 実行可能スクリプト
│   └── README.md                      # テンプレートの使い方
├── mysql/                             # MySQL provider
│   ├── syslenz-provider-mysql         # 実行可能スクリプト
│   └── README.md                      # 設定・使い方
├── postgres/                          # PostgreSQL provider
│   ├── syslenz-provider-postgres
│   └── README.md
├── redis/                             # Redis provider
│   ├── syslenz-provider-redis
│   └── README.md
└── nginx/                             # nginx provider
    ├── syslenz-provider-nginx
    └── README.md
```


## 必要なファイル

各 Provider ディレクトリには、最低限以下の 2 ファイルが必要です:

### 1. `syslenz-provider-<名前>` (実行可能スクリプト)

- 実行権限が付与されていること (`chmod +x`)
- ProcEntry JSON を stdout に 1 つだけ出力すること
- 成功時に exit 0、失敗時に非ゼロで終了すること
- 5 秒以内に完了すること（超過すると kill される）
- stdout 以外に余計な出力をしないこと（デバッグ出力は stderr へ）

### 2. `README.md`

以下のセクションを含めてください:

- **タイトル**: `# syslenz <サービス名> Provider`
- **What it monitors**: 監視対象の説明
- **Requirements**: 前提条件（CLI ツール、ネットワークアクセスなど）
- **Configuration**: 環境変数の表（変数名、デフォルト値、説明）
- **Installation**: インストール手順（コマンド例付き）
- **Testing**: テスト方法


## ProcEntry プロトコル

Provider は以下の JSON スキーマに準拠したオブジェクトを stdout に出力します:

```json
{
  "source": "データソースの説明",
  "fields": [
    {
      "name": "フィールド名",
      "value": {"Integer": 42},
      "unit": null,
      "description": "このフィールドの意味"
    }
  ]
}
```

- `source`: メトリクスの取得元を示す文字列（例: `"mysql SHOW GLOBAL STATUS"`, `"redis INFO"`）
- `fields`: フィールドの配列。各フィールドには `name`, `value`, `unit`, `description` が含まれる
- `unit`: 通常は `null`（syslenz が FieldValue 型に応じて自動フォーマットする）


## FieldValue 型

`value` フィールドは以下のタグ付き enum バリアントのいずれかです:

| バリアント | JSON 表記 | 用途 |
|-----------|-----------|------|
| `Integer` | `{"Integer": 42}` | カウンター、接続数、クエリ数 |
| `Float` | `{"Float": 0.95}` | 比率、パーセンテージ、ヒット率 |
| `Bytes` | `{"Bytes": 1048576}` | メモリ使用量、ディスクサイズ（自動的に KiB/MiB/GiB にフォーマット） |
| `Text` | `{"Text": "running"}` | バージョン、ステータス、名前 |
| `Duration` | `{"Duration": 86400.0}` | 稼働時間、経過時間（秒。自動的に Xd Xh Xm Xs にフォーマット） |
| `Table` | `{"Table": [["a","b"],["c","d"]]}` | 表形式データ |


## Provider の実装手順

### ステップ 1: テンプレートをコピー

```bash
# リポジトリルートで実行
cp -r providers/template/ providers/my-service/
mv providers/my-service/syslenz-provider-template providers/my-service/syslenz-provider-my-service
chmod +x providers/my-service/syslenz-provider-my-service
```

### ステップ 2: Preflight チェック

スクリプトの冒頭で、必要な CLI ツールの存在と接続情報を確認します:

```bash
# 必要なコマンドの存在確認
if ! command -v my-service-cli >/dev/null 2>&1; then
    echo "my-service-cli not found" >&2
    exit 1
fi

# 環境変数から接続情報を取得（デフォルト値付き）
MY_SERVICE_HOST="${MY_SERVICE_HOST:-localhost}"
MY_SERVICE_PORT="${MY_SERVICE_PORT:-9999}"
```

**ポイント:**
- CLI ツールが見つからない場合は stderr にメッセージを出力して `exit 1`
- 環境変数にはサービス名をプレフィックスとして使用（`MYSQL_`, `REDIS_`, `NGINX_` など）
- 合理的なデフォルト値を設定する

### ステップ 3: メトリクスを収集

外部サービスにクエリを発行してメトリクスを取得します:

```bash
# サービスからデータ取得
RAW_OUTPUT=$(my-service-cli --host "$MY_SERVICE_HOST" --port "$MY_SERVICE_PORT" stats 2>/dev/null) || {
    echo "Failed to connect to my-service" >&2
    exit 1
}

# 値をパース
active_connections=$(echo "$RAW_OUTPUT" | awk '/connections/ { print $2 }')
memory_used=$(echo "$RAW_OUTPUT" | awk '/memory/ { print $2 }')
```

**ポイント:**
- 接続失敗時は stderr にエラーメッセージを出力して `exit 1`
- stderr を `/dev/null` にリダイレクトしてノイズを防ぐ
- 値が取得できなかった場合のフォールバック値を用意する（`${value:-0}`）

### ステップ 4: フィールドを構築して出力

テンプレートに含まれるヘルパー関数 `add_field` と `emit_entry` を使います:

```bash
add_field "active_connections" "Integer" "${active_connections:-0}" "null" "Current active connections"
add_field "memory_used"        "Bytes"   "${memory_used:-0}"        "null" "Memory allocated by service"
add_field "uptime"             "Duration" "${uptime:-0}.0"          "null" "Service uptime"

emit_entry "my-service stats"
exit 0
```

**ポイント:**
- Duration 型の値は浮動小数点数で渡す（末尾に `.0` を付与）
- `emit_entry` の引数はデータの取得元を説明する文字列
- 最後に必ず `exit 0`


### ステップ 5: README を作成

`providers/my-service/README.md` を作成します。以下のテンプレートに従ってください:

```markdown
# syslenz My Service Provider

Monitors a My Service instance by querying `my-service-cli stats`.

## What it monitors

- **Connections**: active connections, idle connections
- **Memory**: allocated memory, peak memory
- **Performance**: queries per second, slow queries

## Requirements

- `my-service-cli` installed and on `$PATH`
- Network access to the My Service server

## Configuration

| Variable           | Default     | Description            |
|--------------------|-------------|------------------------|
| `MY_SERVICE_HOST`  | `localhost` | Server hostname        |
| `MY_SERVICE_PORT`  | `9999`      | Server port            |
| `MY_SERVICE_PASS`  | (none)      | Authentication password|

## Installation

\```bash
mkdir -p ~/.config/syslenz/plugins
cp syslenz-provider-my-service ~/.config/syslenz/plugins/
chmod +x ~/.config/syslenz/plugins/syslenz-provider-my-service

export MY_SERVICE_HOST=db.example.com
syslenz
\```

## Testing

\```bash
./syslenz-provider-my-service | jq .
\```
```


## 実装のベストプラクティス

1. **set -euo pipefail**: スクリプト冒頭に記述し、エラーを早期検出する
2. **タイムアウトに注意**: 5 秒以内に完了しなければ kill される。ネットワークリクエストには `--max-time` や `--connect-timeout` を指定する
3. **stdout を汚さない**: ProcEntry JSON 以外を stdout に出力しない。デバッグログは stderr に出力する
4. **フォールバック値**: メトリクス取得に失敗したフィールドには `${value:-0}` のようなデフォルト値を使う
5. **計算値の追加**: 生データだけでなく、hit rate のような計算済みメトリクスを追加すると運用者に有用
6. **説明は英語で記述**: `description` フィールドは英語で書く（syslenz の i18n システムが翻訳を担当する）
7. **環境変数の命名**: `<SERVICE_NAME>_<SETTING>` の形式で統一する（例: `MYSQL_HOST`, `REDIS_PORT`）


## テスト方法

### 基本テスト: JSON 出力の検証

```bash
# Provider を直接実行して jq でフォーマット
./syslenz-provider-my-service | jq .

# JSON として有効かどうか確認
./syslenz-provider-my-service | python3 -c "import sys,json; json.load(sys.stdin); print('OK')"

# 終了コードを確認
./syslenz-provider-my-service; echo "Exit code: $?"
```

### 構造テスト: 必須フィールドの存在確認

```bash
# source フィールドの存在確認
./syslenz-provider-my-service | jq -e '.source'

# fields 配列が空でないことを確認
./syslenz-provider-my-service | jq -e '.fields | length > 0'

# 全フィールドが name, value, description を持つことを確認
./syslenz-provider-my-service | jq -e '.fields[] | select(.name and .value and .description) | .name'
```

### 統合テスト: syslenz での表示確認

```bash
# Provider をプラグインディレクトリにコピー
mkdir -p ~/.config/syslenz/plugins
cp syslenz-provider-my-service ~/.config/syslenz/plugins/
chmod +x ~/.config/syslenz/plugins/syslenz-provider-my-service

# syslenz を起動してサイドバーに表示されることを確認
syslenz

# サイドバーで plugin/syslenz-provider-my-service を選択し、フィールドが正しく表示されることを確認
```

### エラーケースのテスト

```bash
# サービスが停止している場合の挙動確認
# (接続先を存在しないホストに設定)
MY_SERVICE_HOST=nonexistent ./syslenz-provider-my-service
echo "Exit code: $?"
# 非ゼロの終了コードが返り、stderr にエラーメッセージが出力されることを確認
```


## 新しい Provider のスケルトン

以下は新しい Provider を書き始めるための完全なスケルトンです。コピーして `collect_metrics()` を書き換えてください:

```bash
#!/bin/bash
# syslenz provider: <SERVICE_NAME>
# Collects key metrics from <SERVICE_NAME> via <METHOD>.
#
# Configuration via environment variables:
#   <SERVICE>_HOST  (default: localhost)
#   <SERVICE>_PORT  (default: XXXX)
#   <SERVICE>_PASS  (optional)

set -euo pipefail

# ---------------------------------------------------------------------------
# JSON builder helpers (do not modify)
# ---------------------------------------------------------------------------
_SYSLENZ_FIELDS=""
_SYSLENZ_FIELD_COUNT=0

add_field() {
    local name="$1" type="$2" value="$3" unit="$4" desc="$5"
    local json_value
    case "$type" in
        Text)     json_value="{\"Text\": \"$value\"}" ;;
        Integer)  json_value="{\"Integer\": $value}" ;;
        Float)    json_value="{\"Float\": $value}" ;;
        Bytes)    json_value="{\"Bytes\": $value}" ;;
        Duration) json_value="{\"Duration\": $value}" ;;
        *)        json_value="{\"Text\": \"$value\"}" ;;
    esac
    local comma=""
    [ "$_SYSLENZ_FIELD_COUNT" -gt 0 ] && comma=","
    _SYSLENZ_FIELDS="${_SYSLENZ_FIELDS}${comma}{\"name\": \"${name}\", \"value\": ${json_value}, \"unit\": ${unit}, \"description\": \"${desc}\"}"
    _SYSLENZ_FIELD_COUNT=$((_SYSLENZ_FIELD_COUNT + 1))
}

emit_entry() {
    local source="$1"
    echo "{\"source\": \"${source}\", \"fields\": [${_SYSLENZ_FIELDS}]}"
}

# ---------------------------------------------------------------------------
# Preflight checks
# ---------------------------------------------------------------------------
if ! command -v <service-cli> >/dev/null 2>&1; then
    echo "<service-cli> not found" >&2
    exit 1
fi

SERVICE_HOST="${<SERVICE>_HOST:-localhost}"
SERVICE_PORT="${<SERVICE>_PORT:-XXXX}"

# Build CLI arguments
CLI_ARGS=(--host "$SERVICE_HOST" --port "$SERVICE_PORT")
if [ -n "${<SERVICE>_PASS:-}" ]; then
    CLI_ARGS+=(--password "${<SERVICE>_PASS}")
fi

# ---------------------------------------------------------------------------
# Collect raw data
# ---------------------------------------------------------------------------
RAW_OUTPUT=$(<service-cli> "${CLI_ARGS[@]}" stats 2>/dev/null) || {
    echo "Failed to connect to <SERVICE_NAME>" >&2
    exit 1
}

# Parse helper: extract a value by key from the raw output
get_value() {
    local key="$1"
    echo "$RAW_OUTPUT" | awk -v k="$key" '$1 == k { print $2; exit }'
}

# ---------------------------------------------------------------------------
# Extract metrics
# ---------------------------------------------------------------------------
# TODO: Replace with actual metric extraction
metric_a=$(get_value "metric_a")
metric_b=$(get_value "metric_b")
uptime=$(get_value "uptime")

# ---------------------------------------------------------------------------
# Build output
# ---------------------------------------------------------------------------
add_field "metric_a" "Integer"  "${metric_a:-0}"  "null" "Description of metric A"
add_field "metric_b" "Bytes"    "${metric_b:-0}"  "null" "Description of metric B"
add_field "uptime"   "Duration" "${uptime:-0}.0"  "null" "Service uptime"

emit_entry "<service-name> stats"
exit 0
```


## 既存 Provider の参考例

| Provider | 特徴 | 参考にすべき点 |
|----------|------|----------------|
| [MySQL](../../providers/mysql/) | `SHOW GLOBAL STATUS` からメトリクスを抽出 | ステータス出力のパース、buffer pool hit rate の計算 |
| [Redis](../../providers/redis/) | `redis-cli INFO` からメトリクスを抽出 | 複数の接続方法（host/port vs URL）のサポート |
| [nginx](../../providers/nginx/) | `stub_status` ページの HTTP レスポンスをパース | curl + awk でのテキストパース、レスポンス検証 |
| [PostgreSQL](../../providers/postgres/) | `psql` でクエリを実行してメトリクスを取得 | SQL ベースのメトリクス収集 |
| [template](../../providers/template/) | 最小限の雛形 | 基本構造の理解、ヘルパー関数の使い方 |


## 提出手順

新しい Provider を syslenz プロジェクトに提出するには、以下の手順に従ってください:

### 1. フォークとブランチの作成

```bash
git clone https://github.com/<your-username>/syslenz.git
cd syslenz
git checkout -b provider/<service-name>
```

### 2. Provider の作成

```bash
mkdir providers/<service-name>
# 上記の手順に従ってスクリプトと README を作成
```

### 3. チェックリスト

提出前に以下を確認してください:

- [ ] ファイル名が `syslenz-provider-<名前>` 形式である
- [ ] 実行権限が付与されている (`chmod +x`)
- [ ] `set -euo pipefail` がスクリプト冒頭にある
- [ ] 必要な CLI ツールの存在確認がある（Preflight check）
- [ ] 接続設定が環境変数で行える
- [ ] 環境変数に合理的なデフォルト値がある
- [ ] 接続失敗時に stderr にメッセージを出力し非ゼロで終了する
- [ ] 出力が有効な ProcEntry JSON である
- [ ] 5 秒以内に完了する
- [ ] README.md に What it monitors / Requirements / Configuration / Installation / Testing が記載されている
- [ ] `./syslenz-provider-<名前> | jq .` でテスト済み
- [ ] syslenz 上で実際にメトリクスが表示されることを確認済み

### 4. プルリクエストの作成

```bash
git add providers/<service-name>/
git commit -m "feat: add <service-name> provider"
git push origin provider/<service-name>
```

GitHub でプルリクエストを作成し、以下を記載してください:

- 何を監視する Provider か
- どのコマンド/API でメトリクスを取得するか
- テスト環境とテスト結果

---

[<- プラグイン](plugins.md) | [Index](index.md)
