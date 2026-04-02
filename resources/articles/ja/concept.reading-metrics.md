# Linuxメトリクスの読み方

[English version](../en/concept.reading-metrics.md)

---

効果的なメトリクス読み方のための原則。

**カウンターvsゲージ：**
- カウンター：起動以来単調増加（`vmstat.pgfault`等）→ レートを得るには前の読みを引いて時間で割る
- ゲージ：現在の瞬間値（`meminfo.MemAvailable`等）

**絶対値より変化率が重要：**
`vmstat.nr_dirty = 50000`は問題ないかもしれないが、10分間で1000/秒ずつ増加しているのは問題。

**比率が効率を示す：**
`pgsteal / pgscan` = リクレーム効率
`numa_hit / (numa_hit + numa_miss)` = NUMA効率

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
