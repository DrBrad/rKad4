use bencode::variables::bencode_object::BencodeObject;
use crate::messages::inter::message_type::MessageType;
use super::inter::method_message_base::MethodMessageBase;

pub struct PingResponse {
    pub base: MethodMessageBase
}

impl PingResponse {

    //WE DONT ALWAYS NEED THE TID...
    pub fn new(tid: [u8; 6]) -> Self {
        Self {
            base: MethodMessageBase::new(tid, "ping".to_string(), MessageType::RspMsg)
        }
    }

    pub fn encode(&self) -> BencodeObject {
        self.base.encode()
    }

    pub fn decode(&mut self, ben: &BencodeObject) {
        self.base.decode(&ben);
    }

    /*
    fn to_string() -> String {
        None
    }
    */
}