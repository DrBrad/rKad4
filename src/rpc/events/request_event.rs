use crate::messages::inter::message_base::MessageBase;
use crate::rpc::events::inter::event::Event;
use crate::rpc::events::inter::message_event::MessageEvent;
use crate::utils::node::Node;

pub struct RequestEvent {
    prevent_default: bool,
    message: Box<dyn MessageBase>,
    node: Option<Node>,
    received_time: u64,
    response: Option<Box<dyn MessageBase>>
}

impl RequestEvent {

    pub fn new(message: Box<dyn MessageBase>) -> Self {
        Self {
            prevent_default: false,
            message,
            node: None,
            received_time: 0,
            response: None
        }
    }

    pub fn has_response(&self) -> bool {
        false
    }

    pub fn get_response(&self) -> &Box<dyn MessageBase> {
        self.response.as_ref().unwrap()
    }

    pub fn set_response(&mut self, message: Box<dyn MessageBase>) {
        self.response = Some(message);
    }
}

impl Event for RequestEvent {

    fn is_prevent_default(&self) -> bool {
        self.prevent_default
    }

    fn prevent_default(&mut self) {
        self.prevent_default = true;
    }
}

impl MessageEvent for RequestEvent {

    fn get_message(&self) -> &Box<dyn MessageBase> {
        self.response.as_ref().unwrap()
    }

    fn has_node(&self) -> bool {
        self.node.is_some()
    }

    fn set_node(&mut self, node: Node) {
        self.node = Some(node);
    }

    fn get_node(&self) -> Node {
        self.node.unwrap()
    }

    fn set_received_time(&mut self, received_time: u64) {
        self.received_time = received_time;
    }

    fn get_received_time(&self) -> u64 {
        self.received_time
    }

    fn received(&mut self) {
        todo!()
    }
}
