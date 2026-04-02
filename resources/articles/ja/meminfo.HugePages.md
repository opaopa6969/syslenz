# HugePages — meminfo

[English version](../en/meminfo.HugePages.md)

---

## これは何？

Linuxは通常4KBページを使います。**ヒュージページ**は事前に割り当てられた大きなページ（x86_64では通常2MB）で、RAM内に**ロック**されています — スワップアウトできません。大規模なデータセットを管理するアプリケーション（データベース、大きなヒープのJVM）はTLBプレッシャーを減らすために明示的にヒュージページを要求します。

| メトリクス | 意味 |
|-----------|------|
| `HugePages_Total` | プール内のヒュージページ合計 |
| `HugePages_Free` | 未割り当てのヒュージページ |
| `HugePages_Rsvd` | 予約済みだがまだマップされていない |
| `HugePages_Surp` | 余剰ページ（オーバーコミットプールから） |
| `Hugepagesize` | 各ヒュージページのサイズ（通常2048KB） |

PostgreSQLやOracleはヒュージページの設定が必要なことが多いです。`HugePages_Free`がゼロに近い場合はプールが枯渇しています。

---

## 関連項目

- `vmstat.thp` — Transparent Huge Pages（自動、プールなし）
- `sourceguide.meminfo` — meminfoソース全体の概要
