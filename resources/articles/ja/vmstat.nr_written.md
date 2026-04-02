# nr_written

[English version](../en/vmstat.nr_written.md)

---

**[vmstat.nr_dirty](../vmstat.nr_dirty.md)** ファミリーの一部です — 詳細なコンテキスト、チューニング、実際のエピソードはグループ記事を参照してください。

`nr_written` — 起動以来ディスクに書き込まれたページ合計（累積）。

**ソース:** `/proc/vmstat`  
**単位:** 起動からの累積カウント（単調増加）

---

## 関連項目

- `vmstat.nr_dirty` — 詳細なグループ記事
- `sourceguide.vmstat` — vmstatソース全体の概要
- `pressure.memory_some_avg10` — カーネルのメモリストールシグナル
