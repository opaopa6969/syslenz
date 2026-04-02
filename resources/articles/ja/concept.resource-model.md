# Linuxリソースモデル

[English version](../en/concept.resource-model.md)

---

Linuxサーバーには4つの主要リソースがあります。すべてのパフォーマンス問題はそれらの1つ以上に行き着きます。

CPU：`loadavg`、`stat.cpu_user`、`pressure.cpu_some_avg10`
メモリ：`meminfo.MemAvailable`、`vmstat.pswpout`、`pressure.memory_some_avg10`
I/O：`diskstats`、`vmstat.nr_dirty`、`pressure.io_some_avg10`
ネットワーク：`net/dev`、`net/snmp.Tcp_RetransSegs`、`net/sockstat`

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
