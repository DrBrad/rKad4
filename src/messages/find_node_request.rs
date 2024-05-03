use std::net::SocketAddr;
use bencode::variables::bencode_object::{BencodeObject, PutObject};
use crate::messages::inter::message_base::{MessageBase, TID_KEY};
use crate::messages::inter::message_type::{MessageType, TYPE_KEY};
use crate::utils::net::address_utils::{pack_address, unpack_addr};
use crate::utils::uid::{ID_LENGTH, UID};
use super::inter::method_message_base::MethodMessageBase;

pub struct FindNodeRequest {
    uid: Option<UID>,
    tid: [u8; 6],
    //type_: MessageType,
    public_address: Option<SocketAddr>,
    destination_address: Option<SocketAddr>,
    origin_address: Option<SocketAddr>,
    //method: String,
    target: Option<UID>
}

impl FindNodeRequest {

    pub fn new(tid: [u8; 6]) -> Self {
        Self {
            uid: None,
            tid,
            public_address: None,
            destination_address: None,
            origin_address: None,
            target: None
        }
    }

    pub fn set_target(&mut self, target: UID) {
        self.target = Some(target);
    }

    pub fn get_target(&mut self) -> Result<&UID, ()> {
        self.target.as_ref().map_or_else(|| Err(()), |uid| Ok(uid))
    }
}

impl Default for FindNodeRequest {

    fn default() -> Self {
        Self {
            uid: None,
            tid: [0u8; 6],
            public_address: None,
            destination_address: None,
            origin_address: None,
            target: None
        }
    }
}

//I WONDER IF WE CAN MACRO THIS SHIT FOR EVERY CLASS...?
impl MessageBase for FindNodeRequest {

    fn set_uid(&mut self, uid: UID) {
        self.uid = Some(uid);
    }

    fn get_uid(&self) -> UID {
        self.uid.unwrap()
    }

    fn set_transaction_id(&mut self, tid: [u8; 6]) {
        self.tid = tid;
    }

    fn get_transaction_id(&self) -> &[u8; 6] {
        &self.tid
    }

    fn set_public_address(&mut self, public_address: SocketAddr) {
        self.public_address = Some(public_address);
    }

    fn get_public_address(&self) -> SocketAddr {
        self.public_address.unwrap()
    }

    fn set_destination_address(&mut self, destination_address: SocketAddr) {
        self.destination_address = Some(destination_address);
    }

    fn get_destination_address(&self) -> SocketAddr {
        self.destination_address.unwrap()
    }

    fn set_origin_address(&mut self, origin_address: SocketAddr) {
        self.origin_address = Some(origin_address);
    }

    fn get_origin_address(&self) -> SocketAddr {
        self.origin_address.unwrap()
    }

    fn get_type(&self) -> MessageType {
        MessageType::ReqMsg
    }

    fn encode(&self) -> BencodeObject {
        let mut ben = BencodeObject::new();

        ben.put(TID_KEY, self.tid.clone());
        ben.put("v", "1.0");
        ben.put(TYPE_KEY, self.get_type().rpc_type_name());

        match self.get_type() {
            MessageType::ReqMsg => {
                ben.put(self.get_type().rpc_type_name(), self.get_method());
                ben.put(self.get_type().inner_key(), BencodeObject::new());
                ben.get_object_mut(self.get_type().inner_key()).unwrap().put("id", self.uid.unwrap().bid.clone());
            },
            MessageType::RspMsg => {
                ben.put(self.get_type().inner_key(), BencodeObject::new());
                ben.get_object_mut(self.get_type().inner_key()).unwrap().put("id", self.uid.unwrap().bid.clone());

                if let Some(public_address) = self.public_address {
                    ben.put("ip", pack_address(&public_address));
                }
            },
            _ => unimplemented!()
        }

        if let Some(target) = self.target {
            ben.get_object_mut(self.get_type().inner_key()).unwrap().put("target", target.bid.clone());
        }

        ben
    }

    fn decode(&mut self, ben: &BencodeObject) {
        if !ben.contains_key(self.get_type().inner_key()) {
            //throw new MessageException("Protocol Error, such as a malformed packet.", 203);
        }

        if !ben.get_object(self.get_type().inner_key()).unwrap().contains_key("id") {
            //throw new MessageException("Protocol Error, such as a malformed packet.", 203);
        }

        let mut bid = [0u8; ID_LENGTH];
        bid.copy_from_slice(&ben.get_object(self.get_type().inner_key()).unwrap().get_bytes("id").unwrap()[..ID_LENGTH]);
        self.uid = Some(UID::from(bid));

        match self.get_type() {
            MessageType::RspMsg => {
                if ben.contains_key("ip") {
                    self.public_address = unpack_addr(ben.get_bytes("ip").unwrap());
                }
            },
            _ => ()
        };

        if !ben.get_object(self.get_type().inner_key()).unwrap().contains_key("target") {
            //throw new MessageException("Protocol Error, such as a malformed packet.", 203);
        }

        let mut bid = [0u8; ID_LENGTH];
        bid.copy_from_slice(&ben.get_object(self.get_type().inner_key()).unwrap().get_bytes("target").unwrap()[..ID_LENGTH]);
        self.target = Some(UID::from(bid));
    }
}

impl MethodMessageBase for FindNodeRequest {

    fn get_method(&self) -> &str {
        "find_node"
    }
}

/*
use super::inter::method_message_base::MethodMessageBase;

pub struct FindNodeRequest {
    pub base: MethodMessageBase,
    pub target: Option<UID>
}

impl FindNodeRequest {

    //WE DONT ALWAYS NEED THE TID...
    pub fn new(tid: [u8; 6]) -> Self {
        Self {
            base: MethodMessageBase::new(tid, "find_node".to_string(), MessageType::ReqMsg),
            target: None
        }
    }

    pub fn encode(&self) -> BencodeObject {
        let mut ben = self.base.encode();

        if let Some(target) = self.target {
            ben.get_object_mut(self.base.base.type_.inner_key()).unwrap().put("target", target.bid.clone());
        }

        ben
    }

    pub fn decode(&mut self, ben: &BencodeObject) {
        self.base.decode(&ben);

        if !ben.get_object(self.base.base.type_.inner_key()).unwrap().contains_key("target") {
            //throw new MessageException("Protocol Error, such as a malformed packet.", 203);
        }

        let mut bid = [0u8; ID_LENGTH];
        bid.copy_from_slice(&ben.get_object(self.base.base.type_.inner_key()).unwrap().get_bytes("target").unwrap()[..ID_LENGTH]);
        self.target = Some(UID::from(bid));
    }
}
*/