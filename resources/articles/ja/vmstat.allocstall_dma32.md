# allocstall_dma32

[English version](../en/vmstat.allocstall_dma32.md)

---

**[vmstat.allocstall](../vmstat.allocstall.md)** ファミリーの一部です — 詳細なコンテキスト、チューニング、実際のエピソードはグループ記事を参照してください。

`allocstall_dma32` — DMA32ゾーンの割り当てストール（物理アドレス4GB未満）。

**ソース:** `/proc/vmstat`  
**単位:** 起動からの累積カウント（単調増加）

---

## 関連項目

- `vmstat.allocstall` — 詳細なグループ記事
- `sourceguide.vmstat` — vmstatソース全体の概要
- `pressure.memory_some_avg10` — カーネルのメモリストールシグナル
