# DGE Session 011: カテゴリ教育 — ソース間の関係性を教える横断的学習コンテンツ

- **Date**: 2026-03-28
- **Theme**: 個別フィールドの説明を超え、ソース間の関係性をカテゴリ単位で教育する仕組みの設計
- **Parent Gaps**: G-EDU-2 (教育的価値), G-EDU-7 (粒度基準), 新規カテゴリ Gap
- **Characters**: 千石 (品質の番人) + 今泉 (初心者の代弁者) + ハウス (診断の天才) + ヤン (怠惰な簡潔主義者) + 僕
- **Input**: Session 009 で個別フィールドの 3 層教育コンテンツ (概要/解説/診断Tips) と `HelpMode` (Hidden/Compact/Expanded) を設計済み。Session 010 で `/sys` ソース (df, thermal, file-nr) を追加し 46 ソース体制。しかし教育は全てソース単位・フィールド単位。「メモリ全体の話」「CPU 負荷の全体像」「ネットワークの問題切り分け」といった **ソース横断的な教育コンテンツ** が欠落。

---

## 現状の整理

先輩 (ナレーション): 現在の教育コンテンツの限界を整理する。

- `source_description()`: 46 ソースの 1 行説明 (en/ja)
- Session 009 設計: `field_description()` で主要 50 フィールドの i18n、`EducationContent` で 5 ソースの 3 層コンテンツ
- `diagnostics.rs`: meminfo, loadavg, swap, pressure, processes, net/tcp, disk, thermal, fd の 9 チェック
- **全て「1 ソース内」の話**。ソース間の関係性は教えていない

問題の本質:
1. **個別フィールド説明は「症状」** — MemAvailable が低い、load が高い。それぞれの意味は教えられる
2. **「病気」は複数ソースにまたがる** — "メモリが足りない" を理解するには meminfo + vmstat + swaps + pressure memory + buddyinfo を横断する必要がある
3. **ユーザーの疑問はカテゴリ単位** — "なぜサーバーが重い?" "ネットワークが遅い?" は単一ソースでは答えられない
4. **diagnostics.rs は自動分析** — 関係性の「教育」はしない。異常を検出するが、なぜそうなるかを教えない

---

## Scene 1: なぜカテゴリ教育が必要か

先輩: 個別フィールド説明とカテゴリ教育の違いについて議論する。

👤 今泉: 「すみません、根本的な質問なんですけど... Session 009 で MemAvailable の説明は充実しましたよね。"MemFree + 回収可能なキャッシュ + Slab" とか、"MemTotal の 10% 以下で OOM 危険" とか。でも... MemAvailable と SwapFree と Cached の **関係** はどこにも書いてないですよね？」

🏥 ハウス (杖をコンッと鳴らす): 「いい着眼点だ。患者の血圧だけ見ても意味がない。血圧 + 心拍 + 体温 + 血液検査 — これを組み合わせて初めて診断できる。MemAvailable が低い。で、なぜ低い？ Cached が少ないのか？ SwapUsed が増えてるのか？ vmstat の pgmajfault が跳ねてるのか？ それぞれ原因が違う。**個別のメトリクスは症状だ。カテゴリは病気の名前だ。**」

👤 今泉: 「具体的に、今の syslenz で "メモリが足りない" を調べようとすると...」

🏥 ハウス: 「サイドバーで meminfo を開いて MemAvailable を見る。次に swaps に切り替えて SwapUsed を見る。次に pressure に切り替えて memory_some_avg10 を見る。次に vmstat に切り替えて pgmajfault を見る。4 ソースを行ったり来たり。各ソースの説明は個別に出るが、"この 4 つを見るべき" という **ナビゲーション** がない。」

🎋 千石: 「教育コンテンツの質は生命線だ。中途半端なカテゴリ説明なら出さない方がいい。"メモリカテゴリ: meminfo と vmstat を見ましょう" — こんなのは教育ではない。買い物リストだ。**なぜ** その組み合わせを見るのか、**どの順番で** 見るのか、**何が分かるのか** を伝えなければ価値がない。」

☕ ヤン: 「まとめると:」

```
現在: ソース単位の教育
  meminfo → "メモリ使用状況"
  vmstat  → "仮想メモリ統計"
  swaps   → "スワップ領域"
  pressure → "PSI 情報"

必要: カテゴリ単位の教育
  💾 メモリカテゴリ → "メモリ全体のストーリー"
    meminfo で総量と空きを把握
    → Cached/Buffers が大きいなら Linux の正常動作 (慌てるな)
    → MemAvailable が低いなら SwapUsed を確認
    → SwapUsed が増えてるなら vmstat si/so を確認
    → pressure memory_some_avg10 > 0 なら実際にタスクが停滞
    → buddyinfo でメモリ断片化を確認
```

→ **Gap G-CAT-1 発見: ソース間の関係性を教育するカテゴリ単位のコンテンツが存在しない。Session 009 の教育は全てソース内に閉じている。**

---

## Scene 2: カテゴリの定義

先輩: どのカテゴリを定義すべきか。粒度と数の議論。

☕ ヤン: 「46 ソースをカテゴリに分類しよう。最初の案:」

