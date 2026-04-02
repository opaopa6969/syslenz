# kswapd_inodesteal

[English version](../en/vmstat.kswapd_inodesteal.md)

---

**[vmstat.kswapd](../vmstat.kswapd.md)** ファミリーの一部です — 詳細なコンテキスト、チューニング、実際のエピソードはグループ記事を参照してください。

`kswapd_inodesteal` — メモリ回収のためkswapd が解放したinode。

**ソース:** `/proc/vmstat`  
**単位:** 起動からの累積カウント（単調増加）

---

## 関連項目

- `vmstat.kswapd` — 詳細なグループ記事
- `sourceguide.vmstat` — vmstatソース全体の概要
- `pressure.memory_some_avg10` — カーネルのメモリストールシグナル
