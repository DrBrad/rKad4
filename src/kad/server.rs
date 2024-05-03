use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::sleep;
use std::time::Duration;
use bencode::variables::inter::bencode_variable::BencodeVariable;
use crate::kad::kademlia_base::KademliaBase;
use crate::kademlia::Kademlia;
use crate::messages::inter::message_base::MessageBase;
use crate::utils;

const TID_LENGTH: usize = 6;

pub struct Server {
    server: Option<UdpSocket>,
    running: Arc<AtomicBool> //MAY NOT BE NEEDED
}

impl Server {

    pub fn new() -> Self {
        Self {
            server: None,
            running: Arc::new(AtomicBool::new(false)) //MAY NOT BE NEEDED
        }
    }

    pub fn start(&mut self, kademlia: Box<dyn KademliaBase>, port: u16) {
        self.running.store(true, Ordering::Relaxed);
        let running = Arc::clone(&self.running);

        let handle = thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                println!("{}", kademlia.get_routing_table().lock().unwrap().get_derived_uid().to_string());
                sleep(Duration::from_secs(1));
            }
        });


        /*
        let socket = UdpSocket::bind("127.0.0.1:8080").expect("Failed to bind socket");

        // Create a shared Arc<Mutex<UdpSocket>>
        let socket = Arc::new(Mutex::new(socket));

        // Create a channel for passing packets from receiver to handler
        let (tx, rx) = channel();

        // Clone the sender for the receiver thread
        let sender = tx.clone();

        // Clone the socket for the receiver thread
        let receiver_socket = Arc::clone(&socket);

        // Start the receiver thread
        let receiver_handle = thread::spawn(move || {
            // Create a buffer to receive messages
            let mut buf = [0; 1024];

            loop {
                // Receive a message
                let (size, src_addr) = {
                    let mut socket = receiver_socket.lock().unwrap();
                    socket.recv_from(&mut buf).expect("Failed to receive message")
                };

                // Send the received packet to the handler thread
                let packet = buf[..size].to_vec();
                sender.send((packet, src_addr)).expect("Failed to send packet to handler");
            }
        });

        // Start the handler thread
        let handler_handle = thread::spawn(move || {
            loop {
                // Receive packets from the receiver
                match rx.recv() {
                    Ok((packet, src_addr)) => {
                        // Process the received packet (e.g., parse, handle, etc.)
                        let message = String::from_utf8_lossy(&packet);
                        println!("Received message '{}' from {}", message, src_addr);
                    }
                    Err(_) => break, // Break the loop if the channel is closed
                }
            }
        });


        // Join the threads
        receiver_handle.join().expect("Receiver thread panicked");
        handler_handle.join().expect("Handler thread panicked");
        */
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    //REGISTER MESSAGES...

    pub fn is_running(&self) -> bool {
        false
    }

    pub fn on_receive(&self, packet: Vec<u8>) {

    }

    pub fn send(&self, kademlia: &Box<dyn KademliaBase>, mut message: Box<dyn MessageBase>) { //Message.... - needs to be a trait...
        if let Some(server) = &self.server {
            message.set_uid(kademlia.get_routing_table().lock().unwrap().get_derived_uid());
            server.send_to(message.encode().encode().as_slice(), message.get_destination_address().unwrap()).unwrap(); //probably should return if failed to send...
        }
    }

    pub fn generate_transaction_id(&self) -> [u8; TID_LENGTH] {
        //let random = utils::random::gen();
        [0u8; TID_LENGTH]
    }
}
/*
pub fn run(arc: Arc<Mutex<Settings>>) {
//pub fn run(kademlia: Arc<Mutex<dyn KademliaBase>>) {//sender: Sender<Vec<u8>>, receiver: Receiver<Vec<u8>>) {
    while true {
        println!("TEST");
        sleep(Duration::from_secs(1));
    }
}
*/
