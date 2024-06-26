use std::net::Ipv4Addr;

use super::xx_netmask_width;

pub fn iptables_ip_masq(network: Ipv4Addr, netmask: Ipv4Addr, link_name: String) {
    let prefix_len = xx_netmask_width(netmask.octets());
    let source = format!("{}/{}", network, prefix_len);

    let ipt = iptables::new(false).unwrap();
    let rule = format!("-s {} ! -o {} -j MASQUERADE", source, link_name);

    let exists = ipt.exists("nat", "POSTROUTING", rule.as_str()).unwrap();
    if !exists {
        let _ = ipt.insert_unique("nat", "POSTROUTING", rule.as_str(), 1);
    }
}
