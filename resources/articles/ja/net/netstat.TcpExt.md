# TcpExt — 拡張TCP統計

[English version](../../en/net/netstat.TcpExt.md)

---

## これは何？

`TcpExt`は、Linuxが基本的なRFC MIB統計を超えて追跡する約130の拡張TCPカウンターです。`/proc/net/netstat`にあり、TCPスタックの内部を公開します：SYNクッキー、輻輳制御、再送詳細、メモリプレッシャー、Fast Openなど。

`/proc/net/snmp`がTCPサマリーなら、`TcpExt`はTCPデバッグログです。

---

## 最重要カウンター

**SYNフラッド/接続攻撃:**
- `SyncookiesSent` — SYNフラッド保護が作動中
- `ListenOverflows` — acceptキューが満杯
- `ListenDrops` — SYNが廃棄（上昇中なら深刻）

**再送とロス:**
- `TCPFastRetrans` — 高速再送（SACK、ロスへの良い応答）
- `TCPTimeouts` — RTOタイムアウト（遅くコストが高い）
- `TCPLostRetransmit` — ロストした再送（非常に悪い）

**メモリプレッシャー:**
- `TCPMemoryPressures` — TCPソケットがメモリプレッシャーモードに
- `TCPAbortOnMemory` — メモリプレッシャーによる接続中断

---

## 関連項目

- `net/snmp.Tcp` — シンプルなTCPサマリー
- `net/netstat.IpExt` — 拡張IP統計
- `sourceguide.net/netstat` — ソース全体の概要
