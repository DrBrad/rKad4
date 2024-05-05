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

        //self_.server = Some(Arc::new(Mutex::new(Server::new(Box::new(self_.clone())))));

        /*
        let mut refresh = RefreshHandler::new(Box::new(self_.clone()));
        refresh.add_operation(Box::new(BucketRefreshTask::new()));
        refresh.add_operation(Box::new(StaleRefreshTask::new()));
        self_.refresh = Some(Arc::new(Mutex::new(refresh)));
        */

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

        /*
        let mut refresh = RefreshHandler::new(Box::new(self_.clone()));
        refresh.add_operation(Box::new(BucketRefreshTask::new()));
        refresh.add_operation(Box::new(StaleRefreshTask::new()));
        self_.refresh = Some(Arc::new(Mutex::new(refresh)));
        */

        self_
    }
}

impl KademliaBase for Kademlia {

    fn bind(&self, port: u16) {
        self.server.lock().unwrap().start(port);
        self.refresh.lock().unwrap().start();
    }

    fn join(&self, local_port: u16, addr: SocketAddr) {
        self.server.lock().unwrap().start(local_port);
        self.refresh.lock().unwrap().start();
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