```
💾 メモリ:
  meminfo, vmstat, buddyinfo, swaps, pressure (memory), zoneinfo, slabinfo, pagetypeinfo

⚡ CPU/負荷:
  stat, loadavg, cpuinfo, pressure (cpu), schedstat, softirqs, interrupts

🌐 ネットワーク:
  net/dev, net/tcp, net/udp, net/unix, net/arp, net/route, net/sockstat, net/snmp, net/netstat, net/wireless

💿 ストレージ:
  diskstats, df, mounts, partitions, pressure (io), locks

🔧 プロセス:
  processes, file-nr, stat (forks/context_switches)

🛡 セキュリティ:
  modules, crypto, cgroups, net/tcp (open ports)

🌡 ハードウェア:
  thermal, cpuinfo (freq), dma, iomem, ioports
```

☕ ヤン: 「残りの orphans: version, uptime, cmdline, consoles, devices, filesystems, misc — これらは "🔩 システム情報" にまとめる。」

👤 今泉: 「えっと... 8 カテゴリですか？ ユーザーが覚えられますか？」

☕ ヤン: 「7 個？ 多いって。メモリ、CPU、ネット、ディスク、その他の 5 個でいい。ユーザーが "セキュリティカテゴリ" なんて検索する？ modules と crypto を同じカテゴリにして何が嬉しい？ セキュリティの横断的ストーリーが語れるならいいが、それは syslenz の守備範囲じゃない。紅茶ください。」

🏥 ハウス: 「ヤンに半分同意する。カテゴリの存在意義は **横断的ストーリーが語れるかどうか** だ。メモリはストーリーがある。"RAM はどこに消えた?" を meminfo → vmstat → swaps → pressure で追跡できる。CPU もストーリーがある。"サーバーが重い" を loadavg → stat → cpuinfo → pressure で切り分けられる。ネットワークもある。"通信が遅い" を net/dev → net/tcp → net/sockstat → net/snmp で追える。ストレージもある。"ディスクが遅い" を diskstats → df → mounts → pressure io で診断できる。」

🏥 ハウス: 「だがセキュリティは？ modules と crypto の関係は？ カーネルモジュールの一覧と暗号アルゴリズムの一覧を並べて何の診断ストーリーが語れる？ これは **カテゴリではなくチェックリスト** だ。教育コンテンツではない。」

🎋 千石: 「同意する。カテゴリの判定基準を明確にしよう:」

```
カテゴリ成立の条件:
  1. 3 つ以上のソースが関連する
  2. ソース間に因果関係がある (A が異常なら B を確認 → C が原因)
  3. "なぜ X が起きた?" という実践的な問いに答えられる
  4. 診断フローチャートが描ける
```

🎋 千石: 「この基準で判定する:」

| カテゴリ候補 | 関連ソース数 | 因果関係 | 実践的問い | 判定 |
|---|---|---|---|---|
| 💾 メモリ | 8 | MemAvailable→Swap→PSI | "メモリが足りない?" | **採用** |
| ⚡ CPU/負荷 | 7 | load→cpu_stat→PSI→schedstat | "サーバーが重い?" | **採用** |
| 🌐 ネットワーク | 10 | net/dev errors→tcp states→sockstat | "通信が遅い?" | **採用** |
| 💿 ストレージ | 6 | diskstats→df→mounts→PSI io | "ディスクが遅い?" | **採用** |
| 🔧 プロセス | 3 | processes→file-nr→stat forks | "プロセスが多すぎ?" | **採用** |
| 🛡 セキュリティ | 4 | modules + crypto + cgroups | (チェックリスト) | **不採用** |
| 🌡 ハードウェア | 5 | thermal→cpuinfo freq | "温度が高い?" | **保留** |
| 🔩 システム情報 | 7 | なし (独立情報) | なし | **不採用** |

😰 僕: 「...あの、ハードウェアは保留ですか...」

🏥 ハウス: 「ハードウェアは thermal と cpuinfo freq の 2 ソース間にストーリーがある。"温度が高い → サーマルスロットリング → CPU freq が下がる → 性能低下" は因果関係だ。だが dma, iomem, ioports は情報参照用で、教育ストーリーに組み込めない。thermal + cpuinfo だけで 2 ソース。ぎりぎりだな。」

☕ ヤン: 「Phase 1 は 5 カテゴリ。ハードウェアは Phase 2。これでいいだろ。」

→ **Spec 提案: Phase 1 は 5 カテゴリ (メモリ, CPU/負荷, ネットワーク, ストレージ, プロセス)。ハードウェアは Phase 2。セキュリティとシステム情報はカテゴリ不成立。**

---

## Scene 3: カテゴリ教育コンテンツの構造

先輩: 各カテゴリの教育コンテンツに何を含めるか。品質基準を決める。

☕ ヤン: 「各カテゴリの教育コンテンツ構造:」

```
CategoryEducation {
  1. 概要 (overview)      — 2-3 行。このカテゴリが何を扱い、なぜ重要か
  2. ストーリー (story)    — 5-10 行。ソース間の関係を物語として語る
  3. 診断フロー (flow)     — 3-5 ステップ。"もし X なら Y を確認"
  4. よくある問題 (issues)  — 2-3 パターン。典型的な問題と確認すべきメトリクス
}
```

🎋 千石: 「ストーリーの品質基準を決めておく。**初心者が "なるほど!" と思えなければ出す意味がない。** 具体的には:」

