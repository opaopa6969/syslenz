# ボトルネックトリアージ

[English version](../en/concept.bottleneck-triage.md)

---

ボトルネックを見つけるには体系的なアプローチが必要です。4つのリソースタイプ（CPU、メモリ、ストレージI/O、ネットワーク）にはそれぞれ異なるシグナチャーがあります。

**PSI（プレッシャーストール情報）を確認:**
```sh
cat /proc/pressure/{cpu,memory,io}
```
PSIはタスクがどのリソースを待っているかを直接示します。

**USEメソッド（各リソースに対して）:**
- 利用率（Utilization）：どれくらい忙しいか？
- 飽和（Saturation）：キューが形成されているか？
- エラー（Errors）：失敗しているものはあるか？

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
