---
version: v1.3.0
lang: ja
---

# OpenTelemetry 統合

[🇬🇧 English](../en/otel.md)

[<- 前: キーバインド](keybindings.md) | [Index](index.md) | [次: データソース ->](sources.md)


## 目次

- [概要](#概要)
- [セットアップ](#セットアップ)
- [エクスポートされるメトリクス](#エクスポートされるメトリクス)
- [メトリクス命名規則](#メトリクス命名規則)
- [Docker Compose と Prometheus + Grafana](#docker-compose-と-prometheus--grafana)
- [node_exporterとの比較](#node_exporterとの比較)
- [Prometheus エクスポート (v1.3.0)](#prometheus-エクスポート-v130)
- [設定](#設定)

## 概要

syslenz は全数値メトリクスを OpenTelemetry Protocol (OTLP) 対応の任意のバックエンドにエクスポートできます。設定した間隔でスナップショットをキャプチャし、gRPC経由でゲージメトリクスをプッシュするヘッドレスデーモン（TUIなし）として動作します。

OTEL機能はコンパイル時のオプトインが必要です:

```bash
cargo build --release --features otel
```

## セットアップ

### 基本的な使い方

```bash
# デフォルトエンドポイント (http://localhost:4317) に5秒間隔でエクスポート

[🇬🇧 English](../en/otel.md)
syslenz --otel

# カスタムエンドポイント

[🇬🇧 English](../en/otel.md)
syslenz --otel http://otel-collector.example.com:4317

# カスタム間隔 (10秒)

[🇬🇧 English](../en/otel.md)
syslenz --otel http://localhost:4317 --interval 10
```

プロセスはフォアグラウンドで実行され、以下を表示します:

```
syslenz: OTEL export to http://localhost:4317 (interval: 5s)
Press Ctrl+C to stop.
```

### OTEL機能がコンパイルされていない場合

```
OpenTelemetry support is not compiled in.
Rebuild with: cargo build --features otel
```

## エクスポートされるメトリクス

syslenz は全データソースの**全数値フィールド**をOTLPゲージメトリクスとしてエクスポートします:

- 全 `Bytes` フィールド（`f64` としてエクスポート）
- 全 `Integer` フィールド（`f64` としてエクスポート）
- 全 `Float` フィールド
- 全 `Duration` フィールド（秒単位、`f64`）

非数値フィールド（`Text`、`Table`）はスキップされます。

55以上のソースを持つ典型的なシステムでは、以下をカバーする数百のメトリクスが生成されます:

- メモリ (MemTotal, MemAvailable, MemFree, Cached, Buffers, SwapTotal, SwapFree 等)
- CPU (cpu_user, cpu_system, cpu_idle, cpu_iowait, cpu_steal, context_switches 等)
- ロード (load1, load5, load15)
- ネットワーク (インターフェースごとの rx_bytes, tx_bytes, rx_packets, tx_packets)
- ディスク (reads_completed, writes_completed, io_time_ms 等)
- プレッシャー (cpu_some_avg10, memory_some_avg10, io_some_avg10 等)
- サーマル (max_temp, ゾーンごとの温度)
- ファイルディスクリプタ (allocated_fds, max_fds, fd_usage_pct)
- その他55以上のソースから全て

## メトリクス命名規則

メトリクスは以下のパターンに従います:

```
syslenz.<source>.<field_name>
```

ソース名のスラッシュはドットに置換されます。例:

| ソース | フィールド | メトリクス名 |
|--------|---------|------------|
| `meminfo` | `MemTotal` | `syslenz.meminfo.MemTotal` |
| `loadavg` | `load1` | `syslenz.loadavg.load1` |
| `net/dev` | `rx_bytes` | `syslenz.net.dev.rx_bytes` |
| `stat` | `cpu_user` | `syslenz.stat.cpu_user` |
| `pressure` | `cpu_some_avg10` | `syslenz.pressure.cpu_some_avg10` |
| `df` | `root_use_pct` | `syslenz.df.root_use_pct` |
| `thermal` | `max_temp` | `syslenz.thermal.max_temp` |

全メトリクスはOTLPゲージ（カウンターやヒストグラムではない）としてエクスポートされます。スナップショット値はその時点の状態を表すためです。

## Docker Compose と Prometheus + Grafana

syslenz をメトリクスソースとした完全な監視スタック:

```yaml
version: "3.8"

services:
  # syslenz OTELエクスポーター
  syslenz:
    build:
      context: .
      args:
        FEATURES: otel
    command: ["syslenz", "--otel", "http://otel-collector:4317", "--interval", "5"]
    pid: host
    privileged: true
    depends_on:
      - otel-collector

  # OpenTelemetry Collector
  otel-collector:
    image: otel/opentelemetry-collector-contrib:latest
    volumes:
      - ./otel-config.yaml:/etc/otelcol-contrib/config.yaml
    ports:
      - "4317:4317"   # OTLP gRPC
      - "8889:8889"   # Prometheusエクスポーター

  # Prometheus
  prometheus:
    image: prom/prometheus:latest
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
    ports:
      - "9090:9090"
    depends_on:
      - otel-collector

  # Grafana
  grafana:
    image: grafana/grafana:latest
    ports:
      - "3001:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
    depends_on:
      - prometheus
```

### otel-config.yaml

```yaml
receivers:
  otlp:
    protocols:
      grpc:
        endpoint: 0.0.0.0:4317

exporters:
  prometheus:
    endpoint: 0.0.0.0:8889

service:
  pipelines:
    metrics:
      receivers: [otlp]
      exporters: [prometheus]
```

### prometheus.yml

```yaml
global:
  scrape_interval: 10s

scrape_configs:
  - job_name: 'syslenz'
    static_configs:
      - targets: ['otel-collector:8889']
```

### Grafanaダッシュボード

スタック起動後:

1. Grafana を `http://localhost:3001` で開く（admin/admin）
2. Prometheusデータソースを追加: `http://prometheus:9090`
3. 以下のようなクエリでダッシュボードを作成:
   - `syslenz_meminfo_MemAvailable` -- メモリ利用可能量の推移
   - `syslenz_loadavg_load1` -- 1分間ロードアベレージ
   - `syslenz_stat_cpu_user` -- CPUユーザー時間
   - `syslenz_pressure_cpu_some_avg10` -- CPUプレッシャー

## node_exporterとの比較

| 機能 | syslenz OTEL | node_exporter |
|------|-------------|---------------|
| プロトコル | OTLP（プッシュ） | Prometheus（プル） |
| メトリクス | 全syslenzソース (55+) | 約100コレクター |
| 型 | 全てゲージ | ゲージ + カウンター |
| セットアップ | 単一バイナリ | 単一バイナリ |
| カスタムソース | プラグインシステム | カスタムコレクター |
| 構造化データ | スキーマ駆動 | 生のfloat値 |
| TUI | あり（同じバイナリ） | なし |
| 教育コンテンツ | 組み込み | なし |

**syslenz OTELが向いている場合:**
- 対話的な監視にすでに syslenz を使っており、同じデータをダッシュボードにも欲しい
- OTLP対応バックエンド（Datadog、Honeycomb、Grafana Cloudなど）を使っている
- カスタムメトリクス用のプラグインシステムが欲しい

**node_exporterが向いている場合:**
- 型が明確に定義された（カウンター vs ゲージ）本番向けの堅牢なメトリクスが欲しい
- 特定のコレクター（systemd、textfile、NTP）が必要
- 純粋なPrometheusエコシステムを使っている

## Prometheus エクスポート (v1.3.0)

v1.3.0 で、OTLP Collector を経由せずに直接 Prometheus 形式でメトリクスを公開する `--prometheus` フラグが追加されました。syslenz 単体で `/metrics` エンドポイントを提供し、Prometheus から直接スクレイプできます。

### 基本的な使い方

```bash
# デフォルトアドレス (0.0.0.0:9464) で /metrics を公開
syslenz --prometheus

# バインドアドレスを指定
syslenz --prometheus 127.0.0.1:9090

# OTEL と同時に使うことも可能
syslenz --otel http://localhost:4317 --prometheus
```

`--prometheus` フラグを指定すると、syslenz は TUI を表示せずにヘッドレスモードで動作し、HTTP サーバーを起動します。内部的には `serve.rs` の METRICS コマンドハンドラがスナップショットから Prometheus text format に変換して応答します。

### /metrics エンドポイント

`http://<bind_addr>/metrics` にアクセスすると、Prometheus text exposition format でメトリクスが返されます:

```
# HELP syslenz_meminfo_MemAvailable Available memory in bytes
# TYPE syslenz_meminfo_MemAvailable gauge
syslenz_meminfo_MemAvailable 8.589934592e+09
# HELP syslenz_loadavg_load1 1-minute load average
# TYPE syslenz_loadavg_load1 gauge
syslenz_loadavg_load1 1.25
# HELP syslenz_stat_cpu_user CPU user time
# TYPE syslenz_stat_cpu_user gauge
syslenz_stat_cpu_user 12345678
...
```

全ての数値フィールドが `syslenz_<source>_<field>` の命名規則でゲージとしてエクスポートされます。ソース名のスラッシュはアンダースコアに置換されます（例: `net/dev` -> `syslenz_net_dev_rx_bytes`）。

### Prometheus + Grafana 連携

OTLP Collector なしで、syslenz を直接 Prometheus でスクレイプする構成です:

```yaml
# prometheus.yml
global:
  scrape_interval: 5s

scrape_configs:
  - job_name: 'syslenz'
    static_configs:
      - targets: ['syslenz-host:9464']
```

Docker Compose の例:

```yaml
version: "3.8"

services:
  syslenz:
    build: .
    command: ["syslenz", "--prometheus", "0.0.0.0:9464"]
    pid: host
    privileged: true
    ports:
      - "9464:9464"

  prometheus:
    image: prom/prometheus:latest
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
    ports:
      - "9090:9090"

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3001:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
    depends_on:
      - prometheus
```

この構成では OTLP Collector が不要なため、セットアップが簡潔になります。OTLP 経由の構成と比較して、コンポーネントが1つ少なくて済みます。

### OTLP と Prometheus の使い分け

| 方式 | 向いている場面 |
|------|-------------|
| `--otel` (OTLP) | Datadog、Honeycomb 等の OTLP 対応バックエンドに送る場合。複数のテレメトリ信号（メトリクス、トレース、ログ）を統合する場合 |
| `--prometheus` | Prometheus を直接使う場合。Collector のセットアップを省きたい場合。既存の Prometheus インフラに追加する場合 |

## 設定

`~/.config/syslenz/config.toml` で:

```toml
[otel]
endpoint = "http://otel-collector:4317"
interval_secs = 10
```

CLIフラグは設定値より優先されます。詳細は[設定リファレンス](config.md)を参照してください。

---

[<- 前: キーバインド](keybindings.md) | [Index](index.md) | [次: データソース ->](sources.md)
