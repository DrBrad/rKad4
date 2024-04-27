use std::net::{SocketAddr, UdpSocket};
use std::thread;
use std::time::Duration;

pub struct Server {
    server: Option<UdpSocket>
}

impl Server {

    const TID_LENGTH: usize = 6;

    //WE CANNOT HOLD THE KADEMLIA... THIS SHOULD BE FUN TO DEAL WITH...

    pub fn new() -> Self {
        Self {
            server: None
        }
    }

    pub fn start(&mut self, port: u16) {
        //START 2 THREADS - A will be packet receiver - B will be packet poller - Update Java one back to this method...
        self.server = Some(UdpSocket::bind(SocketAddr::from(([127, 0, 0, 1], 0))).unwrap());
        println!("Socket bound to {:?}", self.server.as_ref().unwrap().local_addr());

        let handle = thread::spawn(|| {
            for i in 1..=5 {
                println!("Hello from spawned thread! Count: {}", i);
                thread::sleep(Duration::from_secs(1));
            }
        });

        handle.join().unwrap();
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