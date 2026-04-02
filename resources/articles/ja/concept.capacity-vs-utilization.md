# キャパシティ対利用率

[English version](../en/concept.capacity-vs-utilization.md)

---

70%利用率のシステムと95%利用率のシステムは非常に異なる動作をします。**待ち行列理論**がその理由を説明します。

利用率が100%に近づくにつれて、平均キュー長は**指数関数的に**増大します。

利用率50% → 平均キュー長1x、利用率80% → 4x、利用率90% → 9x、利用率95% → 19x

実用的な閾値：CPU: 80%で警告/90%でページ、ディスク: I/O利用率70%で警告

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
