# DGE Session 002: G1 深掘り — syslenz は誰のためのツールか

- **Date**: 2026-03-28
- **Theme**: ターゲットユーザー / 差別化の明確化
- **Parent Gap**: G1 (Session 001)
- **Characters**: 今泉 + 大和田 + 利根川 + ラインハルト (ゲスト: 僕, ヤン)

---

## Scene 1: 競合との差分

先輩 (ナレーション): Linux のシステム情報ツールは htop, glances, btop, nmon, atop, sar, /proc を直接 cat する方法がある。syslenz は「/proc 全体を構造化・型付きで閲覧する」ツール。現状の差別化ポイントを整理する。

👤 今泉: 「そもそもなんですけど、htop で見れない /proc の情報って何ですか？具体的に。」

先輩: htop はプロセスリスト + CPU/メモリのバーグラフ。/proc/net/tcp, /proc/crypto, /proc/buddyinfo, /proc/slabinfo, /proc/pressure なんかは htop では一切見れない。glances は近いが、構造化データとして export する機能はない。

👤 今泉: 「要するに、htop = プロセスモニタ、syslenz = /proc エクスプローラ。別カテゴリじゃないですか。なのに README で htop と比較される書き方してません？」

→ **Gap 発見: 比較対象が間違っている。htop ではなく /proc を直接 cat する行為の上位互換として位置づけるべき。**

🦈 大和田: 「で、/proc/buddyinfo を見たいユーザーは何人いるんだ？ Linux カーネル開発者か、SRE か、セキュリティ監査か。ニッチすぎないか？」

→ **Gap 発見: ユーザーセグメントの定量化がない。「誰が」「何人」使うのかの仮説がない。**

🎰 利根川: 「答えを教えてやろう。/proc を cat してる人間は世界中にいる。Stack Overflow で "how to read /proc/net/tcp" を検索してみろ。年間何万件だ。その全員が syslenz の潜在ユーザーだ。問題は "知らない" ことだ。」

→ **Gap 発見: 発見可能性 (discoverability) が低い。SEO、Hacker News、Reddit /r/linux, /r/rust での露出戦略がない。**

👑 ラインハルト: 「小さく考えるな。syslenz は "Linux の透明化" だ。/proc は Linux の内臓だが、誰も読み方を教わらない。syslenz はその教科書であり顕微鏡だ。"Wireshark for /proc" — この表現は正しい。Wireshark がなかった時代、パケットを読める人間は少数だった。syslenz がなければ /proc を読める人間は少数のままだ。」

→ **Gap 発見: ビジョンステートメントが壮大だが、それが README に反映されてない。"Linux の内臓を可視化する" というストーリーがない。**

---

## Scene 2: ペルソナの具体化

先輩: 3 つの想定ペルソナで検討する。(A) SRE / インフラエンジニア、(B) Linux 学習者 / CS 学生、(C) セキュリティ監査員。

👤 今泉: 「(A) の SRE って、もう Datadog とか入ってますよね？syslenz をわざわざ追加で入れる理由ってあるんですか？」

🦈 大和田: 「Datadog の月額を知ってるか？ホストあたり $23/月。100 台で年間 $27,600。syslenz はタダだ。... だが、Datadog の代替にはならない。アラート、ダッシュボード、チームコラボ、全部ない。」

→ **Gap 発見: SRE 向けには "Datadog の補助ツール" か "Datadog を入れられない環境の代替" としてのポジショニングが必要。**

🎰 利根川: 「(B) の学生に売れ。"Linux を理解するための TUI ツール" — 教育市場だ。/proc の各フィールドに説明がついてる。これは教科書だ。大学の OS の授業で使える。」

→ **Gap 発見: 教育ユースケースが明示されてない。`?` ヘルプの説明文は既にあるが、"Learn Linux internals with syslenz" のような入口がない。**

