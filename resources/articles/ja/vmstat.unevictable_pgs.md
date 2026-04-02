# 追い出し不可ページ — vmstat

[English version](../en/vmstat.unevictable_pgs.md)

---

## これは何？

一部のメモリページは**追い出せません** — ディスクへの書き込みや破棄ができません。`mlock()`でロック、`ramfs`バックアップ、`SHM_LOCK`共有メモリなどが該当します。カーネルはこれらを別の**追い出し不可LRUリスト**で管理し、リクレーム時にスキャンして時間を無駄にしないようにします。

| メトリクス | カウントするもの |
|-----------|----------------|
| `unevictable_pgs_culled` | 追い出し不可リストへ移動（新たにロック） |
| `unevictable_pgs_scanned` | リクレーム中に確認された追い出し不可ページ（0であるべき） |
| `unevictable_pgs_rescued` | 追い出し不可からLRUに戻ったページ（ロック解除） |
| `unevictable_pgs_mlocked` | mlock()で追い出し不可になったページ |
| `unevictable_pgs_stranded` | 誤ったLRUにあるページ（バグの指標） |

**`unevictable_pgs_scanned` > 0**は警告 — カーネルが追い出せないとわかっているページをスキャンしています。

---

## 関連項目

- `meminfo.Unevictable` — 合計追い出し不可メモリ
- `meminfo.Mlocked` — mlock()でロックされたメモリ
- `sourceguide.vmstat` — vmstatソース全体の概要
