# DGE Session 012: プラグインアーキテクチャと5層ビジョン — syslenz があればコンピュータがわかるようになる

- **Date**: 2026-03-28
- **Theme**: syslenz を「Linux /proc パーサー」から「コンピュータ全体を可視化・教育するプラットフォーム」に進化させるためのプラグインアーキテクチャ設計と5層レイヤーモデルの策定
- **Parent Gaps**: G14 (プロダクトアイデンティティ), G15 (ソースディレクトリ構成), 新規 Gap
- **Characters**: ラインハルト (ビジョナリー) + ヤン (怠惰な簡潔主義者) + 千石 (品質の番人) + リヴァイ (実装の鬼) + 大和田 (ビジネス戦略家) + ハウス (診断の天才) + 今泉 (初心者の代弁者) + 僕
- **Input**: 43 /proc パーサー + 3 /sys パーサー (df, thermal, file-nr) 実装済み。diagnostics.rs にクロス診断エンジン。4 段階ヘルプ。i18n (en/ja)。Session 009 で教育コンテンツの方向性、Session 010 で非 /proc メトリクスの拡張を議論済み。次のステップとして、OS カーネル層の外 — ネットワーク深掘り、アプリケーションランタイム、仮想化/クラウド、ハードウェア詳細 — への拡張戦略が必要。

---

## 現状の整理

先輩 (ナレーション): syslenz の現在地を確認する。

**Layer 1: OS Kernel — 完了**

| カテゴリ | ソース数 | 主要パーサー |
|---------|---------|------------|
| /proc | 43 | meminfo, loadavg, cpuinfo, stat, net_dev, net_tcp, processes, vmstat, pressure... |
| /sys | 3 | df (statfs), thermal, file-nr |
| 診断 | 1 | diagnostics.rs (単体 + クロス診断) |
| 教育 | 1 | 4 段階ヘルプ (概要 / 詳細 / 診断 Tips / Deep Dive) |
| i18n | 2 言語 | en, ja (ソース説明 + 主要フィールド) |

**しかし、これは氷山の一角にすぎない。** 実際のシステム障害の原因は OS カーネルの外にあることが多い:

- JVM の Full GC が 3 秒止まっている → OS の loadavg だけ見ても「負荷が高い」としかわからない
- iptables の DROP ルールがパケットを落としている → /proc/net/tcp だけ見ても「接続できない」としかわからない
- Docker コンテナの cgroup メモリ制限に達している → ホストの meminfo は正常に見える
- ディスクの SMART 値が劣化している → diskstats の I/O 時間が増えても原因がわからない

問題の本質: **syslenz は OS カーネルの「窓」からシステムを覗いているが、窓の外にある原因を診断できない。**

---

## Scene 1: ビジョンの定義 — syslenz の5層レイヤーモデル

先輩: syslenz の将来像について議論する。どこまでカバーすべきか。

🦁 ラインハルト: 「諸君、聞け。syslenz のビジョンを再定義する。"syslenz があればコンピュータがわかるようになる" — これが我々の使命だ。現在の syslenz は Linux カーネルの窓から覗いた情報しか見せていない。だが本当にコンピュータを "わかる" ためには、ハードウェアからアプリケーションまで、全5層を可視化しなければならない。」

```
Layer 5: Hardware Detail
  SMART (disk health), PCI devices (lspci), USB (lsusb), BIOS/DMI (dmidecode)
  「物理的なハードウェアが何か、健康かどうか」

Layer 4: Virtualization / Cloud
  VM info (hypervisor), Cloud metadata (AWS/GCP), Container limits (cgroup v2)
  「仮想化層で何が隠されているか」

Layer 3: Application Runtime (PLUGIN)
  JVM (jstat/jcmd), Node.js, Python, Go, Database (MySQL/PG/Redis), Web server (nginx)
  「アプリケーションの中で何が起きているか」

Layer 2: Network Deep Dive
  iptables/nftables, conntrack, ip route, ip neighbor, ss -s, DNS, bridge/vlan
  「パケットの旅の全貌」

Layer 1: OS Kernel (CURRENT)
  /proc (43), /sys (df, thermal, file-nr), diagnostics, 4-level help, i18n
  「OS が見ているシステムの状態」
```

🦁 ラインハルト: 「syslenz は監視ツールではない。Linux の教科書だ。教科書にはハードウェアの章もネットワークの章もある。」

👤 今泉: 「あの... すみません。全部自分たちで作るんですか？ JVM のパーサーも、iptables のパーサーも、SMART のパーサーも？ 5層全部？ 何年かかります？」

☕ ヤン: 「いい質問。全部自分たちで作る必要はない。Layer 1 と Layer 2 は組み込み (built-in) でいい。OS の情報は安定していて、全環境で使える。だが Layer 3 は環境によって全然違う。JVM が入ってないサーバーに JVM パーサーを組み込んでも無駄だ。ここに **プラグインアーキテクチャ** が必要になる。紅茶ください。」

🏥 ハウス: 「プラグインが必要な理由をもう一つ。JVM の Full GC が 3 秒止まっているとき、OS 側の loadavg は上がる。だが loadavg だけ見ても "なぜ" 負荷が高いのかわからない。JVM の GC ログと OS の memory pressure を**同時に見て**初めてメモリ問題が診断できる。つまり Layer 1 と Layer 3 を同じ画面で見る必要がある。プラグインは syslenz のフレームワーク内に出力を返さなければ意味がない。」

💼 大和田: 「ビジネス的に言えば、プラグインエコシステムがあれば **コミュニティが育つ**。Java エンジニアが JVM プラグインを書き、DBA が PostgreSQL プラグインを書く。各ドメインの専門家が参加できる仕組みがあれば、我々が全部作らなくていい。これはオープンソースの勝ちパターンだ。」

→ **Gap G-PLG-1 発見: 5 層レイヤーモデルの定義。各層の責任範囲、実装方式 (built-in vs plugin)、優先順位の明文化が必要。**

---

## Scene 2: プラグインアーキテクチャの設計

