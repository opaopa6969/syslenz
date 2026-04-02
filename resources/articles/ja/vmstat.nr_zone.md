# ゾーン別ページカウント (nr_zone_*) — vmstat

[English version](../en/vmstat.nr_zone.md)

---

## これは何？

これらのカウンターはグローバルな`nr_active_anon`、`nr_inactive_file`などのメトリクスを**メモリゾーン別**に分解したものです。ほとんどの監視目的ではグローバル版で十分ですが、NUMAの問題、DMAゾーンのプレッシャー、ホットプラグメモリのデバッグ時に有用です。

| メトリクス | カウントするもの |
|-----------|----------------|
| `nr_zone_inactive_anon` | このゾーンの非アクティブ匿名ページ |
| `nr_zone_active_anon` | このゾーンのアクティブ匿名ページ |
| `nr_zone_inactive_file` | このゾーンの非アクティブファイルページ |
| `nr_zone_active_file` | このゾーンのアクティブファイルページ |
| `nr_zone_unevictable` | このゾーンの追い出し不可ページ |
| `nr_zone_write_pending` | このゾーンの書き込み保留ページ |

通常のサーバーでは`nr_zone_normal_*`がほぼすべてのページを保持します。

---

## 関連項目

- `vmstat.nr_active_inactive` — グローバルなアクティブ/非アクティブページカウント
- `buddyinfo.zones` — ゾーンの空きページフラグメンテーション
- `sourceguide.vmstat` — vmstatソース全体の概要
