# 空きメモリ

これは何か
いま何にも使われていない RAM です。

なぜ重要か
Linux ではキャッシュに RAM を使うので、MemFree が少ないこと自体は普通です。MemFree 単独では判断しません。

どう読むか
- MemAvailable が十分なら、MemFree が少なくても問題ないことが多い
- MemFree と MemAvailable が同時に下がるなら圧力あり
- スワップも増えるなら回収だけでは足りない

次に確認
Cached、MemAvailable、スワップ動作を合わせて見ます。