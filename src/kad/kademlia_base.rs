use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use crate::kad::server::Server;
use crate::routing::inter::routing_table::RoutingTable;

pub trait KademliaBase {

    fn bind(&mut self, kad: &Arc<Mutex<dyn KademliaBase>>, port: u16);

    fn join(&self, local_port: u16, addr: SocketAddr);

    fn stop(&self);

    fn get_server(&self) -> &Server;

    //fn get_routing_table(&self) -> &Box<dyn RoutingTable>;
}
