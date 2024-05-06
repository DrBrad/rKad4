use crate::messages::inter::message_base::MessageBase;
use crate::rpc::events::inter::event::Event;
use crate::rpc::events::inter::message_event::MessageEvent;
use crate::utils::node::Node;

pub struct RequestEvent {

}

impl RequestEvent {

    pub fn has_response(&self) -> bool {
        false
    }

    pub fn get_response(&self) -> Box<dyn MessageBase> {
        unimplemented!()
    }

    pub fn set_response(&mut self, message: Box<dyn MessageBase>) {

    }
}

impl Event for RequestEvent {
    fn is_prevent_default(&self) -> bool {
        todo!()
    }

    fn prevent_default(&mut self) {
        todo!()
    }
}

impl MessageEvent for RequestEvent {

    fn get_message(&self) -> Box<dyn MessageBase> {
        todo!()
    }

    fn has_node(&self) -> bool {
        todo!()
    }

    fn set_node(&mut self, node: Node) {
        todo!()
    }

    fn get_node(&self) -> Node {
        todo!()
    }

    fn set_received_time(&mut self, received_time: u64) {
        todo!()
    }

    fn get_received_time(&self) -> u64 {
        todo!()
    }

    fn received(&mut self) {
        todo!()
    }
}
