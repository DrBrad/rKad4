pub mod utils;
pub mod messages;
pub mod routing;
pub mod kad;
pub mod kademlia;
pub mod refresh;
pub mod rpc;
extern crate bencode;

//MAYBE MAKE ROUTING TABLE A BASE SET - IE ABSTRACT - NOT TRAIT
//echo -n "hello" >/dev/udp/localhost/8080
//netcat -ul 8080
//test --package kad4 --lib tests -- --nocapture

/*
TODO
-----

[x] FindNodeResponse
[x] unpack_nodes
[x] Join Node Listener
[x] sending messages with server - node specification...
[x] Ping Response Listener
[x] KRequestListener
[x] Bucket Refresh
[x] Stale Refresh
[ ] onReceive Error messages
*/

#[cfg(test)]
mod tests {

    use std::net::{IpAddr, SocketAddr};
    use std::thread::sleep;
    use std::time::Duration;
    use crate::kad::kademlia_base::KademliaBase;
    use crate::kademlia::Kademlia;

    #[test]
    fn test() {
        println!("TEST");
        let kad = Kademlia::new();
        kad.get_routing_table().lock().unwrap().set_secure_only(false);
        //kad.bind(8080);
        kad.join(8080, SocketAddr::new(IpAddr::from([127, 0, 0, 1]), 8070));
        //kad.get_refresh_handler().lock().unwrap().start();
        //kad.bind(8080);
        println!("{}", kad.get_routing_table().lock().unwrap().get_derived_uid().to_string());
        sleep(Duration::from_secs(5));
        println!("{}", kad.get_routing_table().lock().unwrap().all_nodes().len());
        sleep(Duration::from_secs(30));
    }
}
