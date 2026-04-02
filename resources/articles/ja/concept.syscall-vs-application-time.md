# システムコール対アプリケーション時間

[English version](../en/concept.syscall-vs-application-time.md)

---

CPU時間は**ユーザー空間**（アプリケーションコード）と**カーネル空間**（システムコール）に分かれます。

`stat.cpu_user`: アプリコード実行時間
`stat.cpu_system`: カーネル内時間（システムコール）
`stat.cpu_iowait`: I/O待機時間

`cpu_system`が`cpu_user`に対して高い場合：アプリケーションが実際の処理より多くの時間をカーネルで費やしています（過剰なシステムコール、ロック競合等）。

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
