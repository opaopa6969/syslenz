# DGE Session 006: G8 + G9 — OTEL ユースケース整理 & 設定ファイル設計

- **Date**: 2026-03-28
- **Theme**: OpenTelemetry export の立ち位置確定 + config.toml の設計
- **Parent Gaps**: G8 (OTEL ユースケースの整理が必要, Message gap, Low), G9 (設定ファイルがない, Missing logic, Medium)
- **Characters**: ヤン (simplifier) + 今泉 (questioner) + 僕 (scope reducer)

---

## Scene 1: OTEL — 誰が、なぜ使うのか

先輩 (ナレーション): syslenz には `--otel [endpoint]` フラグがある。ヘッドレスモードで起動し、全 43 ソースの数値フィールド (Bytes, Integer, Float, Duration) を OTLP gauge メトリクスとして gRPC/tonic 経由で push する。feature gate `otel` の背後にあり、`cargo build --features otel` でビルドする。現在のメトリクス命名規則は `syslenz.{source}.{field}` (例: `syslenz.meminfo.mem_total`, `syslenz.loadavg.load_1m`)。endpoint のデフォルトは `http://localhost:4317`、interval のデフォルトは 5 秒。

👤 今泉: 「すみません、基本的な質問なんですけど... これ、誰が使うんですか？ Prometheus + node_exporter でメモリも CPU も見れますよね？ syslenz の OTEL export で何が嬉しいんですか？」

→ **Gap 発見: node_exporter との差別化が言語化されてない。OTEL export の独自価値が不明。**

☕ ヤン: 「node_exporter は /proc の "一部" しか見てないよ。meminfo, stat, loadavg, net/dev あたり。syslenz は 43 ソース全部。buddyinfo, slabinfo, pressure, net/tcp のコネクション数... node_exporter にないメトリクスが大量にある。でも逆に、全部 export して意味ある？ 数百メトリクスがドバッと出たら Grafana のダッシュボード作る人が困るでしょ。紅茶ください。」

→ **Gap 発見: メトリクスの "層" がない。全部フラットに export してるので、「まず見るべき 10 個」と「深掘り用の残り」の区別がない。**

😰 僕: 「...あの、ユースケースを 3 つに絞りません...？ (1) node_exporter の補完 — slabinfo や pressure など node_exporter にないメトリクスだけ export する、(2) node_exporter の代替 — syslenz 1 本で全メトリクスをカバーする、(3) 一時的なデバッグ — 障害調査中だけ起動して詳細メトリクスを Grafana に送る... この 3 つのどれかに絞れば...」

→ **Spec implication: ユースケースは (3) の「一時的デバッグ」が最もユニーク。node_exporter は常駐デーモン、syslenz は ssh して即起動・即停止。この差別化を文書化すべき。**

👤 今泉: 「(3) なら、使い方は "障害発生 → ssh → syslenz --otel --interval 1 → Grafana で見る → Ctrl+C で止める" ですよね。これ、README に書いてあります？」

→ **Gap 発見: OTEL の Quick Start ガイドがない。「docker-compose で OTel Collector + Prometheus + Grafana を立てて syslenz --otel する」までの手順が未文書化。**

☕ ヤン: 「あと、メトリクス名。`syslenz.meminfo.mem_total` って OpenTelemetry の命名規則 (semantic conventions) に沿ってないよ。`system.memory.total` とかにしたほうが Grafana のテンプレートダッシュボードと互換性がある。... まあ、独自名のほうが他ツールと衝突しないけど。」

→ **Spec implication: メトリクス名は `syslenz.*` プレフィックスを維持する（他ツールとの衝突回避）。ただし主要メトリクスには OpenTelemetry semantic conventions へのマッピングをドキュメントに記載する。**

---

## Scene 2: Config — 何を入れるか

先輩 (ナレーション): 現在 `toml = "0.8"` が依存に入っているが、設定ファイルは一切使われていない。全パラメータは CLI 引数のみ。CLI 引数は --export, --export-series, --import, --ssh, --lang, --otel, --web, --widget, --interval, --count。

👤 今泉: 「設定ファイルって、具体的に何を設定するんですか？ --lang ja を毎回打つのが面倒、以上のモチベーションってあるんですか？」

→ **Gap 発見: 設定ファイルに入れるべき項目のリストが未整理。**

