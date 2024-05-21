use std::time::{SystemTime, UNIX_EPOCH};
use crate::messages::inter::message_base::MessageBase;
use crate::rpc::events::inter::event::Event;
use crate::rpc::events::inter::message_event::MessageEvent;
use crate::utils::node::Node;

pub struct ErrorResponseEvent<'a> {
    prevent_default: bool,
    message: &'a dyn MessageBase,
    node: Option<Node>,
    received_time: u128,
    sent_time: u128,
    request: Option<&'a dyn MessageBase>
}

impl<'a> ErrorResponseEvent<'a> {

    pub fn new(message: &'a dyn MessageBase, node: Node) -> Self {
        Self {
            prevent_default: false,
            message,
            node: Some(node),
            received_time: 0,
            sent_time: 0,
            request: None
        }
    }

    pub fn has_request(&self) -> bool {
        self.request.is_some()
    }

    pub fn get_request(&mut self) -> Result<&dyn MessageBase, String> {
        match self.request {
            Some(response) => Ok(response),
            None => Err("No response was set.".to_string())
        }
    }

    pub fn set_request(&mut self, message: &'a dyn MessageBase) {
        self.request = Some(message);
    }

    pub fn set_sent_time(&mut self, sent_time: u128) {
        self.sent_time = sent_time;
    }

    pub fn get_sent_time(&self) -> u128 {
        self.sent_time
    }
}

impl<'a> Event for ErrorResponseEvent<'a> {

    fn is_prevent_default(&self) -> bool {
        self.prevent_default
    }

    fn prevent_default(&mut self) {
        self.prevent_default = true;
    }
}

impl<'a> MessageEvent for ErrorResponseEvent<'a> {

    fn get_message(&self) -> &dyn MessageBase {
        self.message
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

    fn set_received_time(&mut self, received_time: u128) {
        self.received_time = received_time;
    }

    fn get_received_time(&self) -> u128 {
        self.received_time
    }

    fn received(&mut self) {
        self.received_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
    }
}
