# NUMA割り当て統計 — vmstat

[English version](../en/vmstat.numa.md)

---

## これは何？

複数CPUソケットのサーバーでは、各ソケットが独自のRAMを持ちます — これが**NUMA（非一様メモリアクセス）**です。ローカルソケットのメモリアクセスは高速（〜100ns）ですが、リモートソケットはQPI/UPI相互接続を経由するため2〜3倍遅くなります。

| メトリクス | 意味 |
|-----------|------|
| `numa_hit` | 意図したノードへの割り当て成功 |
| `numa_miss` | 別ノードへの割り当て（リモート） |
| `numa_foreign` | このノード向けだが別ノードに配置 |
| `numa_local` | プロセスが実行しているノードへの割り当て |
| `numa_other` | プロセスとは別ノードへの割り当て |
| `numa_interleave` | インターリーブポリシーによる割り当て |

**ミス率 = `numa_miss / (numa_hit + numa_miss)`**

ミス率が高い場合はNUMAノード間のメモリアクセスが発生しており、不必要に2〜3倍のレイテンシを払っています。`numactl`でプロセスをノードに固定してください。

---

## 関連項目

- `cpuinfo.cores_per_socket` — ソケット/コアトポロジー
- `sourceguide.vmstat` — vmstatソース全体の概要