```
品質チェックリスト:
  □ 専門用語を初出時に説明しているか
  □ "なぜそうなるか" の因果関係が明示されているか
  □ 確認する順番に論理的な理由があるか (最も情報量が多いソースから)
  □ 各ステップで "次に何を見るべきか" が明示されているか
  □ 典型的な誤解 (MemFree が少ない = メモリ不足) を正しているか
```

🏥 ハウス: 「具体例。メモリカテゴリのストーリー:」

```
💾 メモリ — "RAM はどこに消えた?" ストーリー

Linux ではメモリの "空き" は見かけより多い。
MemFree が 200MB でも慌てるな。Linux は空き RAM をディスクキャッシュに使う。
これが Cached + Buffers だ。必要になれば回収できる。

本当の空き = MemAvailable (カーネルが計算した "本当に使える量")

確認の順番:
  1. meminfo: MemAvailable を確認 — これが MemTotal の 10% 以下なら要注意
  2. meminfo: Cached + Buffers を確認 — 大きければ Linux が正常にキャッシュしている
  3. swaps: SwapUsed を確認 — 増えていればメモリ圧力がある証拠
  4. vmstat: pgmajfault (メジャーページフォルト) — 増加していればスラッシング
  5. pressure: memory_some_avg10 — 0 より大きければタスクがメモリ待ちで停滞中
  6. buddyinfo: 断片化の確認 — 高オーダーの空きが 0 ならメモリ断片化

MemAvailable は十分なのに "メモリ不足" と思い込むのが最も多い誤解。
MemFree ではなく MemAvailable を見る習慣をつけよう。
```

👤 今泉: 「あ、これは分かりやすいです。1 → 2 → 3 → 4 → 5 → 6 の順番に見ていけばいいんですね。でも... これ長くないですか？ ヘルプパネル 8 行に収まります？」

☕ ヤン: 「収まらない。Session 009 の `HelpMode::Expanded` は 8 行で、ソース単位の教育を想定していた。カテゴリ教育は別の表示場所が必要。」

→ **Gap G-CAT-2 発見: カテゴリ教育コンテンツは HelpMode::Expanded (8行) に収まらない。別の表示メカニズムが必要。**

🎋 千石: 「ストーリーと診断フローを合わせると、1 カテゴリあたり 15-25 行。5 カテゴリ × 2 言語で 150-250 行のコンテンツ量になる。ヘルプパネルの拡張では対応できない。」

😰 僕: 「...全部書くの大変...」

☕ ヤン: 「Phase 1 はメモリ、CPU、ネットワークの 3 カテゴリだけ書く。ストレージとプロセスは Phase 2。3 カテゴリ × 2 言語 × 20 行 = 120 行。これなら現実的。紅茶ください。」

→ **Spec 提案: MVP は 3 カテゴリ (メモリ, CPU/負荷, ネットワーク) のコンテンツ。各カテゴリ: 概要 + ストーリー + 診断フロー + よくある問題。**

---

## Scene 4: UI/UX — どこに表示するか

先輩: カテゴリ教育コンテンツの表示方法と UI について議論する。

👤 今泉: 「そもそもカテゴリ説明ってどこに表示するんですか？ ヘルプパネルに入りきらないのは分かりました。じゃあ... Dashboard? 新しいビュー？ サイドバー？」

☕ ヤン: 「4 つの選択肢を並べよう:」

```
案 A: サイドバーをカテゴリでグループ化
  サイドバーが:
    meminfo
    vmstat
    loadavg
    ...
  から:
    💾 メモリ
      meminfo
      vmstat
      buddyinfo
      swaps
    ⚡ CPU/負荷
      stat
      loadavg
      cpuinfo
    🌐 ネットワーク
      net/dev
      net/tcp
      ...
  に変わる。カテゴリ行を選択すると、右ペインにカテゴリ教育を表示。

案 B: 新しいビュー View::CategoryGuide (独立した教育画面)
  'H' キーで専用画面に遷移。全画面で教育コンテンツを表示。
  左にカテゴリ一覧、右にストーリー + 診断フロー。

案 C: ヘルプパネルの拡張 (スクロール対応)
  HelpMode::Expanded を 8 行固定ではなく、最大画面高の 50% まで可変に。
  カテゴリ教育はスクロール可能なパネルで表示。

案 D: Dashboard にカテゴリセクションを追加
  Dashboard の各セクション (Memory, CPU, Network) にカテゴリ教育へのリンク。
  セクション選択時に教育コンテンツを overlay 表示。
```

🏥 ハウス: 「案 B だ。理由: 教育は集中して読むものだ。パネルの片隅に押し込まれたテキストを読む人間はいない。医学教科書を手術室で読むか？ 読まないだろう。読む場所は図書館だ。教育専用の画面を用意しろ。」

👤 今泉: 「でも、案 A のサイドバーグループ化も便利じゃないですか？ 46 ソースがフラットに並んでると探しにくいし...」

☕ ヤン: 「A と B は排他じゃない。両方やれる。ただし MVP の話をしてる。」

😰 僕: 「...サイドバーをカテゴリでグループ化するだけで十分では... それだけでも十分価値あると思いますが...」

🏥 ハウス: 「グループ化は **ナビゲーション** の改善であって **教育** ではない。46 ソースが 5 グループになれば見やすくなるが、"なぜこの 5 ソースを一緒に見るべきか" は教えていない。教育には文章量が要る。15-25 行のコンテンツはサイドバーの括弧では見せられない。」