先輩: プラグインの具体的なアーキテクチャを議論する。拡張性とシンプルさのバランスが鍵。

⚔️ リヴァイ: 「最初にはっきりさせておく。**5 分で書けるプラグインでなければ誰も書かない。** プラグインの SDK をインストールして、API ドキュメントを 30 ページ読んで、型定義をインポートして、コンパイルして... そんなのは死ぬ。」

☕ ヤン: 「4 つの選択肢を整理する。」

```
方式 A: Built-in Only (現在の方式)
  全パーサーを Rust で書いてコンパイル
  利点: 高速、型安全、依存なし
  欠点: 拡張するたびに本体をビルドし直す。JVM ユーザーは JVM パーサーのために Rust を学ぶ

方式 B: External Script Plugin (stdin/stdout JSON)
  プラグイン = 実行可能ファイル (シェルスクリプト, Python, Ruby, Go, 何でも)
  ~/.config/syslenz/plugins/ に置く
  syslenz が起動時にスキャンして実行、stdout の JSON を ProcEntry として取り込む
  利点: 言語フリー。5 分で書ける。シェルスクリプトでいい
  欠点: プロセス起動コスト。出力が壊れている可能性

方式 C: Dynamic Loading (.so / .dylib)
  Rust の dylib ABI でプラグインをロード
  利点: 高速、メモリ共有
  欠点: ABI 互換性地獄。unsafe 必須。クロスコンパイル問題。事実上 Rust 限定

方式 D: Hybrid (Built-in Core + External Plugin)
  Layer 1, 2 は built-in
  Layer 3, 4, 5 の一部は external plugin
  利点: コアの安定性 + 拡張の柔軟性
  欠点: 2 つのコードパスを保守
```

🦁 ラインハルト: 「方式 D が正解だ。OS カーネルの情報は全環境で必要だから built-in。アプリケーション固有の情報はプラグインで。」

🎋 千石: 「方式 C は論外。ABI 互換性を保証するのは不可能に近い。Rust のコンパイラバージョンが違うだけで動かなくなる。Neovim も Vim も Emacs もプラグインは外部プロセスか組み込みスクリプトだ。.so ロードを選んだプロジェクトで幸せになったものを見たことがない。」

⚔️ リヴァイ: 「方式 B の詳細を詰める。プロトコルを定義しろ。」

☕ ヤン: 「これがプラグインプロトコルだ。stdin/stdout JSON。以上。複雑にするな。」

### プラグインプロトコル v1

```
【ディスカバリ】
  syslenz 起動時に以下をスキャン:
    1. ~/.config/syslenz/plugins/
    2. /etc/syslenz/plugins/ (システムワイド)
    3. コマンドライン引数 --plugin-dir <path>

  実行可能ファイル (chmod +x) をプラグインとして認識
  ファイル名がプラグイン名 (例: jvm-stats → source="jvm-stats")

【実行プロトコル】
  syslenz がプラグインを exec する (子プロセス)
  引数なし (将来の拡張で --interval, --pid を渡す余地あり)
  プラグインは stdout に JSON を 1 行出力して exit 0
  stderr は syslenz がログに記録 (デバッグ用)
  exit code != 0 → エラーとして無視、stderr をログ

【JSON フォーマット】
  ProcEntry と同じ構造:
  {
    "source": "jvm-stats",
    "description": "JVM runtime statistics via jstat",
    "fields": [
      {
        "name": "heap_used",
        "value": {"Float": 1073741824.0},
        "description": "Current heap memory usage in bytes"
      },
      {
        "name": "gc_count",
        "value": {"Uint": 42},
        "description": "Total number of GC cycles"
      }
    ],
    "help": {
      "summary": "JVM heap and GC statistics",
      "detail": "Collected via jstat -gc targeting the first discovered JVM process",
      "diagnostic_tips": "If heap_used / heap_max > 0.85, the JVM may be under memory pressure",
      "deep_dive": "The JVM garbage collector reclaims unused objects..."
    }
  }

【Value 型】
  {"Uint": 123}
  {"Float": 3.14}
  {"Text": "some string"}
  {"Table": [["col1", "col2"], ["val1", "val2"]]}

【タイムアウト】
  デフォルト 5 秒。5 秒以内に exit しなければ SIGKILL
  設定ファイルでプラグインごとに変更可能

【リフレッシュ】
  syslenz のリフレッシュ間隔 (デフォルト 1 秒) ごとにプラグインを re-exec
  重いプラグインは独自の interval 設定可能 (例: jvm-stats は 5 秒おき)
```

⚔️ リヴァイ: 「これなら 5 分で書ける。シェルスクリプトで `jstat` の出力をパースして JSON にすればいい。Rust も Python も不要。」

🎋 千石: 「JSON の validate は必須。フィールドが欠けていたら、型が間違っていたら、無視してログに記録。壊れたプラグインが syslenz 全体を落としてはいけない。」

☕ ヤン: 「もちろん。`serde_json::from_str` で deserialize して、`Err` なら warn ログ。プラグインは壊れる前提で設計する。」

👤 今泉: 「あの... そもそもプラグインって何ですか？ 普通のユーザーが使うものですか？」

☕ ヤン: 「今泉、いい質問。プラグインには 2 種類のユーザーがいる。**書く人** と **使う人**。書く人は開発者だ — シェルスクリプトか Python で自分の環境のメトリクスを追加する。使う人は `syslenz --plugins` で有効化するだけ。プラグインが無効なら syslenz は今まで通り動く。」

→ **Gap G-PLG-2 発見: プラグインプロトコル v1 の仕様策定と実装。ディスカバリ、実行、JSON フォーマット、タイムアウト、エラーハンドリング。**

→ **Gap G-PLG-3 発見: プラグインローダーの実装 (`src/plugin/loader.rs`)。ディレクトリスキャン、exec、JSON パース、ProcEntry への変換、タイムアウト管理。**

---

## Scene 3: JVM プラグインの具体設計

先輩: 最初のプラグインとして JVM を題材に、具体的な実装と教育コンテンツを議論する。

🏥 ハウス: 「なぜ JVM が最初のプラグインか。3 つの理由がある。」

