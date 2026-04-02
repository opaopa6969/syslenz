# 実行中プロセス

これは何か
今すぐ実行可能なプロセス／スレッド数です。

なぜ重要か
スケジューラの需要を直接見られます。CPU 余力を超えて続くと遅延が増えます。

どう読むか
- 小さいスパイクは問題ない
- 高止まりと loadavg 上昇が一緒なら CPU 需要が本物
- pressure.cpu_some_avg10 も上がるなら、実際に待たされている

次に確認
loadavg.running_threads と pressure.cpu_some_avg10 を見ます。