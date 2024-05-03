use crate::utils::uid::UID;
use std::net::SocketAddr;
use bencode::variables::bencode_object::BencodeObject;
use crate::messages::inter::message_type::MessageType;

pub trait MessageBase {

    fn set_uid(&mut self, uid: UID);

    fn get_uid(&self) -> UID;

    fn set_transaction_id(&mut self, tid: [u8; 6]);

    fn get_transaction_id(&self) -> &[u8; 6];

    fn set_public_address(&mut self, public_address: SocketAddr);

    fn get_public_address(&self) -> &SocketAddr;

    fn set_destination_address(&mut self, destination_address: SocketAddr);

    fn get_destination_address(&self) -> &SocketAddr;

    fn set_origin_address(&mut self, origin_address: SocketAddr);

    fn get_origin_address(&self) -> &SocketAddr;

    fn get_type(&self) -> &MessageType;

    fn encode(&self) -> BencodeObject;

    fn decode(&mut self, ben: &BencodeObject);
}

/*
use bencode::variables::bencode_object::{BencodeObject, PutObject};
use crate::messages::inter::message_type::{MessageType, TYPE_KEY};

pub struct MessageBase {
    pub uid: Option<UID>,
    pub tid: [u8; 6],
    pub type_: MessageType,
    pub destination: Option<SocketAddr>,
    pub origin: Option<SocketAddr>,
    pub public_address: Option<SocketAddr>,
}

pub const TID_KEY: &str = "t";

impl MessageBase {

    pub fn new(tid: [u8; 6], type_: MessageType) -> Self {
        Self {
            uid: None,
            tid,
            type_,
            destination: None,
            origin: None,
            public_address: None
        }
    }

    pub fn encode(&self) -> BencodeObject {
        let mut ben = BencodeObject::new();
        ben.put(TID_KEY, self.tid.clone());
        ben.put("v", "1.0");
        ben.put(TYPE_KEY, self.type_.rpc_type_name());

        ben
    }

    pub fn decode(&mut self, ben: &BencodeObject) {
        if !ben.contains_key(self.type_.inner_key()) {
            //throw new MessageException("Protocol Error, such as a malformed packet.", 203);
        }
    }
}
*/
