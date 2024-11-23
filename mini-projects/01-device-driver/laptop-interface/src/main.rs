mod command;
mod controller;
mod model;
mod view;
mod circle_state;

use controller::controller;
use crossbeam::channel::unbounded;
use model::model;
use std::thread;
use view::view;
use circle_state::CircleState;
use std::sync::{Arc, Mutex};

const CONTROLLER_PORTS: [&str; 1] = ["/dev/tty.usbmodem11201"];

fn main() {
    let (model_sender, model_receiver) = unbounded();
    let (controller_sender, controller_receiver) = unbounded();
    let (view_sender, view_receiver) = unbounded();
    let mut threads = Vec::new();

    let circle_state = Arc::new(Mutex::new(CircleState::new()));

    // Spawn thread for central communication
    let circle_state_clone = circle_state.clone();
    threads.push(thread::spawn(move || {
        model(
            model_receiver,
            controller_sender,
            view_sender,
            CONTROLLER_PORTS.len(),
            circle_state_clone,
        );
    }));

    // Spawn threads for each controller port
    for (index, port) in CONTROLLER_PORTS.iter().enumerate() {
        let receiver_clone = controller_receiver.clone();
        let model_sender_clone = model_sender.clone();
        threads.push(thread::spawn(move || {
            controller(port, receiver_clone, model_sender_clone, index);
        }));
    }

    // Spawn thread to display positions
    let circle_state_clone = circle_state.clone();
    threads.push(thread::spawn(move || {
        view(view_receiver, model_sender, circle_state_clone);
    }));

    // Join all threads
    for thread in threads {
        thread.join().unwrap();
    }
}
