use std::net::IpAddr;
use crate::utils::node::Node;
use crate::utils::uid::UID;

pub trait RoutingTable {

    fn update_public_ip_consensus(source: IpAddr, addr: IpAddr);

    fn consensus_external_address() -> IpAddr;

    fn insert(&mut self, n: Node);

    fn derive_uid();

    //getDerivedUID - NOT NEEDED

    //fn add_restart_listener()
    //fn remove_restart_listener()

    //fn is_secure_only() -> bool; - NOT NEEDED

    fn has_queried(&self, n: &Node, now: u64) -> bool;

    fn bucket_uid(&self, k: &UID) -> usize;

    fn all_nodes() -> Vec<Node>;

    fn find_closest(k: &UID, r: u32) -> Vec<Node>;

    fn bucket_size(&self, i: usize) -> usize;

    fn all_unqueried_nodes() -> Vec<Node>;

    fn restart();

    //RESTART LISTENER
}