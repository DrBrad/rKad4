mod utils;
mod messages;
mod routing;
mod kad;
mod kademlia;

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use bencode::variables::bencode_object::BencodeObject;
use bencode::variables::inter::bencode_variable::BencodeVariable;
use crate::kad::kademlia_base::KademliaBase;
use crate::kademlia::Kademlia;
use crate::messages::find_node_request::FindNodeRequest;
use crate::messages::find_node_response::FindNodeResponse;
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

/*
java FindNodeResponse
Line 47 make a new line

java Node Utils UnpackNodes

        if position + ID_LENGTH + addr_length + 2 > buf.len() {
            break;
        }

java bytewrapper change to
    @Override
    public int hashCode(){
        return Arrays.hashCode(b);
    }

TODO
-----

[x] FindNodeResponse
[x] unpack_nodes
[x] Join Node Listener
[-] sending messages with server
[ ] Ping Response Listener
[ ] Bucket Refresh
[ ] Stale Refresh
[ ] onReceive Error messages
*/

fn main() {
    let kad = Kademlia::new();
    kad.get_routing_table().lock().unwrap().set_secure_only(false);
    kad.join(8080, SocketAddr::new(IpAddr::from([127, 0, 0, 1]), 8070));
    //kad.bind(8080);
    println!("{}", kad.get_routing_table().lock().unwrap().get_derived_uid().to_string());
    sleep(Duration::from_secs(5));
    println!("{}", kad.get_routing_table().lock().unwrap().all_nodes().len());
    sleep(Duration::from_secs(30));
}
