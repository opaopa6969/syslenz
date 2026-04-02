# ファイルディスクリプター使用量

[English version](../en/sourceguide.file-nr.md)

---

## これは何？

/proc/sys/fs/file-nrはシステム全体のファイルディスクリプター使用量を示します：割り当て済み、空き（キャッシュ内）、最大値。ファイルディスクリプターが枯渇すると'too many open files'エラーが発生します。

---

## 関連項目

- `net/sockstat.sockets_used`
- `sourceguide.vmstat` — vmstatメモリ統計
- `sourceguide.meminfo` — メモリ情報
