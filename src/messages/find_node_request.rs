use crate::messages::inter::message_type::MessageType;
use super::inter::message_base::MessageBase;
use super::inter::method_message_base::MethodMessageBase;

pub struct FindNodeRequest {
    pub base: MethodMessageBase
}

impl FindNodeRequest {

    //WE DONT ALWAYS NEED THE TID...
    pub fn new(tid: [u8; 6]) -> Self {
        Self {
            base: MethodMessageBase::new(tid, "find_node".to_string(), MessageType::REQ_MSG)
        }
    }

    /*
    fn encode() -> Vec<u8> {

    }

    fn decode(buf: Vec<u8>) {

    }

    fn to_string() -> String {
        None
    }
    */
}
