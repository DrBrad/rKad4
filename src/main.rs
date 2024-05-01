mod utils;
mod messages;
mod routing;
mod kad;
mod kademlia;

use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use bencode::variables::bencode_object::BencodeObject;
use bencode::variables::inter::bencode_variable::BencodeVariable;
use crate::kad::kademlia_base::KademliaBase;
use crate::kad::server::run;
use crate::kademlia::Kademlia;
use crate::messages::find_node_request::FindNodeRequest;
use crate::messages::find_node_response::FindNodeResponse;
use crate::messages::inter::message_base::MessageBase;
use crate::messages::inter::method_message_base::MethodMessageBase;
use crate::messages::ping_request::PingRequest;
use crate::refresh::refresh_handler::RefreshHandler;
use crate::refresh::tasks::bucket_refresh_task::BucketRefreshTask;
use crate::refresh::tasks::stale_refresh_task::StaleRefreshTask;
use crate::utils::hash::crc32c::CRC32c;
use crate::routing::inter::routing_table::RoutingTable;
use crate::routing::kb::k_bucket::KBucket;
use crate::routing::kb::k_routing_table::KRoutingTable;
use crate::utils::net::address_type::AddressType;
use crate::utils::net::address_utils::{pack_address, unpack_addr};
use crate::utils::uid::UID;
use crate::utils::node::Node;
use crate::utils::node_utils::pack_nodes;
//use rand::{Rng, thread_rng};
extern crate bencode;

mod test2;
mod refresh;

//mod test2;

//MessageTypes must be a trait...

//MAYBE MAKE ROUTING TABLE A BASE SET - IE ABSTRACT - NOT TRAIT
//echo -n "hello" >/dev/udp/localhost/8080