☕ ヤン: 「CLI 引数を見てみようよ。毎回同じ値を指定するものが config に行く。`--lang` は毎回同じ。`--interval` もデフォルト変えたい人がいる。`--otel` の endpoint も固定。`--web` の port も。逆に `--export <file>` とか `--import <file>` は毎回違うから config に入れる意味ない。簡単でしょ。」

→ **Spec implication: config に入れるべき項目 = 「毎回同じ値を指定する可能性があるもの」。一時的なファイルパスは対象外。**

😰 僕: 「...でも、config の項目を増やしすぎると... メンテが... 僕が辛い... 最小限にしません...？」

→ **Spec implication: MVP の config は 4 セクション以内、合計 10 キー以内。**

👤 今泉: 「あと、config の優先順位ってどうなるんですか？ config.toml に `lang = "ja"` って書いてあって、CLI で `--lang en` って指定したら、どっちが勝つんですか？」

→ **Spec implication: 優先順位は CLI 引数 > 環境変数 > config.toml > デフォルト値。これは Rust CLI ツールの標準的な慣例。**

☕ ヤン: 「パスは `~/.config/syslenz/config.toml`。XDG_CONFIG_HOME を尊重するのが Linux のお作法。あとファイルがなくても動く。初回起動で雛形を生成する `syslenz --init-config` があると親切かな。でも MVP ではなくてもいい。」

→ **Spec implication: パスは `$XDG_CONFIG_HOME/syslenz/config.toml`、fallback は `~/.config/syslenz/config.toml`。ファイルが存在しなければ全デフォルトで動作。**

😰 僕: 「...Table 型のネストは... 最小限で... TOML の [section] だけで...」

→ **Spec implication: config 構造はフラットな TOML セクション。ネストは 1 段まで。**

---

## Scene 3: 両方の MVP

先輩 (ナレーション): G8 と G9 の MVP を具体的に定義する。

### OTEL の MVP

👤 今泉: 「MVP として最低限必要なのは何ですか？」

☕ ヤン: 「3 つ。(1) ドキュメント — 使い方と主要メトリクス一覧。(2) docker-compose.yml — OTel Collector + Prometheus + Grafana の一発セットアップ。(3) Grafana ダッシュボード JSON — import したらすぐ使える。コードの変更はゼロでいい。今の実装で十分動く。」

→ **Spec implication: G8 の解決は「ドキュメント + 周辺ファイル」のみ。otel.rs のコード変更は不要。**

😰 僕: 「...コード変更なしなら... 安心...」

👤 今泉: 「export されるメトリクスの一覧って、どこかにあります？」

☕ ヤン: 「ないよ。otel.rs を見ると、全 ProcEntry の全 Field をループして、FieldValue が Bytes, Integer, Float, Duration のどれかなら gauge として export してる。つまりメトリクス名は動的に決まる。一覧を出すには実際に動かすか、各パーサーのソースを読むしかない。」

→ **Gap 発見: export されるメトリクス名の静的な一覧がない。ドキュメントに主要メトリクス表が必要。**

### Config の MVP

👤 今泉: 「config の MVP は？」

☕ ヤン: 「4 セクション。`[general]` に lang と interval。`[otel]` に endpoint と interval。`[web]` に port。`[display]` に表示するソースのフィルタ。合計 7 キー。これだけ。」

→ **Spec implication: 以下の config.toml 構造を採用。**

😰 僕: 「...実装は... config.rs を 1 ファイル追加して、main.rs の引数パース前に読み込むだけ... ですよね...？」

→ **Spec implication: 実装は config.rs (新規) + main.rs (読み込み追加) の 2 ファイル変更。toml crate は既存。**

---

## Gap Summary (Session 006)

