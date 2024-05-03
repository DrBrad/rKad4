use std::net::SocketAddr;
use bencode::variables::bencode_object::{BencodeObject, PutObject};
use crate::messages::inter::message_base::MessageBase;
use crate::messages::inter::message_type::MessageType;
use crate::utils::uid::{ID_LENGTH, UID};
use super::inter::method_message_base::MethodMessageBase;

pub struct FindNodeRequest {
    uid: Option<UID>,
    tid: [u8; 6],
    //type_: MessageType,
    public_address: Option<SocketAddr>,
    destination_address: Option<SocketAddr>,
    origin_address: Option<SocketAddr>,
    //method: String,
    target: Option<UID>
}

impl FindNodeRequest {

    pub fn new(tid: [u8; 6]) -> Self {
        Self {
            uid: None,
            tid,
            public_address: None,
            destination_address: None,
            origin_address: None,
            target: None
        }
    }

    pub fn set_target(&mut self, target: UID) {
        self.target = Some(target);
    }

    pub fn get_target(&mut self) -> Result<&UID, ()> {
        self.target.as_ref().map_or_else(|| Err(()), |uid| Ok(uid))
    }
}

impl Default for FindNodeRequest {

    fn default() -> Self {
        Self {
            uid: None,
            tid: [0u8; 6],
            public_address: None,
            destination_address: None,
            origin_address: None,
            target: None
        }
    }
}

//I WONDER IF WE CAN MACRO THIS SHIT FOR EVERY CLASS...?
impl MessageBase for FindNodeRequest {

    fn set_uid(&mut self, uid: UID) {
        self.uid = Some(uid);
    }

    fn get_uid(&self) -> Result<&UID, ()> {
        self.uid.as_ref().map_or_else(|| Err(()), |uid| Ok(uid))
    }

    fn set_transaction_id(&mut self, tid: [u8; 6]) {
        self.tid = tid;
    }

    fn get_transaction_id(&self) -> &[u8; 6] {
        &self.tid
    }

    fn set_public_address(&mut self, public_address: SocketAddr) {
        self.public_address = Some(public_address);
    }

    fn get_public_address(&self) -> Result<&SocketAddr, ()> {
        self.public_address.as_ref().map_or_else(|| Err(()), |addr| Ok(addr))
    }

    fn set_destination_address(&mut self, destination_address: SocketAddr) {
        self.destination_address = Some(destination_address);
    }

    fn get_destination_address(&self) -> Result<&SocketAddr, ()> {
        self.destination_address.as_ref().map_or_else(|| Err(()), |addr| Ok(addr))
    }

    fn set_origin_address(&mut self, origin_address: SocketAddr) {
        self.origin_address = Some(origin_address);
    }

    fn get_origin_address(&self) -> Result<&SocketAddr, ()> {
        self.origin_address.as_ref().map_or_else(|| Err(()), |addr| Ok(addr))
    }

    fn get_type(&self) -> MessageType {
        MessageType::ReqMsg
    }

    fn encode(&self) -> BencodeObject {
        let ben = BencodeObject::new();
        //let ben = MessageBase::encode(self);//<Self as MessageBase>::encode(self);
        println!("2");
        ben
    }

    fn decode(&mut self, ben: &BencodeObject) {
        todo!()
    }
}

impl MethodMessageBase for FindNodeRequest {

    fn get_method(&self) -> &str {
        "find_node"
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