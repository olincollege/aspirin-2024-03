mod command;
mod controller;
mod model;
mod view;

use controller::controller;
use crossbeam::channel::unbounded;
use model::model;
use std::thread;
use view::view;

const CONTROLLER_PORTS: [&str; 1] = ["/dev/ttyACM0"];

fn main() {
    let (model_sender, model_receiver) = unbounded();
    let (controller_sender, controller_receiver) = unbounded();
    let (view_sender, view_receiver) = unbounded();
    let mut threads = Vec::new();

    // Spawn thread for central communication
    threads.push(thread::spawn(move || {
        model(
            model_receiver,
            controller_sender,
            view_sender,
            CONTROLLER_PORTS.len(),
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
    threads.push(thread::spawn(move || {
        view(view_receiver, model_sender);
    }));

    // Join all threads
    for thread in threads {
        thread.join().unwrap();
    }
}
