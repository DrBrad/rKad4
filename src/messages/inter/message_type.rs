use crate::routing::bucket_types::BucketTypes;

pub const TYPE_KEY: &str = "y";

pub enum MessageType {
    ReqMsg,
    RspMsg,
    ErrMsg
}

impl MessageType {

    pub fn from_rpc_type_name(key: String) -> Result<Self, String> {
        let key = key.to_lowercase();

        for value in [MessageType::ReqMsg, MessageType::RspMsg, MessageType::ErrMsg] {
            if value.rpc_type_name() == key {
                return Ok(value);
            }
        }

        Err(format!("No enum constant {}", key))
    }

    pub fn inner_key(&self) -> &str {
        match self {
            MessageType::ReqMsg => "a",
            MessageType::RspMsg => "r",
            MessageType::ErrMsg => "e"
        }
    }

    pub fn rpc_type_name(&self) -> &str {
        match self {
            MessageType::ReqMsg => "q",
            MessageType::RspMsg => "r",
            MessageType::ErrMsg => "e"
        }
    }
}
