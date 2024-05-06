use crate::kad::kademlia_base::KademliaBase;
use crate::rpc::request_listener::RequestListener;

pub struct KRequestListener {
    kademlia: Box<dyn KademliaBase>
}

impl KRequestListener {

    pub fn new(kademlia: Box<dyn KademliaBase>) -> Self {
        Self {
            kademlia
        }
    }
}

impl RequestListener for KRequestListener {

    fn on_request(&self) {
        todo!()
    }
}