```
1. JVM は最も広く使われているランタイム — Java/Kotlin/Scala/Clojure
2. jstat, jcmd, jps は JDK に含まれている — 追加インストール不要
3. JVM のメモリ問題は OS メトリクスだけでは診断不能
   — Full GC で STW (Stop-The-World) が発生
   — OS の loadavg は上がるが原因が見えない
   — ヒープとネイティブメモリの区別ができない
```

🏥 ハウス: 「具体例を出そう。ある Spring Boot アプリが重い。OS 側で見える情報:」

```
OS Layer (syslenz 現在):
  loadavg: 4.2 (4 コア → 100% 超え)
  meminfo: MemAvailable 2GB / MemTotal 16GB (余裕あるように見える)
  cpu stat: user 85%, sys 5%, idle 10%
  processes: java PID=1234, RSS=8GB

→ 「Java プロセスが CPU を食っている」としか言えない。なぜ？

JVM Layer (プラグインで追加):
  heap_used: 7.8GB / heap_max: 8GB → 97.5% 使用
  gc_count: 342 (過去 1 分で Full GC 12 回)
  gc_time: 38 秒 (過去 1 分の GC 時間 → 63% が GC!)
  thread_count: 248 (正常は 50 前後)

→ 「ヒープが満杯で Full GC を連打している。GC に CPU 時間の 63% を消費。
   スレッドが異常増殖 — おそらくリクエストが GC 待ちで滞留。
   ヒープサイズの拡大か、メモリリークの調査が必要。」
```

🏥 ハウス: 「これが Layer 1 と Layer 3 を同時に見る価値だ。OS だけ、JVM だけ、どちらか片方では診断できない。」

### JVM プラグイン実装案

⚔️ リヴァイ: 「実装を見せる。シェルスクリプト 30 行だ。」

```bash
#!/bin/bash
# ~/.config/syslenz/plugins/jvm-stats
# JVM statistics plugin for syslenz
# Requires: JDK (jps, jstat)

set -euo pipefail

# Find first Java process
JAVA_PID=$(jps -q 2>/dev/null | head -1)
if [ -z "$JAVA_PID" ]; then
  echo '{"source":"jvm-stats","description":"No JVM process found","fields":[]}'
  exit 0
fi

# Get GC statistics
GC_RAW=$(jstat -gc "$JAVA_PID" 2>/dev/null | tail -1)
if [ -z "$GC_RAW" ]; then
  echo '{"source":"jvm-stats","description":"jstat failed","fields":[]}'
  exit 0
fi

# Parse jstat -gc output columns:
# S0C S1C S0U S1U EC EU OC OU MC MU CCSC CCSU YGC YGCT FGC FGCT CGC CGCT GCT
read -r S0C S1C S0U S1U EC EU OC OU MC MU CCSC CCSU YGC YGCT FGC FGCT CGC CGCT GCT <<< "$GC_RAW"

# Calculate heap usage
HEAP_USED=$(echo "$S0U + $S1U + $EU + $OU" | bc)
HEAP_MAX=$(echo "$S0C + $S1C + $EC + $OC" | bc)
THREAD_COUNT=$(ls /proc/"$JAVA_PID"/task 2>/dev/null | wc -l)

cat <<EOF
{
  "source": "jvm-stats",
  "description": "JVM runtime statistics for PID $JAVA_PID",
  "fields": [
    {"name": "pid", "value": {"Uint": $JAVA_PID}, "description": "JVM process ID"},
    {"name": "heap_used_kb", "value": {"Float": $HEAP_USED}, "description": "Current heap usage (KB)"},
    {"name": "heap_max_kb", "value": {"Float": $HEAP_MAX}, "description": "Maximum heap capacity (KB)"},
    {"name": "young_gc_count", "value": {"Uint": $YGC}, "description": "Young generation GC count"},
    {"name": "young_gc_time_sec", "value": {"Float": $YGCT}, "description": "Young GC cumulative time (seconds)"},
    {"name": "full_gc_count", "value": {"Uint": $FGC}, "description": "Full GC count"},
    {"name": "full_gc_time_sec", "value": {"Float": $FGCT}, "description": "Full GC cumulative time (seconds)"},
    {"name": "total_gc_time_sec", "value": {"Float": $GCT}, "description": "Total GC time (seconds)"},
    {"name": "metaspace_used_kb", "value": {"Float": $MU}, "description": "Metaspace usage (KB)"},
    {"name": "thread_count", "value": {"Uint": $THREAD_COUNT}, "description": "Number of JVM threads"}
  ],
  "help": {
    "summary": "JVM heap and GC statistics collected via jstat",
    "detail": "Shows heap memory usage, garbage collection frequency and duration, and thread count for the JVM process. Heap is divided into Young (Eden + Survivor) and Old generation.",
    "diagnostic_tips": "heap_used/heap_max > 85% = memory pressure. full_gc_count increasing rapidly = heap too small or memory leak. total_gc_time / uptime > 5% = GC overhead too high.",
    "deep_dive": "The JVM manages memory via generational garbage collection. New objects are allocated in Eden (Young generation). Surviving objects are promoted to Old generation. Young GC is fast (milliseconds), Full GC stops all application threads (Stop-The-World). Frequent Full GC indicates the Old generation is too small or objects are not being released."
  }
}
EOF
```

🎋 千石: 「待ってほしい。3 つの問題がある。」

```
問題 1: jstat の出力はJVM バージョンで変わる
  - JDK 8: CGC, CGCT カラムがない
  - JDK 11+: CGC, CGCT (Concurrent GC) が追加
  - JDK 17+: 一部カラム名が変更
  → 対策: jstat のヘッダー行をパースしてカラム名で取得。位置ベースは壊れる

問題 2: jps は同一ユーザーの JVM しか見えない
  - root で syslenz を動かさないと他ユーザーの JVM は見えない
  - → 対策: エラーではなく「見えた JVM だけ表示」で graceful に

問題 3: 複数 JVM プロセスの扱い
  - head -1 で最初の PID だけ取っている
  - → 対策: 全 PID をループして Table 形式で出力、または PID 指定オプション
```

