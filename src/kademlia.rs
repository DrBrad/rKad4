use std::cell::RefCell;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::kad::kademlia_base::KademliaBase;
use crate::kad::server::Server;
use crate::refresh::refresh_handler::RefreshHandler;
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
        Self {
            routing_table: Arc::new(Mutex::new(KRoutingTable::new())),
            server: Arc::new(Mutex::new(Server::new())),
            refresh: Arc::new(Mutex::new(RefreshHandler::new()))
        }
    }
}

impl From<String> for Kademlia {

    fn from(value: String) -> Self {
        Self {
            routing_table: BucketTypes::from_string(value).unwrap().routing_table(),
            server: Arc::new(Mutex::new(Server::new())),
            refresh: Arc::new(Mutex::new(RefreshHandler::new()))
        }
    }
}

impl KademliaBase for Kademlia {

    fn bind(&self, port: u16) {

        self.server.lock().unwrap().start(Box::new(self.clone()), port);

        //let mut server = Server::new();//Box::new(self));
        //let b: Box<&mut dyn KademliaBase> = Box::new(self);
        //let handle = thread::spawn(move || Server::run());
        //handle.join().unwrap();
        //self.server.start(kad, port);

        //let clone = Arc::clone(&kad);
        //let handle = thread::spawn(move || run(clone));
        //handle.join().unwrap();
        //let clone = Arc::clone(&self.settings);
        //let handle = thread::spawn(move || run(clone));
        //handle.join().unwrap();

    }

    fn join(&self, local_port: u16, addr: SocketAddr) {
        todo!()
    }

    fn stop(&self) {
        self.server.lock().unwrap().stop();
    }

    fn get_server(&self) -> &Server {
        //&self.server
        unimplemented!()
    }

    fn get_routing_table(&self) -> &Arc<Mutex<dyn RoutingTable>> {
        &self.routing_table
    }
}
