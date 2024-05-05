use super::message_base::MessageBase;

pub trait MethodMessageBase: MessageBase {

    fn get_method(&self) -> &str;
}
