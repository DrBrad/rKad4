use std::net::IpAddr;
use crate::utils::node::Node;
use crate::utils::uid::UID;

pub trait RoutingTable: Send {

    fn update_public_ip_consensus(&mut self, source: IpAddr, addr: IpAddr);

    fn get_consensus_external_address(&self) -> IpAddr;

    fn insert(&mut self, n: Node);

    fn derive_uid(&mut self);

    fn get_derived_uid(&self) -> UID;

    fn is_secure_only(&self) -> bool;

    fn set_secure_only(&mut self, secure_only: bool);

    //fn add_restart_listener()
    //fn remove_restart_listener()

    fn has_queried(&self, n: &Node, now: u128) -> bool;

    fn bucket_uid(&self, k: &UID) -> usize;

    fn all_nodes(&self) -> Vec<Node>;

    fn find_closest(&self, k: &UID, r: usize) -> Vec<Node>;

    fn bucket_size(&self, i: usize) -> usize;

    fn all_unqueried_nodes(&self) -> Vec<Node>;

    fn restart(&self);

    //RESTART LISTENER
}