# 待ち行列対飽和

[English version](../en/concept.queueing-vs-saturation.md)

---

**飽和**はリソースが100%利用率になっている状態。**待ち行列**はリソースが忙しいために作業が待っている状態。

リソースが待ち行列を持ちながら飽和していないこともあります（バースト的なリクエストの場合）。飽和したリソースは常に待ち行列があります。

PSIで診断：`io_full_avg10 > 0` = 飽和（全タスクがブロック）、`io_some_avg10 > 0`かつ`io_full_avg10 = 0` = 待ち行列（一部タスクのみ影響）。

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