🎋 千石: 「JVM のバージョンでテストしていない情報は出すな。間違った GC 統計は嘘より悪い。Java エンジニアに笑われるだけだ。」

☕ ヤン: 「MVP は JDK 11+ のみサポート。JDK 8 は TODO。プラグインの先頭でバージョンチェックして、非対応なら空の fields を返す。」

### JVM プラグインの教育コンテンツ

🦁 ラインハルト: 「プラグインにも教育コンテンツを付けるべきだ。GC とは何かを知らないユーザーもいる。」

```
教育コンテンツ (help.deep_dive の拡張):

Level 1 (Beginner): 「GC とは何か」
  "プログラムが使わなくなったメモリを自動的に回収する仕組みです。
   C 言語では手動で free() を呼びますが、Java では GC が自動で行います。
   便利ですが、GC が動いている間はアプリケーションが一時停止します。"

Level 2 (Intermediate): 「Young GC vs Full GC」
  "JVM のヒープは Young 世代と Old 世代に分かれています。
   Young GC: 新しいオブジェクトの回収。高速 (数ms〜数十ms)
   Full GC: ヒープ全体の回収。低速 (数百ms〜数秒)。Stop-The-World。
   full_gc_count が急増していたらヒープサイズの見直しが必要です。"

Level 3 (Advanced): 「ヒープサイジングの考え方」
  "heap_used / heap_max が常に 80% 以上なら -Xmx を増やす。
   ただし Full GC 後も使用率が高い場合はメモリリークの可能性。
   GC ログを有効にして (-Xlog:gc*) Full GC 前後のヒープ使用量を確認。"

Level 4 (Expert): 「GC アルゴリズムの選択」
  "G1GC (JDK 9+ デフォルト): バランス型。大きなヒープに適している。
   ZGC (JDK 15+): 超低レイテンシ。STW < 1ms。ヒープサイズに関係なく。
   Shenandoah: ZGC と同コンセプト。Red Hat 開発。
   ParallelGC: スループット重視。バッチ処理向き。"
```

→ **Gap G-PLG-4 発見: JVM プラグインの実装。jstat ベース、JDK 11+ 対応、教育コンテンツ付き。**

→ **Gap G-PLG-5 発見: プラグインの教育コンテンツフォーマット。help フィールドに 4 段階の教育コンテンツを含める仕様。**

---

## Scene 4: ネットワーク深掘りの設計

先輩: Layer 2 のネットワーク詳細情報について議論する。これは built-in で実装する。

🏥 ハウス: 「ネットワーク問題の診断で最も重要なのは "パケットの旅" を追えることだ。NIC に入ったパケットがアプリケーションに届くまでに何層を通るか、ほとんどのエンジニアは知らない。」

```
パケットの旅 (受信):
  NIC (物理層)
    → ドライバ → ring buffer → NAPI → netfilter (iptables/nftables)
      → conntrack (接続追跡)
        → routing decision (ip route)
          → bridge (Docker/K8s の場合)
            → iptables FORWARD チェーン
              → conntrack NAT
                → destination socket (ss で確認)
                  → application

パケットが "消える" 可能性があるポイント:
  1. iptables INPUT/FORWARD チェーンの DROP ルール
  2. conntrack テーブルの枯渇 (nf_conntrack_max)
  3. ルーティングの不一致 (デフォルトゲートウェイがない)
  4. ARP 解決失敗 (ip neighbor で STALE/FAILED)
  5. ソケットの listen backlog 溢れ
  6. アプリケーションの accept() 遅延
```

🏥 ハウス: 「パケットがどこで落ちているかを追える。iptables の DROP カウンタ + conntrack のタイムアウト + net/tcp の SYN_SENT。これを同時に見て初めてネットワーク問題を診断できる。」

### Network Deep Dive パーサー一覧

```
Built-in (Layer 2) として実装するパーサー:

1. iptables_rules
   ソース: iptables -L -n -v --line-numbers (要 root)
   フィールド: chain, rule_num, target, protocol, source, destination,
               packets, bytes
   教育: "iptables の 3 チェーン (INPUT/FORWARD/OUTPUT) と、
          パケットがどの順番でルールを評価されるか"

2. conntrack
   ソース: conntrack -L (要 root) または /proc/net/nf_conntrack
   フィールド: protocol, state, src, dst, sport, dport, timeout,
               packets_orig, packets_reply
   教育: "conntrack はステートフルファイアウォールの心臓部。
          TCP の ESTABLISHED, SYN_SENT 等の状態を追跡。
          NAT 変換テーブルとしても機能。"
   診断: conntrack エントリ数 / nf_conntrack_max > 80% → 枯渇危険

3. ip_route
   ソース: ip -j route (JSON 出力)
   フィールド: destination, gateway, device, protocol, scope, metric, flags
   教育: "ルーティングテーブルはパケットの地図。
          longest prefix match でどのルートを選ぶか決まる。
          default ルートがなければインターネットに出られない。"

4. ip_neighbor
   ソース: ip -j neighbor
   フィールド: ip, mac, device, state (REACHABLE/STALE/FAILED)
   教育: "ARP (IPv4) / NDP (IPv6) は IP アドレスを MAC アドレスに変換する。
          STALE は一定時間通信がない状態。FAILED は解決失敗。"

5. ss_summary
   ソース: ss -s
   フィールド: total, tcp_estab, tcp_closed, tcp_orphaned, tcp_timewait,
               udp, raw
   教育: "/proc/net/sockstat よりも詳細。
          orphaned = プロセスが close したがカーネルが保持中。
          timewait = 接続終了後の待機中 (2MSL)。"

6. dns_config
   ソース: /etc/resolv.conf + dig (実際の名前解決テスト)
   フィールド: nameservers, search_domains, resolve_test_ms
   教育: "DNS は名前解決の仕組み。resolv.conf の nameserver の順番で
          問い合わせが行われる。resolve_test_ms が高ければ DNS が遅い。"

7. bridge_config
   ソース: bridge -j link, bridge -j vlan
   フィールド: bridge_name, interfaces, vlan_ids, state
   教育: "ブリッジは仮想的な L2 スイッチ。Docker は docker0 ブリッジで
          コンテナ間通信を実現。K8s の CNI プラグインも多くがブリッジを使う。
          Docker/K8s のネットワークが魔法に見えなくなる。"
```

