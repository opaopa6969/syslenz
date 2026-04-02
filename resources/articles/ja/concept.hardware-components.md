# ハードウェアコンポーネント概要

[English version](../en/concept.hardware-components.md)

---

ハードウェアコンポーネントを理解するとメトリクスの解釈が正しくなります。

レイテンシ階層：L1キャッシュ: ~1ns、L2: ~5ns、L3: ~20ns、DRAM: ~100ns、NVMe SSD: ~100μs、HDD: ~10ms

これがキャッシュ追い出し（ワーキングセットリフォルト）がコスト高な理由です。

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
