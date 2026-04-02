# Unixプロセスモデル

[English version](../en/concept.unix-process-model.md)

---

Unixプロセスモデルの理解はプロセス関連メトリクスの解釈に役立ちます。

プロセス状態：
- R（実行/実行可能）: CPU上または実行準備完了
- S（スリープ）: イベント待機（I/O、タイマー、シグナル）
- D（割り込み不可）: I/O待機 — 割り込み不可
- Z（ゾンビ）: 終了したが親がwait()を呼んでいない
- T（停止）: 一時停止（SIGSTOPまたはデバッガー）

**D状態が重要：** `stat.procs_blocked`はD状態プロセスをカウントします。スタックしたD状態はI/Oが完了していないことを意味します。

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
