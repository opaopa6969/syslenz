# Listen queue drop

これは何か
listen queue がいっぱいで落とされた接続です。

なぜ重要か
クライアントに見える失敗です。サーバーが速く accept できないと、接続が拒否されます。

どう読むか
- 非ゼロなら調査対象
- トラフィック急増で backlog が小さいと溢れる
- GC 停止や遅い accept ループでも出る

次に確認
Tcp_RetransSegs と procs_running を見ます。