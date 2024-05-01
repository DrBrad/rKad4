use std::cell::RefCell;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::kad::kademlia_base::KademliaBase;
use crate::kad::server::{Server, run};
use crate::routing::bucket_types::BucketTypes;
use crate::routing::inter::routing_table::RoutingTable;

pub struct Kademlia {
    //pub(crate) routing_table: Box<dyn RoutingTable>,
    //pub(crate) server: Server//RefCell<Server>
}

impl Kademlia {

    pub fn new() -> Arc<Mutex<dyn KademliaBase>> {
        Arc::new(Mutex::new(Self {
            //routing_table: BucketTypes::Kademlia.routing_table(),
            //server: Server::new()//RefCell::new(Server::new())
        }))
    }
}

impl From<String> for Kademlia {

    fn from(value: String) -> Self {
        Self {
            //routing_table: BucketTypes::from_string(value).unwrap().routing_table(),
            //server: Server::new()//RefCell::new(Server::new())
        }
    }
}

impl KademliaBase for Kademlia {

    fn bind(&mut self, port: u16) {
        //let mut server = Server::new();//Box::new(self));
        //let b: Box<&mut dyn KademliaBase> = Box::new(self);
        //let handle = thread::spawn(move || Server::run());
        //handle.join().unwrap();
        //self.server.start(kad, port);

        let clone = Arc::clone(&kad);
        let handle = thread::spawn(move || run(clone));
        handle.join().unwrap();

    }

    fn join(&self, local_port: u16, addr: SocketAddr) {
        todo!()
    }

    fn stop(&self) {
        //self.server.borrow().stop();
    }

    fn get_server(&self) -> &Server {
        //&self.server
        unimplemented!()
    }

    /*
    fn get_routing_table(&self) -> &Box<dyn RoutingTable> {
        //&self.routing_table
        unimplemented!()
    }
    */
}
