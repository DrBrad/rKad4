use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::sleep;
use std::time::Duration;
use crate::kad::kademlia_base::KademliaBase;
use crate::kademlia::Kademlia;
use crate::messages::inter::message_base::MessageBase;
use crate::utils;

const TID_LENGTH: usize = 6;

pub struct Server {
    //kademlia: Box<&'a dyn KademliaBase>,
    server: Option<UdpSocket>
}

impl Server {


    //WE CANNOT HOLD THE KADEMLIA... THIS SHOULD BE FUN TO DEAL WITH...

    pub fn new(/*kademlia: Box<&'a dyn KademliaBase>*/) -> Self {
        Self {
            //kademlia,
            server: None
        }
    }

    pub fn start(&mut self, kademlia: &Arc<Mutex<dyn KademliaBase>>, port: u16) {

        let kademlia = Arc::clone(kademlia);

        //let (sender, receiver) = channel::<Vec<u8>>();
        let handle = thread::spawn(move || {
            let i = 0;
            while true {
                println!("TEST  {}", i);
                sleep(Duration::from_secs(1));
            }
        });//Self::run(600, kademlia));//Self::run(kademlia));//sender, receiver));
        //kademlia: Arc<Mutex<dyn KademliaBase>>

        println!("TEST");

        handle.join().unwrap();

        //START 2 THREADS - A will be packet receiver - B will be packet poller - Update Java one back to this method...
        //self.server = Some(UdpSocket::bind(SocketAddr::from(([127, 0, 0, 1], port))).unwrap());
        //println!("Socket bound to {:?}", self.server.as_ref().unwrap().local_addr());

        //println!("{:?}", kademlia.lock().unwrap().test());//.get_routing_table().as_ref().get_derived_uid());

        /*
        // Create a channel for communication between threads
        let (sender, receiver) = channel::<Vec<u8>>();

        // Spawn the receiver thread
        let receiver_server = self.server.as_ref().unwrap().try_clone().unwrap();
        thread::spawn(move || {
            let buf = &mut [0u8; 65535];
            while let Ok(buf) = receiver_server.recv_from(buf) {
                /*
                if let Some((size, addr)) = buf {
                    let mut packet_data = vec![0u8; size];
                    packet_data.copy_from_slice(&buf[0..size]);
                    if sender.send(packet_data).is_err() {
                        break;
                    }
                }
                */
            }
        });

        // Spawn the processor thread
        let (sender_processor, receiver_processor) = channel::<Vec<u8>>();
        let handle = thread::spawn(move || {
            while let Ok(packet) = receiver_processor.recv() {
                // Process the received packet
                println!("Received packet: {:?}", packet);
                //cloned_self.on_receive(packet);
            }
        });

        handle.join().unwrap();
        */

        //drop(server);
        //drop(receiver_sender);
        //drop(processor_sender);
        //handle.join().unwrap();
    }

    pub fn run(kademlia: Arc<Mutex<dyn KademliaBase>>) {//sender: Sender<Vec<u8>>, receiver: Receiver<Vec<u8>>) {
        while true {
            println!("TEST");
            sleep(Duration::from_secs(1));
        }
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

    pub fn send(&self, message: MessageBase) { //Message.... - needs to be a trait...
        let buf = vec![0, 0, 5, 5, 0];
        self.server.as_ref().unwrap().send_to(&buf, &message.destination.unwrap()).unwrap();
    }

    pub fn generate_transaction_id(&self) -> [u8; TID_LENGTH] {
        //let random = utils::random::gen();
        [0u8; TID_LENGTH]
    }
}