use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not bind to address");

    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");

        if let Err(e) = socket.send_to(input.as_bytes(), "127.0.0.1:8080") {
            eprintln!("Failed to send data: {}", e);
        }

        let mut buf = [0; 1024];
        match socket.recv_from(&mut buf) {
            Ok((amt, _src)) => {
                println!("{}", String::from_utf8_lossy(&buf[..amt]));
            }
            Err(e) => {
                eprintln!("Failed to receive data: {}", e);
            }
        }
    }
}
