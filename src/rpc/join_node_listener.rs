use std::time::{SystemTime, UNIX_EPOCH};
use crate::kad::kademlia_base::KademliaBase;
use crate::messages::find_node_response::FindNodeResponse;
use crate::rpc::events::error_response_event::ErrorResponseEvent;
use crate::rpc::events::inter::message_event::MessageEvent;
use crate::rpc::events::inter::response_callback::ResponseCallback;
use crate::rpc::events::response_event::ResponseEvent;
use crate::rpc::events::stalled_event::StalledEvent;
use crate::rpc::ping_response_listener::PingResponseListener;

pub struct JoinNodeListener {
    kademlia: Box<dyn KademliaBase>
}

impl JoinNodeListener {

    pub fn new(kademlia: &dyn KademliaBase) -> Self {
        Self {
            kademlia: kademlia.clone_dyn()
        }
    }
}

impl ResponseCallback for JoinNodeListener {

    fn on_response(&self, event: ResponseEvent) {
        self.kademlia.get_routing_table().lock().unwrap().insert(event.get_node());
        println!("JOINED {}", event.get_node().to_string());

        let response = event.get_message().as_any().downcast_ref::<FindNodeResponse>().unwrap();

        if response.has_nodes() {
            let nodes = response.get_all_nodes();

            let ping_response_listener = PingResponseListener::new(/*kademlia.get_routing_table()*/);

            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();//current time in millis
            for node in nodes {

            }
        }

        println!("RES  {}", event.get_message().to_string());
    }

    fn on_error_response(&self, event: ErrorResponseEvent) {

    }

    fn on_stalled(&self, event: StalledEvent) {

    }
}
