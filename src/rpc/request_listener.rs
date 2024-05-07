use crate::messages::inter::message_base::MessageBase;
use crate::messages::inter::method_message_base::MethodMessageBase;
/*
pub trait RequestListener {

    fn on_request(&self);
}
*/
pub type RequestCallback = fn(Box<dyn MethodMessageBase>);
