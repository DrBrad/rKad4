use std::net::UdpSocket;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicPtr, Ordering};
use std::sync::mpsc::channel;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

pub fn test() {

    /*
    let kad = Kademlia::new();
    let arc = kad.arc();


    //let arc = Arc::new(Mutex::new(Kademlia::new()));


    let clone = Arc::clone(&arc);

    let handle = thread::spawn(move || run(clone));

    println!("{}", arc.lock().unwrap().to_string());

    handle.join().unwrap();
    */

    let kad = Kademlia::new();
    kad.start();

    sleep(Duration::from_secs(5));
    kad.stop();

    //while true {
        
    //}

}



struct Kademlia {
    routing_table: Arc<Mutex<RoutingTable>>,
    refresh_handler: Arc<Mutex<RefreshHandler>>,
    server: Arc<Mutex<Server>>
}

impl Kademlia {

    pub fn new() -> Self {
        Self {
            routing_table: Arc::new(Mutex::new(RoutingTable::new())),
            refresh_handler: Arc::new(Mutex::new(RefreshHandler::new())),
            server: Arc::new(Mutex::new(Server::new()))
        }
    }

    pub fn start(&self) {
        //let routing_table = Arc::clone(&self.routing_table);//.lock().unwrap().routing_table;
        self.server.lock().unwrap().start(&self.routing_table);

        self.refresh_handler.lock().unwrap().start(&self.routing_table);
    }
    /*
    pub fn get_settings(&self) -> &Arc<Mutex<Settings>> {
        &self.settings
    }
    */

    pub fn stop(&self) {
        self.server.lock().unwrap().stop();
        self.refresh_handler.lock().unwrap().stop();
    }
}

/*
struct Settings {
    routing_table: Arc<Mutex<RoutingTable>>,
    server: Arc<Mutex<Server>>
}

impl Settings {

    pub fn new() -> Self {
        Self {
            routing_table: Arc::new(Mutex::new(RoutingTable::new())),
            server: Arc::new(Mutex::new(Server::new()))
        }
    }
}
*/


struct Server {
    running: Arc<AtomicBool>
}

impl Server {

    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false))
        }
    }

    pub fn start(&self, routing_table: &Arc<Mutex<RoutingTable>>) {
        self.running.store(true, Ordering::Relaxed);
        let running = Arc::clone(&self.running);

        let routing_table = Arc::clone(routing_table);

        println!("STARTING SERVER a");
        //let settings = Arc::clone(settings);
        //let routing_table = Arc::clone(routing_table);

        let handle = thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                println!("Server {}", routing_table.lock().unwrap().msg);
                sleep(Duration::from_secs(1));
            }
        });

        println!("STARTING SERVER");

        //handle.join().unwrap();
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

struct RefreshHandler {
    running: Arc<AtomicBool>
}

impl RefreshHandler {

    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false))
        }
    }

    pub fn start(&self, routing_table: &Arc<Mutex<RoutingTable>>) {
        self.running.store(true, Ordering::Relaxed);
        let running = Arc::clone(&self.running);

        let routing_table = Arc::clone(routing_table);

        println!("STARTING REFRESH a");
        //let settings = Arc::clone(settings);
        //let routing_table = Arc::clone(routing_table);

        let handle = thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                println!("Refresh {}", routing_table.lock().unwrap().msg);
                sleep(Duration::from_secs(1));
                routing_table.lock().unwrap().set_message("Refresh_Change".to_string());
            }
        });

        println!("STARTING REFRESH");

        //handle.join().unwrap();
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }
}





struct RoutingTable {
    msg: String
}

impl RoutingTable {

    pub fn new() -> Self {
        Self {
            msg: "Hello World".to_string()
        }
    }

    pub fn set_message(&mut self, msg: String) {
        self.msg = msg;
    }

    pub fn to_string(&self) -> String {
        self.msg.clone()
    }
}



/*
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
*/

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


