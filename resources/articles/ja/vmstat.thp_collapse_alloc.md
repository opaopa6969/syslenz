# thp_collapse_alloc

[English version](../en/vmstat.thp_collapse_alloc.md)

---

**[vmstat.thp](../vmstat.thp.md)** ファミリーの一部です — 詳細なコンテキスト、チューニング、実際のエピソードはグループ記事を参照してください。

`thp_collapse_alloc` — THP折り畳み割り当て（khugepaged が4KB→2MBにマージ）。

**ソース:** `/proc/vmstat`  
**単位:** 起動からの累積カウント（単調増加）

---

## 関連項目

- `vmstat.thp` — 詳細なグループ記事
- `sourceguide.vmstat` — vmstatソース全体の概要
- `pressure.memory_some_avg10` — カーネルのメモリストールシグナル
