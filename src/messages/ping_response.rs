use std::net::SocketAddr;
use bencode::variables::bencode_object::{BencodeObject, PutObject};
use crate::kad::server::TID_LENGTH;
use crate::messages::inter::message_base::{MessageBase, TID_KEY};
use crate::messages::inter::message_type::{MessageType, TYPE_KEY};
use crate::utils::net::address_utils::{pack_address, unpack_address};
use crate::utils::uid::{ID_LENGTH, UID};
use super::inter::method_message_base::MethodMessageBase;

pub struct PingResponse {
    uid: Option<UID>,
    tid: [u8; TID_LENGTH],
    public: Option<SocketAddr>,
    destination: Option<SocketAddr>,
    origin: Option<SocketAddr>
}

impl PingResponse {

    pub fn new(tid: [u8; TID_LENGTH]) -> Self {
        Self {
            uid: None,
            tid,
            public: None,
            destination: None,
            origin: None
        }
    }
}

impl Default for PingResponse {

    fn default() -> Self {
        Self {
            uid: None,
            tid: [0u8; TID_LENGTH],
            public: None,
            destination: None,
            origin: None
        }
    }
}

//I WONDER IF WE CAN MACRO THIS SHIT FOR EVERY CLASS...?
impl MessageBase for PingResponse {

    fn set_uid(&mut self, uid: UID) {
        self.uid = Some(uid);
    }

    fn get_uid(&self) -> UID {
        self.uid.unwrap()
    }

    fn set_transaction_id(&mut self, tid: [u8; TID_LENGTH]) {
        self.tid = tid;
    }

    fn get_transaction_id(&self) -> &[u8; TID_LENGTH] {
        &self.tid
    }

    fn set_public(&mut self, public: SocketAddr) {
        self.public = Some(public);
    }

    fn get_public(&self) -> SocketAddr {
        self.public.unwrap()
    }

    fn set_destination(&mut self, destination: SocketAddr) {
        self.destination = Some(destination);
    }

    fn get_destination(&self) -> SocketAddr {
        self.destination.unwrap()
    }

    fn set_origin(&mut self, origin: SocketAddr) {
        self.origin = Some(origin);
    }

    fn get_origin(&self) -> SocketAddr {
        self.origin.unwrap()
    }

    fn get_type(&self) -> MessageType {
        MessageType::RspMsg
    }

    fn encode(&self) -> BencodeObject {
        let mut ben = BencodeObject::new();

        ben.put(TID_KEY, self.tid.clone());
        ben.put("v", "1.0");
        ben.put(TYPE_KEY, self.get_type().rpc_type_name());

        ben.put(self.get_type().inner_key(), BencodeObject::new());
        ben.get_object_mut(self.get_type().inner_key()).unwrap().put("id", self.uid.unwrap().bid.clone());

        if let Some(public) = self.public {
            ben.put("ip", pack_address(&public));
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

        if ben.contains_key("ip") {
            self.public = unpack_address(ben.get_bytes("ip").unwrap());
        }
    }
}

impl MethodMessageBase for PingResponse {

    fn get_method(&self) -> &str {
        "ping"
    }
}
