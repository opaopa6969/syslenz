# pgdemote_direct

[English version](../en/vmstat.pgdemote_direct.md)

---

**[vmstat.pgscan_pgsteal](../vmstat.pgscan_pgsteal.md)** ファミリーの一部です — 詳細なコンテキスト、チューニング、実際のエピソードはグループ記事を参照してください。

`pgdemote_direct` — ダイレクトリクレームによって降格したページ（階層型メモリ）。

**ソース:** `/proc/vmstat`  
**単位:** 起動からの累積カウント（単調増加）

---

## 関連項目

- `vmstat.pgscan_pgsteal` — 詳細なグループ記事
- `sourceguide.vmstat` — vmstatソース全体の概要
- `pressure.memory_some_avg10` — カーネルのメモリストールシグナル
