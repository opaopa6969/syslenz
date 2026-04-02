# I/O 待ち CPU

これは何か
ストレージ操作の完了待ちで CPU が空いていた時間です。

なぜ重要か
iowait は CPU の仕事ではなく、ストレージやファイルシステム待ちの症状です。

どう読むか
- 小さい値なら普通
- 継続的に高いならディスク待ち
- pressure.io_some_avg10 と合わせると停滞の有無が分かる

次に確認
diskstats.active_devices と pressure.io_some_avg10 を見ます。