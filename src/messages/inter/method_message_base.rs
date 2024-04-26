use std::net::SocketAddr;
use bencode::variables::bencode_object::{BencodeObject, PutObject};
use crate::messages::inter::message_type::MessageType;
use crate::utils::net::address_utils;
use crate::utils::net::address_utils::{pack_address, unpack_addr}
;
use crate::utils::uid::{ID_LENGTH, UID};
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

    pub fn encode(&self) -> BencodeObject {
        let mut ben = self.base.encode();

        match self.base.type_ {
            MessageType::ReqMsg => {
                ben.put(self.base.type_.rpc_type_name(), self.method.clone());
                ben.put(self.base.type_.inner_key(), BencodeObject::new());
                ben.get_object_mut(self.base.type_.inner_key()).unwrap().put("id", self.base.uid.unwrap().bid.clone());
            },
            MessageType::RspMsg => {
                ben.put(self.base.type_.inner_key(), BencodeObject::new());
                ben.get_object_mut(self.base.type_.inner_key()).unwrap().put("id", self.base.uid.unwrap().bid.clone());

                if let Some(public_address) = self.base.public_address {
                    ben.put("ip", [0u8; 20]);//pack_address(&public_address).to_vec());
                }
            },
            _ => unimplemented!()
        }

        ben
    }

    pub fn decode(&mut self, ben: &BencodeObject) {
        self.base.decode(&ben);

        if !ben.get_object(self.base.type_.inner_key()).unwrap().contains_key("id") {
            //throw new MessageException("Protocol Error, such as a malformed packet.", 203);
        }

        let mut bid = [0u8; ID_LENGTH];
        bid.copy_from_slice(&ben.get_object(self.base.type_.inner_key()).unwrap().get_bytes("id").unwrap()[..ID_LENGTH]);
        self.base.uid = Some(UID::from(bid));

        match self.base.type_ {
            MessageType::RspMsg => {
                if ben.contains_key("ip") {
                    //self.base.public_address = Some(unpack_addr(ben.get_bytes("ip").unwrap()));
                }
            },
            _ => ()
        };
    }

    /*
    fn to_string() -> String {
        None
    }
    */
}
