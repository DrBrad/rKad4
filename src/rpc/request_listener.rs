use crate::kad::kademlia_base::KademliaBase;
use crate::rpc::events::request_event::RequestEvent;

pub type RequestCallback = Box<dyn Fn(&mut RequestEvent) + Send>;
