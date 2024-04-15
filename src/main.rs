use std::net::{UdpSocket, SocketAddr, AddrParseError};

fn send_udp_message(addr: &str, message: &[u8]) -> std::io::Result<()> {
    // Create a UDP socket bound to any available local port
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // Parse the destination address
    let dest_addr: SocketAddr = match addr.parse() {
        Ok(addr) => addr,
        Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
    };

    // Send the UDP message to the destination address
    socket.send_to(message, &dest_addr)?;

    Ok(())
}

fn receive_udp_message(port: u16) -> std::io::Result<()> {
    // Create a UDP socket bound to the specified port
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", port))?;

    // Buffer to store received data
    let mut buf = [0; 1024];

    // Receive a UDP message
    let (amt, src_addr) = socket.recv_from(&mut buf)?;

    // Print the received message and the source address
    println!("Received message from {}: {}", src_addr, String::from_utf8_lossy(&buf[..amt]));

    Ok(())
}

fn main() {
    let dest_addr = "127.0.0.1:8080";
    let message = b"Hello, UDP!";

    // Send a UDP message to the specified destination address
    match send_udp_message(dest_addr, message) {
        Ok(_) => println!("Sent message to {}", dest_addr),
        Err(e) => eprintln!("Error sending message: {}", e),
    }

    // Receive UDP messages on the specified port
    let port = 8080;
    match receive_udp_message(port) {
        Ok(_) => println!("Received UDP message on port {}", port),
        Err(e) => eprintln!("Error receiving message: {}", e),
    }
}

/*
mod utils;
use crate::utils::hash::crc32c::CRC32c;
use crate::utils::uid::UID;
use rand::{Rng, thread_rng};

fn main() {
    //e5af5f 5134c1e664b6f8260e9d99d7a871926b b8
    //e5af5f 5134c1e664b6f8260e9d99d7a8719254 f8
    //11100101101011110101111101010001001101001100000111100110011001001011011011111000001001100000111010011101100110011101011110101000011100011001001001010100
    // 11111000

    let my_local_ip = local_ip().unwrap();

    println!("This is my local IP address: {:?}", my_local_ip);

    let uid = crate::utils::uid::UID::new("e5af5f5134c1e664b6f8260e9d99d7a8719254c7").unwrap();
    //println!("Binary: {}", uid.get_binary());
    println!("Hex: {}", uid.to_string());

    let test = uid.generate_node_id_by_distance(10);
    println!("Test: {}", test.get_binary());
    println!("Test: {}", test.to_string());
}



const V4_MASK: [u8; 4] = [0x03, 0x0f, 0x3f, 0xff];
const V6_MASK: [u8; 8] = [0x01, 0x03, 0x07, 0x0f, 0x1f, 0x3f, 0x7f, 0xff];

pub fn derive_uid(consensus_external_address: &[u8]) -> UID {
    let mut ip = consensus_external_address.to_owned();
    let mask = if ip.len() == 4 { &V4_MASK } else { &V6_MASK };

    for i in 0..ip.len() {
        ip[i] &= mask[i];
    }

    let rand: u8 = thread_rng().gen();
    let r = rand & 0x7;
    ip[0] |= r << 5;

    let crc = CRC32c::checksum_ieee(&ip);

    let mut bid = [0u8; 20];
    bid[0] = ((crc >> 24) & 0xFF) as u8;
    bid[1] = ((crc >> 16) & 0xFF) as u8;
    bid[2] = (((crc >> 8) & 0xF8) | (thread_rng().gen::<u8>() & 0x7)) as u8;

    for i in 3..19 {
        bid[i] = thread_rng().gen();
    }

    bid[19] = rand;

    UID::new(bid)
}
*/