👤 今泉: 「すみません... iptables と nftables ってどう違うんですか？ 両方パースするんですか？」

☕ ヤン: 「nftables は iptables の後継だ。Debian 11+, Ubuntu 22.04+ はデフォルトで nftables バックエンド。ただし iptables コマンドは互換レイヤーで動く。まず iptables コマンドの出力をパースして、nftables ネイティブは Phase 2 でいい。」

🎋 千石: 「root 権限が必要なコマンドが多い。iptables, conntrack は root でないと実行できない。syslenz が一般ユーザーで動いている場合はどうする？」

☕ ヤン: 「取得できなければ空を返す。"Permission denied: run syslenz with sudo for network details" とヘルプに表示。Layer 1 の /proc 情報は一般ユーザーでも見える。Layer 2 の一部は root 限定。それでいい。グレースフルデグラデーション。」

→ **Gap G-PLG-6 発見: ネットワーク深掘りパーサー群の実装 (iptables, conntrack, ip route, ip neighbor, ss, DNS, bridge)。Built-in として `src/net/` に配置。**

→ **Gap G-PLG-7 発見: root 権限が必要なメトリクスのグレースフルデグラデーション設計。権限不足時の表示とヘルプ。**

---

## Scene 5: 教育コンテンツの全体設計 — "コンピュータがわかる" への道

先輩: syslenz を教育プラットフォームとして設計する。5 層すべてに教育コンテンツを体系的に配置する方法を議論する。

🦁 ラインハルト: 「もう一度言う。syslenz は監視ツールではない。**教育プラットフォーム**だ。htop は数値を見せる。Grafana はグラフを見せる。syslenz は "なぜその数値がそうなっているか" を教える。それが我々の差別化だ。」

🎋 千石: 「教育コンテンツの質が低ければ逆効果だ。JVM のヘルプが間違っていたら Java エンジニアに笑われる。ネットワークの説明が不正確だったらインフラエンジニアに使ってもらえない。**各レベルで嘘がないこと。Beginner 向けの簡略化でも技術的に正しいこと。**」

☕ ヤン: 「学習パス (Learning Path) という概念を導入しよう。単にフィールドにヘルプをつけるだけじゃなく、"どういう順番で学べば理解が深まるか" を設計する。」

### Learning Path 設計

```
Path 1: "メモリを理解する" (Beginner → Expert)
  Step 1 (Beginner): Dashboard の Memory セクション
    "メモリとは？ プログラムが動くための作業台です。
     MemTotal = 作業台の広さ。MemAvailable = 今使える面積。"
  Step 2 (Intermediate): meminfo の詳細フィールド
    "Cached は捨てられるメモリ。MemFree + Cached ≒ MemAvailable。
     Dirty はディスクに書き戻していないデータ。"
  Step 3 (Advanced): Pressure Stall Information
    "memory pressure some avg10 > 0 = 一部のプロセスがメモリ待ち。
     full avg10 > 0 = 全プロセスがメモリ待ち。即座に対処が必要。"
  Step 4 (Expert): JVM プラグイン + zoneinfo
    "JVM heap_used vs OS MemAvailable の関係。
     JVM のヒープは OS の Anonymous memory に含まれる。
     zoneinfo の high/low watermark と kswapd の関係。"

Path 2: "ネットワーク障害を追え" (Beginner → Expert)
  Step 1 (Beginner): Dashboard の Network I/O
    "bytes_recv = 受信バイト数。bytes_sent = 送信バイト数。
     グラフが平らなら通信なし。急に上がったら大量のデータ転送。"
  Step 2 (Intermediate): net/tcp + net/arp
    "ESTABLISHED = 通信中。CLOSE_WAIT = 相手が切断したが自分が close していない。
     SYN_SENT が増えている = 接続先が応答しない。"
  Step 3 (Advanced): iptables + conntrack + ip route
    "パケットの旅を追う。どのルールでどのパケットが DROP されたか。
     conntrack テーブルが一杯になると新しい接続ができなくなる。"
  Step 4 (Expert): bridge + Docker ネットワーク
    "Docker のネットワーク分離の仕組み。veth ペア + ブリッジ + iptables NAT。
     コンテナ間通信がどう実現されているか。"

Path 3: "なぜサーバーが重い？" (診断フロー、カテゴリ横断)
  Step 1: loadavg を確認
    "load/cpu_count > 1.0 → CPU バウンド or I/O バウンド"
  Step 2: CPU stat を確認
    "user% 高い → アプリの処理。iowait% 高い → ディスク待ち。
     sys% 高い → カーネルコール多い。"
  Step 3: iowait が高い場合 → diskstats + df
    "どのディスクが忙しいか。ディスクが満杯か。"
  Step 4: user が高い場合 → processes + JVM プラグイン
    "どのプロセスが CPU を使っているか。JVM なら GC を疑え。"
  Step 5: メモリ関連 → meminfo + pressure + swap
    "OOM に向かっているか。swap が使われ始めたか。"
```

💼 大和田: 「学習パスは syslenz の**キラーフィーチャー**になる。htop でも Grafana でも学習パスはない。"syslenz を使っていたら Linux がわかるようになった" — これが口コミを生む。」

🏥 ハウス: 「学習パスの各ステップは、実際のメトリクスと連動していなければ意味がない。教科書を読むのではなく、自分のシステムの実際の数値を見ながら学ぶ。MemAvailable が 500MB しかないとき、"これはあなたのシステムでは作業台が 16GB 中 500MB しか空いていないということです" と具体的に教える。」

🎋 千石: 「学習パスの各ステップで "嘘" がないか検証する仕組みが必要。Step 1 の Beginner 向け簡略化が、Step 3 の Advanced と矛盾していたらダメだ。"メモリは作業台" という比喩は Beginner には有効だが、Advanced で "Anonymous memory + Cached + Slab + ..." と正確に書いたとき、作業台の比喩と整合性があるか確認する。」

