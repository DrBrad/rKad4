use crate::kad::kademlia_base::KademliaBase;
use crate::rpc::events::request_event::RequestEvent;

pub type RequestCallback = fn(&mut dyn KademliaBase, &mut RequestEvent);
