# アクティブ/非アクティブメモリ — meminfo

[English version](../en/meminfo.Active_Inactive.md)

---

## これは何？

Linuxのページ回収アルゴリズムはページを**アクティブ**（最近使用）と**非アクティブ**（最近未使用）のリストに分類します。

| メトリクス | 内容 |
|-----------|------|
| `Active` | アクティブメモリ合計（匿名＋ファイル） |
| `Inactive` | 非アクティブメモリ合計 |
| `Active(anon)` | 最近使用されたヒープ/スタック/mmapメモリ |
| `Inactive(anon)` | 非アクティブ匿名メモリ — スワップ候補 |
| `Active(file)` | 最近使用されたファイルキャッシュ |
| `Inactive(file)` | 追い出し可能なファイルキャッシュ |

**`Inactive(anon)`が増加** → カーネルがスワップを検討中の匿名ページあり。`pswpout`が上昇し始めていないか確認。

---

## 関連項目

- `meminfo.MemAvailable` — 全体的なメモリ利用可能性
- `vmstat.nr_active_inactive` — vmstat形式の同データ
- `sourceguide.meminfo` — meminfoソース全体の概要
