use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::time::{SystemTime, UNIX_EPOCH};
use super::uid::UID;
use super::hash::crc32c::CRC32c;

const V4_MASK: [u8; 4] = [0x03, 0x0f, 0x3f, 0xff];
const V6_MASK: [u8; 8] = [0x01, 0x03, 0x07, 0x0f, 0x1f, 0x3f, 0x7f, 0xff];
const QUERY_TIME: u64 = 3600000;

pub struct Node {
    pub(crate) uid: UID,
    pub(crate) address: SocketAddr,
    pub(crate) stale: u32,
    pub(crate) last_seen: u64,
}

impl Node {
    // Constructor with UID and SocketAddr
    pub fn new(uid: UID, address: SocketAddr) -> Self {
        Self {
            uid,
            address,
            stale: 0,
            last_seen: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        }
    }

    // Check if the node has a secure ID
    pub fn has_secure_id(&self) -> bool {
        false
        /*
        let ip:[u8] = match self.address.ip() {
            IpAddr::V4(ipv4) => ipv4.octets(),
            IpAddr::V6(ipv6) => ipv6.octets(),
        };
        let mask = if ip.len() == 4 { &V4_MASK } else { &V6_MASK };

        let mut masked_ip = ip;
        for i in 0..mask.len() {
            masked_ip[i] &= mask[i];
        }

        let r = self.uid.bid[19] & 0x7;
        masked_ip[0] |= r << 5;

        let crc = CRC32c::checksum_ieee(&masked_ip[..8]);
        let uid_crc = ((u32::from(self.uid.bid[0]) << 24)
            | (u32::from(self.uid.bid[1]) << 16)
            | (u32::from(self.uid.bid[2]) << 8)
            | u32::from(self.uid.bid[3]))
            ^ crc;

        (uid_crc & 0xff_ff_f8_00) == 0
        */
    }

    //DETAILS
    pub fn seen(&mut self) {
        self.stale = 0;
        self.last_seen = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    }

    pub fn mark_stale(&mut self) {
        self.stale += 1;
    }

    pub fn has_queried(&self, now: u64) -> bool {
        self.last_seen > 0 && now - self.last_seen < QUERY_TIME
    }

    pub fn verify(&self, other: &Node) -> bool {
        //self.uid == other.uid
        return false;
    }

    pub fn to_string(&self) -> String {
        return format!("{{ \x1b[34mUID\x1b[0m: \x1b[35m{}\x1b[0m, \x1b[34mADDRESS\x1b[0m: \x1b[35m{}\x1b[0m, \x1b[34mPORT\x1b[0m: \x1b[35m{}\x1b[0m, \x1b[34mSECURE\x1b[0m: \x1b[35m{}\x1b[0m }}",
            self.uid.to_string(),
            self.address.ip(),
            self.address.port(),
            false);
    }
}