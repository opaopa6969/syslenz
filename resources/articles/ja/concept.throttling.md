# CPUとI/Oスロットリング

[English version](../en/concept.throttling.md)

---

**スロットリング**はカーネルがプロセスのリソースアクセスを人為的に制限するときです。飽和（リソースが容量に達している）とは異なり、スロットリングは設定された制限です。

コンテナのcpu.cfs_quota_usが達成されると、プロセスが一時停止されます — 高CPU%なしで高レイテンシとして現れます。

書き込みスロットリング：`vmstat.nr_dirty`が`dirty_ratio`を超えると、カーネルがアプリの書き込みをブロックします。

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
