---
version: v1.4.0
lang: ja
---

# アーキテクチャ

[English](../en/architecture.md)

[← Index](index.md)

---

## 目次

- [概要](#概要)
- [技術スタック](#技術スタック)
- [モジュールマップ](#モジュールマップ)
- [データモデル](#データモデル)
- [データフロー](#データフロー)
- [TUI レイヤー (ratatui)](#tui-レイヤー-ratatui)
- [HTTP サーバーレイヤー (axum)](#http-サーバーレイヤー-axum)
- [TCP サーバーレイヤー](#tcp-サーバーレイヤー)
- [リモート監視](#リモート監視)
- [diff エンジン](#diff-エンジン)
- [i18n とヘルプシステム](#i18n-とヘルプシステム)
- [設定 GUI](#設定-gui)
- [プラグインシステム](#プラグインシステム)
- [フィーチャーフラグ](#フィーチャーフラグ)
- [ロードマップ: Fleet View と認証](#ロードマップ-fleet-view-と認証)

---

## 概要

syslenz は単一バイナリの Rust ツールで、Linux カーネルのデータを `/proc` と `/sys` から読み込み、型付きフィールドに構造化し、複数のフロントエンドでレンダリングします: ターミナル UI (TUI)、Web ダッシュボード、JSON エクスポートパイプライン。

```
CLI 引数 / config.toml
        │
        ▼
    ┌────────┐       ┌──────────┐       ┌───────────┐
    │ main() │──────▶│ Snapshot  │──────▶│ TUI / Web │
    └────────┘       │ .capture()│◀──────│ render()  │
        │            └──────────┘       └───────────┘
        │                 │
        ▼                 ▼
   ┌─────────┐     ┌────────────┐
   │ Remote  │     │ パーサー   │
   │ (SSH /  │     │ /proc (43) │
   │  Docker │     │ /sys  (3)  │
   │  TCP)   │     │ net   (5)  │
   └─────────┘     │ plugins    │
                   └────────────┘
                         │
                         ▼
                 ┌───────────────┐
                 │ エクスポート  │
                 │ JSON / OTEL   │
                 │ Prometheus    │
                 └───────────────┘
```

---

## 技術スタック

| コンポーネント | クレート / 技術 | 備考 |
|--------------|---------------|------|
| 言語 | Rust、edition 2024 | rustc 1.85+ が必要 |
| TUI | ratatui 0.29 | ターミナルレンダリング |
| ターミナル I/O | crossterm 0.28 | 入力イベント、raw モード |
| HTTP サーバー | axum 0.8 + tokio 1 | オプション（`web` feature） |
| SSE ストリーミング | tokio-stream、tower-http | Web UI リアルタイム更新 |
| OpenTelemetry | opentelemetry 0.28 + OTLP | オプション（`otel` feature） |
| シリアライゼーション | serde 1 + serde_json 1 | スナップショット JSON I/O |
| 設定 | toml 0.8 | `~/.config/syslenz/config.toml` |
| エラー処理 | anyhow 1 | |
| X11 ウィジェット | x11rb | オプション（`x11widget` feature） |

---

## モジュールマップ

```
src/
├── main.rs                 CLI パース（外部クレートなし）、TUI イベントループ
├── config.rs               Config 構造体、TOML ローディング、CLI オーバーライドマージ
├── proc/
│   ├── mod.rs              コア型: Snapshot, ProcEntry, Field, FieldValue
│   │                       Snapshot::capture()、diff_snapshots()
│   ├── meminfo.rs          /proc/meminfo
│   ├── cpuinfo.rs          /proc/cpuinfo
│   ├── stat.rs             /proc/stat
│   ├── uptime.rs           /proc/uptime
│   ├── loadavg.rs          /proc/loadavg
│   ├── vmstat.rs           /proc/vmstat（165 フィールド）
│   ├── net_dev.rs          /proc/net/dev
│   ├── net_tcp.rs          /proc/net/tcp
│   ├── net_udp.rs          /proc/net/udp
│   ├── net_snmp.rs         /proc/net/snmp
│   ├── net_netstat.rs      /proc/net/netstat
│   ├── processes.rs        /proc/[pid]/stat, status, cmdline
│   ├── pressure.rs         /proc/pressure/{cpu,memory,io}（PSI）
│   ├── ...                 （他 38 パーサー）
│   ├── platform_macos.rs   macOS: sysctl, vm_stat, netstat, launchd（24 ソース）
│   └── platform_windows.rs Windows: PowerShell, WMI（24 ソース）
├── ui/
│   ├── mod.rs              UI モジュールエクスポート
│   ├── app.rs              App 状態、ナビゲーション、マルチホストタブロジック
│   ├── render.rs           全ビューの ratatui レンダリング
│   └── graph.rs            スパークラインとバーグラフのレンダリング
├── export.rs               JSON スナップショット import/export、タイムシリーズエクスポート
├── remote.rs               SSH リモートキャプチャ、Docker exec キャプチャ
├── serve.rs                TCP サーバー (--serve)、SNAPSHOT + METRICS プロトコル
├── web.rs                  Axum HTTP サーバー、全ルート、設定 GUI HTML
├── alert.rs                AlertRule、条件パーサー、デバウンスステートマシン
├── diagnostics.rs          27 チェック関数、DiagnosticResult、related_metrics
├── education.rs            カテゴリガイドコンテンツ、学習パス
├── history.rs              タイムトラベル diff 用スナップショットリングバッファ
├── i18n.rs                 EN/JA フィールド説明（4 ヘルプレベル）
├── otel.rs                 OpenTelemetry OTLP エクスポート、Prometheus テキスト形式
├── prometheus.rs           Prometheus /metrics format_prometheus()
├── metric_kind.rs          MetricKind 列挙型（8 バリアント）
├── common_metric.rs        CommonMetric 列挙型（15 クロスプラットフォームメトリクス）
├── schema/                 エクスポート形式の JSON スキーマ
└── plugin/                 プラグインローダー（実行ファイル stdout → ProcEntry）
```

---

## データモデル

### コア型

```rust
/// 単一データソース（例: "meminfo"、"loadavg"）
pub struct ProcEntry {
    pub name: String,
    pub fields: Vec<Field>,
}

/// ソース内の1フィールド
pub struct Field {
    pub name: String,
    pub value: FieldValue,
    pub unit: Option<String>,
}

/// 型付きフィールド値
pub enum FieldValue {
    Bytes(u64),
    Integer(i64),
    Float(f64),
    Duration(f64),      // 秒
    Text(String),
    Table(Vec<Vec<String>>),
}

/// 全ソースのある時点のキャプチャ
pub struct Snapshot {
    pub timestamp: SystemTime,
    pub entries: Vec<ProcEntry>,
}
```

### MetricKind (v1.4)

`MetricKind` は任意のフィールドを SDK と OTEL 用の 8 バリアントに分類:

`Counter`、`Gauge`、`Histogram`、`Summary`、`StateSet`、`Info`、`GaugeHistogram`、`Unknown`

### CommonMetric (v1.4)

Linux、macOS、Windows で利用可能な 15 のクロスプラットフォームメトリクス: CPU 使用率、メモリ使用量/利用可能量、スワップ使用量、ディスク読書きバイト、ネットワーク rx/tx バイト、ロードアベレージ、アップタイム、プロセス数、オープン FD 数、CPU 温度、ディスク使用率。

---

## データフロー

1. `main()` が CLI フラグをパースし `config.toml` を読み込む
2. `Snapshot::capture()` が有効な全パーサーを順番に呼び出す
3. 結果をアクティブなフロントエンドに渡す:
   - **TUI**: `App` が現在のスナップショットを保持; `render()` が毎ティックで描画
   - **Web**: `AppState` が現在 + 履歴を保持; SSE がブラウザに更新をプッシュ
   - **エクスポート**: `export_json()` がファイルに書き込んで終了
4. 各更新ティックで新しい `Snapshot` をキャプチャし直前のものと diff

---

## TUI レイヤー (ratatui)

TUI は `main.rs` の crossterm raw モードイベントループで動作。全レンダリングロジックは `src/ui/render.rs` にあります。

### ビュー

| ビュー | キー | 説明 |
|--------|------|------|
| ダッシュボード | `D` | バーグラフとスパークライン付き全幅概要 |
| クラシック（概要） | `O` | サイドバー（ソース一覧）+ 詳細パネル |
| 診断 | `X` | 自動診断結果とジャンプナビゲーション |
| カテゴリガイド | `C` | トピック別教育コンテンツ |
| ウェルカム | `W` | キーバインド一覧とヒント |
| Diff | `d` | 2スナップショットの並列またはデルタ diff |
| グラフ | `g` | 選択フィールドのスパークライン履歴 |

### ViewData

全ビューは `ViewData` 構造体を共有し、Web UI が `/api/view` から取得します。レンダリングロジックを複製せずに TUI と Web UI の表示を同一に保ちます。

### マルチホストタブ

`App.hosts: Vec<HostState>` が接続ごとに1つの `HostState` を追跡。`F1`–`F9` が `App.active_host` を切り替え。各 `HostState` は独自のスナップショット履歴、diff ターゲット、接続ステータスを管理。

---

## HTTP サーバーレイヤー (axum)

`--web [port]`（デフォルト 3000）で起動（`web` feature が必要）。`src/web.rs` に実装。

### ルート

| メソッド | パス | ハンドラ |
|---------|------|---------|
| GET | `/` | `index_handler` — Web UI SPA |
| GET | `/api/snapshot` | `snapshot_handler` — 現在のスナップショット JSON |
| GET | `/api/history` | `history_handler` — スナップショット履歴配列 |
| GET | `/api/sources` | `sources_handler` — 利用可能なソース名 |
| GET | `/api/stream` | `sse_handler` — SSE ライブストリーム |
| GET | `/api/view` | `view_handler` — レンダリング済み ViewData |
| GET | `/api/field-help` | `field_help_handler` — 指定レベルのフィールド説明 |
| GET | `/settings` | `settings_page_handler` — 設定 GUI HTML |
| GET | `/api/v1/settings` | `settings_api_handler` — 設定 JSON |
| POST | `/api/v1/settings/alerts` | `settings_alerts_handler` — アラートルール書込 |

全 `/api/v1/*` レスポンスには `X-Syslenz-API-Version: 1` を含みます。

### AppState

```rust
struct AppState {
    current: Mutex<Snapshot>,
    history: Mutex<Vec<Snapshot>>,
    tx: broadcast::Sender<String>,   // SSE チャンネル
    locale: Locale,
    config_path: Option<PathBuf>,
    alert_rules: Mutex<Vec<AlertRule>>,
    history_config: HistoryTomlConfig,
    diagnostic_runbooks: Vec<RunbookConfig>,
}
```

### セキュリティ

現リリースの HTTP サーバーには認証機能がありません。ループバックでの使用は安全です。ネットワーク公開環境ではループバックのみにバインドするか、TLS と認証を持つリバースプロキシを前段に置いてください。

---

## TCP サーバーレイヤー

`--serve [bind_addr]`（デフォルト `0.0.0.0:9100`）が `src/serve.rs` で軽量 TCP サーバーを起動。

プロトコル: 接続ごとに1コマンド、プレーンテキスト。

| コマンド | レスポンス |
|---------|-----------|
| `SNAPSHOT\n` | JSON エンコードの `Snapshot`、その後 `\n` |
| `METRICS\n` | Prometheus テキスト形式 |

SDK（`syslenz4j`、`syslenz4py`、`syslenz4node`）がこのエンドポイントに接続。

**セキュリティ**: 認証なし。共有やインターネット公開ホストでは `127.0.0.1:9100` にバインドしてください。

---

## リモート監視

| モード | CLI フラグ | 実装 |
|--------|-----------|------|
| SSH | `--ssh user@host` | `remote.rs`: リモートホストで `syslenz --export /dev/stdout` を SSH 経由で実行し JSON をストリーム |
| Docker | `--docker container` | `remote.rs`: `docker exec` 相当 |
| TCP | `--connect host:port` | `--serve` インスタンスに接続 |

同種のフラグを複数組み合わせ可能; それぞれが1つの `HostState` エントリを作成。

---

## diff エンジン

`proc/mod.rs` の `diff_snapshots(old: &Snapshot, new: &Snapshot) -> Vec<DiffItem>`。

- `ProcEntry.name` でエントリ、`Field.name` でフィールドをマッチ
- 型対応比較: `Bytes` と `Integer` は設定可能な閾値（デフォルト 0）; `Float` は 0.001 イプシロン
- フィールドごとに `Added`、`Removed`、`Changed(old, new)` を返す
- タイムトラベル diff: `HostState.diff_target_index` が比較対象の履歴スナップショットを選択

---

## i18n とヘルプシステム

全フィールド説明は `src/i18n.rs` に `(source_name, field_name)` をキーとして格納。

各フィールドに最大 4 レベル:
- **OFF**: ヘルプパネルなし
- **NORMAL**: 1行説明
- **DETAILED**: コンテキスト付き 2〜4 文
- **EXTRA**: 完全説明 + SEE ALSO クロスリンク + 学習ブレッドクラム

`L` キーで実行時に `en` と `ja` のロケールを切替。600 フィールドが英語説明を持ち; 521（86%）が日本語翻訳付き。

---

## 設定 GUI

`/settings` が自己完結型 HTML ページ（インライン JS/CSS、外部 CDN なし）を提供。ページの動作:

1. `GET /api/v1/settings` で現在の設定を取得
2. 編集可能なアラートルールテーブルをレンダリング
3. `POST /api/v1/settings/alerts` で変更を送信
4. `settings_alerts_handler` が更新された `[[alert]]` セクションを `config_path` に書き込み、`AppState` 内の `alert_rules` をメモリ内で再読込

アラートルールの変更は再起動不要で即時反映。

---

## プラグインシステム

プラグインは `~/.config/syslenz/plugins/` に置く実行ファイル。各キャプチャ時:

1. syslenz が各プラグインバイナリを実行
2. プラグインが `ProcEntry` 互換オブジェクトの JSON 配列を stdout に書き出す
3. syslenz がこれらを追加の `ProcEntry` 値として `Snapshot` に追加

同梱サンプル: `plugins/jvm/`（jstat + jcmd）、`plugins/examples/docker-stats.sh`。

---

## フィーチャーフラグ

| フラグ | 追加内容 | クレート |
|-------|---------|---------|
| `web`（デフォルト有効） | HTTP サーバー、設定 GUI | axum, tokio, tower-http, tokio-stream |
| `otel` | OTLP エクスポート、Prometheus エンドポイント | opentelemetry, opentelemetry_sdk, opentelemetry-otlp, tokio |
| `x11widget` | X11 フローティングウィジェット | x11rb |

全機能付きビルド:

```bash
cargo build --release --features "web,otel,x11widget"
```

---

## ロードマップ: Fleet View と認証

**Fleet View**（`/fleet`）は複数の監視ホストのステータスマトリクスをブラウザの単一画面に表示し、ホストごとのメトリクスサマリーと自動更新を提供するよう設計されています。**現時点では未実装**です。

**認証**（Basic Auth と Token Auth、`config.toml` の `[web]` セクションで設定）は HTTP サーバーのセキュリティレイヤーとして設計されています。**現時点では未実装**です。

これらの機能は次のメジャーリリースの主要ターゲットです。
