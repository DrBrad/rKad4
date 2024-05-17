use crate::messages::inter::message_base::MessageBase;
use crate::rpc::events::inter::response_callback::ResponseCallback;
use crate::rpc::response_tracker::STALLED_TIME;
use crate::utils::node::Node;

pub struct Call {
    message: Box<dyn MessageBase>,
    node: Option<Node>,
    callback: Box<dyn ResponseCallback>,
    sent_time: u128
}

impl Call {

    pub fn new(message: Box<dyn MessageBase>, callback: Box<dyn ResponseCallback>) -> Self {
        Self {
            message,
            node: None,
            callback,
            sent_time: 0
        }
    }

    /*
    pub fn get_message(&self) -> &dyn MessageBase {
        self.message
    }
    */

    pub fn has_node(&self) -> bool {
        self.node.is_some()
    }

    pub fn set_node(&mut self, node: Node) {
        self.node = Some(node);
    }

    pub fn get_node(&self) -> Node {
        self.node.unwrap()
    }

    //response callback - has, set, get

    pub fn set_sent_time(&mut self, sent_time: u128) {
        self.sent_time = sent_time;
    }

    pub fn get_sent_time(&self) -> u128 {
        self.sent_time
    }

    pub fn is_stalled(&self, now: u128) -> bool {
        now-self.sent_time > STALLED_TIME
    }
}
