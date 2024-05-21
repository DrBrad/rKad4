use std::sync::{Arc, Mutex};
use crate::kad::kademlia_base::KademliaBase;
use crate::routing::inter::routing_table::RoutingTable;
use crate::rpc::events::error_response_event::ErrorResponseEvent;
use crate::rpc::events::inter::message_event::MessageEvent;
use crate::rpc::events::inter::response_callback::ResponseCallback;
use crate::rpc::events::response_event::ResponseEvent;
use crate::rpc::events::stalled_event::StalledEvent;

#[derive(Clone)]
pub struct PingResponseListener {
    routing_table: Arc<Mutex<dyn RoutingTable>>
}

impl PingResponseListener {

    pub fn new(routing_table: Arc<Mutex<dyn RoutingTable>>) -> Self {
        Self {
            routing_table//: kademlia.clone_dyn()
        }
    }
}

impl ResponseCallback for PingResponseListener {

    fn on_response(&self, event: ResponseEvent) {
        self.routing_table.lock().unwrap().insert(event.get_node());
    }

    fn on_error_response(&self, event: ErrorResponseEvent) {
    }

    fn on_stalled(&self, event: StalledEvent) {
        if event.has_node() {
            event.get_node().mark_stale(); //WILL THIS ACT CORRECTLY...? - THIS GOES FOR JAVA AS WELL...
        }
    }
}
