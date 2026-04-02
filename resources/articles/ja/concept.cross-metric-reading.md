# クロスメトリクス読み方

[English version](../en/concept.cross-metric-reading.md)

---

単一のメトリクスが全体像を語ることはめったにありません。経験豊富なオペレーターはメトリクスをグループで読みます。

**メモリプレッシャートリオ：**
`meminfo.MemAvailable`（低下）+ `vmstat.pswpout`（上昇）+ `pressure.memory_some_avg10`（> 0）= アプリに影響する能動的スワップの確認

**原則：** *裏付け*を探す。1つの高いメトリクスはノイズかもしれない。3つの関連メトリクスが同じ方向を指していればシグナル。

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
