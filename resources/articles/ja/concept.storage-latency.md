# ストレージレイテンシ

[English version](../en/concept.storage-latency.md)

---

ストレージレイテンシはアプリケーションパフォーマンス問題の最も一般的な原因の1つです。

レイテンシ階層：L1キャッシュ ~1ns、DRAM ~100ns、NVMe SSD ~100μs、SATA SSD ~500μs、HDD ~10ms

高いストレージレイテンシの原因：デバイス飽和、I/Oキュー過多、書き込みプレッシャー、ソフトウェアRAID再構築、NVMeの熱スロットリング。

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
