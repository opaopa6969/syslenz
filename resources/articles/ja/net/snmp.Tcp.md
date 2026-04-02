# TCP統計 — /proc/net/snmp

[English version](../../en/net/snmp.Tcp.md)

---

## これは何？

/proc/net/snmpのコアTCPプロトコルカウンター：接続オープン/クローズ、セグメント送受信、再送、エラー。

これらのカウンターは`/proc/net/snmp`から取得され、各プロトコルのRFC定義MIB（管理情報ベース）統計を追跡します。

---

## 関連項目

- `sourceguide.net/snmp` — /proc/net/snmpソース全体の概要
- `net/netstat.TcpExt` — 拡張TCP統計（TcpExt/IpExt）
