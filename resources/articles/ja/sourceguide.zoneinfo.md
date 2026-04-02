# メモリゾーン情報

[English version](../en/sourceguide.zoneinfo.md)

---

## これは何？

/proc/zoneinfoはゾーンごとの詳細なメモリ統計を提供します：ウォーターマーク（min/low/high）、ページ数、CPUごとのページキャッシュ。buddyinfoより詳細です。

---

## 関連項目

- `buddyinfo.zones`
- `vmstat.nr_free_pages`
- `sourceguide.vmstat` — vmstatメモリ統計
- `sourceguide.meminfo` — メモリ情報
