# unevictable_pgs_munlocked

[English version](../en/vmstat.unevictable_pgs_munlocked.md)

---

**[vmstat.unevictable_pgs](../vmstat.unevictable_pgs.md)** ファミリーの一部です — 詳細なコンテキスト、チューニング、実際のエピソードはグループ記事を参照してください。

`unevictable_pgs_munlocked` — mlock()から解放されて追い出し可能LRUに戻ったページ。

**ソース:** `/proc/vmstat`  
**単位:** 起動からの累積カウント（単調増加）

---

## 関連項目

- `vmstat.unevictable_pgs` — 詳細なグループ記事
- `sourceguide.vmstat` — vmstatソース全体の概要
- `pressure.memory_some_avg10` — カーネルのメモリストールシグナル
