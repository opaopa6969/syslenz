---
version: v1.0.0
lang: ja
---

# OpenTelemetry 統合

[<- 前: キーバインド](keybindings.md) | [Index](index.md) | [次: データソース ->](sources.md)

[🇬🇧 English](../en/otel.md)

## 目次

- [概要](#概要)
- [セットアップ](#セットアップ)
- [エクスポートされるメトリクス](#エクスポートされるメトリクス)
- [メトリクス命名規則](#メトリクス命名規則)
- [Docker Compose と Prometheus + Grafana](#docker-compose-と-prometheus--grafana)
- [node_exporterとの比較](#node_exporterとの比較)
- [設定](#設定)

## 概要

syslenz は全ての数値システムメトリクスを OpenTelemetry Protocol (OTLP) 互換の任意のバックエンドにエクスポートできます。設定可能な間隔でスナップショットをキャプチャし、gRPC経由でゲージメトリクスをプッシュするヘッドレスデーモン（TUIなし）として動作します。

OTEL機能はコンパイル時のオプトインが必要です:

```bash
cargo build --release --features otel
```

## セットアップ

### 基本的な使い方

```bash
# デフォルトエンドポイント (http://localhost:4317) に5秒間隔でエクスポート
syslenz --otel

# カスタムエンドポイント
syslenz --otel http://otel-collector.example.com:4317

# カスタム間隔 (10秒)
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

syslenz は全データソースの**全ての数値フィールド**をOTLPゲージメトリクスとしてエクスポートします:

- 全ての `Bytes` フィールド（`f64` としてエクスポート）
- 全ての `Integer` フィールド（`f64` としてエクスポート）
- 全ての `Float` フィールド
- 全ての `Duration` フィールド（秒単位、`f64` として）

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
- その他55以上のソースからの全て

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

全メトリクスはOTLPゲージ（カウンターやヒストグラムではない）としてエクスポートされます。スナップショット値は時点の状態を表すためです。

## Docker Compose と Prometheus + Grafana

syslenz をメトリクスソースとして使用する完全な監視スタック:

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
| 構造化データ | スキーマ駆動型 | 生のfloat値 |
| TUI | あり（同じバイナリ） | なし |
| 教育コンテンツ | 組み込み | なし |

**syslenz OTELを使うべき場合:**
- 対話的な監視にすでにsyslenzを使用しており、同じデータをダッシュボードにも欲しい
- OTLP対応バックエンド（Datadog、Honeycomb、Grafana Cloudなど）を使用している
- カスタムメトリクス用のプラグインシステムが欲しい

**node_exporterを使うべき場合:**
- 明確に定義された型（カウンター vs ゲージ）の本番向け堅牢なメトリクスが欲しい
- 特定のコレクター（systemd、textfile、NTP）が必要
- 純粋なPrometheusエコシステム内にいる

## 設定

`~/.config/syslenz/config.toml` で:

```toml
[otel]
endpoint = "http://otel-collector:4317"
interval_secs = 10
```

CLIフラグは設定値を上書きします。詳細は[設定リファレンス](config.md)を参照してください。

---

[<- 前: キーバインド](keybindings.md) | [Index](index.md) | [次: データソース ->](sources.md)
