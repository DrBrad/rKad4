use crate::routing::bucket_types::BucketTypes;

pub const TYPE_KEY: &str = "y";

pub enum MessageType {
    ReqMsg,
    RspMsg,
    ErrMsg
}

impl MessageType {

    pub fn from_string(name: String) -> Result<Self, String> {
        for value in [MessageType::ReqMsg, MessageType::RspMsg, MessageType::ErrMsg] {
            if value.value() == name {
                return Ok(value);
            }
        }

        Err(format!("No enum constant {}", name))
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
