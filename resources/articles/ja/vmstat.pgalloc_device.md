# pgalloc_device

[English version](../en/vmstat.pgalloc_device.md)

---

**[vmstat.pgalloc](../vmstat.pgalloc.md)** ファミリーの一部です — 詳細なコンテキスト、チューニング、実際のエピソードはグループ記事を参照してください。

`pgalloc_device` — Deviceゾーンからのページ割り当て（GPU/永続メモリ）。

**ソース:** `/proc/vmstat`  
**単位:** 起動からの累積カウント（単調増加）

---

## 関連項目

- `vmstat.pgalloc` — 詳細なグループ記事
- `sourceguide.vmstat` — vmstatソース全体の概要
- `pressure.memory_some_avg10` — カーネルのメモリストールシグナル
