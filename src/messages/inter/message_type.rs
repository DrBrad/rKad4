pub const TYPE_KEY: &str = "y";

pub enum MessageType {
    REQ_MSG,
    RSP_MSG,
    ERR_MSG
}

impl MessageType {

    pub fn inner_key(&self) -> &str {
        match self {
            MessageType::REQ_MSG => "a",
            MessageType::RSP_MSG => "r",
            MessageType::ERR_MSG => "e"
        }
    }

    pub fn rpc_type_name(&self) -> &str {
        match self {
            MessageType::REQ_MSG => "q",
            MessageType::RSP_MSG => "r",
            MessageType::ERR_MSG => "e"
        }
    }
}
