# CPU実行キュー

[English version](../en/concept.cpu-runqueue.md)

---

**実行キュー**は実行準備ができているがCPU時間を待っているプロセスのリストです。ロードアベレージは基本的に平滑化された実行キュー長です。

実行キュー < CPU数：健全
実行キュー ≈ CPU数：バランスが取れている
実行キュー >> CPU数：過負荷（プロセスがCPUを待っている）

例：4 CPU、ロード6.0 = 実行キュー6 = 常に2プロセスが待機

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
