use crate::utils::uid::UID;
use message_type::MessageType;
use std::net::SocketAddr;
use crate::messages::inter::message_type;

pub struct MessageBaseStruct {
    uid: UID,
    tid: [u8; 6],
    typ: MessageType,
    destination: SocketAddr,
    origin: SocketAddr,
    public_address: SocketAddr,
}

pub trait MessageBase {

    /*
    const uid: UID;
    const tid: [u8; 6];
    const typ: MessageType;
    const destination: SocketAddr;
    const origin: SocketAddr;
    const public_address: SocketAddr;
    */

    fn new() -> Self;

    fn encode() -> Vec<u8>;

    fn decode(buf: Vec<u8>);

    fn to_string() -> String;
}