🎋 千石: 「整理しよう。2 つの独立した機能がある:」

```
機能 1: サイドバーのカテゴリグループ化 (ナビゲーション改善)
  - 46 ソースを 5+α カテゴリで折りたたみ
  - Enter でカテゴリ展開/折りたたみ
  - 目的: ソースの発見性向上
  - 実装コスト: 中 (サイドバーの描画ロジック変更)

機能 2: カテゴリガイド画面 (教育)
  - View::CategoryGuide として独立画面
  - カテゴリ選択 → ストーリー + 診断フロー表示
  - 目的: ソース間の関係性の理解
  - 実装コスト: 中 (新 View + コンテンツ)
```

☕ ヤン: 「MVP 判定:」

```
MVP (今やる):
  機能 2: View::CategoryGuide — カテゴリ教育の本体
  キーバインド: 'C' キー (Category) でどの画面からでも遷移可能

Phase 2 (後で):
  機能 1: サイドバーグループ化
  理由: サイドバーの描画ロジック変更は影響範囲が広い。
        折りたたみ状態の管理、スクロール位置の保持、
        検索との統合 (カテゴリ内検索?) など考えることが多い。
        教育コンテンツ自体とは独立した UI 改善。
```

🏥 ハウス: 「`View::CategoryGuide` の画面設計:」

```
┌─ Category Guide ──────────────────────────────┐
│ [💾 メモリ] [⚡ CPU] [🌐 ネット] [💿 ディスク] [🔧 プロセス] │
├───────────────────────────────────────────────┤
│ 💾 メモリ — "RAM はどこに消えた?"              │
│                                               │
│ ■ 概要                                        │
│ Linux ではメモリの "空き" は見かけより多い。     │
│ MemFree が少なくても Cached + Buffers が...     │
│                                               │
│ ■ ストーリー                                   │
│ 1. meminfo: MemAvailable を確認               │
│ 2. meminfo: Cached + Buffers を確認            │
│ 3. swaps: SwapUsed を確認                      │
│ 4. vmstat: pgmajfault を確認                   │
│ 5. pressure: memory_some_avg10 を確認          │
│                                               │
│ ■ 診断フロー                                   │
│ メモリが足りない?                               │
│ └→ MemAvailable < 10%?                        │
│    ├→ Yes → Cached 大きい? → 回収可能         │
│    │       SwapUsed 増えてる? → vmstat確認     │
│    └→ No → 問題なし (MemFree が少ないだけ)     │
│                                               │
│ ■ よくある問題                                 │
│ • MemFree 低い + MemAvailable 高い → 正常      │
│ • SwapUsed 増加 + pgmajfault 増加 → スラッシング│
├───────────────────────────────────────────────┤
│ C:close  ←→:category  ↑↓:scroll  Enter:jump  │
└───────────────────────────────────────────────┘
```

👤 今泉: 「`Enter:jump` って何ですか？」

🏥 ハウス: 「ストーリーの "1. meminfo: MemAvailable を確認" にカーソルを合わせて Enter を押すと、Overview に遷移して meminfo が選択された状態になる。教育から実践へのブリッジだ。"教科書を読んだら実際に患者を見に行け" ということだ。」

🎋 千石: 「重要な設計判断。`Enter:jump` は教育画面の価値を大きく高める。読むだけで終わらず、実際のデータに飛べる。ただし実装は、各ストーリーの行にジャンプ先 (source_name, field_name) のメタデータを持たせる必要がある。」

→ **Gap G-CAT-3 発見: カテゴリガイドからソース/フィールドへのジャンプ機能。教育と実データの接続。**

→ **Spec 提案: MVP は View::CategoryGuide + 'C' キー。Phase 2 でサイドバーグループ化。Enter:jump は MVP に含める (教育の実用性に直結)。**

---

## Scene 5: 診断フローチャートと MVP

先輩: ハウスが各カテゴリの診断フローチャートを設計し、MVP の実装スコープを確定する。

🏥 ハウス (ホワイトボードに図を描き始める): 「各カテゴリの診断フローチャート。これが教育の核心だ。ユーザーが "何が起きてるか分からない" とき、このフローに従えば原因にたどり着ける。」

### 💾 メモリの診断フロー

```
"メモリが足りない?"
 │
 ├→ MemAvailable 確認 (meminfo)
 │   ├→ > MemTotal の 20% → 問題なし。MemFree が少ないだけ
 │   └→ < MemTotal の 10% → 要注意
 │       │
 │       ├→ Cached + Buffers が大きい? (meminfo)
 │       │   └→ Yes → 回収可能。echo 3 > /proc/sys/vm/drop_caches で回収テスト
 │       │
 │       ├→ SwapUsed が増加中? (swaps)
 │       │   └→ Yes → vmstat の si/so 確認
 │       │       ├→ si/so が活発 → スラッシング。メモリ増設 or プロセス特定
 │       │       └→ si/so = 0 → 過去にスワップされたが今は安定
 │       │
 │       ├→ pgmajfault が増加中? (vmstat)
 │       │   └→ Yes → ディスクからページ読込発生。I/O 負荷の原因
 │       │
 │       └→ memory_some_avg10 > 0? (pressure)
 │           └→ Yes → タスクがメモリ待ちで停滞中。即座に対応が必要
 │               memory_full_avg10 > 0 → 全タスク停滞。OOM 直前
```

