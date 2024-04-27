use std::net::SocketAddr;
use crate::kad::kademlia_base::KademliaBase;
use crate::kad::server::Server;
use crate::routing::bucket_types::BucketTypes;
use crate::routing::inter::routing_table::RoutingTable;

pub struct Kademlia {
    pub(crate) routing_table: Box<dyn RoutingTable>
}

impl Kademlia {

    pub fn new() -> Self {
        Self {
            routing_table: BucketTypes::Kademlia.routing_table()
        }
    }
}

impl From<String> for Kademlia {

    fn from(value: String) -> Self {
        Self {
            routing_table: BucketTypes::from_string(value).unwrap().routing_table()
        }
    }
}

impl KademliaBase for Kademlia {

    fn bind(&self, port: u16) {
        let mut server = Server::new();
        server.start(8080);
    }

    fn join(&self, local_port: u16, addr: SocketAddr) {
        todo!()
    }

    fn stop(&self) {
        todo!()
    }
}