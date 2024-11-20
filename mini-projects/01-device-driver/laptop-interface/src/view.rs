use crossbeam::channel::{Receiver, Sender};

pub fn view(receiver: Receiver<(String, String)>, model_sender: Sender<(String, String)>) {
    loop {
        if let Ok((_, message)) = receiver.recv() {
            println!("{}", message);
            if message == "Game started!" {
                break;
            }
        }
    }
    loop {
        model_sender
            .send(("view".to_string(), "".to_string()))
            .unwrap();
        if let Ok((_, message)) = receiver.recv() {
            // Parse message into an array
            let positions: Vec<Vec<i32>> = message
                .trim_matches(|c| c == '[')
                .split("], [")
                .map(|sub| {
                    sub.trim_matches(|c| c == '[' || c == ']')
                        .split(',')
                        .filter_map(|n| n.trim().parse().ok())
                        .collect()
                })
                .collect();
            // Print positions
            for (i, pos) in positions.iter().enumerate() {
                print!(
                    "Player {} Position: ({}, {}){}",
                    i,
                    pos[0],
                    pos[1],
                    if i < positions.len() - 1 { "   " } else { "\n" }
                );
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
