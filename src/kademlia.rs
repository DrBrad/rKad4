use std::cell::RefCell;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::kad::kademlia_base::KademliaBase;
use crate::kad::server::Server;
use crate::refresh::refresh_handler::RefreshHandler;
use crate::refresh::tasks::bucket_refresh_task::BucketRefreshTask;
use crate::refresh::tasks::inter::task::Task;
use crate::refresh::tasks::stale_refresh_task::StaleRefreshTask;
use crate::routing::bucket_types::BucketTypes;
use crate::routing::inter::routing_table::RoutingTable;
use crate::routing::kb::k_routing_table::KRoutingTable;

#[derive(Clone)]
pub struct Kademlia {
    routing_table: Arc<Mutex<dyn RoutingTable>>,
    server: Option<Arc<Mutex<Server>>>,
    refresh: Option<Arc<Mutex<RefreshHandler>>>
}

impl Kademlia {

    pub fn new() -> Self {
        let mut self_ = Self {
            routing_table: Arc::new(Mutex::new(KRoutingTable::new())),
            server: None,
            refresh: None
        };

        self_.server = Some(Arc::new(Mutex::new(Server::new(Box::new(self_.clone())))));

        let mut refresh = RefreshHandler::new(Box::new(self_.clone()));
        refresh.add_operation(Box::new(BucketRefreshTask::new()));
        refresh.add_operation(Box::new(StaleRefreshTask::new()));
        self_.refresh = Some(Arc::new(Mutex::new(refresh)));

        self_
    }
}

impl From<String> for Kademlia {

    fn from(value: String) -> Self {
        let mut self_ = Self {
            routing_table: BucketTypes::from_string(value).unwrap().routing_table(),
            server: None,
            refresh: None
        };

        self_.server = Some(Arc::new(Mutex::new(Server::new(Box::new(self_.clone())))));

        let mut refresh = RefreshHandler::new(Box::new(self_.clone()));
        refresh.add_operation(Box::new(BucketRefreshTask::new()));
        refresh.add_operation(Box::new(StaleRefreshTask::new()));
        self_.refresh = Some(Arc::new(Mutex::new(refresh)));

        self_
    }
}

impl KademliaBase for Kademlia {

    fn bind(&self, port: u16) {
        self.server.as_ref().unwrap().lock().unwrap().start(port);
        self.refresh.as_ref().unwrap().lock().unwrap().start();
    }

    fn join(&self, local_port: u16, addr: SocketAddr) {
        todo!()
    }

    fn stop(&self) {
        self.server.as_ref().unwrap().lock().unwrap().stop();
        self.refresh.as_ref().unwrap().lock().unwrap().stop();
    }

    fn get_server(&self) -> &Arc<Mutex<Server>> {
        self.server.as_ref().unwrap()
    }

    fn get_routing_table(&self) -> &Arc<Mutex<dyn RoutingTable>> {
        &self.routing_table
    }

    fn get_refresh_handler(&self) -> &Arc<Mutex<RefreshHandler>> {
        self.refresh.as_ref().unwrap()
    }

    fn clone_dyn(&self) -> Box<dyn KademliaBase> {
        Box::new(self.clone())
    }
}
