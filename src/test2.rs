use std::net::UdpSocket;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

pub fn test() {

    let kad = Kademlia::new();
    let arc = kad.arc();


    //let arc = Arc::new(Mutex::new(Kademlia::new()));


    let clone = Arc::clone(&arc);

    let handle = thread::spawn(move || run(clone));

    println!("{}", arc.lock().unwrap().to_string());

    handle.join().unwrap();

    //thread::spawn(|_|, run());


    /*
    let kad = Kademlia::new();
    let kad = kad.start();

    println!("{}", kad.lock().unwrap().to_string());

    while true {

    }
    */

    /*
    // Create a UDP socket
    let socket = UdpSocket::bind("127.0.0.1:8080").expect("Failed to bind socket");

    // Create a shared Arc<Mutex<UdpSocket>>
    let socket = Arc::new(Mutex::new(socket));
    test(&socket);
    */


    /*

    println!("Hello, world!");

    let mut kademlia = Kademlia::new();
    kademlia.lock().unwrap().server.start(&kademlia);


    while true {
        println!("Looper {}", kademlia.lock().unwrap().to_string());
        sleep(Duration::from_millis(1000));
    }
    */

    /*
    handle.join().unwrap();
    */
}


pub fn run(arc: Arc<Mutex<dyn KademliaBase>>) {
    let mut i = 0;

    while true {
        println!("TEST  {}", arc.lock().unwrap().to_string());
        sleep(Duration::from_secs(1));

        arc.lock().unwrap().set_message(format!("blank {}", i));
        i += 1;
    }
}

pub trait KademliaBase {

    fn to_string(&self) -> String;

    fn set_message(&mut self, msg: String);
}

pub struct Kademlia {
    msg: String
}

impl Kademlia {

    pub fn new() -> Self {
        Self {
            msg: "another test".to_string()
        }
    }

    pub fn arc(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }
}

impl KademliaBase for Kademlia {

    fn set_message(&mut self, msg: String) {
        self.msg = msg;
    }

    fn to_string(&self) -> String {
        self.msg.clone()
    }
}


/*
pub struct Kademlia {
    server: Server
}

impl Kademlia {

    pub fn new() -> Self {
        Self {
            server: Server::new()
        }
    }

    pub fn start(self) -> Arc<Mutex<Kademlia>> {
        //let server = &self.server;
        let kad = Arc::new(Mutex::new(self));
        //server.start(Arc::clone(&kad));
        kad.lock().unwrap().server.start(Arc::clone(&kad));
        kad
    }

    pub fn to_string(&self) -> String {
        "asdasd".to_string()
    }
}

pub struct Server {
    server: Arc<Mutex<UdpSocket>>
}

impl Server {

    pub fn new() -> Self {
        let server = UdpSocket::bind("127.0.0.1:8080").expect("Failed to bind socket");
        let server = Arc::new(Mutex::new(server));
        Self {
            server
        }
    }

    pub fn start(&self, kad: Arc<Mutex<Kademlia>>) {
        //let socket = UdpSocket::bind("127.0.0.1:8081").expect("Failed to bind socket");
        //let socket = Arc::new(Mutex::new(socket));

        // Create a channel for passing packets from receiver to handler
        let (tx, rx) = channel();

        // Clone the sender for the receiver thread
        let sender = tx.clone();

        // Clone the socket for the receiver thread
        let receiver_socket = Arc::clone(&self.server);

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
            println!("{}", kad.lock().unwrap().to_string());
            loop {
                // Receive packets from the receiver
                match rx.recv() {
                    Ok((packet, src_addr)) => {
                        // Process the received packet (e.g., parse, handle, etc.)
                        let message = String::from_utf8_lossy(&packet);
                        println!("Received message '{}' from {}  {}", message, src_addr, kad.lock().unwrap().to_string());
                    }
                    Err(_) => break, // Break the loop if the channel is closed
                }
            }
        });


        // Join the threads
        //receiver_handle.join().expect("Receiver thread panicked");
        //handler_handle.join().expect("Handler thread panicked");
    }
}
*/


