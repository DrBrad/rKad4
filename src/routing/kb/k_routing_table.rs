use std::net::IpAddr;
use crate::routing::inter::routing_table::RoutingTable;
use super::k_bucket::KBucket;
use crate::utils::node::Node;
use crate::utils::uid::{ UID, ID_LENGTH };

pub struct KRoutingTable {
    uid: UID,
    secure_only: bool,
    k_buckets: [KBucket; ID_LENGTH*8]
}

impl KRoutingTable {

    pub fn new() -> Self {
        let mut k_buckets: [KBucket; ID_LENGTH*8] = [KBucket::new(); ID_LENGTH*8];//Default::default();
        //for i in 0..=ID_LENGTH {
        //    k_buckets[i] = KBucket::new();
        //}

        Self {
            uid: None,
            secure_only: true,
            k_buckets
        }
    }
}

impl RoutingTable for KRoutingTable {

    fn update_public_ip_consensus(source: IpAddr, addr: IpAddr) {
        todo!()
    }

    fn consensus_external_address() -> IpAddr {
        IpAddr::from([0, 0, 0, 0])
    }

    fn insert(&mut self, n: Node) {
        if self.secure_only && !n.has_secure_id() {
            return
        }

        if !self.uid.eq(&n.uid) {
            let id = self.bucket_uid(&n.uid);

            let mut contains_ip = false;
            for b in self.k_buckets {
                if b.contains_ip(&n) {
                    contains_ip = true;
                    break;
                }
            }

            let contains_uid = self.k_buckets[id].contains_uid(&n);

            if contains_ip == contains_uid {
                self.k_buckets[id].insert(n);
            }
        }
    }

    fn derive_uid() {
        todo!()
    }

    fn has_queried(&self, n: &Node, now: u64) -> bool {
        let mut id = self.bucked_uid(&n.uid);

        if !self.k_buckets[id].contains_uid(n) {
            return false;
        }

        self.k_buckets[id].hasQueried(n, now);
    }

    fn bucket_uid(&self, k: &UID) -> usize {
        let id = self.uid.distance(k)-1;
        if id < 0 {
            return 0;
        }
        id
    }

    fn all_nodes() -> Vec<Node> {
        todo!()
    }

    fn find_closest(k: &UID, r: u32) -> Vec<Node> {
        todo!()
    }

    fn bucket_size(&self, i: u32) -> usize {
        self.k_buckets[i].nodes.len()
    }

    fn all_unqueried_nodes() -> Vec<Node> {
        todo!()
    }

    fn restart() {
        todo!()
    }
}