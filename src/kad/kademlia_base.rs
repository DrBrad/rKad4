use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use crate::kad::server::Server;
use crate::routing::inter::routing_table::RoutingTable;
use crate::routing::kb::k_routing_table::KRoutingTable;

pub trait KademliaBase {

    fn bind(&mut self, port: u16);

    fn join(&self, local_port: u16, addr: SocketAddr);

    fn stop(&self);

    fn get_server(&self) -> &Server;

    //fn get_settings(&self) -> &Settings;
    //fn get_routing_table(&self) -> &Box<dyn RoutingTable>;
}


pub struct Settings {
    //routing_table: Box<dyn RoutingTable>,
    //routing_table: Arc<Mutex<dyn RoutingTable>>,
    server: Server
}

impl Settings {

    pub fn new() -> Self {
        Self {
            //routing_table: Box::new(KRoutingTable::new()),
            //routing_table: Arc::new(Mutex::new(KRoutingTable::new())),
            server: Server::new()
        }
    }

    /*
    pub fn get_routing_table(&self) -> &Arc<Mutex<dyn RoutingTable>> {
        &self.routing_table
    }
    */

    pub fn get_server(&self) -> &Server {
        &self.server
    }
}


