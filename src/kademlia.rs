use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use crate::kad::kademlia_base::KademliaBase;
use crate::kad::server::Server;
use crate::messages::find_node_request::FindNodeRequest;
use crate::messages::find_node_response::FindNodeResponse;
use crate::messages::inter::message_base::MessageBase;
use crate::messages::ping_request::PingRequest;
use crate::messages::ping_response::PingResponse;
use crate::refresh::refresh_handler::RefreshHandler;
use crate::refresh::tasks::bucket_refresh_task::BucketRefreshTask;
use crate::refresh::tasks::stale_refresh_task::StaleRefreshTask;
use crate::routing::bucket_types::BucketTypes;
use crate::routing::inter::routing_table::RoutingTable;
use crate::routing::kb::k_routing_table::KRoutingTable;
use crate::rpc::events::inter::event::Event;
use crate::rpc::events::inter::message_event::MessageEvent;
use crate::rpc::join_node_listener::JoinNodeListener;
use crate::rpc::request_listener::RequestCallback;

#[derive(Clone)]
pub struct Kademlia {
    routing_table: Arc<Mutex<dyn RoutingTable>>,
    server: Arc<Mutex<Server>>,
    refresh: Arc<Mutex<RefreshHandler>>
}

impl Kademlia {

    pub fn new() -> Self {
        let mut refresh = RefreshHandler::new();
        refresh.add_operation(Box::new(BucketRefreshTask::new()));
        refresh.add_operation(Box::new(StaleRefreshTask::new()));

        let mut server = Server::new();

        server.register_message(|| Box::new(PingRequest::default()));
        server.register_message(|| Box::new(PingResponse::default()));
        server.register_message(|| Box::new(FindNodeRequest::default()));
        server.register_message(|| Box::new(FindNodeResponse::default()));
        //self_.register_message(|| Box::new(FindNodeResponse::default()));

        //CAN THIS BE MOVED TO k_request_listener?
        let ping_callback: RequestCallback = |event| {
            println!("{}", event.get_message().to_string());

            let mut response = PingResponse::default();
            response.set_transaction_id(*event.get_message().get_transaction_id());
            response.set_destination(event.get_message().get_origin().unwrap());
            response.set_public(event.get_message().get_origin().unwrap());
            event.set_response(Box::new(response));
        };

        let find_node_callback: RequestCallback = |event| {
            println!("- No Response z5 error {}", event.get_message().to_string());
            //println!("{}", self_.get_routing_table().lock().unwrap().get_derived_uid().to_string());

            if event.is_prevent_default() {
                return;
            }

            let request = event.get_message().as_any().downcast_ref::<FindNodeRequest>().unwrap();

            let mut nodes = Vec::new();/*self_.kademlia.as_ref().unwrap().get_routing_table().lock().unwrap()
                    .find_closest(request.get_target().unwrap(), MAX_BUCKET_SIZE);*/
            //nodes.retain(|&x| x != event.get_node());

            if !nodes.is_empty() {
                let mut response = FindNodeResponse::default();
                response.set_destination(event.get_message().get_origin().unwrap());
                response.set_public(event.get_message().get_public().unwrap());
                response.add_nodes(nodes);
                event.set_response(Box::new(response));
            }
        };

        server.register_request_listener("ping", ping_callback);
        server.register_request_listener("find_node", find_node_callback);

        let mut self_ = Self {
            routing_table: Arc::new(Mutex::new(KRoutingTable::new())),
            server: Arc::new(Mutex::new(server)),
            refresh: Arc::new(Mutex::new(refresh))
        };

        self_.server.lock().unwrap().kademlia = Some(Box::new(self_.clone()));
        self_.refresh.lock().unwrap().kademlia = Some(Box::new(self_.clone()));

        self_
    }
}

impl From<String> for Kademlia {

    fn from(value: String) -> Self {
        let mut refresh = RefreshHandler::new();
        refresh.add_operation(Box::new(BucketRefreshTask::new()));
        refresh.add_operation(Box::new(StaleRefreshTask::new()));

        let mut server = Server::new();

        server.register_message(|| Box::new(PingRequest::default()));
        server.register_message(|| Box::new(PingResponse::default()));
        server.register_message(|| Box::new(FindNodeRequest::default()));
        server.register_message(|| Box::new(FindNodeResponse::default()));
        //self_.register_message(|| Box::new(FindNodeResponse::default()));

        /*
        //CAN THIS BE MOVED TO k_request_listener?
        let ping_callback: RequestCallback = |event| {
            println!("{}", event.get_message().to_string());

            let mut response = PingResponse::default();
            response.set_transaction_id(*event.get_message().get_transaction_id());
            response.set_destination(event.get_message().get_origin().unwrap());
            response.set_public(event.get_message().get_origin().unwrap());
            event.set_response(Box::new(response));
        };


        let find_node_callback: RequestCallback = |event| {
            println!("- No Response z5 error {}", event.get_message().to_string());
            //println!("{}", kademlia.get_routing_table().lock().unwrap().get_derived_uid().to_string());

            if event.is_prevent_default() {
                return;
            }

            let request = event.get_message().as_any().downcast_ref::<FindNodeRequest>().unwrap();

            let mut nodes = Vec::new();/*self_.kademlia.as_ref().unwrap().get_routing_table().lock().unwrap()
                    .find_closest(request.get_target().unwrap(), MAX_BUCKET_SIZE);*/
            //nodes.retain(|&x| x != event.get_node());

            if !nodes.is_empty() {
                let mut response = FindNodeResponse::default();
                response.set_destination(event.get_message().get_origin().unwrap());
                response.set_public(event.get_message().get_public().unwrap());
                response.add_nodes(nodes);
                event.set_response(Box::new(response));
            }
        };

        server.register_request_listener("ping", ping_callback);
        server.register_request_listener("find_node", find_node_callback);
        */

        let mut self_ = Self {
            routing_table: BucketTypes::from_string(value).unwrap().routing_table(),
            server: Arc::new(Mutex::new(server)),
            refresh: Arc::new(Mutex::new(refresh))
        };

        self_.server.lock().unwrap().kademlia = Some(Box::new(self_.clone()));
        self_.refresh.lock().unwrap().kademlia = Some(Box::new(self_.clone()));

        self_
    }
}

impl KademliaBase for Kademlia {

    fn bind(&self, port: u16) {
        self.server.lock().unwrap().start(port);
    }

    fn join(&self, local_port: u16, addr: SocketAddr) {
        self.server.lock().unwrap().start(local_port);

        let mut request = FindNodeRequest::default();
        request.set_destination(addr);
        request.set_target(self.routing_table.lock().unwrap().get_derived_uid());

        //NEED TO SEND WITH CALLBACK...
        self.server.lock().unwrap().send_with_callback(&mut request, Box::new(JoinNodeListener::new(self)));
        //self.refresh.lock().unwrap().start();

        //START JOIN HANDLING
    }

    fn stop(&self) {
        self.server.lock().unwrap().stop();
        self.refresh.lock().unwrap().stop();
    }

    fn get_server(&self) -> &Arc<Mutex<Server>> {
        &self.server
    }

    fn get_routing_table(&self) -> &Arc<Mutex<dyn RoutingTable>> {
        &self.routing_table
    }

    fn get_refresh_handler(&self) -> &Arc<Mutex<RefreshHandler>> {
        &self.refresh
    }

    fn clone_dyn(&self) -> Box<dyn KademliaBase> {
        Box::new(self.clone())
    }
}
