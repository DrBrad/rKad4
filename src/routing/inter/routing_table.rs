use std::net::IpAddr;
use crate::utils::node::Node;
use crate::utils::uid::UID;

pub trait RoutingTable {

    fn update_public_ip_consensus(source: IpAddr, addr: IpAddr);

    fn consensus_external_address() -> IpAddr;

    fn insert(n: Node);

    fn derive_uid();

    //getDerivedUID - NOT NEEDED

    //fn add_restart_listener()
    //fn remove_restart_listener()

    //fn is_secure_only() -> bool; - NOT NEEDED

    fn has_queried(n: Node, now: u64) -> bool;

    fn bucked_uid(k: UID) -> u32;

    fn all_nodes() -> Vec(Node);

    fn find_closest(k: UID, r: u32) -> Vec(Node);

    fn bucked_size(i: u32) -> u32;

    fn all_unqueried_nodes() -> Vec(Node);

    fn restart();

    //RESTART LISTENER
}