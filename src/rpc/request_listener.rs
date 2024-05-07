use crate::messages::inter::message_base::MessageBase;
/*
pub trait RequestListener {

    fn on_request(&self);
}
*/
pub type RequestCallback = fn(Box<dyn MessageBase>);
