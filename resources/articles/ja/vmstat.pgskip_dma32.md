# pgskip_dma32

[English version](../en/vmstat.pgskip_dma32.md)

---

**[vmstat.pgalloc](../vmstat.pgalloc.md)** ファミリーの一部です — 詳細なコンテキスト、チューニング、実際のエピソードはグループ記事を参照してください。

`pgskip_dma32` — リクレームスキャン中にDMA32ゾーンをスキップ（回収するものなし）。

**ソース:** `/proc/vmstat`  
**単位:** 起動からの累積カウント（単調増加）

---

## 関連項目

- `vmstat.pgalloc` — 詳細なグループ記事
- `sourceguide.vmstat` — vmstatソース全体の概要
- `pressure.memory_some_avg10` — カーネルのメモリストールシグナル
