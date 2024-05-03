use std::sync::{Arc, Mutex};
use crate::routing::inter::routing_table::RoutingTable;
use crate::routing::kb::k_routing_table::KRoutingTable;

pub enum BucketTypes {
    MainLine,
    Kademlia
}

impl BucketTypes {

    pub fn from_string(name: String) -> Result<Self, String> {
        for value in [BucketTypes::MainLine, BucketTypes::Kademlia] {
            if value.value() == name {
                return Ok(value);
            }
        }

        Err(format!("No enum constant {}", name))
    }

    pub fn value(&self) -> String {
        match self {
            BucketTypes::MainLine => "MainLine".to_string(),
            BucketTypes::Kademlia => "Kademlia".to_string()
        }
    }

    pub fn routing_table(&self) -> Arc<Mutex<dyn RoutingTable>> {
        match self {
            BucketTypes::MainLine => unimplemented!(),
            BucketTypes::Kademlia => Arc::new(Mutex::new(KRoutingTable::new()))
        }
    }
}
