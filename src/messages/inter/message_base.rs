use crate::utils::uid::UID;
use std::net::SocketAddr;
use bencode::variables::bencode_object::{BencodeObject, PutObject};
use crate::messages::inter::message_type::{MessageType, TYPE_KEY};

pub struct MessageBase {
    pub(crate) uid: Option<UID>,
    pub(crate) tid: [u8; 6],
    pub(crate) type_: MessageType,
    pub(crate) destination: Option<SocketAddr>,
    pub(crate) origin: Option<SocketAddr>,
    pub(crate) public_address: Option<SocketAddr>,
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

    pub fn decode(&self, buf: &Vec<u8>) {

        /*
        if(!ben.containsKey(type.innerKey())){
            throw new MessageException("Protocol Error, such as a malformed packet.", 203);
        }
        */

    }

    /*

    fn to_string() -> String {
        None
    }
    */
}
