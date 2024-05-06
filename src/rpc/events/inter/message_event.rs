use crate::messages::inter::message_base::MessageBase;
use crate::rpc::events::inter::event::Event;

pub trait MessageEvent: Event {

    fn get_message(&self) -> Box<dyn MessageBase>;
}
