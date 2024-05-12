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
}
