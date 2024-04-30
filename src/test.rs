use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    // Create a UDP socket
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
}
