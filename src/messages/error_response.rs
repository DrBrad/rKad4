use std::any::Any;
use std::net::SocketAddr;
use bencode::variables::bencode_array::{AddArray, BencodeArray};
use bencode::variables::bencode_object::{BencodeObject, PutObject};
use crate::kad::server::TID_LENGTH;
use crate::messages::inter::message_base::{MessageBase, TID_KEY};
use crate::messages::inter::message_exception::MessageException;
use crate::messages::inter::message_type::{MessageType, TYPE_KEY};
use crate::utils::net::address_utils::pack_address;
use crate::utils::uid::UID;

#[derive(Clone)]
pub struct ErrorResponse {
    uid: Option<UID>,
    tid: [u8; TID_LENGTH],
    public: Option<SocketAddr>,
    destination: Option<SocketAddr>,
    origin: Option<SocketAddr>,
    code: i32,
    description: Option<String>
}

impl ErrorResponse {

    pub fn new(tid: [u8; TID_LENGTH]) -> Self {
        Self {
            tid,
            ..Default::default()
        }
    }

    pub fn set_code(&mut self, code: i32) {
        self.code = code;
    }

    pub fn get_code(&self) -> i32 {
        self.code
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = Some(description.to_string());
    }

    pub fn get_description(&self) -> &String {
        self.description.as_ref().unwrap()
    }
}

impl Default for ErrorResponse {

    fn default() -> Self {
        Self {
            uid: None,
            tid: [0u8; TID_LENGTH],
            public: None,
            destination: None,
            origin: None,
            code: 0,
            description: None
        }
    }
}

impl MessageBase for ErrorResponse {

    fn set_uid(&mut self, uid: UID) {
        self.uid = Some(uid);
    }

    fn get_uid(&self) -> Option<UID> {
        self.uid
    }

    fn set_transaction_id(&mut self, tid: [u8; TID_LENGTH]) {
        self.tid = tid;
    }

    fn get_transaction_id(&self) -> &[u8; TID_LENGTH] {
        &self.tid
    }

    fn set_public(&mut self, public: SocketAddr) {
        self.public = Some(public);
    }

    fn get_public(&self) -> Option<SocketAddr> {
        self.public
    }

    fn set_destination(&mut self, destination: SocketAddr) {
        self.destination = Some(destination);
    }

    fn get_destination(&self) -> Option<SocketAddr> {
        self.destination
    }

    fn set_origin(&mut self, origin: SocketAddr) {
        self.origin = Some(origin);
    }

    fn get_origin(&self) -> Option<SocketAddr> {
        self.origin
    }

    fn get_type(&self) -> MessageType {
        MessageType::ErrMsg
    }

    fn encode(&self) -> BencodeObject {
        let mut ben = BencodeObject::new();

        ben.put(TID_KEY, self.tid.clone());
        ben.put("v", "1.0");
        ben.put(TYPE_KEY, self.get_type().rpc_type_name());

        let mut arr = BencodeArray::new();
        arr.add(self.code);
        arr.add(self.description.as_ref().unwrap().clone());
        ben.put(self.get_type().inner_key(), arr);

        if let Some(public) = self.public {
            ben.put("ip", pack_address(&public));
        }

        ben
    }

    fn decode(&mut self, ben: &BencodeObject) -> Result<(), MessageException> {
        if !ben.contains_key(self.get_type().inner_key()) {
            return Err(MessageException::new("Protocol Error, such as a malformed packet.", 203));
        }

        if ben.get_array(self.get_type().inner_key()).unwrap().size() < 2 {
            return Err(MessageException::new("Protocol Error, such as a malformed packet.", 203));
        }

        self.code = ben.get_array(self.get_type().inner_key()).unwrap().get_number::<i32>(0).map_err(|_| MessageException::new("Protocol Error, such as a malformed packet.", 100))?;
        self.description = Some(ben.get_array(self.get_type().inner_key()).unwrap().get_string(1).map_err(|_| MessageException::new("Protocol Error, such as a malformed packet.", 100))?.to_string());

        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
