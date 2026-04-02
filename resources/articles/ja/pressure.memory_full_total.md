# memory_full_total

[English version](../en/pressure.memory_full_total.md)

---

**[PSI（プレッシャーストール情報）](pressure.memory_some_avg10.md)** メトリクスの一部です — 詳細は`avg10`の記事を参照してください。

`memory_full_total` — 全タスクがメモリを待ってストールした累積マイクロ秒。

**ソース:** `/proc/pressure/{cpu,memory,io}`  
**単位:** パーセント（0〜100）または マイクロ秒（`_total`の場合）

---

## 関連項目

- `pressure.memory_some_avg10` — コンテキストと閾値を含む主要記事
- `sourceguide.pressure` — PSIソース全体の概要
