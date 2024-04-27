use std::net::SocketAddr;
use crate::kad::kademlia_base::KademliaBase;
use crate::kad::server::Server;
use crate::routing::bucket_types::BucketTypes;
use crate::routing::inter::routing_table::RoutingTable;

pub struct Kademlia {
    pub(crate) routing_table: Box<dyn RoutingTable>,
    //pub(crate) server: Server<'a>
}

impl Kademlia {

    pub fn new() -> Self {
        Self {
            routing_table: BucketTypes::Kademlia.routing_table(),
            //server: None
        }
    }
}

impl From<String> for Kademlia {

    fn from(value: String) -> Self {
        Self {
            routing_table: BucketTypes::from_string(value).unwrap().routing_table(),
            //server: None
        }
    }
}

impl KademliaBase for Kademlia {

    fn bind(&mut self, port: u16) {
        let mut server = Server::new(Box::new(self));
        server.start(8080);
    }

    fn join(&self, local_port: u16, addr: SocketAddr) {
        todo!()
    }

    fn stop(&self) {
        unimplemented!()
    }

    fn get_server(&self) -> &Server {
        //&self.server
        unimplemented!()
    }

    fn get_routing_table(&self) -> Box<&dyn RoutingTable> {
        unimplemented!()
    }
}