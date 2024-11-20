use std::{ffi::CString, ptr};

use crossbeam::channel::{Receiver, Sender};
use libc::{c_char, c_int, c_void};

use crate::command::Command;

#[link(name = "serialport")]
extern "C" {
    fn sp_get_port_by_name(name: *const c_char, port: *mut *mut c_void) -> c_int;
    fn sp_open(port: *mut c_void, flags: c_int) -> c_int;
    fn sp_close(port: *mut c_void) -> c_int;
    fn sp_free_port(port: *mut c_void);
    fn sp_blocking_read(port: *mut c_void, buf: *mut u8, count: usize, timeout_ms: u32) -> c_int;
    fn sp_blocking_write(port: *mut c_void, buf: *const u8, count: usize, timeout_ms: u32)
        -> c_int;
}

#[derive(Debug, Default, Clone, PartialEq)]
struct ButtonStates {
    northwest: bool,
    southwest: bool,
    southeast: bool,
    northeast: bool,
}

fn open_port(port_name: &str) -> Result<*mut c_void, String> {
    let port_name_c = CString::new(port_name).expect("CString::new failed");
    let mut port: *mut c_void = ptr::null_mut();

    unsafe {
        if sp_get_port_by_name(port_name_c.as_ptr(), &mut port) != 0 {
            return Err(format!("Failed to get port by name {}", port_name));
        }
        if sp_open(port, 3) != 0 {
            sp_free_port(port);
            return Err(format!("Failed to open port {}", port_name));
        }
    }
    Ok(port)
}

fn send_command(port: *mut c_void, command: Command) -> Result<(), String> {
    let bytes = command.as_bytes();
    unsafe {
        let written = sp_blocking_write(port, bytes.as_ptr(), bytes.len(), 1000);
        if written != bytes.len() as i32 {
            return Err(format!(
                "{:?} failed to send {:?}: {}",
                port, command, written
            ));
        }
    }
    Ok(())
}

fn read_button_states(port: *mut c_void) -> Result<ButtonStates, String> {
    let mut buf = [0u8; 1];
    unsafe {
        let bytes_read = sp_blocking_read(port, buf.as_mut_ptr(), buf.len(), 1000);
        if bytes_read != buf.len() as i32 {
            return Err(format!(
                "{:?} failed to read button states: {}",
                port, bytes_read
            ));
        }
    }
    Ok(ButtonStates {
        northwest: (buf[0] & 0b1000) != 0,
        southwest: (buf[0] & 0b0100) != 0,
        southeast: (buf[0] & 0b0010) != 0,
        northeast: (buf[0] & 0b0001) != 0,
    })
}

pub fn controller(
    port_name: &str,
    receiver: Receiver<(String, String)>,
    model_sender: Sender<(String, String)>,
    player: usize,
) {
    let port = match open_port(port_name) {
        Ok(port) => port,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    model_sender
        .send((player.to_string(), "".to_string()))
        .unwrap();

    loop {
        // Wait for game to start
        if let Ok((_, message)) = receiver.recv() {
            if message == "Escape" {
                break;
            }
        }

        // Start LED command sequence
        loop {
            if let Ok((_, message)) = receiver.recv() {
                if let Err(e) = send_command(port, Command::from_string(&message).unwrap()) {
                    eprintln!("Failed to send command: {}", e);
                };
                if message == "Start" {
                    break;
                }
            }
        }

        // Read button states
        let mut previous_states = ButtonStates::default();
        loop {
            match receiver.try_recv() {
                Ok((_, message)) => {
                    if message == "Game over" {
                        break;
                    }
                }
                Err(_) => {}
            }
            let current_states = match read_button_states(port) {
                Ok(states) => states,
                Err(e) => {
                    eprintln!("{}", e);
                    break;
                }
            };
            let buttons = [
                ("nw", current_states.northwest, previous_states.northwest),
                ("sw", current_states.southwest, previous_states.southwest),
                ("se", current_states.southeast, previous_states.southeast),
                ("ne", current_states.northeast, previous_states.northeast),
            ];
            let message = buttons
                .iter()
                .filter(|(_, current, previous)| current != previous && !current)
                .map(|(name, _, _)| name)
                .fold(String::new(), |acc, name| format!("{}{},", acc, name));

            // Update previous states
            previous_states = current_states;

            if message.len() > 0 {
                model_sender.send((player.to_string(), message)).unwrap();
            }
        }
    }

    unsafe {
        sp_close(port);
        println!("Port {} closed successfully", port_name);
        sp_free_port(port);
    }
}
