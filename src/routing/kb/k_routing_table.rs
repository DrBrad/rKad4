use std::net::IpAddr;
use std::time::{SystemTime, UNIX_EPOCH};
use core::array::from_fn;
//use rand::Rng;
use crate::routing::inter::routing_table::RoutingTable;
use crate::utils;
use crate::utils::hash::crc32c::CRC32c;
use super::k_bucket::KBucket;
use super::k_comparator::KComparator;
use crate::utils::node::{Node, V4_MASK, V6_MASK};
use crate::utils::uid::{ UID, ID_LENGTH };

pub struct KRoutingTable {
    pub(crate) uid: Option<UID>,
    pub(crate) consensus_external_address: IpAddr,
    pub(crate) secure_only: bool,
    pub(crate) k_buckets: [KBucket; ID_LENGTH*8]
}

impl KRoutingTable {

    pub fn new() -> Self {
        let mut routing_table = Self {
            uid: None,
            consensus_external_address: IpAddr::from([127, 0, 1, 1]),
            secure_only: true,
            k_buckets: from_fn(|_| KBucket::new())
        };

        routing_table.derive_uid();
        routing_table
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

        if let Some(uid) = &self.uid {
            if *uid != n.uid {
                let id = self.bucket_uid(&n.uid);

                let mut contains_ip = false;
                for b in &self.k_buckets {
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
    }

    fn derive_uid(&mut self) {
        let mut ip: Vec<u8> = match self.consensus_external_address {
            IpAddr::V4(v4) => v4.octets().to_vec(),
            IpAddr::V6(v6) => v6.octets().to_vec(),
        };

        let mask: Vec<u8> = if ip.len() == 4 {
            V4_MASK.to_vec()
        } else {
            V6_MASK.to_vec()
        };

        for i in 0..mask.len() {
            ip[i] &= mask[i];
        }

        //let mut rng = rand::thread_rng();
        let rand: u8 = /*rng.gen::<u8>()*/utils::random::gen() & 0xFF;
        let r = rand & 0x7;

        ip[0] |= r << 5;

        let mut c = CRC32c::new();
        c.update(&ip, 0, ip.len());
        let crc = c.get_value();

        let mut bid = [0u8; ID_LENGTH];
        bid[0] = (crc >> 24) as u8;
        bid[1] = (crc >> 16) as u8;
        bid[2] = ((crc >> 8) as u8 & 0xF8) | (/*rng.gen::<u8>()*/utils::random::gen() & 0x7);

        for i in 3..19 {
            bid[i] = /*rng.gen::<u8>()*/utils::random::gen() & 0xFF;
        }

        bid[19] = rand & 0xFF;

        self.uid = Some(UID::from(bid));
    }

    fn has_queried(&self, n: &Node, now: u64) -> bool {
        let id = self.bucket_uid(&n.uid);

        if !self.k_buckets[id].contains_uid(n) {
            return false;
        }

        self.k_buckets[id].has_queried(n, now)
    }

    fn bucket_uid(&self, k: &UID) -> usize {
        self.uid.unwrap().distance(k)-1
    }

    fn all_nodes(&self) -> Vec<Node> {
        let mut nodes = vec![];

        for b in &self.k_buckets {
            nodes.extend(&b.nodes);
        }

        nodes
    }

    fn find_closest(&self, k: &UID, r: usize) -> Vec<Node> {
        let mut sorted_set = self.all_nodes();
        let comparator = KComparator::new(k);
        sorted_set.sort_by(|a, b| comparator.compare(a, b));

        let mut closest = Vec::with_capacity(r);
        let mut count = 0;

        for &n in &sorted_set {
            closest.push(n);
            count += 1;

            if count == r {
                break;
            }
        }

        closest
    }

    fn bucket_size(&self, i: usize) -> usize {
        self.k_buckets[i].nodes.len()
    }

    fn all_unqueried_nodes(&self) -> Vec<Node> {
        let mut nodes = vec![];

        let time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
        let now = time.as_secs() * 1000 + time.subsec_millis() as u64;

        for b in &self.k_buckets {
            nodes.extend(&b.unqueried_nodes(now));
        }

        nodes
    }

    fn restart() {
        todo!()
    }
}