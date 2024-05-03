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
    refresh: Arc<Mutex<RefreshHandler>>
}

impl Kademlia {

    pub fn new() -> Self {
        let mut refresh = RefreshHandler::new();
        refresh.add_operation(Box::new(BucketRefreshTask::new()));
        refresh.add_operation(Box::new(StaleRefreshTask::new()));
        let refresh = Arc::new(Mutex::new(refresh));

        let mut self_ = Self {
            routing_table: Arc::new(Mutex::new(KRoutingTable::new())),
            server: None,
            refresh
        };

        //let server = Server::new3(Box::new(self_.clone()));

        self_.server = Some(Arc::new(Mutex::new(Server::new3(Box::new(self_.clone())))));

        self_
    }
}

impl From<String> for Kademlia {

    fn from(value: String) -> Self {
        let mut refresh = RefreshHandler::new();
        refresh.add_operation(Box::new(BucketRefreshTask::new()));
        refresh.add_operation(Box::new(StaleRefreshTask::new()));
        let refresh = Arc::new(Mutex::new(refresh));

        Self {
            routing_table: BucketTypes::from_string(value).unwrap().routing_table(),
            server: None,//Arc::new(Mutex::new(Server::new())),
            refresh
        }
    }
}

impl KademliaBase for Kademlia {

    fn bind(&self, port: u16) {
        self.server.as_ref().unwrap().lock().unwrap().start(port);
        //self.server.lock().unwrap().start(Box::new(self.clone()), port);
        self.refresh.lock().unwrap().start(Box::new(self.clone()));
    }

    fn join(&self, local_port: u16, addr: SocketAddr) {
        todo!()
    }

    fn stop(&self) {
        self.server.as_ref().unwrap().lock().unwrap().stop();
        self.refresh.lock().unwrap().stop();
    }

    fn get_server(&self) -> &Arc<Mutex<Server>> {
        self.server.as_ref().unwrap()
    }

    fn get_routing_table(&self) -> &Arc<Mutex<dyn RoutingTable>> {
        &self.routing_table
    }


    fn clone_dyn(&self) -> Box<dyn KademliaBase> {
        Box::new(self.clone())
    }
}
