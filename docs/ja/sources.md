---
version: v1.0.0
lang: ja
---

# データソースリファレンス

[<- 前: OpenTelemetry](otel.md) | [Index](index.md)

[🇬🇧 English](../en/sources.md)

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
- [プラグイン](#プラグイン)

## 概要

syslenz は `/proc`、`/sys`、システム設定ファイル、コマンド出力にまたがる55以上のデータソースから読み取ります。各ソースは構造化された型付きフィールドにパースされます。このページでは全てのソース、読み取り内容、主要フィールド、使用すべき場面を文書化します。

## メモリ

### meminfo

| | |
|---|---|
| **読み取り元** | `/proc/meminfo` |
| **主要フィールド** | MemTotal (Bytes), MemFree (Bytes), MemAvailable (Bytes), Buffers (Bytes), Cached (Bytes), SwapTotal (Bytes), SwapFree (Bytes), SReclaimable (Bytes), SUnreclaim (Bytes) |
| **使用場面** | メモリ調査の最初のステップ。MemAvailableのMemTotalに対する割合を確認。 |

### vmstat

| | |
|---|---|
| **読み取り元** | `/proc/vmstat` |
| **主要フィールド** | pgpgin (Integer), pgpgout (Integer), pswpin (Integer), pswpout (Integer), pgmajfault (Integer), pgfault (Integer) |
| **使用場面** | スワップ活動（`pswpin`/`pswpout`）とページフォルト率の診断。 |

### swaps

| | |
|---|---|
| **読み取り元** | `/proc/swaps` |
| **主要フィールド** | カラム付きテーブル: ファイル名、タイプ、サイズ、使用量、優先度 |
| **使用場面** | どのスワップデバイスが設定され、各デバイスでどの程度使用されているかを確認。 |

### buddyinfo

| | |
|---|---|
| **読み取り元** | `/proc/buddyinfo` |
| **主要フィールド** | ゾーンごとの各割り当てオーダー (0-10) の空きページ数テーブル |
| **使用場面** | メモリ断片化の診断。高オーダーの列が全て0の場合、大きな連続メモリ割り当てが失敗する。 |

### zoneinfo

| | |
|---|---|
| **読み取り元** | `/proc/zoneinfo` |
| **主要フィールド** | ゾーンごと: free (Integer), min (Integer), low (Integer), high (Integer), managed (Integer) |
| **使用場面** | 高度なメモリデバッグ。NUMAゾーンごとのウォーターマークと空きページ分布の理解。 |

### slabinfo

| | |
|---|---|
| **読み取り元** | `/proc/slabinfo` |
| **主要フィールド** | カラム付きテーブル: name, active_objs, num_objs, objsize, objperslab, pagesperslab |
| **使用場面** | カーネルスラブキャッシュ増大（dentry、inodeキャッシュ）の調査。meminfoでSUnreclaimが大きい場合に確認。 |

### pagetypeinfo

| | |
|---|---|
| **読み取り元** | `/proc/pagetypeinfo` |
| **主要フィールド** | ゾーンごと、タイプごとの空きページ数テーブル |
| **使用場面** | buddyinfoを超える高度なメモリ断片化分析。 |

## CPUとスケジューリング

### stat

| | |
|---|---|
| **読み取り元** | `/proc/stat` |
| **主要フィールド** | cpu_user (Integer), cpu_system (Integer), cpu_idle (Integer), cpu_iowait (Integer), cpu_steal (Integer), cpu_count (Integer), context_switches (Integer), processes_created (Integer), procs_running (Integer), procs_blocked (Integer) |
| **使用場面** | CPU使用率の内訳。user vs system vs iowait を比較してワークロードを分類。 |

### loadavg

| | |
|---|---|
| **読み取り元** | `/proc/loadavg` |
| **主要フィールド** | load1 (Float), load5 (Float), load15 (Float), running_tasks (Integer), total_tasks (Integer) |
| **使用場面** | CPU需要の簡易チェック。CPU数と比較。 |

### cpuinfo

| | |
|---|---|
| **読み取り元** | `/proc/cpuinfo` |
| **主要フィールド** | cpu_count (Integer), model_name (Text), cpu_mhz (Float), cache_size (Text), flags (Text) |
| **使用場面** | ハードウェア情報: CPUモデル、コア数、周波数、機能フラグ。 |

### pressure

| | |
|---|---|
| **読み取り元** | `/proc/pressure/cpu`, `/proc/pressure/memory`, `/proc/pressure/io` |
| **主要フィールド** | cpu_some_avg10 (Float), cpu_some_avg60 (Float), cpu_some_avg300 (Float), memory_some_avg10 (Float), memory_full_avg10 (Float), io_some_avg10 (Float), io_full_avg10 (Float) |
| **使用場面** | リソースコンテンションの最も直接的な指標。PSI > 0 はタスクが停滞中を意味する。 |

### schedstat

| | |
|---|---|
| **読み取り元** | `/proc/schedstat` |
| **主要フィールド** | CPUごと: running_time (Integer), waiting_time (Integer), timeslices (Integer) |
| **使用場面** | 高度なスケジューリング分析。running_timeに対するwaiting_timeが高いとCPUコンテンションを示す。 |

### interrupts

| | |
|---|---|
| **読み取り元** | `/proc/interrupts` |
| **主要フィールド** | IRQ番号ごとのCPU別割り込み回数テーブル |
| **使用場面** | ハードウェア割り込みの分析。IRQストームや不均衡な割り込み分配のチェック。 |

### softirqs

| | |
|---|---|
| **読み取り元** | `/proc/softirqs` |
| **主要フィールド** | CPUごとのソフトIRQ回数テーブル (HI, TIMER, NET_TX, NET_RX, BLOCK 等) |
| **使用場面** | 高いNET_RXソフトIRQは重いネットワークトラフィックを示す。高いBLOCKソフトIRQは重いディスクI/Oを示す。 |

### timer_list

| | |
|---|---|
| **読み取り元** | `/proc/timer_list` |
| **主要フィールド** | アクティブタイマーエントリ |
| **使用場面** | カーネルタイマーとhrtimerの問題のデバッグ。 |

## システム

### uptime

| | |
|---|---|
| **読み取り元** | `/proc/uptime` |
| **主要フィールド** | uptime (Duration), idle (Duration) |
| **使用場面** | システムの稼働時間を確認。 |

### version

| | |
|---|---|
| **読み取り元** | `/proc/version` |
| **主要フィールド** | kernel_version (Text) |
| **使用場面** | カーネルバージョン、ビルド情報の確認。 |

### cmdline

| | |
|---|---|
| **読み取り元** | `/proc/cmdline` |
| **主要フィールド** | cmdline (Text) |
| **使用場面** | カーネルブートパラメータの確認。 |

### modules

| | |
|---|---|
| **読み取り元** | `/proc/modules` |
| **主要フィールド** | カラム付きテーブル: name, size, use_count, used_by, state |
| **使用場面** | ロードされたカーネルモジュールの一覧。 |

### filesystems

| | |
|---|---|
| **読み取り元** | `/proc/filesystems` |
| **主要フィールド** | サポートされるファイルシステムタイプのテーブル |
| **使用場面** | カーネルがサポートするファイルシステムの確認。 |

### devices

| | |
|---|---|
| **読み取り元** | `/proc/devices` |
| **主要フィールド** | キャラクターおよびブロックデバイス番号と名前のテーブル |
| **使用場面** | デバイス番号から名前へのマッピング。 |

### consoles

| | |
|---|---|
| **読み取り元** | `/proc/consoles` |
| **主要フィールド** | コンソールデバイス情報のテーブル |
| **使用場面** | 設定されたコンソールデバイスの確認。 |

### misc

| | |
|---|---|
| **読み取り元** | `/proc/misc` |
| **主要フィールド** | 雑多なデバイス登録のテーブル |
| **使用場面** | miscキャラクターデバイスマイナー番号の確認。 |

### dma

| | |
|---|---|
| **読み取り元** | `/proc/dma` |
| **主要フィールド** | DMAチャネル割り当てのテーブル |
| **使用場面** | ハードウェアDMAチャネルの検査。 |

## ストレージ

### diskstats

| | |
|---|---|
| **読み取り元** | `/proc/diskstats` |
| **主要フィールド** | デバイスごとのテーブル: reads_completed, reads_merged, sectors_read, read_time_ms, writes_completed, writes_merged, sectors_written, write_time_ms, io_in_progress, io_time_ms |
| **使用場面** | ディスクI/O分析。io_time_ms と進行中のI/Oでボトルネックを確認。 |

### df

| | |
|---|---|
| **読み取り元** | `df` コマンド出力（パース済み） |
| **主要フィールド** | root_use_pct (Float)、ファイルシステムごとのテーブル |
| **使用場面** | ファイルシステム使用量の監視。自動診断がroot_use_pctをチェック。 |

### mounts

| | |
|---|---|
| **読み取り元** | `/proc/mounts` |
| **主要フィールド** | カラム付きテーブル: device, mountpoint, fstype, options |
| **使用場面** | マウントオプション（noatime、sync、bind）の確認、期待されるマウントの検証。 |

### partitions

| | |
|---|---|
| **読み取り元** | `/proc/partitions` |
| **主要フィールド** | カラム付きテーブル: major, minor, blocks, name |
| **使用場面** | ブロックデバイスとパーティションサイズの一覧。 |

### locks

| | |
|---|---|
| **読み取り元** | `/proc/locks` |
| **主要フィールド** | アクティブファイルロックのテーブル（type、mode、PID、inode） |
| **使用場面** | プロセス間のファイルロック問題のデバッグ。 |

## ネットワーク (proc)

### net/dev

| | |
|---|---|
| **読み取り元** | `/proc/net/dev` |
| **主要フィールド** | インターフェースごとのテーブル: rx_bytes, rx_packets, rx_errors, rx_drops, tx_bytes, tx_packets, tx_errors, tx_drops |
| **使用場面** | インターフェーストラフィック監視。非ゼロのエラー/ドロップは問題を示す。 |

### net/tcp

| | |
|---|---|
| **読み取り元** | `/proc/net/tcp` |
| **主要フィールド** | カラム付きテーブル: local_addr, remote_addr, state, tx_queue, rx_queue, uid, inode |
| **使用場面** | TCP接続分析。SYN_SENT、CLOSE_WAIT、TIME_WAITの蓄積を確認。 |

### net/udp

| | |
|---|---|
| **読み取り元** | `/proc/net/udp` |
| **主要フィールド** | カラム付きテーブル: local_addr, remote_addr, state, drops, uid, inode |
| **使用場面** | UDPソケット監視。過負荷のUDPサービスのドロップを確認。 |

### net/unix

| | |
|---|---|
| **読み取り元** | `/proc/net/unix` |
| **主要フィールド** | Unixドメインソケットのパス、タイプ、状態のテーブル |
| **使用場面** | ローカルサービス間のUnixソケット接続性の確認。 |

### net/arp

| | |
|---|---|
| **読み取り元** | `/proc/net/arp` |
| **主要フィールド** | カラム付きテーブル: IP, HW_type, Flags, HW_addr, Mask, Device |
| **使用場面** | ARPテーブルの検査。古いまたは欠落したエントリの確認。 |

### net/route

| | |
|---|---|
| **読み取り元** | `/proc/net/route` |
| **主要フィールド** | カーネルルーティングテーブルのテーブル（destination, gateway, mask, flags, interface） |
| **使用場面** | ルーティングテーブルの検査。デフォルトゲートウェイとルートの確認。 |

### net/sockstat

| | |
|---|---|
| **読み取り元** | `/proc/net/sockstat` |
| **主要フィールド** | TCP inuse (Integer), UDP inuse (Integer), TCP mem (Integer), TCP alloc (Integer), orphan (Integer) |
| **使用場面** | ソケット割り当ての概要。高いorphan数は接続クリーンアップの問題を示す。 |

### net/snmp

| | |
|---|---|
| **読み取り元** | `/proc/net/snmp` |
| **主要フィールド** | InSegs, OutSegs, RetransSegs, InErrs, OutRsts、その他プロトコルごとの多数 |
| **使用場面** | プロトコルレベルのエラー分析。高いRetransSegs = ネットワーク輻輳またはパケットロス。 |

### net/netstat

| | |
|---|---|
| **読み取り元** | `/proc/net/netstat` |
| **主要フィールド** | 拡張TCP統計 (TW, TWRecycled, TCPAbortOnTimeout 等) |
| **使用場面** | net/snmpを超える高度なTCPデバッグ。 |

### net/wireless

| | |
|---|---|
| **読み取り元** | `/proc/net/wireless` |
| **主要フィールド** | インターフェースごとのテーブル: status, link, level, noise |
| **使用場面** | WiFi信号品質の監視。 |

## ネットワーク (system)

### dns

| | |
|---|---|
| **読み取り元** | `/etc/resolv.conf` |
| **主要フィールド** | nameservers (Table), search domains (Table) |
| **使用場面** | DNS設定の検証。自動診断がネームサーバーの欠落を確認。 |

### conntrack

| | |
|---|---|
| **読み取り元** | `/proc/sys/net/netfilter/nf_conntrack_*` |
| **主要フィールド** | count (Integer), max (Integer), usage_pct (Float) |
| **使用場面** | ファイアウォールとNAT用のコネクション追跡テーブル監視。 |

### ip_route

| | |
|---|---|
| **読み取り元** | `ip route` コマンド出力 |
| **主要フィールド** | ルーティングエントリのテーブル |
| **使用場面** | 現代的なルーティングテーブルビュー（net/routeを補完）。 |

### ip_neighbor

| | |
|---|---|
| **読み取り元** | `ip neighbor` コマンド出力 |
| **主要フィールド** | ネイバーエントリのテーブル（IP、MAC、状態） |
| **使用場面** | 現代的なARP/NDPテーブルビュー（net/arpを補完）。 |

### ss_summary

| | |
|---|---|
| **読み取り元** | `ss -s` コマンド出力 |
| **主要フィールド** | ソケット統計サマリー |
| **使用場面** | ソケット数の簡易概要。 |

## プロセス

### processes

| | |
|---|---|
| **読み取り元** | 全PIDの `/proc/[pid]/stat`、`/proc/[pid]/status` |
| **主要フィールド** | カラム付きテーブル: PID, name, state, RSS, threads, UID |
| **使用場面** | プロセスリスト。ゾンビ(Z)、D状態、RSS消費の大きいプロセスを確認。 |

### file-nr

| | |
|---|---|
| **読み取り元** | `/proc/sys/fs/file-nr` |
| **主要フィールド** | allocated_fds (Integer), max_fds (Integer), fd_usage_pct (Float) |
| **使用場面** | システム全体のFD使用量。自動診断は80%でアラート。 |

## ハードウェア

### thermal

| | |
|---|---|
| **読み取り元** | `/sys/class/thermal/thermal_zone*/temp` |
| **主要フィールド** | max_temp (Float)、ゾーンごとの温度 |
| **使用場面** | CPU温度監視。自動診断は75Cと90Cでアラート。 |

## セキュリティとカーネル

### crypto

| | |
|---|---|
| **読み取り元** | `/proc/crypto` |
| **主要フィールド** | 登録された暗号アルゴリズムのテーブル |
| **使用場面** | 利用可能なカーネル暗号アルゴリズムの確認。 |

### cgroups

| | |
|---|---|
| **読み取り元** | `/proc/cgroups` |
| **主要フィールド** | cgroupコントローラーのテーブル (name, hierarchy, num_cgroups, enabled) |
| **使用場面** | 利用可能で有効化されたcgroupコントローラーの確認。 |

### iomem

| | |
|---|---|
| **読み取り元** | `/proc/iomem` |
| **主要フィールド** | I/Oメモリマッピングのテーブル（アドレス範囲、説明） |
| **使用場面** | ハードウェアメモリマップの検査。 |

### ioports

| | |
|---|---|
| **読み取り元** | `/proc/ioports` |
| **主要フィールド** | I/Oポート割り当てのテーブル |
| **使用場面** | ハードウェアポート割り当ての検査。 |

## プラグイン

プラグインソースは `plugin/` プレフィックス付きで表示されます:

| | |
|---|---|
| **読み取り元** | `~/.config/syslenz/plugins/` の実行可能ファイル出力 |
| **主要フィールド** | プラグインが定義（任意の有効なFieldValue型） |
| **使用場面** | カスタムデータソース。詳細は[プラグインガイド](plugins.md)を参照。 |

---

[<- 前: OpenTelemetry](otel.md) | [Index](index.md)
