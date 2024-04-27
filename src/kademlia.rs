use crate::kad::kademlia_base::KademliaBase;
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