use std::collections::HashMap;
use std::mem::forget;
use std::net::{SocketAddr, UdpSocket};
use std::slice::from_raw_parts;
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
use crate::messages::inter::message_key::MessageKey;
use crate::messages::inter::message_type::{MessageType, TYPE_KEY};
use crate::messages::inter::method_message_base::MethodMessageBase;
use crate::utils::net::address_utils::is_bogon;

const TID_LENGTH: usize = 6;

pub struct Server {
    pub(crate) kademlia: Option<Box<dyn KademliaBase>>,
    server: Option<Arc<Mutex<UdpSocket>>>,
    running: Arc<AtomicBool>, //MAY NOT BE NEEDED
    messages: HashMap<MessageKey, fn() -> Box<dyn MethodMessageBase>>
}

impl Server {

    pub fn new(/*kademlia: Box<dyn KademliaBase>*/) -> Self {
        Self {
            kademlia: None,
            server: None,
            running: Arc::new(AtomicBool::new(false)), //MAY NOT BE NEEDED
            messages: HashMap::new()
        }
    }

    pub fn start(&mut self, port: u16) {
        self.running.store(true, Ordering::Relaxed);
        let running = Arc::clone(&self.running);

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

            loop {
                let (size, src_addr) = {
                    server.lock().unwrap().recv_from(&mut buf).expect("Failed to receive message")
                };

                let data = &buf[..size];

                let bytes = data.as_ptr();
                let len = data.len();
                forget(data);

                unsafe {
                    sender.send((from_raw_parts(bytes, len), src_addr)).expect("Failed to send packet to handler");
                }
            }
        });


        let kademlia = self.kademlia.clone();

        // Start the handler thread
        let handler_handle = thread::spawn(move || {
            loop {
                // Receive packets from the receiver
                match rx.recv() {
                    Ok((data, src_addr)) => {
                        // Process the received packet (e.g., parse, handle, etc.)
                        //let message = String::from_utf8_lossy(data);
                        //println!("Received message '{}' from {}", message, src_addr);

                        kademlia.as_ref().unwrap().get_server().lock().unwrap().on_receive(data, src_addr);
                        //Server::on_receive(data, src_addr);
                        //kademlia.get_server().lock().unwrap().is_running();

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

    pub fn register_message(&mut self, constructor: fn() -> Box<dyn MethodMessageBase>) {
        let message = constructor();
        self.messages.insert(MessageKey::new(message.get_method(), message.get_type()), constructor);
    }

    pub fn is_running(&self) -> bool {
        false
    }

    pub fn on_receive(&self, data: &[u8], src_addr: SocketAddr) {
        if is_bogon(src_addr) {
            //return;
        }

        println!("RECEIVED: {}", self.kademlia.as_ref().unwrap().get_routing_table().lock().unwrap().get_derived_uid().to_string());


        match BencodeObject::decode(data) {
            Ok(ben) => {
                if !ben.contains_key(TID_KEY) || !ben.contains_key(TYPE_KEY) {
                    //panic
                    return;
                }

                let t = MessageType::from_rpc_type_name(ben.get_string(TYPE_KEY).unwrap().to_string()).unwrap();

                match t {
                    MessageType::ReqMsg => {
                        let message_key = MessageKey::new(ben.get_string(t.rpc_type_name()).expect("Failed to find valid key."), t);

                        //let message_key = ;

                        if let Some(constructor) = self.messages.get(&message_key) {
                            let mut message = constructor();
                            message.set_transaction_id([0u8; 6]);
                            message.decode(&ben);
                            message.set_origin(src_addr);
                            //message.set_transaction_id(ben.get_bytes(TID_KEY).expect("Failed to find TID"));

                            println!("MESSAGE CREATED {}", message.to_string());

                        }








                    },
                    MessageType::RspMsg => {
                        println!("RES  {}", ben.to_string());
                    },
                    MessageType::ErrMsg => {
                        println!("ERR  {}", ben.to_string());
                    }
                }
            },
            Err(e) => {
                println!("{}", e.to_string());
            }
        }

        /*
        if !ben.contains_key(TID_KEY) || !ben.contains_key(TYPE_KEY) {
            //panic
            return;
        }

        let t = MessageType::from_rpc_type_name(ben.get_string(TYPE_KEY).unwrap().to_string()).unwrap();

        match t {
            MessageType::ReqMsg => {
                let k = ben.get_string(t.rpc_type_name()).unwrap(); //PROBABLY SHOULD ERR OUT IF NO RESULT...





                println!("REQ  {}", k);
            },
            MessageType::RspMsg => {
                println!("RES  {}", ben.to_string());
            },
            MessageType::ErrMsg => {
                println!("ERR  {}", ben.to_string());
            }
        }
        */
    }

    pub fn send(&self, mut message: Box<dyn MessageBase>) {
        if let Some(server) = &self.server {
            //message.set_uid(self.kademlia.get_routing_table().lock().unwrap().get_derived_uid());
            //server.lock().unwrap().send_to(message.encode().encode().as_slice(), message.get_destination_address()).unwrap(); //probably should return if failed to send...
        }
    }

    pub fn generate_transaction_id(&self) -> [u8; TID_LENGTH] {
        //let random = utils::random::gen();
        [0u8; TID_LENGTH]
    }
}
