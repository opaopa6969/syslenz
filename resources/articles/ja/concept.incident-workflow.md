# インシデント対応ワークフロー

[English version](../en/concept.incident-workflow.md)

---

体系的なアプローチでシステムインシデントの解決時間を短縮します。

1. **症状を確立**（ユーザーへの影響は？）
2. **最重要指標を最初に確認**（uptime、free -h、df -h、/proc/pressure/*）
3. **仮説を立てて検証**
4. **緩和してから根本原因を調査**（完璧を善の敵にしない）
5. **タイムラインを記録**

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
