# ハードウェアとソフトウェア割り込み

[English version](../en/concept.interrupts-and-softirqs.md)

---

**ハードウェア割り込み（IRQ）**はハードウェアからCPUへのシグナルです。**Softirq**は遅延処理です：IRQハンドラーが最小限の処理を行い（データ保存、割り込みクリア）、その後softirqをスケジュールして後で処理します。

1つのCPUでsoftirqが高い = IRQアフィニティの不均衡 → `irqbalance`を確認。`ksoftirqd`のCPU使用率が高い = softirqバックログ（ネットワークまたはブロックI/Oが飽和）。

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
