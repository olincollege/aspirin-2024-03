use crossbeam::channel::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use crate::circle_state::CircleState;

pub fn view(receiver: Receiver<(String, String)>, model_sender: Sender<(String, String)>, circle_state: Arc<Mutex<CircleState>>) {
    loop {
        if let Ok((_, message)) = receiver.recv() {
            if message == "Escape" {
                break;
            }
        }
        loop {
            // End loop if game is over
            match receiver.try_recv() {
                Ok((_, message)) => {
                    if message == "Game over" {
                        break;
                    }
                }
                Err(_) => {}
            }
            // Request positions
            model_sender
                .send(("view".to_string(), "".to_string()))
                .unwrap();
            // Receive positions
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
                if !positions.is_empty() {
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
            }

            // Display circle state
            let circle = circle_state.lock().unwrap();
            println!("Circle Position: ({}, {}), Radius: {}", circle.x, circle.y, circle.radius);

            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }
}
