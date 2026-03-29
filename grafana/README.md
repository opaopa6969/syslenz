# syslenz Grafana Dashboard

## JA: セットアップガイド

### 前提条件

- Docker と Docker Compose がインストール済み
- syslenz が `--prometheus` フラグで起動可能

### クイックスタート

1. syslenz + Grafana を一括起動:

```bash
docker compose --profile grafana up -d
```

2. ブラウザで Grafana を開く: http://localhost:3001

3. 初期ログイン情報:
   - ユーザー名: `admin`
   - パスワード: `admin`
   (初回ログイン時にパスワード変更を求められます)

4. データソース `syslenz` とダッシュボード `syslenz Overview` が自動でプロビジョニングされています。左メニューの Dashboards > syslenz フォルダ を開いてください。

### 手動セットアップ (Docker Compose を使わない場合)

1. syslenz を Prometheus エンドポイント付きで起動:

```bash
syslenz --prometheus 0.0.0.0:9101
```

2. Grafana にデータソースを追加:
   - タイプ: Prometheus
   - URL: `http://<syslenz-host>:9101`
   - スクレイプ間隔: 15s

3. `grafana/dashboards/syslenz-overview.json` を Grafana にインポート。

### ダッシュボードの内容

| セクション | メトリクス |
|---|---|
| System Overview | Uptime, プロセス数, FD使用率, ネットワークIF数 |
| Memory | MemTotal/Available/Free/Cached/Buffers, Swap, 使用率ゲージ |
| CPU & Load | CPU使用率, User/System/IOWait内訳, ロードアベレージ, プロセス状態 |
| Network | 合計RX/TX, TCP接続数 |
| Disk I/O | アクティブデバイス数 |
| PSI | CPU/Memory/IO の some/full 各avg10/avg60/avg300 |

---

## EN: Setup Guide

### Prerequisites

- Docker and Docker Compose installed
- syslenz capable of running with `--prometheus` flag

### Quick Start

1. Start syslenz + Grafana together:

```bash
docker compose --profile grafana up -d
```

2. Open Grafana in your browser: http://localhost:3001

3. Default credentials:
   - Username: `admin`
   - Password: `admin`
   (You will be prompted to change the password on first login)

4. The `syslenz` data source and `syslenz Overview` dashboard are auto-provisioned. Navigate to Dashboards > syslenz folder in the left menu.

### Manual Setup (without Docker Compose)

1. Start syslenz with the Prometheus endpoint:

```bash
syslenz --prometheus 0.0.0.0:9101
```

2. Add a data source in Grafana:
   - Type: Prometheus
   - URL: `http://<syslenz-host>:9101`
   - Scrape interval: 15s

3. Import `grafana/dashboards/syslenz-overview.json` into Grafana.

### Dashboard Sections

| Section | Metrics |
|---|---|
| System Overview | Uptime, process count, FD usage, network interface count |
| Memory | MemTotal/Available/Free/Cached/Buffers, Swap, usage gauge |
| CPU & Load | CPU usage %, User/System/IOWait breakdown, load averages, process states |
| Network | Total RX/TX, TCP connections |
| Disk I/O | Active device count |
| PSI | CPU/Memory/IO some/full avg10/avg60/avg300 |

### Metric Naming Convention

All syslenz metrics follow the pattern:

```
syslenz_<source>_<field>
```

Where `<source>` is the proc file (with `/` and `-` replaced by `_`) and `<field>` is the measurement name. Examples:

- `syslenz_meminfo_MemTotal` - Total RAM (bytes)
- `syslenz_stat_cpu_usage_pct` - CPU usage percentage
- `syslenz_loadavg_load_1min` - 1-minute load average
- `syslenz_net_dev_total_rx` - Total network bytes received
- `syslenz_pressure_cpu_some_avg10` - CPU pressure (10s average)
- `syslenz_file_nr_fd_allocated` - Allocated file descriptors
