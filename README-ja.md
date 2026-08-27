[English version](README.md)

# syslenz

[![CI](https://github.com/opaopa6969/syslenz/actions/workflows/ci.yml/badge.svg)](https://github.com/opaopa6969/syslenz/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/syslenz.svg)](https://crates.io/crates/syslenz)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> **/proc の Wireshark** — Linux のあらゆるメトリクスを構造化・型付きデータとして。

設定ゼロ。バイナリ1本。データはマシン外に出ない。

---

## 目次

- [なぜ syslenz か](#なぜ-syslenz-か)
- [クイックスタート](#クイックスタート)
- [インストール](#インストール)
- [TUI ビューとキーバインド](#tui-ビューとキーバインド)
- [HTTP サーバーと API](#http-サーバーと-api)
- [設定 GUI](#設定-gui)
- [設定](#設定)
- [データソース (50+)](#データソース-50)
- [自動診断](#自動診断)
- [教育機能](#教育機能)
- [SDK](#sdk)
- [Grafana 連携](#grafana-連携)
- [アーキテクチャ](#アーキテクチャ)
- [ドキュメント](#ドキュメント)
- [コントリビュート](#コントリビュート)
- [ロードマップ](#ロードマップ)
- [ライセンス](#ライセンス)

---

## なぜ syslenz か

| 軸 | syslenz | SaaS 監視ツール | htop/top |
|----|---------|----------------|----------|
| **価値到達時間** | 21秒 | 7分以上 | 即座（表面的） |
| **データ主権** | 100% ローカル | クラウド（ベンダーロックイン） | ローカル |
| **教育機能** | 4段階ヘルプ、カテゴリガイド、チュートリアル | ドキュメントサイト（別途） | なし |
| **組み込み性** | SDK (Java/Python/Node.js)、OTEL、Prometheus、JSON | API（有料） | なし |
| **コスト** | 無料 / MIT | $15–35/ホスト/月 | 無料 |

### SRE 向け

SSH で入り、`syslenz` を実行するだけ。エージェント不要、設定不要。JSON エクスポート、ホスト間 diff、`jq` パイプ、CI 連携。50+ データソースを標準装備。

### 学習者向け

全フィールドに4段階の人間が読める説明付き。`?` を押してサイクル。カテゴリガイドは「RAM はどこへ消えたか」「パケットの一生」などのナラティブにソースをつなぐ。

### セキュリティ監査向け

システム全状態を JSON でキャプチャ、ホスト間比較、変更追跡。カーネルモジュール、オープン接続、cgroup ポリシー、マウントファイルシステム — 1エクスポートで完結。

---

## クイックスタート

```bash
# ワンライナーインストール
curl -sSf https://raw.githubusercontent.com/opaopa6969/syslenz/main/scripts/install.sh | sh

# TUI 起動
syslenz

# 操作: j/k で移動、Enter でドリルイン、Backspace で戻る
# ? でヘルプレベルをサイクル (OFF → NORMAL → DETAILED → EXTRA)
# D でダッシュボード、X で診断、C でカテゴリガイド
# q で終了
```

---

## インストール

```bash
# ワンライナーインストール
curl -sSf https://raw.githubusercontent.com/opaopa6969/syslenz/main/scripts/install.sh | sh

# crates.io から (Rust 1.85+ / edition 2024 が必要)
cargo install syslenz

# ソースから
git clone https://github.com/opaopa6969/syslenz.git
cd syslenz
cargo install --path .

# オプション機能付き
cargo install --path . --features "otel,web"

# Docker — TCP サーバーモード (認証なし。信頼できるネットワークのみで使用)
docker run --rm -p 9100:9100 --pid=host opaopa6969/syslenz --serve
syslenz --connect localhost:9100

# Docker — Web UI
docker compose up -d
# http://localhost:3000 を開く

# Docker — Grafana + Prometheus + syslenz
docker compose --profile grafana up -d
# http://localhost:3001 (Grafana)、http://localhost:9090 (Prometheus)

# ビルド済みバイナリ
# https://github.com/opaopa6969/syslenz/releases を参照
```

> **セキュリティ注意**: `--serve` は認証なしの TCP サーバーを起動します。共有ホストやインターネット公開環境では `127.0.0.1:9100` にバインドするかファイアウォールで保護してください:
> ```bash
> syslenz --serve 127.0.0.1:9100   # ループバックのみ
> ```

---

## TUI ビューとキーバインド

### ビュー切替

| キー | ビュー | 説明 |
|------|--------|------|
| `D` | ダッシュボード | システム概要: ロード、メモリ、CPU、ネットワーク |
| `O` | クラシック | サイドバー + 詳細パネル（従来モード） |
| `W` | ウェルカム | キーバインド一覧とオンボーディング |
| `X` | 診断 | 自動検出された問題と推奨アクション |
| `C` | カテゴリガイド | トピック別の教育コンテンツ |

### ナビゲーション

| キー | アクション |
|------|-----------|
| `j` / `k` | ソース / フィールドを移動 |
| `Enter` / `Backspace` | ドリルイン / 戻る |
| `Tab` | サイドバー / コンテンツのフォーカス切替 |
| `/` | ソースを検索 |
| `d` | diff ビュー |
| `g` | グラフ（スパークライン） |
| `?` | ヘルプレベルをサイクル (OFF / NORMAL / DETAILED / EXTRA) |
| `L` | 言語切替 (EN / JA) |
| `c` | クリップボードにコピー |
| `e` | スナップショットを JSON エクスポート |
| `a` | 自動更新の切替 |
| `r` | 手動更新 |
| `q` | 終了 |

### マルチホストタブ

複数ホスト監視時（`--ssh`、`--docker`、`--connect`）は `F1`–`F9` でホストタブを切替。

---

## HTTP サーバーと API

`--web` で起動すると HTTP サーバーが起動（デフォルトポート 3000、`web` feature が必要）。

### エンドポイント

| メソッド | パス | 説明 |
|---------|------|------|
| `GET` | `/` | Web UI ダッシュボード |
| `GET` | `/api/snapshot` | 現在のスナップショット (JSON) |
| `GET` | `/api/history` | スナップショット履歴 (JSON 配列) |
| `GET` | `/api/sources` | 利用可能なデータソース |
| `GET` | `/api/stream` | SSE ストリーム（リアルタイム更新） |
| `GET` | `/api/view` | レンダリング済みビューデータ |
| `GET` | `/api/field-help` | 指定フィールドの説明 |
| `GET` | `/settings` | 設定 GUI（ブラウザ） |
| `GET` | `/api/v1/settings` | 現在の設定を JSON で返す (API v1) |
| `POST` | `/api/v1/settings/alerts` | アラートルールを設定ファイルに書込 (API v1) |

### API v1

バージョン付きエンドポイントは `X-Syslenz-API-Version: 1` レスポンスヘッダを返します。`/api/v1/*` プレフィックスはメジャーバージョンアップなく変更されません。

```bash
# 現在の設定を取得
curl http://localhost:3000/api/v1/settings

# アラートルールを送信
curl -X POST http://localhost:3000/api/v1/settings/alerts \
  -H 'Content-Type: application/json' \
  -d '[{"source":"meminfo","field":"MemAvailable","condition":"< 500000000","severity":"critical","message":"メモリ不足"}]'
```

> **セキュリティ注意**: 現リリースの HTTP サーバーには認証機能がありません。共有ホストで使用する場合はループバックにバインドするか、リバースプロキシ（TLS + 認証付き）を前段に置いてください。
>
> **Fleet View**（`/fleet` — マルチホスト Web ダッシュボード）と**認証**（Basic Auth / Token）は将来のリリースで実装予定であり、**現時点では未実装**です。

---

## 設定 GUI

`syslenz --web` 動作中に `http://localhost:3000/settings` をブラウザで開く。

設定 GUI でできること:

- 現在の設定の確認（`~/.config/syslenz/config.toml` から読込）
- アラートルールの追加・編集・削除
- syslenz を再起動せずに設定ファイルへ保存

GUI は `/api/v1/settings` と `/api/v1/settings/alerts` と通信します。変更はアラートルールの場合、次の更新サイクルで即時反映（再起動不要）。

---

## 設定

```toml
# ~/.config/syslenz/config.toml

[general]
lang = "ja"                 # "en" または "ja"
interval_ms = 1000          # 自動更新間隔
default_view = "dashboard"  # "dashboard" または "classic"

[web]
port = 3000
# メモリ管理（長時間稼働時のRSS膨張を防ぐ）
capture_interval_secs = 1       # キャプチャ間隔（秒）
max_history_count = 60          # 履歴件数上限
max_history_bytes = 67108864    # 履歴バイト数上限（64MB、0で無効）
truncate_large_tables = true    # 履歴内の巨大テーブルを縮約
truncate_table_rows = 20        # 縮約時の保持行数

[otel]
endpoint = "http://localhost:4317"
interval_secs = 5

[ssh]
hosts = ["user@server1", "user@server2"]

[[alert]]
source = "meminfo"
field = "MemAvailable"
condition = "< 500000000"
severity = "critical"
message = "メモリが危機的に低下"
```

CLI フラグは設定ファイルの値を上書きします。完全なリファレンスは [`docs/ja/config.md`](docs/ja/config.md) を参照。

### 主要 CLI フラグ

| フラグ | 説明 |
|-------|------|
| `--classic` | クラシックサイドバーモードで起動 |
| `--lang ja` | 日本語 UI |
| `--ssh user@host` | SSH 経由リモート監視（複数指定可） |
| `--docker container` | Docker コンテナ監視 |
| `--serve [addr]` | TCP サーバーモード（デフォルト: `0.0.0.0:9100`） |
| `--connect host:port` | TCP サーバーに接続 |
| `--web [addr:port]` | Web UI（デフォルト: `0.0.0.0:3000`、ポートのみの指定は全インターフェースにバインド） |
| `--export file.json` | スナップショットを JSON エクスポート |
| `--import file.json` | スナップショットからリプレイモード |
| `--prometheus [port]` | Prometheus `/metrics` エンドポイント |
| `--otel [endpoint]` | OpenTelemetry エクスポート（`otel` feature が必要） |
| `--tutorial` | インタラクティブ 8 ステップガイドウォークスルー |
| `--widget` | X11 フローティングウィジェット（`x11widget` feature が必要） |

---

## データソース (50+)

<details>
<summary><strong>/proc (43 ソース — Linux)</strong></summary>

| カテゴリ | ソース |
|---------|--------|
| システム | uptime, loadavg, version, cmdline, modules, filesystems, devices, consoles, misc, dma |
| メモリ | meminfo, vmstat, zoneinfo, buddyinfo, slabinfo, pagetypeinfo, swaps |
| CPU | cpuinfo, stat, interrupts, softirqs, schedstat, timer_list, pressure |
| ストレージ | mounts, partitions, diskstats, locks |
| ネットワーク | net/dev, net/tcp, net/udp, net/unix, net/arp, net/route, net/sockstat, net/snmp, net/netstat, net/wireless |
| セキュリティ | crypto, cgroups, iomem, ioports |
| プロセス | processes（全 PID: 名前, 状態, RSS, スレッド数, UID） |

</details>

<details>
<summary><strong>/sys (3 ソース)</strong></summary>

| ソース | 説明 |
|--------|------|
| df | ファイルシステムディスク使用量（statfs 経由） |
| thermal | サーマルゾーンから CPU/GPU 温度 |
| file-nr | システム全体のファイルディスクリプタ使用量 |

</details>

<details>
<summary><strong>ネットワーク詳細 (5 ソース)</strong></summary>

| ソース | 説明 |
|--------|------|
| ip/route | メトリクスとデフォルトゲートウェイを含む完全ルーティングテーブル |
| ip/neighbor | ARP/NDP キャッシュと到達可能性状態 |
| ss | ソケット統計（TCP established、TIME_WAIT、orphaned） |
| dns | DNS 設定 + 名前解決速度テスト |
| conntrack | コネクショントラッキングテーブル使用量 |

</details>

<details>
<summary><strong>プラグイン（無制限）</strong></summary>

`~/.config/syslenz/plugins/` に実行ファイルを置くだけ。stdout に JSON を出力すれば syslenz が自動的に取得。

同梱サンプル: **JVM**（jstat/jcmd）、**Docker**（コンテナ統計）。

詳細は `plugins/examples/` と [`docs/ja/plugins.md`](docs/ja/plugins.md) を参照。

</details>

<details>
<summary><strong>クロスプラットフォーム</strong></summary>

| プラットフォーム | ソース数 | 方式 |
|----------------|---------|------|
| Linux | 51+ | /proc + /sys + コマンド |
| macOS | 24 | sysctl + vm_stat + システムコマンド |
| Windows | 24 | PowerShell + WMI |

</details>

---

## 自動診断

TUI で `X` を押すと 27 チェック関数で 40+ パターンを検出:

- メモリプレッシャー、スワップ枯渇、OOM キル、メモリリーク検出
- CPU オーバーロード、ロードスパイク、プレッシャーストール、コンテキストスイッチ率
- ディスク使用量、inode プレッシャー、温度警告
- ネットワーク: SYN flood、CLOSE_WAIT リーク、TIME_WAIT 過多、orphaned TCP、再送、UDP エラー
- ゾンビプロセス、D ステート停止プロセス、高メモリプロセス検出
- ファイルディスクリプタ枯渇、DNS 設定ミス、conntrack オーバーフロー
- IP フォワーディング検出、カーネル汚染検査、最近の再起動通知

**診断ジャンプ**で任意の検出結果から関連メトリクスソースへ直接ナビゲート。

---

## 教育機能

- **4段階コンテキストヘルプ** — `?` で OFF / NORMAL / DETAILED / EXTRA をサイクル
- **カテゴリガイド**（`C`）— 構造化学習パス: メモリ、CPU、ネットワーク、ストレージ、プロセス、ハードウェア
- **学習ブレッドクラム** — EXTRA レベルで 18 フィールドに「次のステップ」ヒント表示
- **「Did you know?」ヒント** — ウェルカム画面（`W`）にランダムヒント
- **チュートリアルモード**（`--tutorial`）— ライブデータを使った 8 ステップガイドウォークスルー
- **SEE ALSO クロスリンク** — 31 フィールドに 105 件の関連メトリクスへの参照
- **診断ジャンプ** — 任意の検出結果から関連ソースへ直接ジャンプ

---

## SDK

| SDK | 言語 | パッケージ |
|-----|------|----------|
| [syslenz4j](https://github.com/opaopa6969/syslenz4j) | Java 17+ | Maven Central: `org.unlaxer.infra:syslenz4j` |
| syslenz4py | Python 3.8+ | `sdk/python/`（PyPI 予定） |
| syslenz4node | Node.js 18+ | `sdk/node/`（npm 予定） |

すべての SDK は TCP サーバー（`--serve`）に接続し、`MetricKind`（8バリアント）と `CommonMetric`（15クロスプラットフォームメトリクス）で型付きアクセスを提供。

---

## Grafana 連携

```bash
docker compose --profile grafana up -d
# Grafana → http://localhost:3001
# Prometheus → http://localhost:9090
```

事前プロビジョニング済みダッシュボード同梱。syslenz は Prometheus（`--prometheus`）または OpenTelemetry（`--otel`）でメトリクスをエクスポート。Grafana プロファイルがスクレイピングとダッシュボードプロビジョニングを自動設定。

---

## アーキテクチャ

```
CLI 引数 / config.toml
        |
        v
    +--------+       +----------+       +-----------+
    | main() | ----> | Snapshot  | ----> | TUI / Web |
    +--------+       | .capture()| <---- | render()  |
        |            +----------+       +-----------+
        |                 |
        v                 v
   +---------+     +------------+
   | Remote  |     | パーサー   |
   | (SSH /  |     | /proc (43) |
   |  Docker |     | /sys  (3)  |
   |  TCP)   |     | net   (5)  |
   +---------+     | plugins    |
                   +------------+
                         |
                         v
                 +---------------+
                 | エクスポート  |
                 | JSON / OTEL   |
                 | Prometheus    |
                 +---------------+
```

各パーサーは `/proc` または `/sys` ファイルを読み込み、型付き値（`Bytes`、`Integer`、`Float`、`Duration`、`Text`、`Table`）を持つ `Vec<Field>` を返します。`Snapshot` 構造体が全パーサー出力を1時点のキャプチャにまとめます。TUI と Web UI は共有の `ViewData` 構造体からレンダリング。diff エンジンは型対応の閾値で2スナップショットを比較。

詳細は [`docs/ja/architecture.md`](docs/ja/architecture.md) を参照。

---

## ドキュメント

| ドキュメント | 日本語 | English |
|-------------|--------|---------|
| はじめに | [`docs/ja/getting-started.md`](docs/ja/getting-started.md) | [`docs/en/getting-started.md`](docs/en/getting-started.md) |
| アーキテクチャ | [`docs/ja/architecture.md`](docs/ja/architecture.md) | [`docs/en/architecture.md`](docs/en/architecture.md) |
| 設定 | [`docs/ja/config.md`](docs/ja/config.md) | [`docs/en/config.md`](docs/en/config.md) |
| ダッシュボード | [`docs/ja/dashboard.md`](docs/ja/dashboard.md) | [`docs/en/dashboard.md`](docs/en/dashboard.md) |
| 診断 | [`docs/ja/diagnostics.md`](docs/ja/diagnostics.md) | [`docs/en/diagnostics.md`](docs/en/diagnostics.md) |
| リモート監視 | [`docs/ja/remote.md`](docs/ja/remote.md) | [`docs/en/remote.md`](docs/en/remote.md) |
| プラグイン | [`docs/ja/plugins.md`](docs/ja/plugins.md) | [`docs/en/plugins.md`](docs/en/plugins.md) |

---

## コントリビュート

```bash
git clone https://github.com/opaopa6969/syslenz.git
cd syslenz
cargo build
cargo test
cargo run
```

PR 提出前にチェックを実行:

```bash
cargo fmt --check && cargo clippy && cargo test
```

### リリース手順

1. `Cargo.toml` の `version` を更新
2. `CHANGELOG.md` に `## [x.y.z] - YYYY-MM-DD` エントリを追加
3. コミット、タグ、プッシュ — リリースワークフローが Linux (x86_64, aarch64, musl)、macOS、Windows のバイナリをビルドし、SHA256 チェックサムを生成して crates.io に公開。

---

## ロードマップ

| 機能 | 状態 |
|------|------|
| TUI ダッシュボード、クラシック、診断、カテゴリガイド | リリース済み (v1.0) |
| アラートシステム、タイムトラベル diff、マルチホスト | リリース済み (v1.1–v1.2) |
| Prometheus エクスポート、GPU メトリクス、クロスプラットフォーム | リリース済み (v1.3) |
| 教育機能: チュートリアル、ブレッドクラム、ヒント、SDK | リリース済み (v1.4) |
| HTTP API v1、設定 GUI (`/settings`) | リリース済み (v1.4) |
| Fleet View (`/fleet`) — マルチホスト Web ダッシュボード | **計画中**（未実装） |
| Web 認証（Basic Auth / Token） | **計画中**（未実装） |

---

## MCP (Model Context Protocol)

syslenz は MCP サーバとして動作し、volta ファサード経由でエージェントがシステム監視能力を利用できる。

- **namespace**: `syslenz`
- **エンドポイント**: `http://<host>:3009/mcp` (Streamable HTTP, JSON-RPC 2.0)
- **tools**: `snapshot`, `history`, `sources`, `view`, `field_help`, `article`, `diagnostics`, `get_settings`, `set_alerts`
- **resources**: `syslenz://spec`, `syslenz://guide`, `syslenz://sources`, `syslenz://diagnostics`, `syslenz://metric-kinds`
- **仕様**: `syslenz://spec` resource を参照

### 起動

```sh
syslenz --web 3009 --lang en
```

MCP エンドポイントは Web サーバと同じプロセス・同じポートで提供される。

### volta 参加状況

- volta catalog に `syslenz` として登録済み（prod: 3009）
- MCP 化により `mcp` 項が有効化され、ファサード経由で `syslenz__*` tools が利用可能

---

## ライセンス

MIT

---

v1.4.0 | [変更履歴](CHANGELOG.md) | [GitHub](https://github.com/opaopa6969/syslenz)
