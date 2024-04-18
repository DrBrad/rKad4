use crate::messages::inter::message_type::MessageType;
use super::inter::method_message_base::MethodMessageBase;

pub struct FindNodeResponse {
    pub base: MethodMessageBase
}

impl FindNodeResponse {

    //WE DONT ALWAYS NEED THE TID...
    pub fn new(tid: [u8; 6]) -> Self {
        Self {
            base: MethodMessageBase::new(tid, "find_node".to_string(), MessageType::RSP_MSG)
        }
    }

    fn encode(&self) -> Vec<u8> {
        self.base.encode()
    }

    fn decode(&self, buf: Vec<u8>) {
        self.base.decode(&buf);
    }

    /*
    fn to_string() -> String {
        None
    }
    */
}