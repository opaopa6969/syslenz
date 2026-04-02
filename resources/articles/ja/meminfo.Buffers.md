# Buffers

[English version](../en/meminfo.Buffers.md)

---

## これは何？

`Buffers` は、ブロックデバイスに関するメタデータをキャッシュするために使われるメモリです——ディレクトリエントリ（dentry）、inodeテーブル、ファイルシステムの構造情報などです。ファイルの内容は含まれません（それは `Cached` です）。

`Buffers` はカーネルのファイルシステムの地図、`Cached` は実際のファイルの内容と考えてください。カーネルがあるファイルがディスクのどこにあるか調べる時は `Buffers` を確認します。ファイルのデータが必要な時は `Cached` を確認します。

```
  ファイルシステム検索："/var/log/app.log はディスクのどこ？"
    → Buffers を確認（dentry/inodeキャッシュ）
    → 発見：inode 4821, block 0x1F3A...

  ファイル読み取り："block 0x1F3A の内容をくれ"
    → Cached を確認（ページキャッシュ）
    → RAMで発見、ディスク読み取り不要
```

モダンなLinuxカーネルでは、`Buffers` は主にinodeとdentryのメタデータを保持します。古来の「ブロック層バッファ」と「ページキャッシュ」の区別は概ね統合されましたが、`Buffers` は `/proc/meminfo` に独立した行として引き続き表示されます。

---

## なぜ重要？

**ほとんどの場合、バックグラウンドノイズです。** `Buffers` は通常小さく——ほとんどのシステムで最大でも数百MB——そして回収可能です。ファイルキャッシュと同様に、カーネルは圧力がかかると解放します。

**注意すべき点：**

- **正常な範囲**：数MB〜数百MB。多くのファイルとディレクトリを持つサーバー（ビルドサーバーやファイルサーバーなど）では数GBもあり得ますが、一般的ではありません。
- **異常に大きい Buffers**（通常のアプリサーバーで複数GB）：何かが多くのファイルやディレクトリをスキャンし、メタデータをウォームな状態に保っています。バックアップジョブ、ファイルインデックス化、暴走したディレクトリ探索が一般的な原因です。
- **Buffers ≠ Cached**：よくある混同です。総RAM消費量が合わない場合は両方を確認してください。

---

## 読み方

```sh
# Cached や MemAvailable と一緒に Buffers を確認
grep -E "MemAvailable|Buffers|Cached" /proc/meminfo

# 注意：/proc/meminfo の "Cached:" は Buffers を除く
# free(1) は両方を合わせた "buff/cache" を表示
free -h
```

**`free` の出力：**

```
              total        used        free      shared  buff/cache   available
Mem:           15Gi       4.2Gi       1.1Gi       256Mi       9.7Gi        10Gi
Swap:           4Gi          0B        4Gi
```

`buff/cache` は `Buffers + Cached` を合算します。どちらも回収可能です。`available`（MemAvailable）はすでに両方を考慮しています。

---

## 実際のエピソード

チームが `rsync` を使って大きなNFS共有をバックアップするジョブを毎晩実行していました——50万ディレクトリにまたがる約200万ファイル。ジョブ実行後、`Buffers` が80 MBから4.2 GBに跳ね上がりました。監視システムが「メモリ使用量が高い」とフラグを立てました。

オンコールのエンジニアは不要ないくつかのサービスを再起動しました。メモリ圧力は本物ではありませんでした：`MemAvailable` はほとんど変化していなかったのです。`Buffers` の増加は、250万個のファイルシステムエントリをスキャンしたことによるdentry/inodeキャッシュのウォームアップにすぎませんでした。バックアップが終了してから1時間以内に、カーネルがそれらのページを他の用途に回収し、`Buffers` は正常値に戻りました。

教訓：個別のコンポーネントではなく `MemAvailable` を確認してください。ファイルシステムスキャンによる大きな `Buffers` は一時的で無害です。

---

## よくある間違い

**`Buffers` と `Cached` を混同する。** `Buffers` = ファイルシステムのメタデータ。`Cached` = ファイルの内容。`free` コマンドは両方を "buff/cache" として合算しますが、これは「回収可能な量」という観点では正しいものの、区別を隠してしまいます。

**`Buffers` のサイズだけでアラートを出す。** ファイルが多いワークロードでは数GBの `Buffers` が正常です。`MemAvailable` を確認して余力に実際に影響しているかどうかを見てください。

**`Buffers` を直接制御しようとする。** 個別にチューニングすることはできません。カーネルは `vfs_cache_pressure`（`/proc/sys/vm/vfs_cache_pressure`）でdentry/inodeキャッシュサイズを管理します。デフォルトは100で、値を大きくするとカーネルがinode/dentryキャッシュをより積極的に回収します。

---

## 関連項目

- `meminfo.Cached` — ファイルの内容キャッシュ（より大きな回収可能プール；Buffersと混同されやすい）
- `meminfo.MemAvailable` — BuffersとCachedの両方を含む総利用可能メモリ
- `meminfo.SReclaimable` — 回収可能なスラブメモリ（スラブで追跡されるdentry/inodeキャッシュを含む）
