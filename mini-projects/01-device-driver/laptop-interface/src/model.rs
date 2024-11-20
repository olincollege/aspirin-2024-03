use std::io::{self, BufRead};

use crossbeam::channel::{Receiver, Sender};

use crate::command::Command;

fn send_commands(
    controller_sender: Sender<(String, String)>,
    view_sender: Sender<(String, String)>,
) {
    // Reset and initialize controllers
    let commands = vec![Command::Stop, Command::Reset, Command::Init];
    for cmd in commands {
        controller_sender
            .send((cmd.to_string(), "".to_string()))
            .unwrap();
    }
    // Set LEDs
    let commands = vec![Command::SetReadyLed, Command::SetSetLed, Command::SetGoLed];
    for cmd in commands {
        controller_sender
            .send((cmd.to_string(), "".to_string()))
            .unwrap();
        println!("{}", cmd.to_string());
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    // Start game
    let commands = vec![Command::SetAllLeds, Command::Start];
    for cmd in commands {
        controller_sender
            .send((cmd.to_string(), "".to_string()))
            .unwrap();
    }

    view_sender
        .send(("model".to_string(), "Game started!".to_string()))
        .unwrap();
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
                println!("Press Enter to start the game...");
                let stdin = io::stdin();
                let _ = stdin.lock().lines().next();
                break;
            }
        }
    }

    // Start LED command sequence
    send_commands(controller_sender.clone(), view_sender.clone());

    // Run the game
    let mut positions = vec![[0, 0]; num_players];

    loop {
        if let Ok((sender, message)) = receiver.recv() {
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
    }
}
