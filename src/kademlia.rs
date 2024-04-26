use crate::kad::kademlia_base::KademliaBase;
use crate::routing::inter::routing_table::RoutingTable;

pub struct Kademlia {
    //routing_table: RoutingTable
}

impl Kademlia {

    pub fn new() -> Self {
        Self {

        }
    }
}

impl KademliaBase for Kademlia {

    fn bind(port: u16) {
        todo!()
    }

    fn join() {
        todo!()
    }

    fn stop() {
        todo!()
    }
}