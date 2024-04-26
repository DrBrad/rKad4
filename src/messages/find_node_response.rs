use bencode::variables::bencode_object::{BencodeObject, PutObject};
use crate::messages::inter::message_type::MessageType;
use crate::utils::net::address_type::AddressType;
use crate::utils::node_utils::pack_nodes;
use crate::utils::node::Node;
use super::inter::method_message_base::MethodMessageBase;

pub const NODE_CAP: usize = 20;

pub struct FindNodeResponse {
    pub base: MethodMessageBase,
    pub nodes: Vec<Node>
}

impl FindNodeResponse {

    //WE DONT ALWAYS NEED THE TID...
    pub fn new(tid: [u8; 6]) -> Self {
        Self {
            base: MethodMessageBase::new(tid, "find_node".to_string(), MessageType::RspMsg),
            nodes: Vec::new()
        }
    }

    pub fn encode(&self) -> BencodeObject {
        let mut ben = self.base.encode();

        if self.nodes.is_empty() {
            return ben;
        }

        let nodes = self.ipv4_nodes();
        if !self.nodes.is_empty() {
            ben.get_object_mut(self.base.base.type_.inner_key()).unwrap().put("nodes", pack_nodes(nodes, AddressType::IPv4));
        }

        let nodes = self.ipv6_nodes();
        if !self.nodes.is_empty() {
            ben.get_object_mut(self.base.base.type_.inner_key()).unwrap().put("nodes", pack_nodes(nodes, AddressType::IPv6));
        }

        ben
    }

    pub fn decode(&mut self, ben: &BencodeObject) {
        self.base.decode(&ben);

        if !ben.get_object(self.base.base.type_.inner_key()).unwrap().contains_key("nodes") &&
                !ben.get_object(self.base.base.type_.inner_key()).unwrap().contains_key("nodes6") {
            //throw new MessageException("Protocol Error, such as a malformed packet.", 203);
        }

        if ben.get_object(self.base.base.type_.inner_key()).unwrap().contains_key("nodes") {
            //nodes.addAll(unpackNodes(ben.getBencodeObject(type.innerKey()).getBytes("nodes"), AddressType.IPv4));
        }

        if ben.get_object(self.base.base.type_.inner_key()).unwrap().contains_key("nodes6") {
            //nodes.addAll(unpackNodes(ben.getBencodeObject(type.innerKey()).getBytes("nodes6"), AddressType.IPv6));
        }
    }

    pub fn ipv4_nodes(&self) -> Vec<Node> {
        Vec::new()
    }

    pub fn ipv6_nodes(&self) -> Vec<Node> {
        Vec::new()
    }

    /*
    fn to_string() -> String {
        None
    }
    */
}