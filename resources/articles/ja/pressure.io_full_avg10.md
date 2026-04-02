# io_full_avg10

[English version](../en/pressure.io_full_avg10.md)

---

**[PSI（プレッシャーストール情報）](pressure.io_some_avg10.md)** メトリクスの一部です — 詳細は`avg10`の記事を参照してください。

`io_full_avg10` — 全タスクがI/Oでストールした時間の割合（10秒平均）— ストレージが完全にブロック。

**ソース:** `/proc/pressure/{cpu,memory,io}`  
**単位:** パーセント（0〜100）または マイクロ秒（`_total`の場合）

---

## 関連項目

- `pressure.io_some_avg10` — コンテキストと閾値を含む主要記事
- `sourceguide.pressure` — PSIソース全体の概要
