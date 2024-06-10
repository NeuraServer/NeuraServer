use std::net::IpAddr;
use std::str::FromStr;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;
use std::time::Duration;
use std::thread;
use std::process::Command;

pub fn get_ip_from_hostname(hostname: &str) -> Option<IpAddr> {
    let resolver = Resolver::from_system_conf().ok()?;
    let response = resolver.lookup_ip(hostname).ok()?;
    response.iter().next()
}

pub fn is_valid_ip(ip: &str) -> bool {
    IpAddr::from_str(ip).is_ok()
}

pub fn ping_ip(ip: &str) -> bool {
    if !is_valid_ip(ip) {
        return false;
    }

    let output = Command::new("ping")
        .arg("-c 1")
        .arg(ip)
        .output()
        .expect("Failed to execute ping command");

    output.status.success()
}

pub fn get_latency(ip: &str) -> Option<Duration> {
    if !is_valid_ip(ip) {
        return None;
    }

    let start = std::time::Instant::now();
    let output = Command::new("ping")
        .arg("-c 1")
        .arg(ip)
        .output()
        .expect("Failed to execute ping command");

    if output.status.success() {
        Some(start.elapsed())
    } else {
        None
    }
}

pub fn resolve_domain(domain: &str) -> Option<Vec<IpAddr>> {
    let resolver = Resolver::from_system_conf().ok()?;
    let response = resolver.lookup_ip(domain).ok()?;
    Some(response.iter().collect())
}

pub fn get_local_ip() -> Option<IpAddr> {
    let addrs = local_ipaddress::get().ok()?;
    IpAddr::from_str(&addrs).ok()
}

pub fn scan_ports(ip: &str, ports: &[u16]) -> Vec<u16> {
    ports.iter()
        .filter(|&&port| {
            let result = Command::new("nc")
                .arg("-zv")
                .arg(ip)
                .arg(port.to_string())
                .output();

            result.map_or(false, |output| output.status.success())
        })
        .copied()
        .collect()
}
