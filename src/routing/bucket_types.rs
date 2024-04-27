use crate::routing::inter::routing_table::RoutingTable;
use crate::routing::kb::k_routing_table::KRoutingTable;

pub enum BucketTypes {
    MainLine,
    Kademlia
}

impl BucketTypes {

    /*
    public static BucketTypes fromString(String name){
        for(BucketTypes value : values()){
            if(value.name().equalsIgnoreCase(name)){
                return value;
            }
        }
        throw new IllegalArgumentException("No enum constant "+BucketTypes.class.getName()+"."+name);
    }
    */

    pub fn value(&self) -> String {
        match self {
            BucketTypes::MainLine => "MainLine".to_string(),
            BucketTypes::Kademlia => "Kademlia".to_string()
        }
    }

    pub fn routing_table(&self) -> Box<dyn RoutingTable> {
        match self {
            BucketTypes::MainLine => unimplemented!(),
            BucketTypes::Kademlia => Box::new(KRoutingTable::new())
        }
    }
}
