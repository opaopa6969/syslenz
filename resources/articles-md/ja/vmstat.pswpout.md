# スワップアウト

これは何か
RAM から swap へ追い出されたページ数です。

なぜ重要か
pswpout の増加は、RAM 圧力が実際に動き始めた証拠です。

どう読むか
- 過去のスワップアウトが少しある程度なら問題ないこともある
- ワークロード稼働中に増えるなら回収が追いついていない
- pressure.memory_full_avg10 が非ゼロなら影響が見えています

次に確認
MemAvailable と pressure.memory_full_avg10 を見ます。