use std::net::SocketAddr;
use crate::messages::inter::message_type::MessageType;
use crate::utils::uid::UID;
use super::message_base::MessageBase;//{MessageBase, MessageBaseStruct};


pub struct MethodMessageBase {
    pub(crate) base: MessageBase,
    pub(crate) method: String
}

impl MethodMessageBase {

    pub fn new(tid: [u8; 6], method: String, type_: MessageType) -> Self {
        Self {
            base: MessageBase::new(tid, type_),
            method //FIGURE OUT WHY WE CANT JUST USE "BLANK"
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
