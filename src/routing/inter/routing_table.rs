use std::net::IpAddr;
use crate::utils::node::Node;
use crate::utils::uid::UID;

pub trait RoutingTable {

    fn update_public_ip_consensus(&self, source: IpAddr, addr: IpAddr);

    fn insert(&mut self, n: Node);

    fn derive_uid(&mut self);

    //getDerivedUID - NOT NEEDED

    //fn add_restart_listener()
    //fn remove_restart_listener()

    //fn is_secure_only() -> bool; - NOT NEEDED

    fn has_queried(&self, n: &Node, now: u64) -> bool;

    fn bucket_uid(&self, k: &UID) -> usize;

    fn all_nodes(&self) -> Vec<Node>;

    fn find_closest(&self, k: &UID, r: usize) -> Vec<Node>;

    fn bucket_size(&self, i: usize) -> usize;

    fn all_unqueried_nodes(&self) -> Vec<Node>;

    fn restart(&self);

    //RESTART LISTENER
}