→ **Gap G-PLG-8 発見: 学習パスシステムの設計と実装。パス定義 (JSON/TOML)、ステップ遷移、メトリクス連動、UI 統合。**

→ **Gap G-PLG-9 発見: 教育コンテンツの技術的正確性検証プロセス。Beginner 簡略化と Advanced 正確性の整合性チェック。**

---

## Scene 6: MVP と実装ロードマップ

先輩: 全てを一度に実装することはできない。何を、どの順番で、どこまでやるか。

☕ ヤン: 「全部やろうとすると 2 年かかる。5 層ビジョンは正しいが、ロードマップを切らないと空中分解する。**プラグインプロトコルだけ作って、JVM プラグインを 1 個書いて、それで十分。** プロトコルが良ければコミュニティが残りを書いてくれる。紅茶ください。」

⚔️ リヴァイ: 「実装量を見積もる。」

```
Phase 1: プラグインプロトコル定義 + ローダー実装 (1-2 週間)
  src/plugin/mod.rs         — モジュール宣言                    ~20 行
  src/plugin/protocol.rs    — PluginEntry 構造体, JSON 型定義     ~80 行
  src/plugin/loader.rs      — ディレクトリスキャン + exec + parse  ~150 行
  src/plugin/timeout.rs     — タイムアウト管理 (5 秒 SIGKILL)     ~40 行
  src/main.rs               — --plugin-dir 引数追加               ~10 行
  src/config.rs             — plugins 設定追加                    ~20 行
  src/ui/render.rs          — プラグインソースの表示対応           ~30 行
                                                         合計: ~350 行

Phase 2: 組み込みネットワーク詳細 (2-3 週間)
  src/net/mod.rs            — モジュール宣言                     ~20 行
  src/net/ip_route.rs       — ip -j route パーサー               ~60 行
  src/net/ip_neighbor.rs    — ip -j neighbor パーサー            ~50 行
  src/net/ss_summary.rs     — ss -s パーサー                    ~60 行
  src/net/dns.rs            — /etc/resolv.conf パーサー          ~40 行
  src/net/conntrack.rs      — conntrack パーサー (root)          ~80 行
  src/net/iptables.rs       — iptables 出力パーサー (root)       ~100 行
  src/net/bridge.rs         — bridge link パーサー               ~50 行
  src/diagnostics.rs        — ネットワーク診断パターン追加         ~100 行
                                                         合計: ~560 行

Phase 3: JVM プラグイン (1 週間)
  plugins/jvm-stats          — シェルスクリプト                   ~50 行
  教育コンテンツ (help JSON)  — 4 段階                            ~200 行 (テキスト)
  テスト                     — jstat 出力モック + JSON 検証        ~40 行
                                                         合計: ~290 行

Phase 4: DB プラグイン (1-2 週間)
  plugins/mysql-stats        — MySQL SHOW STATUS パーサー         ~60 行
  plugins/pg-stats           — PostgreSQL pg_stat パーサー        ~60 行
  plugins/redis-stats        — Redis INFO パーサー                ~40 行
                                                         合計: ~160 行

Phase 5: 学習パス UI (2-3 週間)
  src/learn/mod.rs           — 学習パスエンジン                   ~100 行
  src/learn/paths.rs         — パス定義                          ~200 行
  src/ui/learn_view.rs       — 学習パス UI                       ~150 行
  learn/memory.toml          — メモリパス定義                     ~100 行 (テキスト)
  learn/network.toml         — ネットワークパス定義                ~100 行 (テキスト)
  learn/troubleshoot.toml    — 診断フローパス定義                  ~100 行 (テキスト)
                                                         合計: ~750 行
```

🦁 ラインハルト: 「Phase 1 が最も重要だ。プラグインプロトコルは syslenz の将来のアーキテクチャを決定する。ここを正しく設計すれば、Phase 3 以降はコミュニティが加速してくれる。」

🎋 千石: 「Phase 1 のプラグインローダーでエラーハンドリングを徹底しろ。壊れたプラグインが syslenz を落とすようでは話にならない。」

```
エラーケースの一覧:
  1. プラグインが実行できない (権限なし)           → warn ログ、スキップ
  2. プラグインがタイムアウト (5 秒)               → SIGKILL、warn ログ
  3. プラグインが exit code != 0                   → stderr をログ、スキップ
  4. stdout が空                                   → warn ログ、スキップ
  5. stdout が不正な JSON                          → パースエラーをログ、スキップ
  6. JSON のフィールドが欠けている                  → デフォルト値で補完、warn ログ
  7. source 名が既存の built-in と衝突             → "plugin:" prefix を付与
  8. プラグインが大量の stdout を出力 (> 1MB)       → 切り捨て、warn ログ
```

💼 大和田: 「Phase 1 を出したら、README に "Write your first plugin in 5 minutes" チュートリアルを書け。これがコミュニティの入口になる。」

⚔️ リヴァイ: 「チュートリアルより動くサンプルだ。`plugins/examples/` に 3 つ入れろ:」

```
plugins/examples/
  hello-world      — 最小プラグイン (3 行)。プロトコルの確認用
  system-uptime    — uptime コマンドの出力を ProcEntry に変換 (10 行)
  jvm-stats        — JVM 統計 (30 行)。本格的なプラグインの見本
```

```bash
#!/bin/bash
# plugins/examples/hello-world — The simplest possible syslenz plugin
echo '{"source":"hello","description":"Hello from plugin","fields":[{"name":"message","value":{"Text":"Hello, syslenz!"},"description":"A greeting"}]}'
```

☕ ヤン: 「3 行。これ以上シンプルにはできない。このプラグインが動けばプロトコルは正しい。」

→ **Gap G-PLG-10 発見: サンプルプラグインの作成 (hello-world, system-uptime, jvm-stats)。**

→ **Gap G-PLG-11 発見: プラグイン作成チュートリアルドキュメント。5 分で初めてのプラグインを書けるガイド。**

