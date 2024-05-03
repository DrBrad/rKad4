use std::net::SocketAddr;
use bencode::variables::bencode_object::{BencodeObject, PutObject};
use crate::messages::inter::message_base::MessageBase;
use crate::messages::inter::message_type::MessageType;
use crate::utils::uid::{ID_LENGTH, UID};
use super::inter::method_message_base::MethodMessageBase;

pub struct FindNodeRequest {
    uid: Option<UID>,
    tid: [u8; 6],
    type_: MessageType,
    destination: Option<SocketAddr>,
    origin: Option<SocketAddr>,
    public_address: Option<SocketAddr>,
    method: String,
    target: Option<UID>
}

impl FindNodeRequest {

    pub fn new() -> Self {
        /*
        Self {

        }
        */
        unimplemented!()
    }
}

impl MessageBase for FindNodeRequest {

    fn set_uid(&mut self, uid: UID) {
        todo!()
    }

    fn get_uid(&self) -> UID {
        todo!()
    }

    fn set_transaction_id(&mut self, tid: [u8; 6]) {
        todo!()
    }

    fn get_transaction_id(&self) -> &[u8; 6] {
        todo!()
    }

    fn set_public_address(&mut self, public_address: SocketAddr) {
        todo!()
    }

    fn get_public_address(&self) -> &SocketAddr {
        todo!()
    }

    fn set_destination_address(&mut self, destination_address: SocketAddr) {
        todo!()
    }

    fn get_destination_address(&self) -> &SocketAddr {
        todo!()
    }

    fn set_origin_address(&mut self, origin_address: SocketAddr) {
        todo!()
    }

    fn get_origin_address(&self) -> &SocketAddr {
        todo!()
    }

    fn get_type(&self) -> &MessageType {
        todo!()
    }

    fn encode(&self) -> BencodeObject {
        todo!()
    }

    fn decode(&mut self, ben: &BencodeObject) {
        todo!()
    }
}

impl MethodMessageBase for FindNodeRequest {

    fn get_method(&self) -> String {
        todo!()
    }
}

/*
use super::inter::method_message_base::MethodMessageBase;

pub struct FindNodeRequest {
    pub base: MethodMessageBase,
    pub target: Option<UID>
}

impl FindNodeRequest {

    //WE DONT ALWAYS NEED THE TID...
    pub fn new(tid: [u8; 6]) -> Self {
        Self {
            base: MethodMessageBase::new(tid, "find_node".to_string(), MessageType::ReqMsg),
            target: None
        }
    }

    pub fn encode(&self) -> BencodeObject {
        let mut ben = self.base.encode();

        if let Some(target) = self.target {
            ben.get_object_mut(self.base.base.type_.inner_key()).unwrap().put("target", target.bid.clone());
        }

        ben
    }

    pub fn decode(&mut self, ben: &BencodeObject) {
        self.base.decode(&ben);

        if !ben.get_object(self.base.base.type_.inner_key()).unwrap().contains_key("target") {
            //throw new MessageException("Protocol Error, such as a malformed packet.", 203);
        }

        let mut bid = [0u8; ID_LENGTH];
        bid.copy_from_slice(&ben.get_object(self.base.base.type_.inner_key()).unwrap().get_bytes("target").unwrap()[..ID_LENGTH]);
        self.target = Some(UID::from(bid));
    }
}
*/