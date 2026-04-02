# ワーキングセット — vmstat

[English version](../en/vmstat.workingset.md)

---

## これは何？

**ワーキングセット**はプロセスが実際に使用しているページの集合です。メモリが逼迫するとカーネルはページキャッシュからページを追い出します。追い出されたページに再アクセスした場合が**リフォルト**です。

| メトリクス | カウントするもの |
|-----------|----------------|
| `workingset_refault_anon` | 匿名ページリフォルト（追い出されたが再要求） |
| `workingset_refault_file` | ファイルページリフォルト |
| `workingset_activate_anon` | リフォルト後にアクティブリストへ昇格した匿名ページ |
| `workingset_activate_file` | リフォルト後にアクティブリストへ昇格したファイルページ |
| `workingset_nodereclaim` | シャドウノード回収イベント |

**`workingset_refault_file`が高い = ページキャッシュが不足。**  
ファイルが繰り返し追い出されて再読み込みされています — 不要なディスクI/Oが発生中。

**`workingset_refault_anon`が高い = スワップスラッシュ。**  
スワップページが繰り返しディスクから読み込まれています — 非常にコストが高い。

---

## 関連項目

- `meminfo.Cached` — ファイルページキャッシュサイズ
- `vmstat.pgmajfault` — メジャーページフォルト
- `sourceguide.vmstat` — vmstatソース全体の概要
