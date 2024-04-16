use crate::utils::uid::UID;
use message_type::MessageType;
use std::net::SocketAddr;
use crate::messages::inter::message_type;

pub struct MessageBase {
    pub(crate) uid: Option<UID>,
    pub(crate) tid: [u8; 6],
    pub(crate) type_: MessageType,
    pub(crate) destination: Option<SocketAddr>,
    pub(crate) origin: Option<SocketAddr>,
    pub(crate) public_address: Option<SocketAddr>,
}

impl MessageBase {

    pub fn new(tid: [u8; 6], type_: MessageType) -> Self {
        Self {
            uid: None,
            tid,
            type_,
            destination: None,
            origin: None,
            public_address: None
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
