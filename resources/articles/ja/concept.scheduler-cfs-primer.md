# CFSスケジューラー入門

[English version](../en/concept.scheduler-cfs-primer.md)

---

**CFS（完全公平スケジューラー）**はLinuxのデフォルトCPUスケジューラーです。優先度（nice値）に基づいて各プロセスに比例したCPU時間を割り当てます。

**仮想実行時間（vruntime）**：タスクが使用したCPU時間（優先度で重み付け）。CFSは常に最も低いvruntime（CPU時間が最も「遅れている」）のタスクを選択します。

nice値：-20（最高優先度）から19（最低）。デフォルト0（すべて同等）。

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
