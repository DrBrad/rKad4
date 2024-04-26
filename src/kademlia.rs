use crate::kad::kademlia_base::KademliaBase;
use crate::routing::inter::routing_table::RoutingTable;
use crate::routing::kb::k_routing_table::KRoutingTable;

pub struct Kademlia {
    pub(crate) routing_table: Box<dyn RoutingTable>
}

impl Kademlia {

    pub fn new() -> Self {
        Self {
            routing_table: Box::new(KRoutingTable::new())
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