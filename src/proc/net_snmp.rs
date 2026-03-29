use super::{Field, FieldValue, ProcEntry};
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/proc/net/snmp")?;
    parse_content(&content)
}

pub fn parse_content(content: &str) -> anyhow::Result<ProcEntry> {
    let mut fields = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    // /proc/net/snmp has pairs of lines: header then values
    let mut i = 0;
    while i + 1 < lines.len() {
        let header_parts: Vec<&str> = lines[i].split_whitespace().collect();
        let value_parts: Vec<&str> = lines[i + 1].split_whitespace().collect();

        if header_parts.len() == value_parts.len() && header_parts.len() >= 2 {
            let category = header_parts[0].trim_end_matches(':');
            for j in 1..header_parts.len() {
                let val: i64 = value_parts[j].parse().unwrap_or(0);
                let field_name = format!("{}_{}", category, header_parts[j]);
                let description = describe_snmp_field(category, header_parts[j]);
                fields.push(Field {
                    name: field_name,
                    value: FieldValue::Integer(val),
                    unit: None,
                    description,
                });
            }
        }
        i += 2;
    }

    Ok(ProcEntry {
        source: "/proc/net/snmp".into(),
        fields,
    })
}

fn describe_snmp_field(category: &str, field: &str) -> String {
    match (category, field) {
        // --- Ip ---
        ("Ip", "Forwarding") => "IP forwarding enabled (1=yes gateway, 2=no host-only)".into(),
        ("Ip", "DefaultTTL") => "Default TTL value for outgoing IP packets".into(),
        ("Ip", "InReceives") => "Total IP packets received".into(),
        ("Ip", "InHdrErrors") => "IP packets discarded due to header errors".into(),
        ("Ip", "InAddrErrors") => "IP packets discarded due to invalid destination address".into(),
        ("Ip", "ForwDatagrams") => "IP packets forwarded to another destination".into(),
        ("Ip", "InUnknownProtos") => "IP packets with unknown or unsupported protocol".into(),
        ("Ip", "InDiscards") => "Inbound IP packets discarded (not due to errors)".into(),
        ("Ip", "InDelivers") => "IP packets successfully delivered to upper protocols".into(),
        ("Ip", "OutRequests") => "IP packets supplied by local protocols for sending".into(),
        ("Ip", "OutDiscards") => "Outbound IP packets discarded (not due to errors)".into(),
        ("Ip", "OutNoRoutes") => "IP packets discarded because no route was found".into(),
        ("Ip", "ReasmTimeout") => "IP reassembly timeouts (fragments dropped)".into(),
        ("Ip", "ReasmReqds") => "IP fragments received that needed reassembly".into(),
        ("Ip", "ReasmOKs") => "IP datagrams successfully reassembled".into(),
        ("Ip", "ReasmFails") => "IP reassembly failures".into(),
        ("Ip", "FragOKs") => "IP datagrams successfully fragmented".into(),
        ("Ip", "FragFails") => "IP datagrams that could not be fragmented (DF set)".into(),
        ("Ip", "FragCreates") => "IP fragments created from fragmentation".into(),

        // --- Icmp ---
        ("Icmp", "InMsgs") => "Total ICMP messages received".into(),
        ("Icmp", "InErrors") => "ICMP messages received with errors".into(),
        ("Icmp", "InCsumErrors") => "ICMP messages with checksum errors".into(),
        ("Icmp", "InDestUnreachs") => "ICMP Destination Unreachable messages received".into(),
        ("Icmp", "InTimeExcds") => "ICMP Time Exceeded messages received".into(),
        ("Icmp", "InParmProbs") => "ICMP Parameter Problem messages received".into(),
        ("Icmp", "InSrcQuenchs") => "ICMP Source Quench messages received".into(),
        ("Icmp", "InRedirects") => "ICMP Redirect messages received".into(),
        ("Icmp", "InEchos") => "ICMP Echo Request (ping) messages received".into(),
        ("Icmp", "InEchoReps") => "ICMP Echo Reply messages received".into(),
        ("Icmp", "InTimestamps") => "ICMP Timestamp Request messages received".into(),
        ("Icmp", "InTimestampReps") => "ICMP Timestamp Reply messages received".into(),
        ("Icmp", "InAddrMasks") => "ICMP Address Mask Request messages received".into(),
        ("Icmp", "InAddrMaskReps") => "ICMP Address Mask Reply messages received".into(),
        ("Icmp", "OutMsgs") => "Total ICMP messages sent".into(),
        ("Icmp", "OutErrors") => "ICMP messages not sent due to errors".into(),
        ("Icmp", "OutDestUnreachs") => "ICMP Destination Unreachable messages sent".into(),
        ("Icmp", "OutTimeExcds") => "ICMP Time Exceeded messages sent".into(),
        ("Icmp", "OutParmProbs") => "ICMP Parameter Problem messages sent".into(),
        ("Icmp", "OutSrcQuenchs") => "ICMP Source Quench messages sent".into(),
        ("Icmp", "OutRedirects") => "ICMP Redirect messages sent".into(),
        ("Icmp", "OutEchos") => "ICMP Echo Request (ping) messages sent".into(),
        ("Icmp", "OutEchoReps") => "ICMP Echo Reply messages sent".into(),
        ("Icmp", "OutTimestamps") => "ICMP Timestamp Request messages sent".into(),
        ("Icmp", "OutTimestampReps") => "ICMP Timestamp Reply messages sent".into(),
        ("Icmp", "OutAddrMasks") => "ICMP Address Mask Request messages sent".into(),
        ("Icmp", "OutAddrMaskReps") => "ICMP Address Mask Reply messages sent".into(),

        // --- IcmpMsg (per-type counters) ---
        ("IcmpMsg", f) if f.starts_with("InType") => format!("ICMP messages received of type {}", &f[6..]),
        ("IcmpMsg", f) if f.starts_with("OutType") => format!("ICMP messages sent of type {}", &f[7..]),

        // --- Tcp ---
        ("Tcp", "RtoAlgorithm") => "TCP retransmission timeout algorithm (1=other, 2=constant, 3=MIL-STD-1778, 4=Van Jacobson)".into(),
        ("Tcp", "RtoMin") => "Minimum TCP retransmission timeout (ms)".into(),
        ("Tcp", "RtoMax") => "Maximum TCP retransmission timeout (ms)".into(),
        ("Tcp", "MaxConn") => "Maximum number of TCP connections allowed (-1 = dynamic)".into(),
        ("Tcp", "ActiveOpens") => "TCP connections opened actively (connect)".into(),
        ("Tcp", "PassiveOpens") => "TCP connections opened passively (listen/accept)".into(),
        ("Tcp", "AttemptFails") => "TCP connection attempts that failed".into(),
        ("Tcp", "EstabResets") => "Established TCP connections reset".into(),
        ("Tcp", "CurrEstab") => "TCP connections currently in ESTABLISHED or CLOSE-WAIT state".into(),
        ("Tcp", "InSegs") => "Total TCP segments received".into(),
        ("Tcp", "OutSegs") => "Total TCP segments sent".into(),
        ("Tcp", "RetransSegs") => "TCP segments retransmitted".into(),
        ("Tcp", "InErrs") => "TCP segments received with errors".into(),
        ("Tcp", "OutRsts") => "TCP RST (reset) segments sent".into(),
        ("Tcp", "InCsumErrors") => "TCP segments with checksum errors".into(),

        // --- Udp ---
        ("Udp", "InDatagrams") => "UDP datagrams received and delivered".into(),
        ("Udp", "NoPorts") => "UDP datagrams received for port with no listener".into(),
        ("Udp", "InErrors") => "UDP datagrams that could not be delivered (not NoPorts)".into(),
        ("Udp", "OutDatagrams") => "UDP datagrams sent".into(),
        ("Udp", "RcvbufErrors") => "UDP datagrams dropped due to receive buffer full".into(),
        ("Udp", "SndbufErrors") => "UDP datagrams dropped due to send buffer full".into(),
        ("Udp", "InCsumErrors") => "UDP datagrams with checksum errors".into(),
        ("Udp", "IgnoredMulti") => "UDP multicast datagrams ignored".into(),
        ("Udp", "MemErrors") => "UDP datagrams dropped due to memory allocation failures".into(),

        // --- UdpLite ---
        ("UdpLite", "InDatagrams") => "UDP-Lite datagrams received and delivered".into(),
        ("UdpLite", "NoPorts") => "UDP-Lite datagrams received for port with no listener".into(),
        ("UdpLite", "InErrors") => "UDP-Lite datagrams that could not be delivered".into(),
        ("UdpLite", "OutDatagrams") => "UDP-Lite datagrams sent".into(),
        ("UdpLite", "RcvbufErrors") => "UDP-Lite datagrams dropped due to receive buffer full".into(),
        ("UdpLite", "SndbufErrors") => "UDP-Lite datagrams dropped due to send buffer full".into(),
        ("UdpLite", "InCsumErrors") => "UDP-Lite datagrams with checksum errors".into(),
        ("UdpLite", "IgnoredMulti") => "UDP-Lite multicast datagrams ignored".into(),
        ("UdpLite", "MemErrors") => "UDP-Lite datagrams dropped due to memory allocation failures".into(),

        _ => format!("{}: {}", category, field),
    }
}
