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


fn main() {

    /*
    let ping_callback = |message: Box<dyn MessageBase>| {
        println!("{}", message.to_string());
    };

    let mut map = HashMap::new();
    map.insert(MessageKey::new("find_node", MessageType::ReqMsg), &ping_callback);


    let mut request = FindNodeRequest::default();
    request.set_target(UID::try_from("e5af5f5134c1e664b6f8260e9d99d7a8719254c3").unwrap());
    request.set_destination(SocketAddr::from(([127, 2, 0, 1], 1080)));
    request.set_uid(UID::try_from("6a677a188b9c209021eb185ed0c9d44a1347f1bb").unwrap());

    let key = MessageKey::new("find_node", MessageType::ReqMsg);
    if map.contains_key(&key) {
        map.get(&key).unwrap()(Box::new(request));
    }
    */


    /*
    // Define a callback for Greeting messages
    let greeting_callback = |msg: &str| {
        println!("Received greeting: {}", msg);
    };

    // Define a callback for Farewell messages
    let farewell_callback = |msg: &str| {
        println!("Received farewell: {}", msg);
    };

    // Process messages with different callbacks
    process_message(Message::Greeting("Hello".to_string()), &greeting_callback);
    process_message(Message::Farewell("Goodbye".to_string()), &farewell_callback);
    */


    let kad = Kademlia::new();
    kad.get_routing_table().lock().unwrap().set_secure(false);
    kad.get_server().lock().unwrap().register_message(|| Box::new(PingRequest::default()));
    kad.get_server().lock().unwrap().register_message(|| Box::new(FindNodeRequest::default()));


    let ping_callback: RequestCallback = |event| {
        println!("{}", event.get_message().to_string());

        let mut response = PingResponse::default();
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
