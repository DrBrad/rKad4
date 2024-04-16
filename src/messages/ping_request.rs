use super::inter::message_base::MessageBaseStruct;
use super::inter::method_message_base::MethodMessageBase;


struct PingRequest {
    //inner: MessageBaseStruct
}

impl PingRequest {

    fn new(&self) -> Self {
        Self {
        }
    }
}

impl MethodMessageBase for PingRequest {

    fn encode() -> Vec<u8> {
        todo!()
    }

    fn decode(buf: Vec<u8>) {
        todo!()
    }
}
