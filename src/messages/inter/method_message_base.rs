use std::net::SocketAddr;
use bencode::variables::bencode_object::{BencodeObject, PutObject};
use crate::messages::inter::message_type::MessageType;
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
        //println!("{:?}", bid);

        match self.base.type_ {
            MessageType::ReqMsg => {
                ben.put(self.base.type_.inner_key(), BencodeObject::new());
                ben.get_object_mut(self.base.type_.inner_key()).unwrap().put("id", self.base.uid.unwrap().bid.clone());
            },
            MessageType::RspMsg => {
                ben.put(self.base.type_.inner_key(), BencodeObject::new());
                ben.get_object_mut(self.base.type_.inner_key()).unwrap().put("id", self.base.uid.unwrap().bid.clone());
            },
            _ => unimplemented!()
        }

        ben

        /*
        switch(type){
            case REQ_MSG:
                ben.put(type.getRPCTypeName(), method);
            ben.put(type.innerKey(), new BencodeObject());
            ben.getBencodeObject(type.innerKey()).put("id", uid.getBytes());
            break;

            case RSP_MSG:
                ben.put(type.innerKey(), new BencodeObject());
            ben.getBencodeObject(type.innerKey()).put("id", uid.getBytes());

            if(publicAddress != null){
                ben.put("ip", AddressUtils.packAddress(publicAddress)); //PACK MY IP ADDRESS
            }
            break;
        }
        */

        //BencodeObject::new()
    }

    pub fn decode(&mut self, ben: &BencodeObject) {
        self.base.decode(&ben);

        if !ben.get_object(self.base.type_.inner_key()).unwrap().contains_key("id") {
            //throw new MessageException("Protocol Error, such as a malformed packet.", 203);
        }

        //: &[u8; ID_LENGTH]
        //let bid = ben.get_object(self.base.type_.inner_key()).unwrap().get_bytes("id").unwrap();
        //self.base.uid = Some(UID::from(bid));

        match self.base.type_ {
            MessageType::RspMsg => {
                if ben.contains_key("ip") {
                    //self.base.public_address = AddressUtils.unpackAddress(ben.getBytes("ip"));
                }
            },
            _ => unimplemented!()
        };
    }

    /*
    fn to_string() -> String {
        None
    }
    */
}
