# メモリプレッシャー

[English version](../en/concept.memory-pressure.md)

---

**メモリプレッシャー**はカーネルが新しい割り当てにメモリを提供するために苦労している状態です。

プレッシャーの段階：
1. メモリ十分 → キャッシュが満杯、アプリ正常動作
2. 軽プレッシャー → カーネルがファイルキャッシュを追い出し始める
3. 中プレッシャー → ワーキングセットリフォルト開始、pgscan_direct上昇
4. 高プレッシャー → allocstallが発火、スワップ開始
5. 深刻 → OOMキラーが発火

各段階の主要メトリクス：`meminfo.MemAvailable` → `workingset_refault_file` → `allocstall_normal` → `vmstat.oom_kill`

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
