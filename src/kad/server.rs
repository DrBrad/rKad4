use std::net::{SocketAddr, UdpSocket};
use std::thread;
use std::sync::mpsc::{channel, Sender};
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

        /*
        // Create a channel for communication between threads
        let (sender, receiver) = channel::<Vec<u8>>();

        // Spawn the receiver thread
        let receiver_server = self.server.as_ref().unwrap().try_clone();
        thread::spawn(move || {
            while let Ok(mut buf) = receiver_server.unwrap().recv_from(&mut [0u8; 65535]) {
                if let Some(packet) = buf {
                    if sender.send(packet).is_err() {
                        break;
                    }
                }
            }
        });

        // Spawn the processor thread
        let (sender_processor, receiver_processor) = channel::<Vec<u8>>();
        thread::spawn(move || {
            while let Ok(packet) = receiver_processor.recv() {
                // Process the received packet
                println!("Received packet: {:?}", packet);
                self.on_receive(packet);
            }
        });
        */

        //drop(server);
        //drop(receiver_sender);
        //drop(processor_sender);
        //handle.join().unwrap();
    }

    pub fn stop(&self) {
        //self.server.as_ref().unwrap().drop();
    }

    //REGISTER MESSAGES...

    pub fn is_running(&self) -> bool {
        false
    }

    pub fn on_receive(&self, packet: Vec<u8>) {

    }

    pub fn send(&self) { //Message.... - needs to be a trait...
        //self.server.as_ref().unwrap().send_to()
    }

    pub fn generate_transaction_id(&self) {

    }
}