### ⚡ CPU/負荷の診断フロー

```
"サーバーが重い?"
 │
 ├→ load_1min 確認 (loadavg)
 │   ├→ < CPU コア数 → CPU は飽和していない。他の原因を探す
 │   └→ > CPU コア数 × 2 → CPU 飽和 or I/O 待ち
 │       │
 │       ├→ cpu_iowait 確認 (stat)
 │       │   ├→ > 20% → I/O がボトルネック。ストレージカテゴリへ
 │       │   └→ < 5% → CPU バウンド
 │       │
 │       ├→ cpu_user + cpu_system 確認 (stat)
 │       │   ├→ user 高い → アプリケーションの CPU 消費
 │       │   └→ system 高い → カーネルの CPU 消費 (syscall 多い? context_switch 多い?)
 │       │
 │       ├→ cpu_some_avg10 確認 (pressure)
 │       │   └→ > 25% → 確実に CPU 圧力。PSI が load average より正確
 │       │
 │       └→ context_switches 確認 (stat)
 │           └→ 異常に多い → プロセス切替コストが高い。スレッド数過多?
```

### 🌐 ネットワークの診断フロー

```
"通信が遅い?"
 │
 ├→ net/dev: errors + drops 確認
 │   ├→ > 0 → 物理層 or ドライバの問題。NIC 設定確認
 │   └→ = 0 → 物理層は正常。上位層を確認
 │       │
 │       ├→ net/tcp: 状態分布確認
 │       │   ├→ SYN_SENT 多数 → 接続先に到達できない。FW? DNS?
 │       │   ├→ TIME_WAIT 大量 → エフェメラルポート枯渇リスク
 │       │   ├→ CLOSE_WAIT 増加 → アプリの close 忘れ。FD リーク
 │       │   └→ ESTABLISHED のみ → TCP レベルは正常
 │       │
 │       ├→ net/sockstat: メモリ使用確認
 │       │   └→ TCP mem が high に近い → カーネルの TCP バッファ逼迫
 │       │
 │       └→ net/snmp: エラーカウンタ確認
 │           ├→ RetransSegs 増加 → パケットロス。再送が発生
 │           └→ InErrs 増加 → 不正パケット受信
```

### 💿 ストレージの診断フロー

```
"ディスクが遅い?"
 │
 ├→ df: 使用率確認
 │   ├→ > 90% → 空き容量不足。ログローテーション確認
 │   └→ 余裕あり → 容量は問題なし。I/O 性能を確認
 │       │
 │       ├→ diskstats: read/write ops と await 確認
 │       │   ├→ await 高い → ディスクが応答遅い。HDD? RAID degraded?
 │       │   └→ ops 多い + await 正常 → I/O 量が多いだけ
 │       │
 │       ├→ mounts: ファイルシステム種別確認
 │       │   └→ NFS/CIFS → ネットワーク FS。ネットワークカテゴリも確認
 │       │
 │       └→ pressure: io_some_avg10 確認
 │           └→ > 25% → タスクが I/O 待ちで停滞。深刻なボトルネック
```

### 🔧 プロセスの診断フロー

```
"プロセスが多すぎ?"
 │
 ├→ processes: 状態分布確認
 │   ├→ Zombie (Z) > 10 → 親プロセスが wait() していない
 │   ├→ D-state > 5 → I/O 待ちプロセス多数。ストレージカテゴリへ
 │   └→ Running (R) > CPU数 → CPU 飽和。CPU カテゴリへ
 │       │
 │       ├→ stat: forks (processes_created) 確認
 │       │   └→ 急増 → fork bomb? or 短命プロセスの大量生成
 │       │
 │       └→ file-nr: FD 使用率確認
 │           └→ 使用率 > 80% → FD 枯渇リスク。リーク元を lsof で特定
```

🎋 千石: 「フローチャートの品質は良い。各ステップに **どのソースの何を見るか** が明示されている。Enter:jump の対象が明確だ。ただし注意: フローチャートの分岐を文字で表現するとインデントが深くなる。ターミナル幅 80 文字を考慮すると、ネストは 3 段階が限界。」

☕ ヤン: 「MVP スコープの最終確認:」

```
MVP (Phase 1):
  カテゴリ数: 5 定義、コンテンツは 3 カテゴリ (メモリ, CPU, ネットワーク)
  UI: View::CategoryGuide (独立画面)
  キーバインド: 'C' キー
  コンテンツ: 概要 + ストーリー + 診断フロー + よくある問題 (en/ja)
  ジャンプ: Enter でソースに遷移
  i18n: en/ja 両方

Phase 2:
  カテゴリコンテンツ追加: ストレージ, プロセス
  サイドバーグループ化
  ハードウェアカテゴリ追加
  diagnostics.rs との統合 (診断フローの自動実行)

Phase 3:
  コンテナ認識 (cgroups + meminfo の関係)
  カスタムカテゴリ (ユーザー定義グルーピング)
```

😰 僕: 「...コンテンツの実装場所は？ i18n.rs に全部入れると長くなりますが...」

☕ ヤン: 「Session 009 の結論を踏襲する。最初は `src/education.rs` に独立ファイルとして作る。i18n.rs はフィールド翻訳で成長するから、カテゴリ教育まで入れると 1000 行を超える。教育コンテンツは性質が違う — UI ラベルでもフィールド翻訳でもない。別ファイルが妥当。」

