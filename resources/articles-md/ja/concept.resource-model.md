# 記事リソースモデル

これは何か
この記事は resources/articles/index.json と en/ja バンドルから読み込まれます。

なぜ重要か
Rust のソースを変更せずに記事テキストを更新できます。

使い方
- ID を安定維持する
- locale ごとの本文を resource 側で編集する
- metric/group/concept を id で相互リンクする