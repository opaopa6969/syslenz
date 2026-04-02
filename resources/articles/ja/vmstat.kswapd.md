# kswapd — vmstat

[English version](../en/vmstat.kswapd.md)

---

## これは何？

**kswapd**はカーネルのバックグラウンドメモリ回収デーモンです。空きメモリが**低ウォーターマーク**を下回るとkswapd が起動し、**高ウォーターマーク**を超えるまでページキャッシュからページを回収します（必要なら匿名ページをスワップアウト）。

| メトリクス | カウントするもの |
|-----------|----------------|
| `kswapd_inodesteal` | メモリ回収のためkswapd が解放したinode |
| `kswapd_low_wmark_hit_quickly` | kswapd が素早く低ウォーターマークに到達 |
| `kswapd_high_wmark_hit_quickly` | kswapd が素早く高ウォーターマークに到達 |

**`kswapd_inodesteal`が上昇**している場合、kswapd がinodeキャッシュを積極的に解放しています。その後のファイル操作が遅くなり、持続的なメモリ圧迫を示します。

---

## 関連項目

- `vmstat.allocstall` — ダイレクトリクレームストール（kswapd より深刻）
- `pressure.memory_some_avg10` — ストールシグナル
- `sourceguide.vmstat` — vmstatソース全体の概要
