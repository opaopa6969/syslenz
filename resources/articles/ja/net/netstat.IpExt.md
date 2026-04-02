# IpExt — 拡張IP統計

[English version](../../en/net/netstat.IpExt.md)

---

## これは何？

`IpExt`は`/proc/net/netstat`にある拡張IPレイヤーカウンターで、`/proc/net/snmp`の基本RFC MIB統計を補完します。

| メトリクス | カウントするもの |
|-----------|----------------|
| `InOctets` / `OutOctets` | 総バイト数（ヘッダー含む） |
| `InMcastPkts` / `OutMcastPkts` | マルチキャストパケット |
| `InCsumErrors` | 受信IPチェックサムエラー |
| `InNoRoutes` | ルートなしで廃棄されたパケット |
| `ReasmOverlaps` | 重複IPフラグメント（攻撃の可能性） |

`InNoRoutes`が上昇 → ルーティングテーブルの設定ミスまたは誤った宛先への送信。`ReasmOverlaps`は0近辺であるべき。

---

## 関連項目

- `net/snmp.Ip` — 基本IP統計
- `sourceguide.net/netstat` — ソース全体の概要
