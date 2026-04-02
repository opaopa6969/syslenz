# メモリフラグメンテーション

[English version](../en/concept.fragmentation.md)

---

物理メモリフラグメンテーションは空きページが散らばり、大きな連続した割り当てを妨げるときに発生します。

THPには2MB連続ブロックが必要 → `thp_fault_fallback`が上昇。`cat /proc/buddyinfo`でフラグメンテーションを確認。高いorder-0と低いorder-10 = フラグメント化。

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
