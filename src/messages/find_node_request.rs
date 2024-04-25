use bencode::variables::bencode_object::{BencodeObject, PutObject};
use crate::messages::inter::message_type::MessageType;
use crate::utils::uid::UID;
use super::inter::message_base::MessageBase;
use super::inter::method_message_base::MethodMessageBase;

pub struct FindNodeRequest {
    pub base: MethodMessageBase,
    pub target: Option<UID>
}

impl FindNodeRequest {

    //WE DONT ALWAYS NEED THE TID...
    pub fn new(tid: [u8; 6]) -> Self {
        Self {
            base: MethodMessageBase::new(tid, "find_node".to_string(), MessageType::REQ_MSG),
            target: None
        }
    }

    pub fn encode(&self) -> BencodeObject {
        let mut ben = self.base.encode();
        ben.get_object(self.base.base.type_.inner_key()).unwrap().put("target", "TEST");
        ben
    }

    pub fn decode(buf: Vec<u8>) {

    }

    /*
    fn to_string() -> String {
        None
    }
    */
}
