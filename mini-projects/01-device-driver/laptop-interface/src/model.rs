use crate::command::Command;
use crossbeam::channel::{Receiver, Sender};
use device_query::{DeviceQuery, DeviceState, Keycode};

fn send_commands(
    controller_sender: Sender<(String, String)>,
    view_sender: Sender<(String, String)>,
) {
    // Set up controller and view
    controller_sender
        .send(("model".to_string(), "Start game".to_string()))
        .unwrap();
    view_sender
        .send(("model".to_string(), "Start game".to_string()))
        .unwrap();

    // Reset and initialize controllers
    let commands = vec![Command::Stop, Command::Reset, Command::Init];
    for cmd in commands {
        controller_sender
            .send(("model".to_string(), cmd.to_string()))
            .unwrap();
    }
    // Set LEDs
    let commands = vec![Command::SetReadyLed, Command::SetSetLed, Command::SetGoLed];
    for cmd in commands {
        controller_sender
            .send(("model".to_string(), cmd.to_string()))
            .unwrap();
        println!("{}", cmd.to_string());
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    // Start game
    let commands = vec![Command::SetAllLeds, Command::Start];
    for cmd in commands {
        controller_sender
            .send(("model".to_string(), cmd.to_string()))
            .unwrap();
    }
}

fn update_positions(
    positions: &mut Vec<[i32; 2]>,
    receiver: Receiver<(String, String)>,
    view_sender: Sender<(String, String)>,
) {
    match receiver.try_recv() {
        Ok((sender, message)) => {
            if sender == "view" {
                view_sender
                    .send(("model".to_string(), format!("{:?}", positions)))
                    .unwrap();
            } else {
                let player = sender.parse::<usize>().unwrap();
                if message.contains("nw") {
                    positions[player][0] -= 1;
                    positions[player][1] += 1;
                }
                if message.contains("sw") {
                    positions[player][0] -= 1;
                    positions[player][1] -= 1;
                }
                if message.contains("se") {
                    positions[player][0] += 1;
                    positions[player][1] -= 1;
                }
                if message.contains("ne") {
                    positions[player][0] += 1;
                    positions[player][1] += 1;
                }
            }
        }
        Err(_) => {}
    }
}

fn determine_winner(positions: &Vec<[i32; 2]>) -> (usize, f64) {
    let mut winner = 0;
    let mut max_distance = 0.0;
    for (index, position) in positions.iter().enumerate() {
        let distance = (position[0].pow(2) + position[1].pow(2)) as f64;
        let distance = distance.sqrt();
        if distance > max_distance {
            max_distance = distance;
            winner = index;
        }
    }
    (winner, max_distance)
}

pub fn model(
    receiver: Receiver<(String, String)>,
    controller_sender: Sender<(String, String)>,
    view_sender: Sender<(String, String)>,
    num_players: usize,
) {
    // Wait for all controllers to connect
    let mut ready = vec![false; num_players];
    loop {
        if let Ok((sender, _)) = receiver.recv() {
            println!("Controller {} Initialized.", sender);
            ready[sender.parse::<usize>().unwrap()] = true;
            let mut all_ready = true;
            for player in ready.iter() {
                if !player {
                    all_ready = false;
                }
            }
            if all_ready {
                break;
            }
        }
    }

    loop {
        // Start LED command sequence
        println!("Press Enter to start the game. Press Escape to exit.");
        let device_state = DeviceState::new();
        loop {
            let _ = device_state.get_keys();
            let keys = device_state.get_keys();
            match keys.first() {
                // Start game
                Some(&Keycode::Enter) => {
                    send_commands(controller_sender.clone(), view_sender.clone());
                    break;
                }
                // Exit program
                Some(&Keycode::Escape) => {
                    println!("Exiting game.");
                    controller_sender
                        .send(("model".to_string(), "Escape".to_string()))
                        .unwrap();
                    view_sender
                        .send(("model".to_string(), "Escape".to_string()))
                        .unwrap();
                    return;
                }
                _ => std::thread::sleep(std::time::Duration::from_millis(100)),
            }
        }

        // Run the game
        let device_state = DeviceState::new();
        let mut positions = vec![[0, 0]; num_players];
        loop {
            // Check for game over
            let keys = device_state.get_keys();
            if keys.contains(&Keycode::Space) {
                view_sender
                    .send(("model".to_string(), "Game over".to_string()))
                    .unwrap();
                controller_sender
                    .send(("model".to_string(), "Game over".to_string()))
                    .unwrap();
                break;
            }
            // Receive controller messages and send to view
            update_positions(&mut positions, receiver.clone(), view_sender.clone());

            // Small delay to prevent excessive CPU usage
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        // Determine winner
        let (winner, distance) = determine_winner(&positions);
        println!(
            "Player {} wins with a distance of {:.2} meters!",
            winner, distance
        );
    }
}
