use bencode::variables::bencode_object::BencodeObject;
use crate::messages::inter::message_type::MessageType;
use super::inter::method_message_base::MethodMessageBase;

pub struct FindNodeResponse {
    pub base: MethodMessageBase
}

impl FindNodeResponse {

    //WE DONT ALWAYS NEED THE TID...
    pub fn new(tid: [u8; 6]) -> Self {
        Self {
            base: MethodMessageBase::new(tid, "find_node".to_string(), MessageType::RspMsg)
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