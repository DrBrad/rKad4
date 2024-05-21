use std::time::{SystemTime, UNIX_EPOCH};
use crate::kad::kademlia_base::KademliaBase;
use crate::kad::server::Server;
use crate::messages::find_node_response::FindNodeResponse;
use crate::messages::inter::message_base::MessageBase;
use crate::messages::ping_request::PingRequest;
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

    fn on_response(&self, server: &mut Server, event: ResponseEvent) {
        self.kademlia.get_routing_table().lock().unwrap().insert(event.get_node());
        println!("JOINED {}", event.get_node().to_string());

        let response = event.get_message().as_any().downcast_ref::<FindNodeResponse>().unwrap();

        if response.has_nodes() {
            let nodes = response.get_all_nodes();

            let ping_response_listener = PingResponseListener::new(self.kademlia.as_ref());

            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis();
            for node in nodes {
                if (self.kademlia.get_routing_table().lock().unwrap().is_secure_only() && !node.has_secure_id()) || node.has_queried(now) {
                    //System.out.println("SKIPPING "+now+"  "+n.getLastSeen()+"  "+n);
                    continue;
                }

                let mut req = PingRequest::default();
                req.set_destination(node.address);

                server.send_with_callback(&mut req, Box::new(ping_response_listener.clone()));
            }
        }

        if !self.kademlia.get_refresh_handler().lock().unwrap().is_running() {
            self.kademlia.get_refresh_handler().lock().unwrap().start();
        }
    }

    fn on_error_response(&self, server: &mut Server, event: ErrorResponseEvent) {

    }

    fn on_stalled(&self, event: StalledEvent) {

    }
}
