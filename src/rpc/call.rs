use crate::messages::inter::message_base::MessageBase;
use crate::utils::node::Node;

pub struct Call<'a> {
    message: &'a dyn MessageBase,
    node: Option<Node>,
    //callback: callback
    sent_time: u64
}

impl<'a> Call<'a> {

    pub fn new(message: &'a dyn MessageBase) -> Self {
        Self {
            message,
            node: None,
            sent_time: 0
        }
    }

    pub fn get_message(&self) -> &dyn MessageBase {
        self.message
    }

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

    pub fn set_sent_time(&mut self, sent_time: u64) {
        self.sent_time = self.sent_time;
    }

    pub fn get_sent_time(&self) -> u64 {
        self.sent_time
    }

    pub fn is_stalled(&self) -> bool {
        false
        //return (now-sentTime > STALLED_TIME);
    }
}
