# ハードウェア割り込み

[English version](../en/sourceguide.interrupts.md)

---

## これは何？

/proc/interruptsは各IRQのCPUごとの割り込み数を示します。ネットワークカード、ディスクコントローラー、タイマーなどが割り込みを生成します。割り込みの不均衡（IRQアフィニティ）の診断に役立ちます。

---

## 関連項目

- `stat.cpu_user`
- `cpuinfo.logical_cpus`
- `sourceguide.vmstat` — vmstatメモリ統計
- `sourceguide.meminfo` — メモリ情報
