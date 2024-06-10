use std::net::IpAddr;
use std::str::FromStr;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;

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

    let output = std::process::Command::new("ping")
        .arg("-c 1")
        .arg(ip)
        .output()
        .expect("Failed to execute ping command");

    output.status.success()
}
