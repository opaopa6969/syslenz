# pgactivate

[English version](../en/vmstat.pgactivate.md)

---

**[vmstat.nr_active_inactive](../vmstat.nr_active_inactive.md)** ファミリーの一部です — 詳細なコンテキスト、チューニング、実際のエピソードはグループ記事を参照してください。

`pgactivate` — 非アクティブから アクティブLRUリストに移動したページ。

**ソース:** `/proc/vmstat`  
**単位:** 起動からの累積カウント（単調増加）

---

## 関連項目

- `vmstat.nr_active_inactive` — 詳細なグループ記事
- `sourceguide.vmstat` — vmstatソース全体の概要
- `pressure.memory_some_avg10` — カーネルのメモリストールシグナル
