use std::net::SocketAddr;

pub trait KademliaBase {

    fn bind(&self, port: u16);

    fn join(&self, local_port: u16, addr: SocketAddr);

    fn stop(&self);
}
