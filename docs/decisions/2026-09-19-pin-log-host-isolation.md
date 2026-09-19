# ピンログのホスト分離 — Builder 反復1/3

対象: [引継ぎIssue #55](https://github.com/opaopa6969/syslenz/issues/55)。
ベース: `22008b5`。専用worktreeのブランチ: `fix/pin-log-host-isolation`。

## 観測事実

引継ぎworktreeはclean、open PRは0件、直近mainのCIは成功。
指定のAGENTS.md・CLAUDE.md・docs/product-brief.mdはcheckoutに存在しない。
README・PROJECT.md・既存のpin仕様、CIと最近の変更から既存製品の修正を選んだ。
`--log` はローカルSnapshotだけを取得するが、出力時にはピンのhostを照合せず、
リモート名でローカル値を記録していた。TUIでは空hostをローカルとして扱う。

## 仮説と実施内容

ローカル専用の出力境界で非空hostをスキップすれば、誤ったホストの計測値を
下流に渡さず、既存のローカル出力形式と追記動作を維持できる。
このガード、回帰テスト2件、既存仕様の更新だけを行った。依存追加はない。
リモート対応の新規実装より小さく可逆で、既存のローカル限定仕様に一致するため採用。

## 更新した終了要件と検証

- リモートのsource全体／field指定ピンは、ローカルに同名項目があっても0行。
- 混在時はローカルピンだけをJSONL出力し、複数回の呼出しで追記する。
- 既存のローカルsource全体／field指定／未一致／追記の動作を維持する。
- `cargo test pin_log::tests -- --nocapture` を修正前に実行すると新規2件が失敗。
  リモート専用は期待0行に対して2行、混在は期待1行に対して3行だった。
- 修正後の `cargo test`: unit 288成功・既存2件ignore、parser marker 1成功、smoke 8成功。
- `cargo fmt --check` と `cargo clippy -- -D warnings`: 成功。
- Judgeは上記の再現とPRのCIを確認する。

## 次の判断と残る不確実性

独立Judgeへ差分と証拠を渡し、accept後だけFinalizerがmergeする。
Builderはaccept判定・merge・Issue closeを行わない。
リモート取得は未対応のまま。リモート専用ピンしかない場合も既存ループは継続し、
未一致警告を出す。この修正は空ログからの自動終了やCLI引数検証までは拡張しない。
次の候補は必要に応じてリモート専用設定時の終了・案内改善。
補償: merge commitを `git revert -m 1 <merge-commit>` する通常PRとIssue reopen。
検収・mergeまで今回のworktreeを保持し、Finalizerが後片付けする。

実行主体: Codex Builder。開始: 2026-09-19 12:53 UTC。
モデル詳細・課金額は取得していない。実行時間と検証の最終状態はPR／Issueに記録する。

## 情報源と再現

取得日: 2026-09-19。外部コードや文章の転用なし。

- [対象コードと仕様](https://github.com/opaopa6969/syslenz/tree/22008b5bec973cc9e424b8b6a82b4a069a106240): MIT（repoのLICENSE）。
- [引継ぎ](https://github.com/opaopa6969/syslenz/issues/55)、[開始時CI](https://github.com/opaopa6969/syslenz/actions/runs/35442862284): GitHub作業メタデータ、[GitHub利用規約](https://docs.github.com/en/site-policy/github-terms/github-terms-of-service)の対象。
- 取得手順: `gh issue view 55 --json body,comments,state`、`gh issue list --state open`、
  `gh pr list --state open`、`gh run list --limit 6`、`git worktree list`、`git log -6 --oneline`。
- 回帰テストの再現: `cargo test pin_log::tests`。既存動作との比較は修正前のガードなしで同じテストを実行する。