---

## Gap Summary

| Gap ID | タイトル | 重要度 | Phase | 推定工数 |
|--------|---------|--------|-------|---------|
| G-PLG-1 | 5 層レイヤーモデルの定義と文書化 | **Critical** | 0 | 小 (ドキュメント) |
| G-PLG-2 | プラグインプロトコル v1 仕様策定 | **Critical** | 1 | 小 (仕様書) |
| G-PLG-3 | プラグインローダー実装 (`src/plugin/`) | **Critical** | 1 | 中 (~350 行) |
| G-PLG-4 | JVM プラグイン実装 (jstat ベース) | Important | 3 | 小 (~50 行 + テキスト) |
| G-PLG-5 | プラグイン教育コンテンツフォーマット | Important | 1 | 小 (仕様追加) |
| G-PLG-6 | ネットワーク深掘りパーサー群 (7 パーサー) | **Critical** | 2 | 大 (~560 行) |
| G-PLG-7 | root 権限グレースフルデグラデーション | Important | 2 | 小 (~50 行) |
| G-PLG-8 | 学習パスシステム設計・実装 | Important | 5 | 大 (~750 行) |
| G-PLG-9 | 教育コンテンツ正確性検証プロセス | Medium | 5 | 中 (プロセス定義) |
| G-PLG-10 | サンプルプラグイン作成 (3 種) | Important | 1 | 小 (~50 行) |
| G-PLG-11 | プラグイン作成チュートリアル | Medium | 1 | 小 (ドキュメント) |

---

## Plugin Protocol Spec v1

### JSON Format

```json
{
  "source": "string (required) — unique name, used as key in Snapshot",
  "description": "string (required) — one-line summary",
  "fields": [
    {
      "name": "string (required) — field identifier",
      "value": "Value enum (required) — one of Uint/Float/Text/Table",
      "description": "string (required) — field explanation"
    }
  ],
  "help": {
    "summary": "string (optional) — Level 1: one-line overview",
    "detail": "string (optional) — Level 2: paragraph explanation",
    "diagnostic_tips": "string (optional) — Level 3: what to look for",
    "deep_dive": "string (optional) — Level 4: expert-level knowledge"
  }
}
```

### Value Types

| Type | JSON | Rust Equivalent | Example |
|------|------|-----------------|---------|
| Uint | `{"Uint": 123}` | `FieldValue::Uint(u64)` | `{"Uint": 1048576}` |
| Float | `{"Float": 3.14}` | `FieldValue::Float(f64)` | `{"Float": 0.85}` |
| Text | `{"Text": "..."}` | `FieldValue::Text(String)` | `{"Text": "G1GC"}` |
| Table | `{"Table": [[...]]}` | `FieldValue::Table(Vec<Vec<String>>)` | `{"Table": [["pid","heap"],["1234","8GB"]]}` |

### Directory Convention

```
~/.config/syslenz/plugins/      — user plugins (priority 1)
/etc/syslenz/plugins/            — system-wide plugins (priority 2)
--plugin-dir <path>              — CLI override (priority 0, highest)
```

### Lifecycle

```
1. Discovery: scan plugin directories for executable files
2. Execution: fork + exec each plugin as child process
3. Timeout: 5 seconds default, SIGKILL on timeout
4. Parse: read stdout, deserialize JSON to PluginEntry
5. Validate: check required fields, type-check values
6. Merge: convert PluginEntry → ProcEntry, insert into Snapshot
7. Display: show in source list with "plugin:" prefix
8. Repeat: re-execute on each refresh interval
```

### Error Handling

| Error | Action |
|-------|--------|
| Not executable | warn log, skip |
| Timeout (5s) | SIGKILL, warn log, skip |
| Non-zero exit | log stderr, skip |
| Empty stdout | warn log, skip |
| Invalid JSON | log parse error, skip |
| Missing fields | fill defaults, warn log |
| Source name conflict | prefix with `plugin:` |
| Stdout > 1MB | truncate, warn log |

---

## JVM Plugin Example

```bash
#!/bin/bash
# jvm-stats — JVM statistics plugin for syslenz
# Place in: ~/.config/syslenz/plugins/jvm-stats
# Requires: JDK 11+ (jps, jstat)
set -euo pipefail

JAVA_PID=$(jps -q 2>/dev/null | head -1)
if [ -z "$JAVA_PID" ]; then
  echo '{"source":"jvm-stats","description":"No JVM process found","fields":[]}'
  exit 0
fi

# Verify JDK version
JSTAT_OUT=$(jstat -gc "$JAVA_PID" 2>/dev/null | tail -1) || {
  echo '{"source":"jvm-stats","description":"jstat failed","fields":[]}'
  exit 0
}

read -r S0C S1C S0U S1U EC EU OC OU MC MU CCSC CCSU YGC YGCT FGC FGCT CGC CGCT GCT <<< "$JSTAT_OUT"

HEAP_USED=$(echo "$S0U + $S1U + $EU + $OU" | bc)
HEAP_MAX=$(echo "$S0C + $S1C + $EC + $OC" | bc)
THREAD_COUNT=$(ls /proc/"$JAVA_PID"/task 2>/dev/null | wc -l)

cat <<PLUGIN_EOF
{
  "source": "jvm-stats",
  "description": "JVM runtime statistics for PID $JAVA_PID",
  "fields": [
    {"name":"pid","value":{"Uint":$JAVA_PID},"description":"JVM process ID"},
    {"name":"heap_used_kb","value":{"Float":$HEAP_USED},"description":"Heap usage (KB)"},
    {"name":"heap_max_kb","value":{"Float":$HEAP_MAX},"description":"Heap capacity (KB)"},
    {"name":"young_gc_count","value":{"Uint":$YGC},"description":"Young GC count"},
    {"name":"young_gc_time_sec","value":{"Float":$YGCT},"description":"Young GC time (s)"},
    {"name":"full_gc_count","value":{"Uint":$FGC},"description":"Full GC count"},
    {"name":"full_gc_time_sec","value":{"Float":$FGCT},"description":"Full GC time (s)"},
    {"name":"total_gc_time_sec","value":{"Float":$GCT},"description":"Total GC time (s)"},
    {"name":"metaspace_used_kb","value":{"Float":$MU},"description":"Metaspace usage (KB)"},
    {"name":"thread_count","value":{"Uint":$THREAD_COUNT},"description":"JVM thread count"}
  ],
  "help": {
    "summary": "JVM heap and GC statistics collected via jstat",
    "detail": "Shows heap memory usage, garbage collection frequency and duration, and thread count. Heap is divided into Young (Eden + Survivor) and Old generation. Metaspace stores class metadata.",
    "diagnostic_tips": "heap_used/heap_max > 85% = memory pressure. full_gc_count increasing = heap too small or leak. gc_time/uptime > 5% = GC overhead too high. thread_count > 200 = possible thread leak.",
    "deep_dive": "The JVM uses generational GC. New objects go to Eden (Young). Survivors promote to Old. Young GC is fast (ms), Full GC stops the world (seconds). G1GC (JDK 9+ default) divides heap into regions. ZGC (JDK 15+) achieves sub-ms pauses regardless of heap size."
  }
}
PLUGIN_EOF
```

