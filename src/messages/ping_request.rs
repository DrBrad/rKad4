use bencode::variables::bencode_object::{BencodeObject, PutObject};
use crate::messages::inter::message_type::MessageType;
use super::inter::message_base::MessageBase;
/*
use super::inter::method_message_base::MethodMessageBase;

pub struct PingRequest {
    pub base: MethodMessageBase
}

impl PingRequest {

    //WE DONT ALWAYS NEED THE TID...
    pub fn new(tid: [u8; 6]) -> Self {
        Self {
            base: MethodMessageBase::new(tid, "ping".to_string(), MessageType::ReqMsg)
        }
    }

    pub fn encode(&self) -> BencodeObject {
        self.base.encode()
    }

    pub fn decode(&mut self, ben: &BencodeObject) {
        self.base.decode(&ben);
    }
}
*/