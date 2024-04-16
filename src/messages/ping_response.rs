use crate::messages::inter::message_type::MessageType;
use super::inter::method_message_base::MethodMessageBase;

pub struct PingResponse {
    pub base: MethodMessageBase
}

impl PingResponse {

    //WE DONT ALWAYS NEED THE TID...
    pub fn new(tid: [u8; 6]) -> Self {
        Self {
            base: MethodMessageBase::new(tid, "ping".to_string(), MessageType::RSP_MSG)
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