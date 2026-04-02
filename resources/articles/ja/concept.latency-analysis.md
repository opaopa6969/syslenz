# レイテンシ分析

[English version](../en/concept.latency-analysis.md)

---

レイテンシの問題は断続的で複合原因が多いため、診断が最も難しい問題の1つです。

レイテンシの原因：計算（CPU過多）、待ち行列（リソース忙しい）、ブロッキング（ロック/I/O/ネットワーク待ち）、GC（JVM等のランタイム一時停止）

テールレイテンシ問題：平均レイテンシは良く見えてもp99が酷い場合があります。

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
