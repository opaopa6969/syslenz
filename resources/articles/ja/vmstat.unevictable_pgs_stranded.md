# unevictable_pgs_stranded

[English version](../en/vmstat.unevictable_pgs_stranded.md)

---

**[vmstat.unevictable_pgs](../vmstat.unevictable_pgs.md)** ファミリーの一部です — 詳細なコンテキスト、チューニング、実際のエピソードはグループ記事を参照してください。

`unevictable_pgs_stranded` — 誤ったLRUに取り残された追い出し不可ページ（カーネルバグの指標）。

**ソース:** `/proc/vmstat`  
**単位:** 起動からの累積カウント（単調増加）

---

## 関連項目

- `vmstat.unevictable_pgs` — 詳細なグループ記事
- `sourceguide.vmstat` — vmstatソース全体の概要
- `pressure.memory_some_avg10` — カーネルのメモリストールシグナル
