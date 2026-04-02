# Transparent Huge Pages (THP) — vmstat

[English version](../en/vmstat.thp.md)

---

## これは何？

Linuxカーネルは通常4KBページでメモリを管理します。**Transparent Huge Pages (THP)**は、アプリケーションを変更せずに匿名メモリ（ヒープ、スタック、mmap）に対して2MBページを透過的に使用できる機能です。CPUのTLBにより多くのデータが収まり、TLBミスが減少してメモリアクセスが高速化されます。

```
  通常のページング:             THP:
  ┌──┐┌──┐┌──┐┌──┐            ┌────────────────────────┐
  │4k││4k││4k││4k│  ×512      │         2 MB           │
  └──┘└──┘└──┘└──┘            └────────────────────────┘
  512 TLBエントリ必要           1 TLBエントリで済む
```

カーネルはプロセスが大きな連続領域に触れた際に2MBページを割り当てようとします。フラグメンテーションやメモリ不足で失敗した場合は透過的に4KBへフォールバックします。

---

## THPメトリクス早見表

| メトリクス | カウントするもの |
|-----------|----------------|
| `thp_fault_alloc` | ページフォルト時の2MB割り当て成功 |
| `thp_fault_fallback` | 4KBへのフォールバック（2MB取得不可） |
| `thp_collapse_alloc` | khugepaged による4K→2M折り畳み成功 |
| `thp_collapse_alloc_failed` | khugepaged 折り畳み失敗 |
| `thp_split_page` | 2MBページが4KBに分割 |
| `thp_deferred_split_page` | 分割待ちキューに入ったページ |
| `thp_swpout` | 2MBページ丸ごとスワップアウト |
| `thp_swpout_fallback` | スワップ前に分割が必要だったケース |

---

## なぜ重要？

**THPはトレードオフです。**

**メリット：** TLBミス減少→メモリ集中ワークロード高速化（DB、JVM、ML）

**コスト：**
- `thp_fault_fallback`が高い→フラグメンテーションでTHPが確保できていない
- `thp_split_page`の増加→2MBを確保後に分割（munmap、mprotect等）— 無駄なオーバーヘッド
- RedisやPostgreSQLなどのDBは2MBページへの部分書き込みで書き込み増幅が発生し*悪化*することがある

---

## よくある間違い

**THPが常に有効と思い込む。** データベース、Redis、部分ページ操作の多いアプリは`madvise`または`never`モードが適していることが多い。

**フォールバック率を無視する。** `thp_fault_fallback / thp_fault_alloc`比率が高ければTHPのオーバーヘッドだけ払って恩恵がない。

---

## 関連項目

- `vmstat.compact` — THP割り当てを可能にするメモリ圧縮
- `meminfo.AnonHugePages` — 現在使用中のTHPメモリ
- `vmstat.pgmajfault` — メジャーフォルト（THPで削減可能）
- `sourceguide.vmstat` — vmstatソース全体の概要
