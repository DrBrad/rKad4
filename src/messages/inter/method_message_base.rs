use std::net::SocketAddr;
use crate::messages::inter::message_type::MessageType;
use crate::utils::uid::UID;
use super::message_base::MessageBase;

struct MethodMessageBase {
    method: str
}

impl MessageBase for MethodMessageBase {

    fn new() -> Self {
        None
    }

    fn encode() -> Vec<u8> {
        None
    }

    fn decode(buf: Vec<u8>) {

    }

    fn to_string() -> String {
        None
    }
}