🎋 千石: 「同意する。`education.rs` の責務:」

```
src/education.rs:
  - Category enum 定義
  - CategoryEducation 構造体 (概要, ストーリー, 診断フロー, よくある問題)
  - 各カテゴリのコンテンツ (en/ja)
  - カテゴリとソースのマッピング
  - 診断フローのデータ構造
  - ジャンプ先のメタデータ
```

→ **Spec 提案: `src/education.rs` を新規作成。i18n.rs とは独立。カテゴリ教育コンテンツの定義と取得を担当。**

---

## Gap Summary

| ID | Gap | Severity | Phase | 関連 Session |
|---|---|---|---|---|
| G-CAT-1 | ソース間の関係性を教育するカテゴリ単位のコンテンツが存在しない | High | 1 | 新規 |
| G-CAT-2 | カテゴリ教育コンテンツは HelpMode::Expanded (8行) に収まらない。別の表示メカニズムが必要 | High | 1 | Session 009 |
| G-CAT-3 | カテゴリガイドからソース/フィールドへのジャンプ機能 (教育と実データの接続) | Medium | 1 | 新規 |
| G-CAT-4 | サイドバーのカテゴリグループ化 (ナビゲーション改善) | Medium | 2 | 新規 |
| G-CAT-5 | diagnostics.rs の自動診断とカテゴリ診断フローの統合 | Medium | 2 | Session 009 |
| G-CAT-6 | ストレージ・プロセスカテゴリのコンテンツ作成 | Medium | 2 | 新規 |
| G-CAT-7 | ハードウェアカテゴリ (thermal + cpuinfo freq) の成立判断 | Low | 2 | Session 010 |
| G-CAT-8 | コンテナ環境での cgroups + meminfo の関係性教育 | Low | 3 | 新規 |
| G-CAT-9 | カテゴリ教育コンテンツの品質保証 (正確性レビュー基準) | Low | 2 | Session 009 |

---

## Concrete Spec

### S1: Category enum とデータ構造

```rust
// src/education.rs

use crate::i18n::Locale;

/// カテゴリ定義
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    Memory,     // 💾 メモリ
    Cpu,        // ⚡ CPU/負荷
    Network,    // 🌐 ネットワーク
    Storage,    // 💿 ストレージ
    Process,    // 🔧 プロセス
}

impl Category {
    pub const ALL: &[Category] = &[
        Category::Memory,
        Category::Cpu,
        Category::Network,
        Category::Storage,
        Category::Process,
    ];

    pub fn icon(self) -> &'static str {
        match self {
            Category::Memory => "💾",
            Category::Cpu => "⚡",
            Category::Network => "🌐",
            Category::Storage => "💿",
            Category::Process => "🔧",
        }
    }

    pub fn label(self, locale: Locale) -> &'static str {
        match (self, locale) {
            (Category::Memory, Locale::En) => "Memory",
            (Category::Memory, Locale::Ja) => "メモリ",
            (Category::Cpu, Locale::En) => "CPU / Load",
            (Category::Cpu, Locale::Ja) => "CPU / 負荷",
            (Category::Network, Locale::En) => "Network",
            (Category::Network, Locale::Ja) => "ネットワーク",
            (Category::Storage, Locale::En) => "Storage",
            (Category::Storage, Locale::Ja) => "ストレージ",
            (Category::Process, Locale::En) => "Processes",
            (Category::Process, Locale::Ja) => "プロセス",
        }
    }

    /// このカテゴリに属するソース名のリスト
    pub fn sources(self) -> &'static [&'static str] {
        match self {
            Category::Memory => &[
                "meminfo", "vmstat", "buddyinfo", "swaps",
                "pressure", "zoneinfo", "slabinfo", "pagetypeinfo",
            ],
            Category::Cpu => &[
                "stat", "loadavg", "cpuinfo", "pressure",
                "schedstat", "softirqs", "interrupts",
            ],
            Category::Network => &[
                "net/dev", "net/tcp", "net/udp", "net/unix", "net/arp",
                "net/route", "net/sockstat", "net/snmp", "net/netstat", "net/wireless",
            ],
            Category::Storage => &[
                "diskstats", "df", "mounts", "partitions", "pressure", "locks",
            ],
            Category::Process => &[
                "processes", "file-nr", "stat",
            ],
        }
    }
}

/// ソース名からカテゴリを逆引き (複数カテゴリに属する場合あり: pressure, stat)
pub fn categories_for_source(source: &str) -> Vec<Category> {
    Category::ALL
        .iter()
        .filter(|cat| cat.sources().contains(&source))
        .copied()
        .collect()
}
```

### S2: カテゴリ教育コンテンツの構造

