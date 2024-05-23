use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use crate::kad::server::Server;
use crate::refresh::refresh_handler::RefreshHandler;
use crate::routing::inter::routing_table::RoutingTable;

pub trait KademliaBase: Send {

    fn bind(&self, port: u16);

    fn join(&self, local_port: u16, addr: SocketAddr);

    fn stop(&self);

    fn get_server(&self) -> &Arc<Mutex<Server>>;

    fn get_routing_table(&self) -> &Arc<Mutex<dyn RoutingTable>>;

    fn get_refresh_handler(&self) -> &Arc<Mutex<RefreshHandler>>;

    fn clone_dyn(&self) -> Box<dyn KademliaBase>;
}

impl Clone for Box<dyn KademliaBase> {

    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}