---

## Network Deep-Dive Parser List

| パーサー | ソース | 必要権限 | 出力フィールド | Phase |
|---------|--------|---------|--------------|-------|
| `ip_route` | `ip -j route` | user | destination, gateway, device, metric, flags | 2 |
| `ip_neighbor` | `ip -j neighbor` | user | ip, mac, device, state | 2 |
| `ss_summary` | `ss -s` | user | total, tcp_estab, tcp_closed, tcp_orphaned, tcp_timewait | 2 |
| `dns_config` | `/etc/resolv.conf` + `dig` | user | nameservers, search_domains, resolve_time_ms | 2 |
| `conntrack` | `conntrack -L` or `/proc/net/nf_conntrack` | **root** | protocol, state, src, dst, packets, timeout | 2 |
| `iptables_rules` | `iptables -L -n -v` | **root** | chain, target, protocol, src, dst, packets, bytes | 2 |
| `bridge_config` | `bridge -j link` | user | bridge, interfaces, vlan_ids, state | 2 |

---

## Learning Path Structure

```
learn/
  memory.toml           — "メモリを理解する" (4 steps: Beginner → Expert)
  network.toml          — "ネットワーク障害を追え" (4 steps)
  troubleshoot.toml     — "なぜサーバーが重い？" (5 steps, cross-category)
  jvm.toml              — "JVM の GC を理解する" (4 steps, plugin-dependent)
  containers.toml       — "コンテナのリソース分離" (4 steps, future)

Each TOML file:
  [[step]]
  level = "beginner"        # beginner | intermediate | advanced | expert
  title = "メモリとは？"
  sources = ["meminfo"]     # which syslenz sources are relevant
  content = """
  メモリとは、プログラムが動くための作業台です。
  MemTotal はあなたのシステムの作業台の広さ — {meminfo.MemTotal} です。
  MemAvailable は今使える面積 — {meminfo.MemAvailable} です。
  """
  # {source.field} placeholders are replaced with actual live values
```

---

## File Structure Proposal

```
src/
  plugin/
    mod.rs              — pub mod protocol, loader, timeout;
    protocol.rs         — PluginEntry, PluginField, PluginHelp structs
                          + serde Deserialize
                          + PluginEntry → ProcEntry conversion
    loader.rs           — scan_plugin_dirs() → Vec<PathBuf>
                          execute_plugin(path) → Result<PluginEntry>
                          load_all_plugins(dirs) → Vec<ProcEntry>
    timeout.rs          — run_with_timeout(cmd, duration) → Result<String>
  net/                  — Layer 2: Network Deep Dive (built-in)
    mod.rs              — pub mod ip_route, ip_neighbor, ss_summary, ...
    ip_route.rs
    ip_neighbor.rs
    ss_summary.rs
    dns.rs
    conntrack.rs
    iptables.rs
    bridge.rs
  learn/                — Learning Path engine
    mod.rs              — PathEngine, Step, load_paths()
    paths.rs            — built-in path definitions (or TOML loader)
  proc/                 — (existing, unchanged)
    mod.rs
    meminfo.rs
    ... (43 parsers)
  sys/                  — (existing, unchanged)
    mod.rs
    df.rs
    thermal.rs
    file_nr.rs
  ui/
    render.rs           — updated: plugin source display
    learn_view.rs       — learning path UI (Phase 5)
    ...
  diagnostics.rs        — updated: network cross-diagnostics
  config.rs             — updated: plugin_dirs, plugin_timeout settings
  main.rs               — updated: --plugin-dir arg, plugin loading

plugins/                — Plugin directory (in repo, for distribution)
  examples/
    hello-world         — 3-line minimal plugin
    system-uptime       — 10-line uptime plugin
    jvm-stats           — 30-line JVM statistics plugin

learn/                  — Learning path definitions
  memory.toml
  network.toml
  troubleshoot.toml
```

---

## Key Decisions

| 決定事項 | 選択 | 理由 |
|---------|------|------|
| Layer 1-2 の実装方式 | Built-in (Rust) | OS 情報は安定。全環境で必要。パフォーマンス重視 |
| Layer 3+ の拡張方式 | External script plugin | 言語フリー。5 分で書ける。環境依存 |
| プラグイン IPC | stdin/stdout JSON | 最もシンプル。デバッグ容易。Unix 哲学 |
| .so dynamic loading | 却下 | ABI 地獄。unsafe 必須。メリットがコストに見合わない |
| プラグインの help | 4 段階 (summary/detail/tips/deep_dive) | Session 009 の教育コンテンツ設計と統一 |
| JDK サポート範囲 | JDK 11+ (MVP) | JDK 8 は出力形式が異なる。TODO |
| root 必要メトリクス | グレースフルデグラデーション | 取得できなければ空 + ヘルプ表示。落とさない |
| 学習パスのデータ形式 | TOML (ライブ値テンプレート) | 人間が読みやすい。コンパイル不要 |
