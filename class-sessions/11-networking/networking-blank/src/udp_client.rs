use std::{io::stdin, net::UdpSocket};

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("failed to bind");

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if let Err(e) = socket.send_to(input.as_bytes(), "127.0.0.1:12701") {
            eprintln!("Failed to send error: {}", e);
        }

        let mut buf = [0; 1024];
        match socket.recv(&mut buf) {
            Ok(amt) => {
                println!("Received: {}", String::from_utf8_lossy(&buf[..amt]))
            }
            Err(e) => {
                eprintln!("Failed to receive data: {}", e);
            }
        }
    }
}
