use crate::kad::kademlia_base::KademliaBase;
use crate::rpc::events::error_response_event::ErrorResponseEvent;
use crate::rpc::events::inter::response_callback::ResponseCallback;
use crate::rpc::events::response_event::ResponseEvent;
use crate::rpc::events::stalled_event::StalledEvent;

#[derive(Clone)]
pub struct PingResponseListener {
    kademlia: Box<dyn KademliaBase>
}

impl PingResponseListener {

    pub fn new(kademlia: &dyn KademliaBase) -> Self {
        Self {
            kademlia: kademlia.clone_dyn()
        }
    }
}

impl ResponseCallback for PingResponseListener {

    fn on_response(&self, event: ResponseEvent) {

    }

    fn on_error_response(&self, event: ErrorResponseEvent) {
        todo!()
    }

    fn on_stalled(&self, event: StalledEvent) {
        todo!()
    }
}
