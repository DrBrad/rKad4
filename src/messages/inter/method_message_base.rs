use std::net::SocketAddr;
use bencode::variables::bencode_object::{BencodeObject, PutObject};
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

    pub fn encode(&self) -> BencodeObject {
        let mut ben = self.base.encode();
        let bid = self.base.uid.unwrap();
        println!("{:?}", bid.bid);

        match self.base.type_ {
            MessageType::REQ_MSG => {
                let mut inner = BencodeObject::new();
                inner.put("id", &bid.bid);
                ben.put(self.base.type_.inner_key(), inner);
                //ben.put(self.base.type_.inner_key(), BencodeObject::new());
                //ben.get_object(self.base.type_.inner_key()).unwrap().put("id", &self.base.uid.unwrap().bid);
            },
            MessageType::RSP_MSG => {
                let mut inner = BencodeObject::new();
                inner.put("id", &bid.bid);
                ben.put(self.base.type_.inner_key(), inner);
                //ben.put(self.base.type_.inner_key(), BencodeObject::new());
                //ben.get_object(self.base.type_.inner_key()).unwrap().put("id", &self.base.uid.unwrap().bid);
            },
            _ => unimplemented!()
        }

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

        ben
    }

    pub fn decode(&self, buf: &Vec<u8>) {
        self.base.decode(&buf);
    }

    /*
    fn to_string() -> String {
        None
    }
    */
}
