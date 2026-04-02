# カーネルエビデンスチェーン

[English version](../en/concept.kernel-evidence-chain.md)

---

Linuxシステムの問題診断は法医学のようなもの：症状から原因まで証拠の連鎖をたどります。

**メモリプレッシャーエビデンスチェーン：**
「アプリが遅い」→ `pressure.memory_some_avg10 > 0` → `vmstat.pswpin`上昇 → `meminfo.MemAvailable < 10%` → 高RSSのプロセス → 根本原因：サービスXのメモリリーク

各仮説を確認または否定するメトリクスを知ることが重要なスキルです。

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
