# unevictable_pgs_mlocked

[English version](../en/vmstat.unevictable_pgs_mlocked.md)

---

**[vmstat.unevictable_pgs](../vmstat.unevictable_pgs.md)** ファミリーの一部です — 詳細なコンテキスト、チューニング、実際のエピソードはグループ記事を参照してください。

`unevictable_pgs_mlocked` — mlock()システムコールで追い出し不可になったページ。

**ソース:** `/proc/vmstat`  
**単位:** 起動からの累積カウント（単調増加）

---

## 関連項目

- `vmstat.unevictable_pgs` — 詳細なグループ記事
- `sourceguide.vmstat` — vmstatソース全体の概要
- `pressure.memory_some_avg10` — カーネルのメモリストールシグナル
