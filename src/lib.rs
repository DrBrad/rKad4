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

#[cfg(test)]
mod tests {

    use std::net::{IpAddr, SocketAddr};
    use std::thread::sleep;
    use std::time::Duration;
    use crate::kad::kademlia_base::KademliaBase;
    use crate::kademlia::Kademlia;
    use crate::messages::inter::message_base::MessageBase;

    #[test]
    fn test() {
        let kad = Kademlia::try_from("Kademlia").unwrap();
        kad.get_routing_table().lock().unwrap().set_secure_only(false);
        kad.get_server().lock().unwrap().set_allow_bogon(true);
        //kad.bind(8080);
        kad.join(8080, SocketAddr::new(IpAddr::from([127, 0, 0, 1]), 8070));
        //kad.join(6881, SocketAddr::new(IpAddr::from("router.bittorrent.com"), 6881)); //- not sure how to use domains yet...

        loop {
            sleep(Duration::from_secs(10));
            let routing_table = kad.get_routing_table().lock().unwrap();
            println!("CONSENSUS: {}  {}  {}",
                     routing_table.get_derived_uid().to_string(),
                     routing_table.get_consensus_external_address().to_string(),
                     routing_table.all_nodes().len());
        }
    }
}
