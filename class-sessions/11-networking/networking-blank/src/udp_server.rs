use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:12701").expect("couldn't bind to local address");
    let mut buf = [0; 1024];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                println!("Received {} bytes from {}", amt, src);
                socket
                    .send_to(&buf[..amt], src)
                    .expect("failed to send message");
            }
            Err(e) => {
                eprintln!("Failed to receive data: {}", e);
            }
        }
    }
}
