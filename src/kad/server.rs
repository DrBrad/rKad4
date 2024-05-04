use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::sleep;
use std::time::Duration;
use bencode::variables::bencode_object::BencodeObject;
use bencode::variables::inter::bencode_variable::BencodeVariable;
use crate::kad::kademlia_base::KademliaBase;
use crate::kademlia::Kademlia;
use crate::messages::inter::message_base::{MessageBase, TID_KEY};
use crate::messages::inter::message_type::{MessageType, TYPE_KEY};
use crate::utils;

const TID_LENGTH: usize = 6;

pub struct Server {
    kademlia: Box<dyn KademliaBase>,
    server: Option<Arc<Mutex<UdpSocket>>>,
    running: Arc<AtomicBool> //MAY NOT BE NEEDED
}

impl Server {

    pub fn new(kademlia: Box<dyn KademliaBase>) -> Self {
        Self {
            kademlia,
            server: None,
            running: Arc::new(AtomicBool::new(false)) //MAY NOT BE NEEDED
        }
    }

    pub fn start(&mut self, port: u16) {
        self.running.store(true, Ordering::Relaxed);
        let running = Arc::clone(&self.running);
        let kademlia = self.kademlia.clone();

        /*
        let handle = thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                println!("{}", kademlia.get_routing_table().lock().unwrap().get_derived_uid().to_string());
                sleep(Duration::from_secs(1));
            }
        });
        */


        self.server = Some(Arc::new(Mutex::new(UdpSocket::bind("127.0.0.1:8080").expect("Failed to bind socket"))));
        let (tx, rx) = channel();
        let sender = tx.clone();
        let server = Arc::clone(self.server.as_ref().unwrap());

        let receiver_handle = thread::spawn(move || {
            let mut buf = [0u8; 65535];
            //let mut packet = [0u8; 65535];

            loop {
                //let mut buf = [0u8; 65535];

                let (size, src_addr) = {
                    server.lock().unwrap().recv_from(&mut buf).expect("Failed to receive message")
                };

                //packet[..size].copy_from_slice(&buf[..size]);
                //let packet = buf[..size].to_vec();

                let packet = Packet::new(buf[..size].to_vec(), src_addr);

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
                        //let message = String::from_utf8_lossy(&packet);
                        //println!("Received message '{}' from {}", message, src_addr);

                        //self.on_receive(&packet.as_slice());

                    }
                    Err(_) => break, // Break the loop if the channel is closed
                }
            }
        });


        // Join the threads
        //receiver_handle.join().expect("Receiver thread panicked");
        //handler_handle.join().expect("Handler thread panicked");
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    //REGISTER MESSAGES...

    pub fn is_running(&self) -> bool {
        false
    }

    pub fn on_receive(&self, packet: Packet) {
        //WE ALSO NEED ADDRESS...
        //if(AddressUtils.isBogon(packet.getAddress(), packet.getPort())){
        //    return;
        //}


        let ben = BencodeObject::decode(packet.data);

        if !ben.contains_key(TID_KEY) || !ben.contains_key(TYPE_KEY) {
            //panic
            return;
        }

        let t = MessageType::from_rpc_type_name(ben.get_string(TYPE_KEY).unwrap().to_string()).unwrap();

        match t {
            MessageType::ReqMsg => {
                println!("REQ");
            },
            MessageType::RspMsg => {
                println!("RES");
            },
            MessageType::ErrMsg => {
                println!("ERR");
            }
        }
    }

    pub fn send(&self, mut message: Box<dyn MessageBase>) {
        if let Some(server) = &self.server {
            message.set_uid(self.kademlia.get_routing_table().lock().unwrap().get_derived_uid());
            server.lock().unwrap().send_to(message.encode().encode().as_slice(), message.get_destination_address()).unwrap(); //probably should return if failed to send...
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

pub struct Packet {
    data: Vec<u8>,
    src: SocketAddr
}

impl Packet {

    pub fn new(data: Vec<u8>, src: SocketAddr) -> Self {
        Self {
            data,
            src
        }
    }
}
