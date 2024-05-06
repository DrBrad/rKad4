use crate::messages::inter::message_base::MessageBase;
use crate::rpc::events::inter::event::Event;
use crate::rpc::events::inter::message_event::MessageEvent;

pub struct RequestEvent {

}

impl RequestEvent {

}

impl Event for RequestEvent {
    fn is_prevent_default() -> bool {
        todo!()
    }

    fn prevent_default() {
        todo!()
    }
}

impl MessageEvent for RequestEvent {

    fn get_message(&self) -> Box<dyn MessageBase> {
        todo!()
    }
}
