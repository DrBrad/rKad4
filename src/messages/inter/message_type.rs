pub const TYPE_KEY: &str = "y";

pub enum MessageType {
    ReqMsg,
    RspMsg,
    ErrMsg
}

impl MessageType {

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
