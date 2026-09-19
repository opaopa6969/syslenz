# TCP IPv6表示の修正 — Builder 反復1/3

- 対象: [Issue #44](https://github.com/opaopa6969/syslenz/issues/44)
- ベース: `bca7ce0970edd549962b3d6704a10c9c40c9a1d9`
- 作業場所: 専用worktree、ブランチ `autonomy/fan-mu82ofcn-1bz`

## 観測事実

`AGENTS.md`・`CLAUDE.md`・指定の `docs/product-brief.md` は、このcheckoutと元worktreeに存在しなかった。会話の指示、README-ja.md、PROJECT.md、docs/ROADMAP.md、CI定義と最近の変更を確認し、既存製品の不具合修正に限定した。開始時のopen Issue/PRは0件。直近mainのCIは成功していた。

`net_tcp::parse()` はIPv6も収集するが、アドレス変換はIPv4のみで、IPv6の32桁hexをそのまま返していた。実機のIPv6 loopbackソケットで `/proc/net/tcp6` に `00000000000000000000000001000000` が出ることを確認した。追加テストを修正前に実行し、loopback表示とTCP行表示の2件が失敗した。

## 仮説・実施内容

32bit単位でネイティブエンディアンのバイト列に戻し、Rust標準の `Ipv6Addr` で整形すれば、接続先を通常のIPv6表記と照合できる。既存パーサーに変換を追加し、日英のソースリファレンスを実装に合わせた。新規依存や実行基盤の追加はない。

不正入力は従来の表示にフォールバックする。32文字かつASCII hexの検査後にスライスすることで、UTF-8境界のpanicを防ぐ。IPv4やUDPの扱いは今回の変更対象外。

## 観測から更新した終了要件・検証結果

- loopback、unspecified、通常IPv6、IPv4-mapped、ポート0/65535を可読形式で表示: 回帰テストで確認。
- IPv4表示、不正入力のフォールバック、TCP行のproto/state/uid/件数を維持: 回帰テストで確認。
- `cargo test`: unit 281成功・既存benchmark 2件ignore、parser marker 1成功、smoke 8成功。
- `cargo clippy -- -D warnings` と `cargo fmt --check`: 成功。最初のClippyで指摘された新しいchunks lintは、Rustの最低対応版を引き上げないループ表現で解消。
- 依存パッチ更新後も上記テスト・fmt/clippyが成功。`cargo deny check` は advisories/bans/licenses/sources の全項目成功（既存設定等のwarningあり）。
- 実機CLIで一時的なloopbackソケットが `['tcp6', '[::1]:47097', '[::]:0', 'LISTEN', '1000']` と表示された（ポート・UIDは環境依存）。

再現コマンド:

```sh
cargo test proc::net_tcp::tests
cargo test
cargo fmt --check
cargo clippy -- -D warnings
cargo deny check
python3 - <<'PY'
import json, socket, subprocess
with socket.socket(socket.AF_INET6, socket.SOCK_STREAM) as listener:
    listener.bind(('::1', 0))
    listener.listen(1)
    expected = f'[::1]:{listener.getsockname()[1]}'
    output = subprocess.check_output(
        ['target/debug/syslenz', '--query', 'net/tcp.connections', '--json'], text=True)
    rows = json.loads(output)['value']['Table']
    matches = [r for r in rows if r[0] == 'tcp6' and r[1] == expected and r[3] == 'LISTEN']
    assert len(matches) == 1, (expected, matches)
    print(matches[0])
PY
```

## 次の判断・残る不確実性

PR作成後の[CI依存監査](https://github.com/opaopa6969/syslenz/actions/runs/35430004215/job/105862765486)で、既存の `rustls 0.23.43` が `RUSTSEC-2026-0285` に該当して失敗した。監査ログが示した修正版に `cargo update -p rustls --precise 0.23.45` で限定更新し、要求される `rustls-webpki 0.103.15` も更新した。Cargo.tomlと監査ポリシーは変更しない。終了要件に依存監査成功を追加し、同じPRを検収可能にするための最小の前提修正とした。

独立JudgeへPRと上記証拠を渡す。Builder自身はaccept判定・mergeを行わない。JudgeはPRのCIと差分を確認し、acceptの場合だけFinalizerがmergeしてIssueをcloseする。Big-endian実機は未検証（テストfixtureはエンディアン別に用意）。UIは共通データを使うが、ブラウザの描画テストは未実施。

補償はmerge commitを `git revert -m 1 <merge-commit>` する別PRとIssue reopenで行う。次の候補は未選定。実行主体はCodex Builder、開始は2026-09-19 07:35 UTC。モデルの詳細識別子・課金額は取得していない。

## 情報源・利用条件

取得日: 2026-09-19。外部コード・文章の転用なし。

- [対象コード・文書](https://github.com/opaopa6969/syslenz/tree/bca7ce0970edd549962b3d6704a10c9c40c9a1d9): repoのMITライセンス（LICENSE）。
- [開始時のCI](https://github.com/opaopa6969/syslenz/actions/runs/33612041535)、[Issue一覧](https://github.com/opaopa6969/syslenz/issues)、[PR一覧](https://github.com/opaopa6969/syslenz/pulls): GitHubの作業メタデータ。[GitHub利用規約](https://docs.github.com/en/site-policy/github-terms/github-terms-of-service)の対象。`gh issue list`、`gh pr list`、`gh run list` で取得。
- エンディアンの確認元は実機の `/proc/net/tcp6`。再現手順は上記に記載。
- 上記CIログの依存監査結果（GitHub利用規約）を根拠にパッチ更新。[rustls 0.23.45](https://crates.io/crates/rustls/0.23.45) のライセンスは `Apache-2.0 OR ISC OR MIT`、[rustls-webpki 0.103.15](https://crates.io/crates/rustls-webpki/0.103.15) は `ISC`（取得済みcrateのCargo.tomlで確認）。
