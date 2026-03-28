# DGE Session 001: syslenz プロダクト価値の Gap 探索

- **Date**: 2026-03-28
- **Theme**: プロダクト全体の価値向上
- **Characters**: 今泉 + ヤン + 僕 + 利根川
- **Template**: feature-planning (カスタム)

---

## Scene 1: そもそも誰が使うのか

先輩 (ナレーション): syslenz は /proc を構造化して TUI で見せるツール。43 ソース対応、スナップショットの diff、sparkline グラフ、JSON export、SSH リモート監視、Web UI、OpenTelemetry export、X11 ウィジェット、日英対応まで実装済み。

👤 今泉: 「すみません、そもそもなんですけど... これ、誰が使うんですか？ htop じゃダメなんですか？」

→ **Gap 発見: ターゲットユーザーのペルソナが未定義。htop/glances/btop との差別化が明文化されてない。**

☕ ヤン: 「htop はプロセスリスト特化でしょ。syslenz は /proc "全部" を構造化してる。でも... 43 ソース全部見る人っているの？普段使いなら meminfo, loadavg, net/dev, processes の 4 つで十分じゃない？紅茶ください。」

→ **Gap 発見: 情報量が多すぎて初心者が迷う。「おすすめビュー」や「ダッシュボード」がない。**

😰 僕: 「...あの、htop 使ったことある人はいいんですけど... そもそも /proc って何かわからない人が、これ起動して何すればいいか... わからないんじゃ...」

→ **Gap 発見: オンボーディング / チュートリアルが存在しない。`?` ヘルプはあるが「最初の 30 秒で何をすべきか」のガイドがない。**

🎰 利根川: 「現実を見ろ。htop は apt install 一発。syslenz は cargo build。Rust のツールチェインがない環境では使えない。バイナリ配布はしてるのか？」

→ **Gap 発見: 配布方法が `cargo install --path .` のみ。バイナリリリース (GitHub Releases)、パッケージマネージャ (brew, apt) 対応がない。**

---

## Scene 2: 実際に使う場面

先輩: ユーザーが syslenz を起動する場面を考える。サーバーの調査、パフォーマンスの監視、障害時の切り分け。

👤 今泉: 「障害時に使うとして... 5 分前の状態と今を比べたいんですけど、それってどうやるんですか？diff はあるけど "1 回前" との差分だけですよね？」

→ **Gap 発見: 時系列の diff がない。「5 分前」「1 時間前」のスナップショットとの比較ができない。リングバッファの 60 スナップショットはあるが、任意の時点を選んで diff する UI がない。**

☕ ヤン: 「あと、アラートないよね。"メモリ 90% 超えたら赤くする" とか。ずっと見てなきゃいけないのは辛い。」

→ **Gap 発見: 閾値アラート / 通知機能がない。監視ツールとしての能動的な通知がゼロ。**

😰 僕: 「...あの、SSH で複数ホスト見たいんですけど... `--ssh` って 1 台ずつですよね... 10 台あったら 10 個ターミナル開くんですか...？」

→ **Gap 発見: マルチホスト監視が未対応。複数ホストを一覧で比較する画面がない。**

🎰 利根川: 「Web UI がある。なら Prometheus + Grafana で良くないか？syslenz の Web UI に何千万もかけた Grafana と戦えるだけの理由があるのか？言ってみろ。」

→ **Gap 発見: Web UI の差別化が不明確。「/proc 全構造化 + ゼロ設定」が強みなら、それをもっと前面に出すべき。Grafana は設定地獄、syslenz は起動 1 秒。**

---

## Scene 3: 運用とエコシステム

先輩: 実装された機能群の運用面を考える。特に OpenTelemetry、Web UI、X11 ウィジェットの実用性。

👤 今泉: 「OpenTelemetry で export して... それをどこで見るんですか？Jaeger？Grafana？ "syslenz --otel して Grafana で見てね" って、結局 Grafana 要るんじゃ...」

