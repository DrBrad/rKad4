mod utils;
mod messages;
mod routing;
mod kad;
mod kademlia;

use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use bencode::variables::bencode_object::BencodeObject;
use bencode::variables::inter::bencode_variable::BencodeVariable;
use crate::kad::kademlia_base::KademliaBase;
use crate::kademlia::Kademlia;
use crate::messages::find_node_request::FindNodeRequest;
//use crate::messages::ping_request::PingRequest;
//use crate::messages::find_node_response::FindNodeResponse;
use crate::messages::inter::message_base::MessageBase;
use crate::messages::inter::message_key::MessageKey;
use crate::messages::inter::message_type::MessageType;
use crate::messages::inter::method_message_base::MethodMessageBase;
use crate::messages::ping_request::PingRequest;
use crate::messages::ping_response::PingResponse;
//use crate::messages::ping_request::PingRequest;
use crate::refresh::refresh_handler::RefreshHandler;
use crate::refresh::tasks::bucket_refresh_task::BucketRefreshTask;
use crate::refresh::tasks::stale_refresh_task::StaleRefreshTask;
use crate::utils::hash::crc32c::CRC32c;
use crate::routing::inter::routing_table::RoutingTable;
use crate::routing::kb::k_bucket::KBucket;
use crate::routing::kb::k_routing_table::KRoutingTable;
use crate::rpc::events::inter::message_event::MessageEvent;
use crate::rpc::request_listener::RequestCallback;
use crate::utils::net::address_type::AddressType;
use crate::utils::net::address_utils::{pack_address, unpack_address};
use crate::utils::uid::UID;
use crate::utils::node::Node;
use crate::utils::node_utils::pack_nodes;
//use rand::{Rng, thread_rng};
extern crate bencode;

//mod test2;
mod refresh;
mod rpc;

//mod test2;

//MessageTypes must be a trait...

//MAYBE MAKE ROUTING TABLE A BASE SET - IE ABSTRACT - NOT TRAIT
//echo -n "hello" >/dev/udp/localhost/8080
//netcat -ul 8080

//FIX ERROR HANDLING WITH THIS AND BENCODE SIDE - No panic - dont just unwrap - CHECK
//REGISTER MESSAGE TYPES...

//Java version - register message as MethodMessageBase not MessageBase...

//GENERATE RANDOM TIDS...


fn main() {
    let kad = Kademlia::new();
    kad.get_routing_table().lock().unwrap().set_secure(false);
    kad.get_server().lock().unwrap().register_message(|| Box::new(PingRequest::default()));
    kad.get_server().lock().unwrap().register_message(|| Box::new(FindNodeRequest::default()));


    let ping_callback: RequestCallback = |event| {
        println!("{}", event.get_message().to_string());

        let mut response = PingResponse::default();
        response.set_transaction_id(*event.get_message().get_transaction_id());
        response.set_destination(event.get_message().get_origin());
        response.set_public(event.get_message().get_origin());
        event.set_response(Box::new(response));
    };


    let find_node_callback: RequestCallback = |event| {
        println!("{}", event.get_message().to_string());
    };

    kad.get_server().lock().unwrap().register_request_listener("ping", ping_callback);
    kad.get_server().lock().unwrap().register_request_listener("find_node", find_node_callback);


    kad.bind(8080);
    println!("{}", kad.get_routing_table().lock().unwrap().get_derived_uid().to_string());
    sleep(Duration::from_secs(5));
    println!("{}", kad.get_routing_table().lock().unwrap().all_nodes().len());
    sleep(Duration::from_secs(30));
}
