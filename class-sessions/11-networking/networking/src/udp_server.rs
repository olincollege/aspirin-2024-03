use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8080").expect("Could not bind to address");
    let mut buf = [0; 1024];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                println!("Received {} bytes from {}", amt, src);
                let response = format!("Echoing: {}", String::from_utf8_lossy(&buf[..amt]));
                socket
                    .send_to(response.as_bytes(), &src)
                    .expect("Failed to send a response");
            }
            Err(e) => {
                eprintln!("Failed to receive a datagram: {}", e);
            }
        }
    }
}
