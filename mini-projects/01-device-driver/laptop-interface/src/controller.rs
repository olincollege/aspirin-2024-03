use std::{ffi::CString, ptr, time::Instant};

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
    fn sp_blocking_write(port: *mut c_void, buf: *const u8, count: usize, timeout_ms: u32) -> c_int;
}

#[derive(Debug, Clone, Copy)]
struct ControllerState {
    buttons: u8,
    adc_value: u16,
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

fn read_controller_state(port: *mut c_void) -> Result<ControllerState, String> {
    let mut buffer = [0u8; 3];
    unsafe {
        let bytes_read = sp_blocking_read(port, buffer.as_mut_ptr(), buffer.len(), 1000);
        if bytes_read != buffer.len() as i32 {
            return Err(format!(
                "{:?} failed to read controller state: {}",
                port, bytes_read
            ));
        }
    }
    let adc_value = u16::from_be_bytes([buffer[1], buffer[2]]);
    let buttons = buffer[0];
    // println!("Read buttons: {:08b}, ADC value: {}", buttons, adc_value); // Debug print
    Ok(ControllerState {
        buttons,
        adc_value,
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

    let mut last_button_state = 0;
    let mut last_debounce_time = Instant::now();
    let debounce_delay = std::time::Duration::from_millis(50);

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

        // Read controller state
        loop {
            match receiver.try_recv() {
                Ok((_, message)) => {
                    if message == "Game over" {
                        break;
                    }
                }
                Err(_) => {}
            }
            let controller_state = match read_controller_state(port) {
                Ok(state) => state,
                Err(e) => {
                    eprintln!("{}", e);
                    break;
                }
            };

            // Debounce button state
            if controller_state.buttons != last_button_state {
                last_debounce_time = Instant::now();
            }

            if Instant::now().duration_since(last_debounce_time) > debounce_delay {
                last_button_state = controller_state.buttons;

                // Send button states
                let buttons = [
                    ("n", controller_state.buttons & 0b10000000 != 0),
                    ("nw", controller_state.buttons & 0b01000000 != 0),
                    ("w", controller_state.buttons & 0b00100000 != 0),
                    ("sw", controller_state.buttons & 0b00010000 != 0),
                    ("s", controller_state.buttons & 0b00001000 != 0),
                    ("se", controller_state.buttons & 0b00000100 != 0),
                    ("e", controller_state.buttons & 0b00000010 != 0),
                    ("ne", controller_state.buttons & 0b00000001 != 0),
                ];
                let message = buttons
                    .iter()
                    .filter(|(_, pressed)| *pressed)
                    .map(|(name, _)| *name)
                    .fold(String::new(), |acc, name| format!("{}{},", acc, name));

                if !message.is_empty() {
                    model_sender.send((player.to_string(), message)).unwrap();
                }

                // Send ADC value
                model_sender
                    .send((player.to_string(), format!("adc:{}", controller_state.adc_value)))
                    .unwrap();
            }
        }
    }

    unsafe {
        sp_close(port);
        println!("Port {} closed successfully", port_name);
        sp_free_port(port);
    }
}