| # | Gap | Category | Severity | Observe / Suggest / Act |
|---|-----|----------|----------|-------------------------|
| G8-1 | node_exporter との差別化が未言語化 | Message gap | Low | Observe: 43 ソース vs node_exporter の ~15 ソース → Suggest: 差分メトリクスを明示 → Act: ドキュメントに比較表 |
| G8-2 | メトリクスに「層」がない | Design gap | Low | Observe: 全メトリクスがフラット → Suggest: "core" (10 個) と "extended" (残り) の分類 → Act: 将来の `--otel-level core\|full` フラグ (MVP 外) |
| G8-3 | OTEL Quick Start がない | Message gap | Medium | Observe: docker-compose も手順もない → Suggest: docs/otel.md + docker-compose.yml + Grafana JSON → Act: 下記ドキュメント |
| G8-4 | メトリクス名の静的一覧がない | Message gap | Low | Observe: 動的生成のため一覧不在 → Suggest: 主要 30 メトリクスの表を作成 → Act: docs/otel.md に記載 |
| G9-1 | config に入れるべき項目が未整理 | Design gap | Medium | Observe: CLI 引数のみ → Suggest: 永続的設定を config に → Act: 下記 config.toml 仕様 |
| G9-2 | 優先順位ルールが未定義 | Design gap | Medium | Observe: config と CLI の競合解決なし → Suggest: CLI > env > config > default → Act: config.rs で実装 |
| G9-3 | XDG_CONFIG_HOME 未対応 | Design gap | Low | Observe: パスが固定 → Suggest: XDG 準拠 → Act: config.rs で `$XDG_CONFIG_HOME` を確認 |

---

## Spec: config.toml の構造

### ファイルパス

```
$XDG_CONFIG_HOME/syslenz/config.toml
~/.config/syslenz/config.toml  (fallback)
```

ファイルが存在しなければ全てデフォルト値で動作する。エラーにはしない。

### 優先順位

```
CLI 引数  >  環境変数 (SYSLENZ_*)  >  config.toml  >  デフォルト値
```

### config.toml の雛形

```toml
# syslenz configuration file
# Place at: ~/.config/syslenz/config.toml

[general]
# 表示言語: "en" | "ja"
lang = "en"

# リフレッシュ間隔 (ミリ秒)。TUI の auto-refresh で使用。
interval_ms = 1000

# 起動時に表示するソースのフィルタ (空 = 全ソース表示)
# sources = ["meminfo", "loadavg", "stat", "net/dev", "processes", "pressure"]

[otel]
# OTLP gRPC endpoint
endpoint = "http://localhost:4317"

# メトリクス push 間隔 (秒)
interval_secs = 5

[web]
# Web UI の listen ポート
port = 3000

[ssh]
# デフォルトの SSH ホスト (省略可)
# host = "user@192.168.1.100"
```

### Rust 構造体 (config.rs)

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub otel: OtelConfig,
    #[serde(default)]
    pub web: WebConfig,
    #[serde(default)]
    pub ssh: SshConfig,
}

#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    pub lang: Option<String>,        // "en" | "ja"
    pub interval_ms: Option<u64>,    // default: 1000
    pub sources: Option<Vec<String>>, // default: all
}

#[derive(Debug, Deserialize)]
pub struct OtelConfig {
    pub endpoint: Option<String>,     // default: "http://localhost:4317"
    pub interval_secs: Option<u64>,   // default: 5
}

#[derive(Debug, Deserialize)]
pub struct WebConfig {
    pub port: Option<u16>,            // default: 3000
}

#[derive(Debug, Deserialize)]
pub struct SshConfig {
    pub host: Option<String>,         // default: None
}

impl Config {
    pub fn load() -> Self {
        let path = config_path();
        match std::fs::read_to_string(&path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }
}

fn config_path() -> std::path::PathBuf {
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        std::path::PathBuf::from(xdg).join("syslenz/config.toml")
    } else if let Ok(home) = std::env::var("HOME") {
        std::path::PathBuf::from(home).join(".config/syslenz/config.toml")
    } else {
        std::path::PathBuf::from("config.toml")
    }
}
```

### main.rs への統合 (差分イメージ)

```rust
mod config;

