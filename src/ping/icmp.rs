use std::time::Duration;
use surge_ping::{Client, Config, IcmpPacket, PingIdentifier, PingSequence, SurgeError, ICMP};
use crate::config::PingTarget;
pub async fn ping(target: &PingTarget) -> Result<(IcmpPacket, Duration), SurgeError> {
    let ip = tokio::net::lookup_host(format!("{}:0", target.host))
        .await
        .expect("host lookup error")
        .next()
        .map(|val| val.ip())
        .unwrap();

    let mut config_builder = Config::builder();

    if let Some(interface) = target.iface.clone() {
        config_builder = config_builder.interface(&interface);
    }

    if ip.is_ipv6() {
        config_builder = config_builder.kind(ICMP::V6);
    }
    let config = config_builder.build();

    let payload = vec![0; target.size];
    let client = Client::new(&config)?;
    let mut pinger = client.pinger(ip, PingIdentifier(111)).await;
    pinger.timeout(Duration::from_secs(target.timeout.unwrap_or(3)));
    pinger.ping(PingSequence(0), &payload).await
}