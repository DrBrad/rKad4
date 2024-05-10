use crate::messages::inter::message_base::MessageBase;
use crate::rpc::events::inter::event::Event;
use crate::rpc::events::inter::message_event::MessageEvent;
use crate::utils::node::Node;

pub struct ResponseEvent<'a> {
    prevent_default: bool,
    message: &'a dyn MessageBase,
    node: Option<Node>,
    received_time: u64,
    sent_time: u64,
    request: Option<Box<dyn MessageBase>>
}

impl<'a> ResponseEvent<'a> {

    pub fn new(message: &'a dyn MessageBase) -> Self {
        Self {
            prevent_default: false,
            message,
            node: None,
            received_time: 0,
            sent_time: 0,
            request: None
        }
    }

    pub fn has_request(&self) -> bool {
        self.request.is_some()
    }

    pub fn get_request(&mut self) -> Result<&mut dyn MessageBase, String> {
        match self.request {
            Some(ref mut response) => Ok(response.as_mut()),
            None => Err("No response was set.".to_string())
        }
    }

    pub fn set_request(&mut self, message: Box<dyn MessageBase>) {
        self.request = Some(message);
    }
}

impl<'a> Event for ResponseEvent<'a> {

    fn is_prevent_default(&self) -> bool {
        self.prevent_default
    }

    fn prevent_default(&mut self) {
        self.prevent_default = true;
    }
}

impl<'a> MessageEvent for ResponseEvent<'a> {

    fn get_message(&self) -> &dyn MessageBase {
        self.message
        // self.response.as_ref().unwrap()
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