---
version: v1.0.0
lang: ja
---

# 設定リファレンス

[<- 前: プラグイン](plugins.md) | [Index](index.md) | [次: キーバインド ->](keybindings.md)

[🇬🇧 English](../en/config.md)

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

または、`$XDG_CONFIG_HOME` が設定されている場合:

```
$XDG_CONFIG_HOME/syslenz/config.toml
```

ファイルが存在しない場合、syslenz はサイレントにデフォルト値を使用します。ファイルが存在するがエラーを含む場合、警告がstderrに出力されデフォルト値が使用されます。

## 優先順位

設定は以下の順序で解決されます（優先度が高い順）:

1. **CLIフラグ**（例: `--lang ja` は設定を上書き）
2. **環境変数**（例: `$XDG_CONFIG_HOME`）
3. **config.toml**
4. **組み込みデフォルト値**

## 設定セクション

### `[general]`

TUIの動作に影響するコア設定。

| キー | 型 | デフォルト | 説明 |
|------|-----|---------|------|
| `lang` | String | `"en"` | インターフェース言語。有効な値: `"en"`、`"ja"` |
| `interval_ms` | Integer | `1000` | 自動リフレッシュ間隔（ミリ秒） |
| `default_view` | String | `"dashboard"` | 起動時のビュー。有効な値: `"dashboard"`、`"classic"` |
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
- `history_size` はグラフビューに表示されるデータポイント数を制御。大きい値ほどメモリを使用するが、より長いトレンドを表示
- `default_view = "classic"` は `--classic` CLIフラグと同等

### `[otel]`

OpenTelemetryエクスポート設定（`--otel` モードで使用）。

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
- エンドポイントはOTLP互換のgRPCエンドポイントである必要がある
- CLIフラグ `--otel [endpoint]` は `otel.endpoint` を上書き
- CLIフラグ `--interval <secs>` は `otel.interval_secs` を上書き

### `[web]`

Web UI設定（`--web` モードで使用）。

| キー | 型 | デフォルト | 説明 |
|------|-----|---------|------|
| `port` | Integer | `3000` | WebサーバーのHTTPポート |

```toml
[web]
port = 3000
```

**注意:**
- Webサーバーは常に `0.0.0.0`（全インターフェース）にバインド
- CLIフラグ `--web [port]` は `web.port` を上書き

### `[ssh]`

SSH関連の設定。

| キー | 型 | デフォルト | 説明 |
|------|-----|---------|------|
| `hosts` | String配列 | `[]` | 事前設定済みリモートホスト（将来のマルチホストビュー用） |

```toml
[ssh]
hosts = [
    "admin@web-server-1",
    "admin@web-server-2",
    "root@db-server",
]
```

**注意:**
- このフィールドは将来のマルチホスト監視サポート用に予約されています。現在は `--ssh` CLIフラグで単一ホストの監視を使用してください。

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
# 事前設定済みリモートホスト
hosts = [
    "admin@prod-web-01",
    "admin@prod-web-02",
    "root@prod-db-01",
]
```

## 最小限の例

ほとんどのユーザーにとって、設定ファイルは不要です。言語だけを変更したい場合:

```toml
[general]
lang = "ja"
```

---

[<- 前: プラグイン](plugins.md) | [Index](index.md) | [次: キーバインド ->](keybindings.md)
