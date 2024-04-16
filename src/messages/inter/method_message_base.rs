use std::net::SocketAddr;
use crate::messages::inter::message_type::MessageType;
use crate::utils::uid::UID;
use super::message_base::MessageBase;//{MessageBase, MessageBaseStruct};

pub trait MethodMessageBase {

    //HOW CAN WE ENCODE...

    fn encode() -> Vec<u8>;

    fn decode(buf: Vec<u8>);
}



/*
pub struct MethodMessageBase {
    method: str
}

//INIT FOR MESSAGE BASE
impl MessageBase for MethodMessageBase {

    fn new() -> Self {
    }

    fn encode() -> Vec<u8> {
        todo!()
    }

    fn decode(buf: Vec<u8>) {
        todo!()
    }

    fn to_string() -> String {
        todo!()
    }
}
*/




/*

trait Animal {
    fn print_name(&self);
}

trait Attack {
    fn can_kick(&self) -> bool {
        false
    }
}

trait Behavior {
    fn can_fly(&self) -> bool {
        true
    }
}

struct Bird {

}

impl Animal for Bird {
    fn print_name(&self) {
        todo!()
    }
}
impl Behavior for Bird {}
impl Attack for Bird {}


/*
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
*/
