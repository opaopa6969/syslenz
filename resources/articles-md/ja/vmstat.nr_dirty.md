# Dirty ページ

これは何か
RAM 上で変更されたが、まだストレージへ書き戻されていないページです。

なぜ重要か
Dirty が増え続けると、あとから書き戻し圧力として現れます。

どう読むか
- 短いスパイクはバーストなら普通
- 継続的な増加は書き戻し遅れ
- pressure.io_some_avg10 も上がるなら I/O 影響あり

次に確認
nr_writeback と diskstats.active_devices を見ます。