fn main() {
    //test2::test();

    let mut refresh = RefreshHandler::new();
    refresh.add_operation(Box::new(BucketRefreshTask::new()));
    refresh.add_operation(Box::new(StaleRefreshTask::new()));
    refresh.start();

    sleep(Duration::from_secs(5));
    refresh.stop();
    sleep(Duration::from_secs(3));

    //let mut kad = Kademlia::new();//Arc::new(Mutex::new(Kademlia::new()));

    //kad.bind(8080);
    //let kad = Kademlia {};

    //kad.lock().unwrap().bind(Arc::clone(&kad), 8080);
    //let clone = Arc::clone(&kad);
    //let handle = thread::spawn(move || run(clone));

    //handle.join().unwrap();


    //kad.lock().unwrap().bind(&kad, 1080);

    //let clone = Arc::clone(&kad);
    //let handle = thread::spawn(move || run(clone));
    //let clone = Arc::clone(&kad);
    //let handle = thread::spawn(move || crate::kad::server::run(clone));
    //handle.join().unwrap();

    //kad.lock().unwrap().stop();




    /*
    let node = Node::new(UID::from("e5af5f5134c1e664b6f8260e9d99d7a8719254c3"), SocketAddr::from(([127, 2, 0, 1], 1080)));
    kad.routing_table.insert(node);

    println!("{}", kad.routing_table.all_nodes().len());

    let tid = [ 0u8, 0u8, 0u8, 0u8, 0u8, 0u8 ];
    let mut response = FindNodeResponse::new(tid);
    //response.target = Some(UID::from("e5af5f5134c1e664b6f8260e9d99d7a8719254c3"));
    response.base.base.destination = Some(SocketAddr::from(([127, 2, 0, 1], 1080)));
    response.base.base.public_address = Some(SocketAddr::from(([127, 2, 0, 1], 1080)));
    response.base.base.uid = Some(UID::from("6a677a188b9c209021eb185ed0c9d44a1347f1bb"));

    let ben = response.encode();
    println!("{}", ben.to_string());


    let mut res = FindNodeResponse::new(tid);
    res.decode(&ben);

    println!("{}", res.encode().to_string());




    let tid = [ 0u8, 0u8, 0u8, 0u8, 0u8, 0u8 ];
    let mut request = FindNodeRequest::new(tid);
    request.target = Some(UID::from("e5af5f5134c1e664b6f8260e9d99d7a8719254c3"));
    request.base.base.destination = Some(SocketAddr::from(([127, 2, 0, 1], 1080)));
    request.base.base.uid = Some(UID::from("6a677a188b9c209021eb185ed0c9d44a1347f1bb"));

    let ben = request.encode();
    println!("{}", ben.to_string());


    let mut req = FindNodeRequest::new(tid);
    req.decode(&ben);

    println!("{}", req.encode().to_string());
    */








    //DECODE CHECK
    /*
    let encoded = ben.encode();
    let decoded = BencodeObject::decode(encoded.as_slice());
    println!("{}", decoded.to_string());
    */

    //println!("{}", request.encode().to_string());



    /*
    let mut routing_table: KRoutingTable = KRoutingTable::new();
    routing_table.secure_only = false;

    let node = Node::new(UID::from("e5af5f5134c1e664b6f8260e9d99d7a8719254c3"), SocketAddr::from(([127, 2, 0, 1], 1080)));
    routing_table.insert(node);

    let node2 = Node::new(UID::from("e5af5f5134c1e664b6f8260e9d99d7a8719254c8"), SocketAddr::from(([127, 0, 1, 1], 1080)));
    routing_table.insert(node2);

    let node3 = Node::new(UID::from("6a677a188b9c209021eb185ed0c9d44a1347f1bb"), SocketAddr::from(([139, 135, 64, 57], 1080)));
    routing_table.insert(node3);

    let nodes = routing_table.all_nodes();
    println!("{}", nodes.len());
    println!("{}", node3.has_secure_id());

    let nodes = routing_table.find_closest(&node3.uid, 0);
    println!("{}", nodes.len());

    println!("{}", routing_table.uid.unwrap().to_string());
    */


    //let closest = routing_table.find_closest(&UID::from("e5af5f5134c1e664b6f8260e9d99d7a8719254c8"), 3);

    //println!("{}", closest.len());
    //println!("{}", closest.len());


    /*

    //let node = Node::new(UID::from("e5af5f5134c1e664b6f8260e9d99d7a8719254c7"), SocketAddr::from(([127, 0, 0, 1], 1080)));
    //println!("{}", node.to_string());


    let node = Node::new(UID::from("e5af5f5134c1e664b6f8260e9d99d7a8719254c8"), SocketAddr::from(([127, 0, 0, 1], 1080)));
    routing_table.insert(node);
    println!("{}", node.to_string());

    let node = Node::new(UID::from("e5af5f5134c1e664b6f8260e9d99d7a8719254c7"), SocketAddr::from(([127, 0, 0, 1], 1080)));
    routing_table.insert(node);

    //let size: usize = routing_table.bucket_size(3);
    println!("{}", routing_table.bucket_uid(&node.uid));
    //println!("{}", node.uid.distance(&node2.uid));
    */

    //let mut bucket = KBucket::new();

    /*
    bucket.insert(Node::new(UID::from("e5af5f5134c1e664b6f8260e9d99d7a8719254c7"), SocketAddr::from(([127, 0, 0, 1], 1080))));

    bucket.insert(Node::new(UID::from("e5af5f5134c1e664b6f8260e9d99d7a8719254c7"), SocketAddr::from(([127, 0, 0, 1], 1080))));
    println!("{}", bucket.nodes.len());
    bucket.insert(Node::new(UID::from("e5af5f5134c1e664b6f8260e9d99d7a8719254c8"), SocketAddr::from(([127, 0, 1, 1], 1080))));
    println!("{}", bucket.nodes.len());
    println!("{}", bucket.contains_ip(&Node::new(UID::from("e5af5f5134c1e664b6f8260e9d99d7a8719254c8"), SocketAddr::from(([127, 0, 1, 1], 1080)))));
    println!("{}", bucket.contains_uid(&Node::new(UID::from("e5af5f5134c1e664b6f8260e9d99d7a871925458"), SocketAddr::from(([127, 0, 1, 1], 1080)))));
    println!("{}", bucket.contains_uid(&Node::new(UID::from("e5af5f5134c1e664b6f8260e9d99d7a8719254c8"), SocketAddr::from(([127, 0, 1, 1], 1080)))));
    */

    //let message = PingRequest::new([0, 0, 0, 1, 0, 1]);
    //IpAddr::f

    //let message = MethodMessageBase::new();
    //message


    /*
    let my_local_ip = local_ip().unwrap();

    println!("This is my local IP address: {:?}", my_local_ip);

    let uid = crate::utils::uid::UID::new("e5af5f5134c1e664b6f8260e9d99d7a8719254c7").unwrap();
    //println!("Binary: {}", uid.get_binary());
    println!("Hex: {}", uid.to_string());

    let test = uid.generate_node_id_by_distance(10);
    println!("Test: {}", test.get_binary());
    println!("Test: {}", test.to_string());
    */
}

fn vec_u8_to_hex_string(data: &[u8]) -> String {
    let hex_chars: Vec<String> = data.iter()
        .map(|byte| format!("{:02X}", byte)) // Format each byte as a two-digit hexadecimal string
        .collect();

    hex_chars.join("") // Concatenate all hexadecimal strings into one string
}

/*

const V4_MASK: [u8; 4] = [0x03, 0x0f, 0x3f, 0xff];
const V6_MASK: [u8; 8] = [0x01, 0x03, 0x07, 0x0f, 0x1f, 0x3f, 0x7f, 0xff];

pub fn derive_uid(consensus_external_address: &[u8]) -> UID {
    let mut ip = consensus_external_address.to_owned();
    let mask = if ip.len() == 4 { &V4_MASK } else { &V6_MASK };

    for i in 0..ip.len() {
        ip[i] &= mask[i];
    }

    let rand: u8 = thread_rng().gen();
    let r = rand & 0x7;
    ip[0] |= r << 5;

    let crc = CRC32c::checksum_ieee(&ip);

    let mut bid = [0u8; 20];
    bid[0] = ((crc >> 24) & 0xFF) as u8;
    bid[1] = ((crc >> 16) & 0xFF) as u8;
    bid[2] = (((crc >> 8) & 0xF8) | (thread_rng().gen::<u8>() & 0x7)) as u8;

    for i in 3..19 {
        bid[i] = thread_rng().gen();
    }

    bid[19] = rand;

    UID::new(bid)
}
*/