```rust
/// 診断フローの 1 ステップ
pub struct FlowStep {
    pub text: &'static str,
    pub jump_source: Option<&'static str>,   // ジャンプ先ソース名
    pub jump_field: Option<&'static str>,    // ジャンプ先フィールド名
    pub children: &'static [FlowStep],       // 分岐 (最大 3 段階)
}

/// よくある問題パターン
pub struct CommonIssue {
    pub symptom: &'static str,      // "MemFree 低い + MemAvailable 高い"
    pub diagnosis: &'static str,    // "正常。Linux のキャッシュ動作"
    pub sources: &'static [&'static str],  // 確認すべきソース
}

/// カテゴリ教育コンテンツ
pub struct CategoryEducation {
    pub title: &'static str,          // "RAM はどこに消えた?"
    pub overview: &'static [&'static str],   // 概要 (2-3 行)
    pub story: &'static [StoryStep],  // ストーリー (5-10 ステップ)
    pub flow: &'static [FlowStep],    // 診断フロー
    pub issues: &'static [CommonIssue], // よくある問題
}

/// ストーリーの 1 ステップ (ジャンプ可能)
pub struct StoryStep {
    pub text: &'static str,
    pub source: &'static str,
    pub field: Option<&'static str>,
}

/// カテゴリ教育コンテンツの取得
pub fn category_education(category: Category, locale: Locale) -> Option<&'static CategoryEducation> {
    match (category, locale) {
        (Category::Memory, Locale::En) => Some(&MEMORY_EN),
        (Category::Memory, Locale::Ja) => Some(&MEMORY_JA),
        (Category::Cpu, Locale::En) => Some(&CPU_EN),
        (Category::Cpu, Locale::Ja) => Some(&CPU_JA),
        (Category::Network, Locale::En) => Some(&NETWORK_EN),
        (Category::Network, Locale::Ja) => Some(&NETWORK_JA),
        // Phase 2: Storage, Process
        _ => None,
    }
}
```

### S3: View::CategoryGuide の App state 変更

```rust
// app.rs

pub enum View {
    Dashboard,
    Welcome,
    Overview,
    Detail,
    Diff,
    TableView,
    Graph,
    Diagnostics,
    CategoryGuide,  // 新規追加
}

pub struct App {
    // ... 既存フィールド ...
    pub selected_category: usize,       // CategoryGuide でのカテゴリ選択
    pub category_scroll: usize,         // コンテンツのスクロール位置
    pub category_cursor: usize,         // ストーリー/フロー内のカーソル位置 (Enter:jump 用)
}

// キーバインド:
// 'C' or 'c' → View::CategoryGuide に遷移 (どのビューからでも)
// ←→         → カテゴリ切替 (selected_category)
// ↑↓         → コンテンツスクロール
// Enter      → カーソル位置のソースにジャンプ (Overview に遷移 + ソース選択)
// 'C' or Esc → 前のビューに戻る
```

### S4: サイドバーグループ化の実装 (Phase 2)

```rust
// app.rs に追加 (Phase 2)

pub struct SidebarState {
    pub grouped: bool,                           // グループ表示モード
    pub collapsed: std::collections::HashSet<Category>,  // 折りたたまれたカテゴリ
}

// サイドバーの描画ロジック (render.rs):
// grouped = false → 現在と同じフラットリスト
// grouped = true  →
//   💾 メモリ (8)           ← カテゴリ行。Enter で展開/折りたたみ
//     meminfo               ← 展開時のみ表示
//     vmstat
//     ...
//   ⚡ CPU/負荷 (7)
//     stat
//     ...
//   ── その他 (7) ──        ← カテゴリ未所属ソース
//     version
//     uptime
//     ...

// キーバインド:
// 'G' → grouped トグル
// Enter (カテゴリ行) → 折りたたみトグル
```

### S5: メモリカテゴリのコンテンツ例 (en/ja)

```rust
static MEMORY_EN: CategoryEducation = CategoryEducation {
    title: "Where did all the RAM go?",
    overview: &[
        "In Linux, 'free' memory is more than it appears.",
        "Linux uses free RAM for disk cache — this is normal and efficient.",
        "MemAvailable (not MemFree) is the true measure of available memory.",
    ],
    story: &[
        StoryStep { text: "Check MemAvailable — the true 'free' memory", source: "meminfo", field: Some("MemAvailable") },
        StoryStep { text: "Check Cached + Buffers — large values are normal (disk cache)", source: "meminfo", field: Some("Cached") },
        StoryStep { text: "Check SwapUsed — increasing means memory pressure", source: "swaps", field: None },
        StoryStep { text: "Check pgmajfault — increasing means thrashing", source: "vmstat", field: Some("pgmajfault") },
        StoryStep { text: "Check memory_some_avg10 — >0 means tasks stalled on memory", source: "pressure", field: Some("memory_some_avg10") },
        StoryStep { text: "Check buddyinfo — no free high-order blocks means fragmentation", source: "buddyinfo", field: None },
    ],
    flow: &[/* see diagnostic flow above */],
    issues: &[
        CommonIssue {
            symptom: "MemFree low + MemAvailable high",
            diagnosis: "Normal. Linux is using RAM for cache. No action needed.",
            sources: &["meminfo"],
        },
        CommonIssue {
            symptom: "SwapUsed increasing + pgmajfault increasing",
            diagnosis: "Thrashing. Memory severely insufficient. Identify top RSS process.",
            sources: &["swaps", "vmstat", "processes"],
        },
        CommonIssue {
            symptom: "memory_full_avg10 > 0",
            diagnosis: "All tasks stalled on memory. OOM imminent. Act immediately.",
            sources: &["pressure", "meminfo"],
        },
    ],
};

static MEMORY_JA: CategoryEducation = CategoryEducation {
    title: "RAM はどこに消えた?",
    overview: &[
        "Linux ではメモリの「空き」は見かけより多い。",
        "Linux は空き RAM をディスクキャッシュに使う — これは正常で効率的。",
        "MemAvailable (MemFree ではない) が本当の空きメモリの指標。",
    ],
    story: &[
        StoryStep { text: "MemAvailable を確認 — 本当の「空き」メモリ", source: "meminfo", field: Some("MemAvailable") },
        StoryStep { text: "Cached + Buffers を確認 — 大きくても正常 (ディスクキャッシュ)", source: "meminfo", field: Some("Cached") },
        StoryStep { text: "SwapUsed を確認 — 増加中ならメモリ圧力の証拠", source: "swaps", field: None },
        StoryStep { text: "pgmajfault を確認 — 増加中ならスラッシング", source: "vmstat", field: Some("pgmajfault") },
        StoryStep { text: "memory_some_avg10 を確認 — 0 より大きければタスクがメモリ待ちで停滞", source: "pressure", field: Some("memory_some_avg10") },
        StoryStep { text: "buddyinfo を確認 — 高オーダーの空きが 0 ならメモリ断片化", source: "buddyinfo", field: None },
    ],
    flow: &[/* 上記の診断フローと同じ */],
    issues: &[
        CommonIssue {
            symptom: "MemFree 低い + MemAvailable 高い",
            diagnosis: "正常。Linux がキャッシュに RAM を使っている。対応不要。",
            sources: &["meminfo"],
        },
        CommonIssue {
            symptom: "SwapUsed 増加 + pgmajfault 増加",
            diagnosis: "スラッシング。メモリが深刻に不足。RSS 上位プロセスを特定せよ。",
            sources: &["swaps", "vmstat", "processes"],
        },
        CommonIssue {
            symptom: "memory_full_avg10 > 0",
            diagnosis: "全タスクがメモリ待ちで停滞。OOM 直前。即座に対応。",
            sources: &["pressure", "meminfo"],
        },
    ],
};
```

