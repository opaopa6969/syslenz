---
version: v1.3.0
lang: ja
---

# データソースリファレンス

[🇬🇧 English](../en/sources.md)

[<- 前: OpenTelemetry](otel.md) | [Index](index.md)


## 目次

- [概要](#概要)
- [メモリ](#メモリ)
- [CPUとスケジューリング](#cpuとスケジューリング)
- [システム](#システム)
- [ストレージ](#ストレージ)
- [ネットワーク (proc)](#ネットワーク-proc)
- [ネットワーク (system)](#ネットワーク-system)
- [プロセス](#プロセス)
- [ハードウェア](#ハードウェア)
- [セキュリティとカーネル](#セキュリティとカーネル)
- [GPU](#gpu)
- [systemd](#systemd)
- [プラグイン](#プラグイン)

## 概要

syslenz は `/proc`、`/sys`、システム設定ファイル、コマンド出力にまたがる55以上のデータソースから読み取ります。各ソースは構造化された型付きフィールドにパースされます。このページでは全ソースの読み取り内容、主要フィールド、活用場面をまとめます。

## メモリ

### meminfo

| | |
|---|---|
| **読み取り元** | `/proc/meminfo` |
| **主要フィールド** | MemTotal (Bytes), MemFree (Bytes), MemAvailable (Bytes), Buffers (Bytes), Cached (Bytes), SwapTotal (Bytes), SwapFree (Bytes), SReclaimable (Bytes), SUnreclaim (Bytes) |
| **活用場面** | メモリ調査の第一歩。MemAvailableのMemTotalに対する割合を確認。 |

### vmstat

| | |
|---|---|
| **読み取り元** | `/proc/vmstat` |
| **主要フィールド** | pgpgin (Integer), pgpgout (Integer), pswpin (Integer), pswpout (Integer), pgmajfault (Integer), pgfault (Integer) |
| **活用場面** | スワップ活動（`pswpin`/`pswpout`）やページフォルト率の診断。 |

### swaps

| | |
|---|---|
| **読み取り元** | `/proc/swaps` |
| **主要フィールド** | カラム付きテーブル: ファイル名、タイプ、サイズ、使用量、優先度 |
| **活用場面** | どのスワップデバイスが設定されているか、各デバイスの使用量を確認。 |

### buddyinfo

| | |
|---|---|
| **読み取り元** | `/proc/buddyinfo` |
| **主要フィールド** | ゾーンごとの各割り当てオーダー (0-10) の空きページ数テーブル |
| **活用場面** | メモリ断片化の診断。高オーダーの列が全て0なら、大きな連続メモリ割り当てが失敗する。 |

### zoneinfo

| | |
|---|---|
| **読み取り元** | `/proc/zoneinfo` |
| **主要フィールド** | ゾーンごと: free (Integer), min (Integer), low (Integer), high (Integer), managed (Integer) |
| **活用場面** | 高度なメモリデバッグ。NUMAゾーンごとのウォーターマークと空きページ分布の把握。 |

### slabinfo

| | |
|---|---|
| **読み取り元** | `/proc/slabinfo` |
| **主要フィールド** | カラム付きテーブル: name, active_objs, num_objs, objsize, objperslab, pagesperslab |
| **活用場面** | カーネルスラブキャッシュの肥大化（dentry、inodeキャッシュ）の調査。meminfoでSUnreclaimが大きい場合に確認。 |

### pagetypeinfo

| | |
|---|---|
| **読み取り元** | `/proc/pagetypeinfo` |
| **主要フィールド** | ゾーンごと、タイプごとの空きページ数テーブル |
| **活用場面** | buddyinfoより詳細なメモリ断片化分析。 |

## CPUとスケジューリング

### stat

| | |
|---|---|
| **読み取り元** | `/proc/stat` |
| **主要フィールド** | cpu_user (Integer), cpu_system (Integer), cpu_idle (Integer), cpu_iowait (Integer), cpu_steal (Integer), cpu_count (Integer), context_switches (Integer), processes_created (Integer), procs_running (Integer), procs_blocked (Integer) |
| **活用場面** | CPU使用率の内訳。user vs system vs iowait を比較してワークロードを分類。 |

### loadavg

| | |
|---|---|
| **読み取り元** | `/proc/loadavg` |
| **主要フィールド** | load1 (Float), load5 (Float), load15 (Float), running_tasks (Integer), total_tasks (Integer) |
| **活用場面** | CPU需要の簡易チェック。CPU数と比較。 |

### cpuinfo

| | |
|---|---|
| **読み取り元** | `/proc/cpuinfo` |
| **主要フィールド** | cpu_count (Integer), model_name (Text), cpu_mhz (Float), cache_size (Text), flags (Text) |
| **活用場面** | ハードウェア情報: CPUモデル、コア数、周波数、機能フラグ。 |

### pressure

| | |
|---|---|
| **読み取り元** | `/proc/pressure/cpu`, `/proc/pressure/memory`, `/proc/pressure/io` |
| **主要フィールド** | cpu_some_avg10 (Float), cpu_some_avg60 (Float), cpu_some_avg300 (Float), memory_some_avg10 (Float), memory_full_avg10 (Float), io_some_avg10 (Float), io_full_avg10 (Float) |
| **活用場面** | リソース競合の最も直接的な指標。PSI > 0 はタスクが停滞中を意味する。 |

### schedstat

| | |
|---|---|
| **読み取り元** | `/proc/schedstat` |
| **主要フィールド** | CPUごと: running_time (Integer), waiting_time (Integer), timeslices (Integer) |
| **活用場面** | 高度なスケジューリング分析。running_timeに対してwaiting_timeが高ければCPU競合。 |

### interrupts

| | |
|---|---|
| **読み取り元** | `/proc/interrupts` |
| **主要フィールド** | IRQ番号ごとのCPU別割り込み回数テーブル |
| **活用場面** | ハードウェア割り込みの分析。IRQストームや割り込み分配の偏りをチェック。 |

### softirqs

| | |
|---|---|
| **読み取り元** | `/proc/softirqs` |
| **主要フィールド** | CPUごとのソフトIRQ回数テーブル (HI, TIMER, NET_TX, NET_RX, BLOCK 等) |
| **活用場面** | NET_RXソフトIRQが高ければネットワークトラフィックが重い。BLOCKソフトIRQが高ければディスクI/Oが重い。 |

### timer_list

| | |
|---|---|
| **読み取り元** | `/proc/timer_list` |
| **主要フィールド** | アクティブタイマーエントリ |
| **活用場面** | カーネルタイマーとhrtimerの問題のデバッグ。 |

## システム

### uptime

| | |
|---|---|
| **読み取り元** | `/proc/uptime` |
| **主要フィールド** | uptime (Duration), idle (Duration) |
| **活用場面** | システムの稼働時間を確認。 |

### version

| | |
|---|---|
| **読み取り元** | `/proc/version` |
| **主要フィールド** | kernel_version (Text) |
| **活用場面** | カーネルバージョンとビルド情報の確認。 |

### cmdline

| | |
|---|---|
| **読み取り元** | `/proc/cmdline` |
| **主要フィールド** | cmdline (Text) |
| **活用場面** | カーネルブートパラメータの確認。 |

### modules

| | |
|---|---|
| **読み取り元** | `/proc/modules` |
| **主要フィールド** | カラム付きテーブル: name, size, use_count, used_by, state |
| **活用場面** | ロード済みカーネルモジュールの一覧。 |

### filesystems

| | |
|---|---|
| **読み取り元** | `/proc/filesystems` |
| **主要フィールド** | 対応ファイルシステムタイプのテーブル |
| **活用場面** | カーネルが対応するファイルシステムの確認。 |

### devices

| | |
|---|---|
| **読み取り元** | `/proc/devices` |
| **主要フィールド** | キャラクターおよびブロックデバイス番号と名前のテーブル |
| **活用場面** | デバイス番号から名前へのマッピング。 |

### consoles

| | |
|---|---|
| **読み取り元** | `/proc/consoles` |
| **主要フィールド** | コンソールデバイス情報のテーブル |
| **活用場面** | 設定されたコンソールデバイスの確認。 |

### misc

| | |
|---|---|
| **読み取り元** | `/proc/misc` |
| **主要フィールド** | 雑多なデバイス登録のテーブル |
| **活用場面** | miscキャラクターデバイスのマイナー番号を確認。 |

### dma

| | |
|---|---|
| **読み取り元** | `/proc/dma` |
| **主要フィールド** | DMAチャネル割り当てのテーブル |
| **活用場面** | ハードウェアDMAチャネルの検査。 |

## ストレージ

### diskstats

| | |
|---|---|
| **読み取り元** | `/proc/diskstats` |
| **主要フィールド** | デバイスごとのテーブル: reads_completed, reads_merged, sectors_read, read_time_ms, writes_completed, writes_merged, sectors_written, write_time_ms, io_in_progress, io_time_ms |
| **活用場面** | ディスクI/O分析。io_time_ms と進行中I/Oでボトルネックを確認。 |

### df

| | |
|---|---|
| **読み取り元** | `df` コマンド出力（パース済み） |
| **主要フィールド** | root_use_pct (Float)、ファイルシステムごとのテーブル |
| **活用場面** | ファイルシステム使用量の監視。自動診断がroot_use_pctをチェック。 |

### mounts

| | |
|---|---|
| **読み取り元** | `/proc/mounts` |
| **主要フィールド** | カラム付きテーブル: device, mountpoint, fstype, options |
| **活用場面** | マウントオプション（noatime、sync、bind）の確認、想定どおりにマウントされているかの検証。 |

### partitions

| | |
|---|---|
| **読み取り元** | `/proc/partitions` |
| **主要フィールド** | カラム付きテーブル: major, minor, blocks, name |
| **活用場面** | ブロックデバイスとパーティションサイズの一覧。 |

### locks

| | |
|---|---|
| **読み取り元** | `/proc/locks` |
| **主要フィールド** | アクティブファイルロックのテーブル（type、mode、PID、inode） |
| **活用場面** | プロセス間のファイルロック問題のデバッグ。 |

## ネットワーク (proc)

### net/dev

| | |
|---|---|
| **読み取り元** | `/proc/net/dev` |
| **主要フィールド** | インターフェースごとのテーブル: rx_bytes, rx_packets, rx_errors, rx_drops, tx_bytes, tx_packets, tx_errors, tx_drops |
| **活用場面** | インターフェーストラフィック監視。エラーやドロップが非ゼロなら問題あり。 |

### net/tcp

| | |
|---|---|
| **読み取り元** | `/proc/net/tcp` |
| **主要フィールド** | カラム付きテーブル: local_addr, remote_addr, state, tx_queue, rx_queue, uid, inode |
| **活用場面** | TCP接続の分析。SYN_SENT、CLOSE_WAIT、TIME_WAITの蓄積を確認。 |

### net/udp

| | |
|---|---|
| **読み取り元** | `/proc/net/udp` |
| **主要フィールド** | カラム付きテーブル: local_addr, remote_addr, state, drops, uid, inode |
| **活用場面** | UDPソケット監視。過負荷のUDPサービスでのドロップを確認。 |

### net/unix

| | |
|---|---|
| **読み取り元** | `/proc/net/unix` |
| **主要フィールド** | Unixドメインソケットのパス、タイプ、状態のテーブル |
| **活用場面** | ローカルサービス間のUnixソケット接続性の確認。 |

### net/arp

| | |
|---|---|
| **読み取り元** | `/proc/net/arp` |
| **主要フィールド** | カラム付きテーブル: IP, HW_type, Flags, HW_addr, Mask, Device |
| **活用場面** | ARPテーブルの検査。古いエントリや欠落の確認。 |

### net/route

| | |
|---|---|
| **読み取り元** | `/proc/net/route` |
| **主要フィールド** | カーネルルーティングテーブル（destination, gateway, mask, flags, interface） |
| **活用場面** | ルーティングテーブルの検査。デフォルトゲートウェイとルートの確認。 |

### net/sockstat

| | |
|---|---|
| **読み取り元** | `/proc/net/sockstat` |
| **主要フィールド** | TCP inuse (Integer), UDP inuse (Integer), TCP mem (Integer), TCP alloc (Integer), orphan (Integer) |
| **活用場面** | ソケット割り当ての概要。orphanが多ければ接続クリーンアップに問題あり。 |

### net/snmp

| | |
|---|---|
| **読み取り元** | `/proc/net/snmp` |
| **主要フィールド** | InSegs, OutSegs, RetransSegs, InErrs, OutRsts ほか多数 |
| **活用場面** | プロトコルレベルのエラー分析。RetransSegs が高い = ネットワーク輻輳またはパケットロス。 |

### net/netstat

| | |
|---|---|
| **読み取り元** | `/proc/net/netstat` |
| **主要フィールド** | 拡張TCP統計 (TW, TWRecycled, TCPAbortOnTimeout 等) |
| **活用場面** | net/snmpを超える高度なTCPデバッグ。 |

### net/wireless

| | |
|---|---|
| **読み取り元** | `/proc/net/wireless` |
| **主要フィールド** | インターフェースごとのテーブル: status, link, level, noise |
| **活用場面** | WiFi信号品質の監視。 |

## ネットワーク (system)

### dns

| | |
|---|---|
| **読み取り元** | `/etc/resolv.conf` |
| **主要フィールド** | nameservers (Table), search domains (Table) |
| **活用場面** | DNS設定の検証。自動診断がネームサーバーの欠落を確認。 |

### conntrack

| | |
|---|---|
| **読み取り元** | `/proc/sys/net/netfilter/nf_conntrack_*` |
| **主要フィールド** | count (Integer), max (Integer), usage_pct (Float) |
| **活用場面** | ファイアウォールとNAT用のコネクション追跡テーブル監視。 |

### ip_route

| | |
|---|---|
| **読み取り元** | `ip route` コマンド出力 |
| **主要フィールド** | ルーティングエントリのテーブル |
| **活用場面** | 最新のルーティングテーブル表示（net/routeを補完）。 |

### ip_neighbor

| | |
|---|---|
| **読み取り元** | `ip neighbor` コマンド出力 |
| **主要フィールド** | ネイバーエントリのテーブル（IP、MAC、状態） |
| **活用場面** | 最新のARP/NDPテーブル表示（net/arpを補完）。 |

### ss_summary

| | |
|---|---|
| **読み取り元** | `ss -s` コマンド出力 |
| **主要フィールド** | ソケット統計サマリー |
| **活用場面** | ソケット数の簡易概要。 |

## プロセス

### processes

| | |
|---|---|
| **読み取り元** | 全PIDの `/proc/[pid]/stat`、`/proc/[pid]/status` |
| **主要フィールド** | カラム付きテーブル: PID, name, state, RSS, threads, UID |
| **活用場面** | プロセスリスト。ゾンビ(Z)、D状態、RSS消費の大きいプロセスを確認。 |

### file-nr

| | |
|---|---|
| **読み取り元** | `/proc/sys/fs/file-nr` |
| **主要フィールド** | allocated_fds (Integer), max_fds (Integer), fd_usage_pct (Float) |
| **活用場面** | システム全体のFD使用量。自動診断は80%でアラート。 |

## ハードウェア

### thermal

| | |
|---|---|
| **読み取り元** | `/sys/class/thermal/thermal_zone*/temp` |
| **主要フィールド** | max_temp (Float)、ゾーンごとの温度 |
| **活用場面** | CPU温度監視。自動診断は75Cと90Cでアラート。 |

## セキュリティとカーネル

### crypto

| | |
|---|---|
| **読み取り元** | `/proc/crypto` |
| **主要フィールド** | 登録済み暗号アルゴリズムのテーブル |
| **活用場面** | カーネルで利用可能な暗号アルゴリズムの確認。 |

### cgroups

| | |
|---|---|
| **読み取り元** | `/proc/cgroups` |
| **主要フィールド** | cgroupコントローラーのテーブル (name, hierarchy, num_cgroups, enabled) |
| **活用場面** | 利用可能なcgroupコントローラーと有効/無効状態の確認。 |

### iomem

| | |
|---|---|
| **読み取り元** | `/proc/iomem` |
| **主要フィールド** | I/Oメモリマッピングのテーブル（アドレス範囲、説明） |
| **活用場面** | ハードウェアメモリマップの検査。 |

### ioports

| | |
|---|---|
| **読み取り元** | `/proc/ioports` |
| **主要フィールド** | I/Oポート割り当てのテーブル |
| **活用場面** | ハードウェアポート割り当ての検査。 |

## GPU

### nvidia-smi (v1.3.0)

| | |
|---|---|
| **読み取り元** | `nvidia-smi --query-gpu=...` コマンド出力 |
| **主要フィールド** | gpu_temp (Float), gpu_util (Float), mem_util (Float), mem_total (Bytes), mem_used (Bytes), mem_free (Bytes), fan_speed (Float), power_draw (Float), power_limit (Float) |
| **活用場面** | NVIDIA GPU の温度・使用率・VRAM 消費を監視。機械学習ワークロードや GPU レンダリングのボトルネック分析に。 |

`nvidia-smi` がシステムに存在しない場合、このソースは自動的にスキップされます。NVIDIA ドライバがインストールされている環境でのみ利用可能です。

**フィールド詳細:**

| フィールド | 型 | 説明 |
|-----------|-----|------|
| `gpu_temp` | Float | GPU コアの現在温度 (℃) |
| `gpu_util` | Float | GPU コアの使用率 (%) |
| `mem_util` | Float | VRAM 帯域幅の使用率 (%) |
| `mem_total` | Bytes | VRAM 総容量 |
| `mem_used` | Bytes | VRAM 使用量 |
| `mem_free` | Bytes | VRAM 空き容量 |
| `fan_speed` | Float | ファン回転速度 (%) |
| `power_draw` | Float | 現在の消費電力 (W) |
| `power_limit` | Float | 電力制限値 (W) |

## systemd

### systemd (v1.3.0)

| | |
|---|---|
| **読み取り元** | `systemctl` コマンド出力 |
| **主要フィールド** | system_state (Text), service_count (Integer), running_count (Integer), failed_count (Integer), failed_services (Table) |
| **活用場面** | systemd サービスの全体的な健全性を把握。失敗サービスの早期発見や、起動後のサービス状態確認に。 |

systemd が存在しない環境（コンテナ内など）ではこのソースはスキップされます。

**フィールド詳細:**

| フィールド | 型 | 説明 |
|-----------|-----|------|
| `system_state` | Text | systemd 全体の状態 (`running`, `degraded`, `maintenance` 等) |
| `service_count` | Integer | 登録されたサービスユニットの総数 |
| `running_count` | Integer | 現在実行中のサービス数 |
| `failed_count` | Integer | 失敗状態のサービス数 |
| `failed_services` | Table | 失敗サービスの一覧 (ユニット名、状態、最終変更時刻) |

`system_state` が `degraded` の場合、1つ以上のサービスが失敗しています。`failed_services` テーブルで具体的なユニット名を確認し、`journalctl -u <unit>` でログを調査してください。

## プラグイン

プラグインソースは `plugin/` プレフィックス付きで表示されます:

| | |
|---|---|
| **読み取り元** | `~/.config/syslenz/plugins/` の実行可能ファイル出力 |
| **主要フィールド** | プラグインが定義（任意の有効なFieldValue型） |
| **活用場面** | カスタムデータソース。詳細は[プラグインガイド](plugins.md)を参照。 |

---

[<- 前: OpenTelemetry](otel.md) | [Index](index.md)