fn main() -> Result<()> {
    let cfg = config::Config::load();
    let args: Vec<String> = std::env::args().collect();

    // 例: --lang の解決 (CLI > config > default)
    let locale = if let Some(pos) = args.iter().position(|a| a == "--lang") {
        i18n::Locale::from_str(args.get(pos + 1).unwrap())
    } else if let Some(ref lang) = cfg.general.lang {
        i18n::Locale::from_str(lang)
    } else {
        i18n::Locale::En
    };

    // 例: --otel endpoint の解決
    // CLI 引数があればそれ、なければ config、なければデフォルト
    // ... 同様のパターンを interval, port, ssh host に適用
}
```

---

## Spec: OTEL ユースケースドキュメント

### 想定ドキュメント: docs/otel.md

#### 概要

syslenz の `--otel` モードは、ヘッドレス (TUI なし) で起動し、/proc の全数値メトリクスを OpenTelemetry Protocol (OTLP) で gRPC push する。node_exporter の代替ではなく、**一時的な深掘りデバッグツール**として位置づける。

#### ユースケース

| シナリオ | 説明 |
|----------|------|
| 障害時のメトリクス深掘り | ssh → `syslenz --otel --interval 1` → Grafana で slabinfo, pressure, buddyinfo を確認 → Ctrl+C で停止 |
| node_exporter の補完 | 常駐の node_exporter ではカバーしない /proc ソースを一時的に export |
| CI/CD パイプラインの計測 | テスト実行中の memory pressure をキャプチャして性能リグレッションを検知 |

#### node_exporter との比較

| メトリクス | node_exporter | syslenz |
|-----------|:---:|:---:|
| meminfo (MemTotal, MemFree, ...) | Yes | Yes |
| loadavg | Yes | Yes |
| stat (CPU) | Yes | Yes |
| net/dev | Yes | Yes |
| diskstats | Yes | Yes |
| **pressure (PSI)** | No (*) | **Yes** |
| **buddyinfo** | No | **Yes** |
| **slabinfo** | No (*) | **Yes** |
| **net/tcp (接続数)** | No | **Yes** |
| **net/sockstat** | No | **Yes** |
| **vmstat (全フィールド)** | Partial | **Yes (全 ~100 フィールド)** |
| **zoneinfo** | No | **Yes** |
| **schedstat** | No | **Yes** |
| **timer_list** | No | **Yes** |
| **interrupts (全 IRQ)** | No | **Yes** |

(*) node_exporter にも collector はあるが、デフォルト無効

#### 主要メトリクス一覧 (抜粋)

**メモリ系** (`syslenz.meminfo.*`)
| メトリクス名 | 型 | 説明 |
|-------------|-----|------|
| `syslenz.meminfo.mem_total` | Bytes (gauge) | 物理メモリ総量 |
| `syslenz.meminfo.mem_free` | Bytes (gauge) | 空きメモリ |
| `syslenz.meminfo.mem_available` | Bytes (gauge) | 利用可能メモリ |
| `syslenz.meminfo.buffers` | Bytes (gauge) | バッファキャッシュ |
| `syslenz.meminfo.cached` | Bytes (gauge) | ページキャッシュ |
| `syslenz.meminfo.swap_total` | Bytes (gauge) | スワップ総量 |
| `syslenz.meminfo.swap_free` | Bytes (gauge) | スワップ空き |
| `syslenz.meminfo.slab` | Bytes (gauge) | カーネル SLAB 総量 |

**CPU / 負荷系** (`syslenz.loadavg.*`, `syslenz.stat.*`)
| メトリクス名 | 型 | 説明 |
|-------------|-----|------|
| `syslenz.loadavg.load_1m` | Float (gauge) | 1 分間ロードアベレージ |
| `syslenz.loadavg.load_5m` | Float (gauge) | 5 分間ロードアベレージ |
| `syslenz.loadavg.load_15m` | Float (gauge) | 15 分間ロードアベレージ |
| `syslenz.loadavg.running_processes` | Integer (gauge) | 実行中プロセス数 |
| `syslenz.loadavg.total_processes` | Integer (gauge) | 総プロセス数 |
| `syslenz.stat.cpu_user` | Integer (gauge) | CPU user jiffies |
| `syslenz.stat.cpu_system` | Integer (gauge) | CPU system jiffies |
| `syslenz.stat.context_switches` | Integer (gauge) | コンテキストスイッチ累計 |
| `syslenz.stat.processes_created` | Integer (gauge) | fork 累計 |

**Pressure Stall Information** (`syslenz.pressure.*`)
| メトリクス名 | 型 | 説明 |
|-------------|-----|------|
| `syslenz.pressure.cpu_some_avg10` | Float (gauge) | CPU pressure some avg10 |
| `syslenz.pressure.memory_some_avg10` | Float (gauge) | メモリ pressure some avg10 |
| `syslenz.pressure.memory_full_avg10` | Float (gauge) | メモリ pressure full avg10 |
| `syslenz.pressure.io_some_avg10` | Float (gauge) | I/O pressure some avg10 |
| `syslenz.pressure.io_full_avg10` | Float (gauge) | I/O pressure full avg10 |

**ネットワーク** (`syslenz.net.*`)
| メトリクス名 | 型 | 説明 |
|-------------|-----|------|
| `syslenz.net.dev.{iface}_rx_bytes` | Bytes (gauge) | 受信バイト数 |
| `syslenz.net.dev.{iface}_tx_bytes` | Bytes (gauge) | 送信バイト数 |
| `syslenz.net.sockstat.tcp_inuse` | Integer (gauge) | TCP ソケット使用数 |
| `syslenz.net.sockstat.udp_inuse` | Integer (gauge) | UDP ソケット使用数 |

**ディスク** (`syslenz.diskstats.*`)
| メトリクス名 | 型 | 説明 |
|-------------|-----|------|
| `syslenz.diskstats.{dev}_reads_completed` | Integer (gauge) | 読み取り完了数 |
| `syslenz.diskstats.{dev}_writes_completed` | Integer (gauge) | 書き込み完了数 |
| `syslenz.diskstats.{dev}_read_bytes` | Bytes (gauge) | 読み取りバイト数 |
| `syslenz.diskstats.{dev}_write_bytes` | Bytes (gauge) | 書き込みバイト数 |

**カーネル内部** (node_exporter にない)
| メトリクス名 | 型 | 説明 |
|-------------|-----|------|
| `syslenz.buddyinfo.*` | Integer (gauge) | メモリフラグメンテーション (order 別) |
| `syslenz.slabinfo.{name}_active_objs` | Integer (gauge) | SLAB キャッシュのアクティブオブジェクト数 |
| `syslenz.vmstat.*` | Integer (gauge) | 仮想メモリ統計 (~100 フィールド) |
| `syslenz.zoneinfo.*` | Integer (gauge) | メモリゾーン情報 |
| `syslenz.schedstat.*` | Integer (gauge) | スケジューラ統計 |

#### Quick Start: Prometheus + Grafana で syslenz メトリクスを見る

```bash
# 1. syslenz を OTEL feature 付きでビルド
cargo build --release --features otel