### S6: 診断フローのデータ構造

```rust
/// 診断フローの表示用ノード (ツリー構造)
pub struct FlowNode {
    pub question: &'static str,            // "MemAvailable < 10%?"
    pub source: Option<&'static str>,       // ジャンプ先 "meminfo"
    pub field: Option<&'static str>,        // ジャンプ先 "MemAvailable"
    pub yes_branch: Option<&'static [FlowNode]>,  // Yes の場合の次ステップ
    pub no_branch: Option<&'static [FlowNode]>,   // No の場合の次ステップ
    pub conclusion: Option<&'static str>,   // 最終結論 (leaf node)
}

// 描画時にツリーを再帰的にレンダリング:
//   メモリが足りない?
//   └→ MemAvailable < 10%?            [meminfo:MemAvailable]
//      ├─ Yes → Cached 大きい?        [meminfo:Cached]
//      │  └─ Yes → 回収可能
//      │  └─ No → SwapUsed 増加中?    [swaps]
//      └─ No → 問題なし (MemFree が少ないだけ)

// ターミナル幅を考慮し、ネストは最大 3 段階
// 各行の [source:field] 部分は別色 (Cyan) で表示し、Enter でジャンプ可能
```

### S7: ファイル構造の提案

```
Phase 1 (MVP):
  src/education.rs              ← 新規作成 (~400 行)
    - Category enum
    - CategoryEducation, StoryStep, FlowNode, CommonIssue 構造体
    - メモリ/CPU/ネットワークの en/ja コンテンツ
    - category_education(), categories_for_source()

  src/ui/app.rs                 ← View::CategoryGuide 追加、selected_category 等の state 追加
  src/ui/render.rs              ← draw_category_guide() 関数追加
  src/main.rs                   ← 'C' キーバインド追加

Phase 2:
  src/education.rs              ← ストレージ/プロセスコンテンツ追加 (~200 行増)
  src/ui/render.rs              ← サイドバーグループ化描画
  src/ui/app.rs                 ← SidebarState 追加

Phase 3:
  src/education/mod.rs           ← 分割 (education.rs が 800 行超の場合)
  src/education/categories.rs    ← Category enum, sources mapping
  src/education/content.rs       ← CategoryEducation コンテンツ本体
  src/education/flow.rs          ← FlowNode, 診断フロー定義
```

---

## Next Actions

- [ ] Phase 1-A: `src/education.rs` 新規作成 — `Category` enum、`CategoryEducation` 構造体、ソースマッピング
- [ ] Phase 1-B: メモリカテゴリのコンテンツ作成 (en/ja) — 概要、ストーリー、診断フロー、よくある問題
- [ ] Phase 1-C: CPU/負荷カテゴリのコンテンツ作成 (en/ja)
- [ ] Phase 1-D: ネットワークカテゴリのコンテンツ作成 (en/ja)
- [ ] Phase 1-E: `View::CategoryGuide` 追加 — App state、キーバインド ('C')、画面遷移
- [ ] Phase 1-F: `draw_category_guide()` 実装 — カテゴリタブ、コンテンツ描画、スクロール
- [ ] Phase 1-G: Enter:jump 実装 — カテゴリガイドから Overview + ソース選択への遷移
- [ ] Phase 2-A: ストレージ・プロセスカテゴリのコンテンツ追加
- [ ] Phase 2-B: サイドバーのカテゴリグループ化 ('G' キー)
- [ ] Phase 2-C: diagnostics.rs との統合 (カテゴリ単位の自動診断)
