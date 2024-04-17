use std::net::IpAddr;
use crate::routing::inter::routing_table::RoutingTable;
use crate::utils::node::Node;
use crate::utils::uid::UID;

struct KRoutingTable {

}

impl RoutingTable for KRoutingTable {

    fn update_public_ip_consensus(source: IpAddr, addr: IpAddr) {
        todo!()
    }

    fn consensus_external_address() -> IpAddr {
        todo!()
    }

    fn insert(n: Node) {
        todo!()
    }

    fn derive_uid() {
        todo!()
    }

    fn has_queried(n: Node, now: u64) -> bool {
        todo!()
    }

    fn bucked_uid(k: UID) -> usize {
        todo!()
    }

    fn all_nodes() -> Vec(Node) {
        todo!()
    }

    fn find_closest(k: UID, r: u32) -> Vec(Node) {
        todo!()
    }

    fn bucked_size(i: u32) -> usize {
        todo!()
    }

    fn all_unqueried_nodes() -> Vec(Node) {
        todo!()
    }

    fn restart() {
        todo!()
    }
}