👑 ラインハルト: 「(C) のセキュリティ監査。/proc/net/tcp の全コネクション、/proc/modules のカーネルモジュール、/proc/cgroups。これらを JSON で export して監査レポートに添付する。Wireshark の pcap と同じだ。これは唯一無二の機能だ。」

→ **Gap 発見: セキュリティ監査向けの "export → レポート" ワークフローが文書化されてない。JSON export はあるが、それを監査にどう使うかの例がない。**

😰 僕 (ゲスト参加): 「...全部のペルソナに対応するのは...無理じゃないですか... 1 つ選びません...？」

☕ ヤン (ゲスト参加): 「README の最初の 1 行で決まるよ。今の "Wireshark for /proc" は (A) 向け。"Learn Linux internals interactively" なら (B)。"Audit Linux systems in one command" なら (C)。どれ？」

→ **Gap 発見: README の 1 行目がペルソナに紐づいてない。複数ペルソナ対応するなら「タグライン + 3 ペルソナの使い方セクション」が必要。**

---

## Scene 3: 差別化の言語化

先輩: ここまでの議論から、syslenz のユニークな強みを 3 つに整理する。

👑 ラインハルト: 「1つ目。**ゼロ設定**。バイナリ 1 つ、設定ファイルゼロ、root 不要（ほとんどの /proc は読める）。ssh して実行するだけ。」

🦈 大和田: 「2 つ目。**構造化 export**。htop や glances は画面表示だけ。syslenz は JSON で全データを吐ける。パイプライン、CI、監査レポートに使える。」

🎰 利根川: 「3 つ目。**教育性**。全フィールドに型と説明がついてる。/proc の "辞書" だ。他のツールにこれはない。」

👤 今泉: 「じゃあ、README は... "syslenz = ゼロ設定 + 構造化 + 教育性" って書けばいいんですか？」

→ **Spec 提案: README のリード文を以下に変更**

```
syslenz — Wireshark for /proc

Explore every /proc file as structured, typed data.
Zero config. One binary. Full JSON export.

• SREs: Instant deep-dive without Datadog
• Students: Interactive Linux internals textbook
• Auditors: One-command system snapshot for compliance
```

---

## Gap Summary (Session 002)

| # | Gap | Category | Observe → Suggest → Act |
|---|-----|----------|------------------------|
| G1-1 | 比較対象が htop になってる | Message gap | Observe: README が htop 比較を暗示 → Suggest: /proc cat の上位互換として位置づけ → Act: README 書き換え |
| G1-2 | ユーザーセグメントの定量化なし | Business gap | Observe: 「誰が何人」の仮説なし → Suggest: 3 ペルソナ (SRE/学生/監査) を定義 → Act: README に 3 セクション |
| G1-3 | 発見可能性が低い | Ops gap | Observe: 露出チャネルなし → Suggest: HN, Reddit, awesome-rust 登録 → Act: 公開時のチェックリスト作成 |
| G1-4 | ビジョンが README に反映されてない | Message gap | Observe: "Linux の透明化" が語られてない → Suggest: リード文刷新 → Act: 上記 README 案 |
| G1-5 | ペルソナ別の使い方が未記載 | Message gap | Observe: 使い方が 1 通りだけ → Suggest: ペルソナ別セクション → Act: README に 3 ペルソナ追加 |
| G1-6 | 教育ユースケース未明示 | Message gap | Observe: `?` ヘルプはあるが入口がない → Suggest: "Learn mode" や Tutorial → Act: `--tutorial` フラグ or Welcome 画面 |
| G1-7 | 監査ワークフロー未文書化 | Message gap | Observe: JSON export はあるが使い方の例がない → Suggest: 監査レポート例 → Act: docs/ に例を追加 |

## Next Actions

- [ ] G1-4 を実装 → README リード文刷新
- [ ] G1-2, G1-5 を実装 → ペルソナ別セクション追加
- [ ] G2 (ダッシュボード) の DGE Session → Session 003
- [ ] G1-6 (教育) の DGE Session → Tutorial 設計
