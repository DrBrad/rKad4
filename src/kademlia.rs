use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use crate::kad::kademlia_base::KademliaBase;
use crate::kad::server::Server;
use crate::messages::find_node_request::FindNodeRequest;
use crate::messages::inter::message_base::MessageBase;
use crate::refresh::refresh_handler::RefreshHandler;
use crate::refresh::tasks::bucket_refresh_task::BucketRefreshTask;
use crate::refresh::tasks::stale_refresh_task::StaleRefreshTask;
use crate::routing::bucket_types::BucketTypes;
use crate::routing::inter::routing_table::RoutingTable;
use crate::routing::kb::k_routing_table::KRoutingTable;
use crate::rpc::join_node_listener::JoinNodeListener;

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

        let mut self_ = Self {
            routing_table: Arc::new(Mutex::new(KRoutingTable::new())),
            server: Arc::new(Mutex::new(Server::new())),
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

        let mut self_ = Self {
            routing_table: BucketTypes::from_string(value).unwrap().routing_table(),
            server: Arc::new(Mutex::new(Server::new())),
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
        self.server.lock().unwrap().send_with_callback(&mut request, Box::new(JoinNodeListener::new()));
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
