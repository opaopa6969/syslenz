# システムコールのライフサイクル

[English version](../en/concept.system-call-lifecycle.md)

---

プログラムが`read()`、`write()`、`socket()`を呼び出すと、**システムコール**が発生します — ユーザー空間からカーネル空間への制御された切り替えです。

各システムコールには約1μsのコンテキストスイッチオーバーヘッドがあります。`stat.cpu_system`が高い場合、アプリケーションは過剰なシステムコールを行っている可能性があります（小さなI/O操作を多数行うなど）。

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
