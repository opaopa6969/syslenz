# プロセス競合

[English version](../en/concept.process-contention.md)

---

複数のプロセスが同じリソースを競合すると、競合と予測不可能なレイテンシが発生します。

PSI `some` > `full`の大きな差 = 1つのプロセスが他のプロセスに問題を引き起こしています。

CPU競合：`loadavg > CPU数`。メモリ競合：`pressure.memory_some_avg10 > 0`。I/O競合：`diskstats.io_queue_depth_distribution`が大きい。

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
