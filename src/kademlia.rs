use std::cell::RefCell;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::kad::kademlia_base::KademliaBase;
use crate::kad::server::Server;
use crate::refresh::refresh_handler::RefreshHandler;
use crate::refresh::tasks::bucket_refresh_task::BucketRefreshTask;
use crate::refresh::tasks::stale_refresh_task::StaleRefreshTask;
use crate::routing::bucket_types::BucketTypes;
use crate::routing::inter::routing_table::RoutingTable;
use crate::routing::kb::k_routing_table::KRoutingTable;

#[derive(Clone)]
pub struct Kademlia {
    pub routing_table: Arc<Mutex<dyn RoutingTable>>,
    pub server: Arc<Mutex<Server>>,
    pub refresh: Arc<Mutex<RefreshHandler>>
}

impl Kademlia {

    pub fn new() -> Self {
        let refresh = Arc::new(Mutex::new(RefreshHandler::new()));
        refresh.lock().unwrap().add_operation(Box::new(BucketRefreshTask::new()));
        refresh.lock().unwrap().add_operation(Box::new(StaleRefreshTask::new()));

        Self {
            routing_table: Arc::new(Mutex::new(KRoutingTable::new())),
            server: Arc::new(Mutex::new(Server::new())),
            refresh
        }
    }
}

impl From<String> for Kademlia {

    fn from(value: String) -> Self {
        let refresh = Arc::new(Mutex::new(RefreshHandler::new()));
        refresh.lock().unwrap().add_operation(Box::new(BucketRefreshTask::new()));
        refresh.lock().unwrap().add_operation(Box::new(StaleRefreshTask::new()));

        Self {
            routing_table: BucketTypes::from_string(value).unwrap().routing_table(),
            server: Arc::new(Mutex::new(Server::new())),
            refresh
        }
    }
}

impl KademliaBase for Kademlia {

    fn bind(&self, port: u16) {
        self.server.lock().unwrap().start(Box::new(self.clone()), port);
        self.refresh.lock().unwrap().start(Box::new(self.clone()));
    }

    fn join(&self, local_port: u16, addr: SocketAddr) {
        todo!()
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
}
