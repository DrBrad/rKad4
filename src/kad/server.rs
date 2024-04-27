use std::net::{SocketAddr, UdpSocket};

pub struct Server {

}

impl Server {

    const TID_LENGTH: usize = 6;

    //WE CANNOT HOLD THE KADEMLIA... THIS SHOULD BE FUN TO DEAL WITH...

    pub fn new() -> Self {
        Self {

        }
    }

    pub fn start(&self, port: u16) {
        //START 2 THREADS - A will be packet receiver - B will be packet poller - Update Java one back to this method...
        let socket = UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], 0)))?;




    }

    pub fn stop(&self) {

    }

    //REGISTER MESSAGES...

    pub fn is_running(&self) -> bool {
        false
    }

    pub fn on_receive(&self) {

    }

    pub fn send(&self) {

    }

    pub fn generate_transaction_id(&self) {

    }
}