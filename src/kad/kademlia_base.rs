use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use crate::kad::server::Server;
use crate::routing::inter::routing_table::RoutingTable;
use crate::routing::kb::k_routing_table::KRoutingTable;

pub trait KademliaBase: Send {

    fn bind(&self, port: u16);

    fn join(&self, local_port: u16, addr: SocketAddr);

    fn stop(&self);

    fn get_server(&self) -> &Arc<Mutex<Server>>;

    //fn get_settings(&self) -> &Settings;
    fn get_routing_table(&self) -> &Arc<Mutex<dyn RoutingTable>>;
}