→ **Gap 発見: OTEL export のユースケースが中途半端。syslenz 単体で完結するストーリーと、エコシステムに乗るストーリーの整理が必要。**

☕ ヤン: 「設定ファイルがないよね。毎回 `--lang ja --web 8080` って打つの？ `~/.config/syslenz/config.toml` くらい欲しい。toml クレートもう入ってるし。」

→ **Gap 発見: 設定ファイルが未実装。toml 依存はあるのに使ってない。デフォルト言語、リフレッシュ間隔、表示ソースのカスタマイズ等。**

😰 僕: 「...テスト... テストないですよね... cargo test 走らせても何も... 怖い...」

→ **Gap 発見: テストがほぼゼロ。remote.rs に 1 つだけ。パーサーのユニットテスト、UI のスナップショットテスト、export/import のラウンドトリップテストが必要。**

🎰 利根川: 「README に "MIT License" と書いてあるが、LICENSE ファイルがない。GitHub に公開するなら CI もない。GitHub Actions、リリース自動化、README のバッジ。ユーザーは "最終コミット 6 ヶ月前" のツールは使わない。」

→ **Gap 発見: OSS としてのインフラが未整備。LICENSE ファイル、CI/CD、バッジ、CHANGELOG がない。**

---

## Scene 4: プロダクトとしてのポジショニング

先輩: syslenz の立ち位置を整理する。CLI ツールとしての htop/glances との比較、監視プラットフォームとしての Prometheus/Grafana との比較。

👤 今泉: 「要するに、syslenz って何なんですか？htop の亜種？Grafana の簡易版？/proc の勉強ツール？... 全部やろうとしてません？」

→ **Gap 発見: プロダクトのポジショニング / タグラインが曖昧。"Wireshark for /proc" は良いが、それが htop と何が違うのかが README から伝わらない。**

☕ ヤン: 「個人的には "ゼロ設定の /proc エクスプローラ" が一番良いと思う。Grafana はインフラ運用チーム用、syslenz は開発者が ssh して 1 コマンドで全部見える。でもそれなら Web UI とか OTEL とか削ったほうがブレないよ。紅茶おかわり。」

→ **Gap 発見: フィーチャーの優先順位が不明確。コアバリュー（ゼロ設定 /proc ビューア）とエコシステム機能（OTEL, Web UI）の線引きが必要。**

🎰 利根川: 「ユーザーに聞け。10 人に見せて "これ使いたいか" と聞け。想像で語るな。... まあ、1 つだけ言えるのは "スクリーンショット" が README にないことだ。ツールの README にスクリーンショットがないのは、レストランのメニューに写真がないのと同じだ。」

→ **Gap 発見: README にスクリーンショット / GIF がない。TUI ツールにとって致命的。**

---

## Gap Summary

| # | Gap | Category | Severity |
|---|-----|----------|----------|
| G1 | ターゲットユーザー / 差別化が未定義 | Message gap | High |
| G2 | ダッシュボード / おすすめビューがない | Missing logic | Medium |
| G3 | オンボーディングがない | UX gap | Medium |
| G4 | バイナリ配布がない | Ops gap | High |
| G5 | 時系列 diff (任意時点の比較) がない | Missing logic | Medium |
| G6 | 閾値アラート / 通知がない | Missing logic | Medium |
| G7 | マルチホスト監視が未対応 | Missing logic | Low |
| G8 | OTEL ユースケースの整理が必要 | Message gap | Low |
| G9 | 設定ファイル (~/.config/syslenz/config.toml) がない | Missing logic | Medium |
| G10 | テストがほぼゼロ | Test coverage | High |
| G11 | OSS インフラ未整備 (LICENSE, CI, CHANGELOG) | Ops gap | High |
| G12 | ポジショニング / タグラインが曖昧 | Message gap | Medium |
| G13 | README にスクリーンショットがない | Message gap | High |

## Next Actions

- [ ] G1 を深掘り → Session 002 で差別化の会話劇
- [ ] 各 Gap を Spec に落とす
- [ ] 人間レビュー結果を反映
