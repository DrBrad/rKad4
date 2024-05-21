use crate::kad::kademlia_base::KademliaBase;
use crate::messages::inter::message_base::MessageBase;
use crate::messages::inter::method_message_base::MethodMessageBase;
use crate::rpc::events::inter::event::Event;
use crate::rpc::events::request_event::RequestEvent;
/*
pub trait RequestListener {

    fn on_request(&self);
}
*/
pub type RequestCallback = fn(&mut RequestEvent);
