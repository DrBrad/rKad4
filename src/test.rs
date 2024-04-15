use std::net::{SocketAddr, UdpSocket};

fn main() -> std::io::Result<()> {
    {
        let send = SocketAddr::from(([127, 0, 0, 1], 1080));

        let socket = UdpSocket::bind(SocketAddr::from(([127, 0, 0, 1], 0)))?;//"127.0.0.1:0")?;
        let buf = "asdasdasdasd".as_bytes();;
        socket.send_to(&buf, &send)?;

        /*
        let mut buf = [0; 65535];
        let (amt, src) = socket.recv_from(&mut buf)?;
        */
    }
    Ok(())
}
