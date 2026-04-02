# CPU ランキュー

これは何か
CPU 実行待ちの runnable task の集合です。

なぜ重要か
ランキューが増えると、仕事は準備できていてもすぐ実行できません。これは競合の直接的なサインです。

どう使うか
- runnable task 数と CPU 数を比べる
- context switch の増加と load average の伸びを見る
- 負荷が compute bound か、別の待ちがあるか確認する

よくある誤り
- load average を CPU 使用率と同じものだと思う
- 多コア環境の runnable task を無視する
- 仮想化環境の steal time や throttling を見落とす

診断フロー
1. load と runnable 数を確認する
2. CPU 圧力と利用率を確認する
3. 単一プロセスか全体かを確認する
4. 本当の原因が CPU、IO、ロックのどれか確認する
