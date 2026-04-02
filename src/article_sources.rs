use crate::article::{ArticleKind, ArticleLink, EducationArticle};

macro_rules! source_article {
    ($id:expr, $src:expr) => {
        EducationArticle {
            id: $id,
            kind: ArticleKind::Concept,
            title_en: "Source Guide",
            title_ja: "ソースガイド",
            body_en: $src,
            body_ja: $src,
            links: &[
                ArticleLink::Article {
                    label_en: "Cross-metric reading",
                    label_ja: "クロスメトリクスの読み方",
                    id: "concept.cross-metric-reading",
                },
                ArticleLink::Article {
                    label_en: "Operations methodology",
                    label_ja: "運用の進め方",
                    id: "concept.ops-methodology",
                },
            ],
        }
    };
}

pub const ARTICLES_SOURCES: &[EducationArticle] = &[
    source_article!(
        "sourceguide.buddyinfo",
        "Source guide for buddyinfo.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは buddyinfo の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.cgroups",
        "Source guide for cgroups.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは cgroups の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.cmdline",
        "Source guide for cmdline.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは cmdline の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.conntrack",
        "Source guide for conntrack.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは conntrack の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.consoles",
        "Source guide for consoles.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは consoles の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.cpuinfo",
        "Source guide for cpuinfo.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは cpuinfo の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.crypto",
        "Source guide for crypto.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは crypto の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.devices",
        "Source guide for devices.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは devices の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.df",
        "Source guide for df.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは df の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.diskstats",
        "Source guide for diskstats.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは diskstats の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.dma",
        "Source guide for dma.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは dma の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.dns",
        "Source guide for dns.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは dns の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.file-nr",
        "Source guide for file-nr.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは file-nr の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.filesystems",
        "Source guide for filesystems.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは filesystems の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.interrupts",
        "Source guide for interrupts.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは interrupts の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.iomem",
        "Source guide for iomem.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは iomem の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.ioports",
        "Source guide for ioports.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは ioports の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.loadavg",
        "Source guide for loadavg.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは loadavg の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.locks",
        "Source guide for locks.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは locks の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.meminfo",
        "Source guide for meminfo.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは meminfo の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.misc",
        "Source guide for misc.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは misc の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.modules",
        "Source guide for modules.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは modules の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.mounts",
        "Source guide for mounts.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは mounts の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.net/arp",
        "Source guide for net/arp.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは net/arp の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.net/dev",
        "Source guide for net/dev.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは net/dev の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.net/netstat",
        "Source guide for net/netstat.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは net/netstat の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.net/route",
        "Source guide for net/route.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは net/route の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.net/snmp",
        "Source guide for net/snmp.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは net/snmp の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.net/sockstat",
        "Source guide for net/sockstat.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは net/sockstat の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.net/tcp",
        "Source guide for net/tcp.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは net/tcp の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.net/udp",
        "Source guide for net/udp.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは net/udp の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.net/unix",
        "Source guide for net/unix.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは net/unix の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.net/wireless",
        "Source guide for net/wireless.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは net/wireless の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.partitions",
        "Source guide for partitions.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは partitions の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.pressure",
        "Source guide for pressure.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは pressure の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.processes",
        "Source guide for processes.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは processes の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.schedstat",
        "Source guide for schedstat.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは schedstat の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.softirqs",
        "Source guide for softirqs.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは softirqs の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.ss",
        "Source guide for ss.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは ss の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.stat",
        "Source guide for stat.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは stat の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.swaps",
        "Source guide for swaps.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは swaps の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.uptime",
        "Source guide for uptime.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは uptime の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.version",
        "Source guide for version.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは version の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.vmstat",
        "Source guide for vmstat.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは vmstat の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
    source_article!(
        "sourceguide.zoneinfo",
        "Source guide for zoneinfo.\n\nWhat to read\n- Start from trend and compare with related sources.\n- Use Diff/Graph to verify direction and persistence.\n\nHow to use\n- Treat this source as a structured evidence table, not a single value.\n- Combine with pressure, latency, and capacity metrics before action.\n\nこのガイドは zoneinfo の全体像を読むための基礎記事です。\nまず時系列と関連ソースを合わせて確認し、単一値で判断しないでください。"
    ),
];
