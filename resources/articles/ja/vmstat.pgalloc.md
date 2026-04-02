# ゾーン別ページ割り当て (pgalloc/pgskip) — vmstat

[English version](../en/vmstat.pgalloc.md)

---

## これは何？

カーネルがページを割り当てる際、複数の**メモリゾーン**から選択します：

| ゾーン | 物理アドレス範囲 | 目的 |
|--------|----------------|------|
| `dma32` | 4GB未満 | レガシーDMAデバイス |
| `normal` | 4GB超（64bit） | メインシステムRAM |
| `movable` | 設定可能 | ホットプラグメモリ |

`pgalloc_*`は各ゾーンからの割り当て成功回数、`pgskip_*`はリクレーム中にスキップされた回数をカウントします。

**`pgskip_normal`が高い**場合、メモリ圧迫時にNormalゾーンが枯渇しています。

---

## 関連項目

- `vmstat.pgscan_pgsteal` — リクレームパイプライン
- `vmstat.allocstall` — ゾーンが割り当てを満たせないときのストール
- `sourceguide.vmstat` — vmstatソース全体の概要
