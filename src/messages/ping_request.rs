use super::inter::message_base::MessageBase;
use super::inter::method_message_base::MethodMessageBase;

pub struct PingRequest {
    pub base: MethodMessageBase
}

impl PingRequest {

    //WE DONT ALWAYS NEED THE TID...
    pub fn new(tid: [u8; 6]) -> Self {
        Self {
            base: MethodMessageBase::new(tid, "ping".to_string())
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
