# ソフトウェア割り込み

[English version](../en/sourceguide.softirqs.md)

---

## これは何？

/proc/softirqsはCPUごとのソフトウェア割り込み累積数を示します：NET_RX、NET_TX、BLOCK、TIMER、RCUなど。1つのCPUでsoftirqレートが高い場合は割り込みアフィニティの問題を示します。

---

## 関連項目

- `interrupts.interrupts`
- `stat.cpu_user`
- `sourceguide.vmstat` — vmstatメモリ統計
- `sourceguide.meminfo` — メモリ情報
