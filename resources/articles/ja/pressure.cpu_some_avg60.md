# cpu_some_avg60

[English version](../en/pressure.cpu_some_avg60.md)

---

**[PSI（プレッシャーストール情報）](pressure.cpu_some_avg10.md)** メトリクスの一部です — 詳細は`avg10`の記事を参照してください。

`cpu_some_avg60` — 一部タスクがCPUでストールした時間の割合（60秒平均）。

**ソース:** `/proc/pressure/{cpu,memory,io}`  
**単位:** パーセント（0〜100）または マイクロ秒（`_total`の場合）

---

## 関連項目

- `pressure.cpu_some_avg10` — コンテキストと閾値を含む主要記事
- `sourceguide.pressure` — PSIソース全体の概要
