use std::net::SocketAddr;
use crate::utils::uid::ID_LENGTH;
use super::net::address_type::AddressType;
use super::node::Node;

pub fn pack_nodes(nodes: Vec<Node>, addr_type: AddressType) -> Vec<u8> {
    let addr_length = match addr_type {
        AddressType::IPv4 => 4,
        AddressType::IPv6 => 16,
    };
    let mut buf = vec![0; nodes.len() * (ID_LENGTH + addr_length + 2)];
    let mut position = 0;

    for n in nodes {
        buf[position..position + ID_LENGTH].copy_from_slice(&n.uid.bid);
        position += ID_LENGTH;

        match n.address {
            SocketAddr::V4(ipv4) => {
                buf[position..position + addr_length].copy_from_slice(&ipv4.ip().octets());
            }
            SocketAddr::V6(ipv6) => {
                buf[position..position + addr_length].copy_from_slice(&ipv6.ip().octets());
            }
        }
        position += addr_length;

        buf[position] = (n.address.port() >> 8) as u8;
        buf[position + 1] = n.address.port() as u8;
        position += 2;
    }

    buf
}

/*
pub fn pack_nodes(nodes: &[Node], addr_type: AddressType) -> Vec<u8> {
    let addr_length = match addr_type {
        AddressType::IPv4 => 4,
        AddressType::IPv6 => 16,
    };
    let mut buf = vec![0; nodes.len() * (ID_LENGTH + addr_length + 2)];
    let mut position = 0;

    for n in nodes {
        buf[position..position + ID_LENGTH].copy_from_slice(&n.uid);
        position += ID_LENGTH;

        match n.host_address() {
            SocketAddr::V4(ipv4) => {
                buf[position..position + addr_length].copy_from_slice(&ipv4.ip().octets());
            }
            SocketAddr::V6(ipv6) => {
                buf[position..position + addr_length].copy_from_slice(&ipv6.ip().octets());
            }
        }
        position += addr_length;

        buf[position] = (n.port() >> 8) as u8;
        buf[position + 1] = n.port() as u8;
        position += 2;
    }

    buf
}

pub fn unpack_nodes(buf: &[u8], addr_type: AddressType) -> Vec<Node> {
    let addr_length = match addr_type {
        AddressType::IPv4 => 4,
        AddressType::IPv6 => 16,
    };

    let mut nodes = Vec::new();
    let mut position = 0;

    while position < buf.len() {
        let mut uid = [0; ID_LENGTH];
        uid.copy_from_slice(&buf[position..position + ID_LENGTH]);
        position += ID_LENGTH;

        let mut addr = [0; 16]; // Maximum size for IPv6
        addr.copy_from_slice(&buf[position..position + addr_length]);
        position += addr_length;

        let port = ((buf[position] as u16) << 8) | (buf[position + 1] as u16);
        position += 2;

        match addr_type {
            AddressType::IPv4 => {
                let ipv4_addr = Ipv4Addr::new(addr[0], addr[1], addr[2], addr[3]);
                let socket_addr = SocketAddr::new(ipv4_addr.into(), port);
                nodes.push(Node::new(uid, socket_addr, port));
            }
            AddressType::IPv4 => {
                let ipv6_addr = Ipv6Addr::new(
                    u16::from_be_bytes([addr[0], addr[1]]),
                    u16::from_be_bytes([addr[2], addr[3]]),
                    u16::from_be_bytes([addr[4], addr[5]]),
                    u16::from_be_bytes([addr[6], addr[7]]),
                    u16::from_be_bytes([addr[8], addr[9]]),
                    u16::from_be_bytes([addr[10], addr[11]]),
                    u16::from_be_bytes([addr[12], addr[13]]),
                    u16::from_be_bytes([addr[14], addr[15]]),
                );
                let socket_addr = SocketAddr::new(ipv6_addr.into(), port);
                nodes.push(Node::new(uid, socket_addr, port));
            }
        }
    }

    nodes
}
*/
