---
version: v1.1.0
lang: ja
---

# 設定リファレンス

[🇬🇧 English](../en/config.md)

[<- 前: プラグイン](plugins.md) | [Index](index.md) | [次: キーバインド ->](keybindings.md)


## 目次

- [ファイルの場所](#ファイルの場所)
- [優先順位](#優先順位)
- [設定セクション](#設定セクション)
  - [general](#general)
  - [otel](#otel)
  - [web](#web)
  - [ssh](#ssh)
- [完全な例](#完全な例)
- [最小限の例](#最小限の例)

## ファイルの場所

syslenz は以下から設定を読み取ります:

```
~/.config/syslenz/config.toml
```

`$XDG_CONFIG_HOME` が設定されている場合:

```
$XDG_CONFIG_HOME/syslenz/config.toml
```

ファイルが存在しなければ、デフォルト値をそのまま使います。ファイルがあるがエラーを含む場合、警告をstderrに出力してデフォルト値を使います。

## 優先順位

設定は以下の順で解決されます（上ほど優先）:

1. **CLIフラグ**（例: `--lang ja` は設定ファイルより優先）
2. **環境変数**（例: `$XDG_CONFIG_HOME`）
3. **config.toml**
4. **組み込みデフォルト値**

## 設定セクション

### `[general]`

TUIの動作に関するコア設定です。

| キー | 型 | デフォルト | 説明 |
|------|-----|---------|------|
| `lang` | String | `"en"` | インターフェース言語。`"en"` または `"ja"` |
| `interval_ms` | Integer | `1000` | 自動リフレッシュ間隔（ミリ秒） |
| `default_view` | String | `"dashboard"` | 起動時のビュー。`"dashboard"` または `"classic"` |
| `history_size` | Integer | `60` | グラフと差分用のリングバッファに保持するスナップショット数 |

```toml
[general]
lang = "en"
interval_ms = 1000
default_view = "dashboard"
history_size = 60
```

**注意:**
- `interval_ms` はTUIの自動リフレッシュとリモート接続（SSH、Docker、TCP）のポーリング間隔の両方に影響
- `history_size` はグラフビューのデータポイント数を制御。大きい値ほどメモリを使うが、長いトレンドを表示できる
- `default_view = "classic"` は `--classic` CLIフラグと同等

### `[otel]`

OpenTelemetryエクスポート設定（`--otel` モード用）。

| キー | 型 | デフォルト | 説明 |
|------|-----|---------|------|
| `endpoint` | String | `"http://localhost:4317"` | OTLP gRPCエンドポイントURL |
| `interval_secs` | Integer | `5` | メトリクスプッシュ間隔（秒） |

```toml
[otel]
endpoint = "http://localhost:4317"
interval_secs = 5
```

**注意:**
- エンドポイントはOTLP互換のgRPCエンドポイントである必要あり
- CLIフラグ `--otel [endpoint]` は `otel.endpoint` を上書き
- CLIフラグ `--interval <secs>` は `otel.interval_secs` を上書き

### `[web]`

Web UI設定（`--web` モード用）。

| キー | 型 | デフォルト | 説明 |
|------|-----|---------|------|
| `port` | Integer | `3000` | WebサーバーのHTTPポート |
| `capture_interval_secs` | Integer | `1` | スナップショットのキャプチャ間隔（秒） |
| `max_history_count` | Integer | `60` | メモリ内履歴の件数上限 |
| `max_history_bytes` | Integer | `67108864` | メモリ内履歴の合計バイト数上限（概算、0 で無効） |
| `truncate_large_tables` | Boolean | `true` | 履歴内の巨大テーブルを縮約するか |
| `truncate_table_rows` | Integer | `20` | テーブル縮約時の保持行数しきい値 |

```toml
[web]
port = 3000
capture_interval_secs = 1
max_history_count = 60
max_history_bytes = 67108864  # 64 MB
truncate_large_tables = true
truncate_table_rows = 20
```

**メモリ使用量について:**

`--web` モードはバックグラウンドタスクで定期的に `/proc` 全体をキャプチャし、
メモリ内に履歴を保持します。長時間稼働でもRSSが膨張しないよう、以下の対策を講じています。

- **件数上限** (`max_history_count`): 履歴のスナップショット数を上限で頭打ち
- **バイト数上限** (`max_history_bytes`): 履歴の合計サイズが上限を超えたら古いものから破棄
- **テーブル縮約** (`truncate_large_tables`): 履歴内の巨大テーブル（プロセス一覧など）を
  先頭 `truncate_table_rows` 行＋ `[truncated: N rows]` 注記に縮約。
  最新の1件（`/api/snapshot`）は常にフルサイズ
- **malloc_trim**: 60秒ごとに `malloc_trim(0)` を呼び、glibc malloc の断片化メモリをカーネルに返す
- **MALLOC_ARENA_MAX**: 起動時に `mallopt(M_ARENA_MAX, 2)` で glibc malloc の arena 数を制限

`/healthz` で現在のメモリ設定と履歴件数を確認できます。

```bash
curl http://localhost:3000/healthz
# {"status":"ok","history_len":42,"max_history_count":60,...}
```

**注意:**
- Web UI の自動起動では `0.0.0.0` と `web.port` を使用
- 単独起動の `--web [addr:port]` は明示的なバインドアドレスを受け付け、ポートのみなら `0.0.0.0` の動作を維持

### `[ssh]`

SSH関連の設定。

| キー | 型 | デフォルト | 説明 |
|------|-----|---------|------|
| `hosts` | String配列 | `[]` | 事前登録のリモートホスト（将来のマルチホストビュー用） |

```toml
[ssh]
hosts = [
    "admin@web-server-1",
    "admin@web-server-2",
    "root@db-server",
]
```

**注意:**
- このフィールドは将来のマルチホスト監視サポート用に予約されています。現在は `--ssh` CLIフラグで単一ホストを監視してください。

## 完全な例

```toml
# syslenz 設定

# 場所: ~/.config/syslenz/config.toml


[general]
# インターフェース言語: "en" または "ja"

lang = "ja"

# 自動リフレッシュ間隔 (ミリ秒)

interval_ms = 1000

# 起動時のビュー: "dashboard" または "classic"

default_view = "dashboard"

# グラフ用に保持するスナップショット数

history_size = 60

[otel]
# OTLP gRPCエンドポイント

endpoint = "http://otel-collector.local:4317"

# メトリクスプッシュ間隔 (秒)

interval_secs = 10

[web]
# Web UIポート

port = 8080

[ssh]
# 事前登録のリモートホスト

hosts = [
    "admin@prod-web-01",
    "admin@prod-web-02",
    "root@prod-db-01",
]
```

## 最小限の例

ほとんどの場合、設定ファイルは不要です。言語だけ変えたいなら:

```toml
[general]
lang = "ja"
```

---

[<- 前: プラグイン](plugins.md) | [Index](index.md) | [次: キーバインド ->](keybindings.md)
