use crate::proc::{Field, FieldValue, ProcEntry};
use std::fs;
use std::net::ToSocketAddrs;
use std::time::Instant;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let content = fs::read_to_string("/etc/resolv.conf")?;

    let mut nameserver_rows = Vec::new();
    let mut search_domains = Vec::new();
    let mut options = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }

        match parts[0] {
            "nameserver" => {
                let ip = parts[1].to_string();
                let ns_type = if ip.contains(':') {
                    "IPv6".to_string()
                } else {
                    "IPv4".to_string()
                };
                nameserver_rows.push(vec![ip, ns_type]);
            }
            "search" | "domain" => {
                for domain in &parts[1..] {
                    search_domains.push(domain.to_string());
                }
            }
            "options" => {
                for opt in &parts[1..] {
                    options.push(opt.to_string());
                }
            }
            _ => {}
        }
    }

    let search_text = if search_domains.is_empty() {
        "(none)".to_string()
    } else {
        search_domains.join(" ")
    };

    let options_text = if options.is_empty() {
        "(none)".to_string()
    } else {
        options.join(" ")
    };

    let mut fields = vec![
        Field {
            name: "nameservers".into(),
            value: FieldValue::Table(nameserver_rows),
            unit: None,
            description: "Configured DNS nameservers: IP, type (IPv4/IPv6)".into(),
        },
        Field {
            name: "search_domains".into(),
            value: FieldValue::Text(search_text),
            unit: None,
            description: "DNS search domains from /etc/resolv.conf".into(),
        },
        Field {
            name: "options".into(),
            value: FieldValue::Text(options_text),
            unit: None,
            description: "DNS resolver options from /etc/resolv.conf".into(),
        },
    ];

    // Optional: measure DNS resolution time for localhost
    if let Some(ms) = measure_dns_resolution() {
        fields.push(Field {
            name: "dns_resolution_ms".into(),
            value: FieldValue::Float(ms),
            unit: Some("ms".into()),
            description: "Time to resolve 'localhost' via system resolver".into(),
        });
    }

    Ok(ProcEntry {
        source: "/etc/resolv.conf".into(),
        fields,
    })
}

fn measure_dns_resolution() -> Option<f64> {
    let start = Instant::now();
    // Attempt a simple resolution of "localhost"
    let _result = ("localhost", 0u16).to_socket_addrs().ok()?;
    let elapsed = start.elapsed();
    Some(elapsed.as_secs_f64() * 1000.0)
}