# 2. OTel Collector + Prometheus + Grafana を起動
cd docs/otel-quickstart/
docker-compose up -d

# 3. syslenz の OTEL export を開始
syslenz --otel http://localhost:4317 --interval 5

# 4. Grafana を開く
#    http://localhost:3001 (admin/admin)
#    "syslenz" ダッシュボードが自動で読み込まれる
```

#### docker-compose.yml (docs/otel-quickstart/)

```yaml
version: "3.8"
services:
  otel-collector:
    image: otel/opentelemetry-collector-contrib:latest
    ports:
      - "4317:4317"    # OTLP gRPC
    volumes:
      - ./otel-collector-config.yaml:/etc/otelcol-contrib/config.yaml

  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9090:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3001:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
    volumes:
      - ./grafana/provisioning:/etc/grafana/provisioning
      - ./grafana/dashboards:/var/lib/grafana/dashboards
```

#### otel-collector-config.yaml

```yaml
receivers:
  otlp:
    protocols:
      grpc:
        endpoint: "0.0.0.0:4317"

exporters:
  prometheus:
    endpoint: "0.0.0.0:8889"
    namespace: "syslenz"

service:
  pipelines:
    metrics:
      receivers: [otlp]
      exporters: [prometheus]
```

#### prometheus.yml

```yaml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: "otel-collector"
    static_configs:
      - targets: ["otel-collector:8889"]
```

---

## Next Actions

- [ ] G9: `src/config.rs` を新規作成し、上記 Spec の Config 構造体を実装
- [ ] G9: `src/main.rs` を修正し、CLI 引数解決前に `Config::load()` を呼ぶ
- [ ] G8: `docs/otel.md` を作成 (上記 Spec の内容)
- [ ] G8: `docs/otel-quickstart/` ディレクトリに docker-compose.yml, collector config, prometheus.yml, Grafana ダッシュボード JSON を配置
- [ ] G8-2 (将来): `--otel-level core|full` フラグの検討 (MVP 外)
- [ ] G9 テスト: config.toml がない場合、壊れた TOML の場合、CLI 上書きの場合の 3 パターン
