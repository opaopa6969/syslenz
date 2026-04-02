# リクレームとキャッシュ

[English version](../en/concept.reclaim-vs-cache.md)

---

LinuxはフリーメモリをファイルキャッシュとしてIPを使用します — これは正常で望ましいです。

**ページキャッシュ** = ディスクから読み込まれたファイル、再利用のためにRAMに保持。追い出し可能。

**ワーキングセット** = 実行中プロセスが実際に必要とするページ。追い出すとリフォルト（ディスク読み込み）が発生。

リクレームが健全な場合：`Inactive(file)`が縮小、`workingset_refault_file`がゼロ近辺。
リクレームが痛みを与えている場合：`workingset_refault_file`が上昇、`vmstat.pgscan_direct`が高い。

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
