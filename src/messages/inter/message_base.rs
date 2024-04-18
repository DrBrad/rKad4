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

    pub fn encode(&self) -> Vec<u8> {

        /*
        BencodeObject ben = new BencodeObject();

        ben.put(TID_KEY, tid); //TRANSACTION ID
        ben.put("v", VERSION_CODE); //VERSION

        ben.put(MessageType.TYPE_KEY, type.getRPCTypeName());
        */

        vec![]
    }

    pub fn decode(&self, buf: &Vec<u8>) {

        /*
        if(!ben.containsKey(type.innerKey())){
            throw new MessageException("Protocol Error, such as a malformed packet.", 203);
        }
        */

    }

    /*

    fn to_string() -> String {
        None
    }
    */
}
