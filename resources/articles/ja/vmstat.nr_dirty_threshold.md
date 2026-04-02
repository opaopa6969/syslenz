# nr_dirty_threshold

[English version](../en/vmstat.nr_dirty_threshold.md)

---

**[vmstat.nr_dirty](../vmstat.nr_dirty.md)** ファミリーの一部です — 詳細なコンテキスト、チューニング、実際のエピソードはグループ記事を参照してください。

`nr_dirty_threshold` — ページ単位の計算済みフォアグラウンドダーティ閾値（アプリの書き込みをブロック）。

**ソース:** `/proc/vmstat`  
**単位:** 起動からの累積カウント（単調増加）

---

## 関連項目

- `vmstat.nr_dirty` — 詳細なグループ記事
- `sourceguide.vmstat` — vmstatソース全体の概要
- `pressure.memory_some_avg10` — カーネルのメモリストールシグナル
