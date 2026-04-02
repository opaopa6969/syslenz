# 1 分負荷

これは何か
過去 1 分の runnable / uninterruptible タスクの平均です。

なぜ重要か
負荷は使用率ではなく需要です。スケジューラの行列が詰まっているかを示します。

どう読むか
- CPU 数と比べる
- CPU 数前後なら多くの環境で許容範囲
- CPU 数超え + PSI 上昇なら競合あり

次に確認
procs_running と pressure.cpu_some_avg10 を見ます。