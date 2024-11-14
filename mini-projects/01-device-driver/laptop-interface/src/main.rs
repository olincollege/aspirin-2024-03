use crossbeam::channel::{unbounded, Receiver, Sender};
use libc::{c_char, c_int, c_void};
use std::ffi::CString;
use std::ptr;
use std::thread;

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

#[derive(Debug)]
enum Command {
    Init,
    SetReadyLed,
    SetSetLed,
    SetGoLed,
    SetAllLeds,
    Start,
    Stop,
    Reset,
}

impl Command {
    fn as_bytes(&self) -> &[u8] {
        match self {
            Command::Init => b"init controller\n",
            Command::SetReadyLed => b"set ready led\n",
            Command::SetSetLed => b"set set led\n",
            Command::SetGoLed => b"set go led\n",
            Command::SetAllLeds => b"set all leds\n",
            Command::Start => b"start controller\n",
            Command::Stop => b"stop controller\n",
            Command::Reset => b"reset\n",
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
struct ButtonStates {
    northwest: bool,
    southwest: bool,
    southeast: bool,
    northeast: bool,
}

fn send_commands(port: *mut c_void) -> Result<(), String> {
    // Reset controller
    let commands = vec![Command::Stop, Command::Reset, Command::Init];
    for cmd in commands {
        let bytes = cmd.as_bytes();
        unsafe {
            let written = sp_blocking_write(port, bytes.as_ptr(), bytes.len(), 1000);
            if written != bytes.len() as i32 {
                return Err(format!("Failed to send {:?}: {}", cmd, written));
            }
            // Small delay between commands
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }

    // Set LED states
    let commands = vec![Command::SetReadyLed, Command::SetSetLed, Command::SetGoLed];
    for cmd in commands {
        let bytes = cmd.as_bytes();
        unsafe {
            let written = sp_blocking_write(port, bytes.as_ptr(), bytes.len(), 1000);
            println!("{:?}", cmd);
            if written != bytes.len() as i32 {
                return Err(format!("Failed to send {:?}: {}", cmd, written));
            }
            // Small delay between commands
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }

    let commands = vec![Command::SetAllLeds, Command::Start];
    for cmd in commands {
        let bytes = cmd.as_bytes();
        unsafe {
            let written = sp_blocking_write(port, bytes.as_ptr(), bytes.len(), 1000);
            if written != bytes.len() as i32 {
                return Err(format!("Failed to send {:?}: {}", cmd, written));
            }
        }
    }

    Ok(())
}

fn parse_button_states(data: &str) -> Option<ButtonStates> {
    if !data.starts_with("buttons:") {
        return None;
    }

    let states = data.trim_start_matches("buttons:").trim();
    if let Ok(bitmask) = u8::from_str_radix(states, 2) {
        Some(ButtonStates {
            northwest: (bitmask & 0b1000) != 0,
            southwest: (bitmask & 0b0100) != 0,
            southeast: (bitmask & 0b0010) != 0,
            northeast: (bitmask & 0b0001) != 0,
        })
    } else {
        None
    }
}

fn write_command(port_name: &str, sender: Sender<(String, String)>) {
    let port_name_c = CString::new(port_name).expect("CString::new failed");
    let mut port: *mut c_void = ptr::null_mut();

    unsafe {
        if sp_get_port_by_name(port_name_c.as_ptr(), &mut port) != 0 {
            eprintln!("Failed to get port by name {}", port_name);
            return;
        }

        if sp_open(port, 3) != 0 {
            eprintln!("Failed to open port {}", port_name);
            sp_free_port(port);
            return;
        }

        println!("Port {} opened successfully", port_name);

        // Send LED command sequence
        if let Err(e) = send_commands(port) {
            eprintln!("Command sequence failed: {}", e);
        }

        // Read button states
        let mut previous_states = ButtonStates::default();
        let mut buf = [0u8; 1];
        loop {
            let bytes_read = sp_blocking_read(port, buf.as_mut_ptr(), buf.len(), 1000);
            if bytes_read > 0 {
                // Format byte as binary string with "buttons:" prefix
                let button_data = format!("buttons:{:04b}", buf[0]);
                println!("Received: {}", button_data);
                let current_states = ButtonStates {
                    northwest: (buf[0] & 0b1000) != 0,
                    southwest: (buf[0] & 0b0100) != 0,
                    southeast: (buf[0] & 0b0010) != 0,
                    northeast: (buf[0] & 0b0001) != 0,
                };
                // Check for state changes
                if let Some(current_states) = parse_button_states(&button_data) {
                    // Check for state changes
                    if current_states != previous_states {
                        // Send messages for changed buttons
                        if current_states.northwest != previous_states.northwest {
                            sender
                                .send((
                                    port_name.to_string(),
                                    format!(
                                        "Northwest button {}",
                                        if current_states.northwest {
                                            "pressed"
                                        } else {
                                            "released"
                                        }
                                    ),
                                ))
                                .unwrap();
                        }
                    }
                    if current_states.southwest != previous_states.southwest {
                        sender
                            .send((
                                port_name.to_string(),
                                format!(
                                    "Southwest button {}",
                                    if current_states.southwest {
                                        "pressed"
                                    } else {
                                        "released"
                                    }
                                ),
                            ))
                            .unwrap();
                    }
                    if current_states.southeast != previous_states.southeast {
                        sender
                            .send((
                                port_name.to_string(),
                                format!(
                                    "Southeast button {}",
                                    if current_states.southeast {
                                        "pressed"
                                    } else {
                                        "released"
                                    }
                                ),
                            ))
                            .unwrap();
                    }
                    if current_states.northeast != previous_states.northeast {
                        sender
                            .send((
                                port_name.to_string(),
                                format!(
                                    "Northeast button {}",
                                    if current_states.northeast {
                                        "pressed"
                                    } else {
                                        "released"
                                    }
                                ),
                            ))
                            .unwrap();
                    }

                    // Update previous states
                    previous_states = current_states;
                }
            }
        }

        sp_close(port);
        println!("Port {} closed successfully", port_name);
        sp_free_port(port);
    }
}

fn display_positions(receiver: Receiver<(String, String)>) {
    loop {
        if let Ok((port_name, position)) = receiver.recv() {
            println!("Controller {}: Position {}", port_name, position);
        }
    }
}

fn main() {
    let (sender, receiver) = unbounded();

    let controller_ports = vec!["/dev/ttyACM0"];
    for port in controller_ports {
        let sender_clone = sender.clone();
        thread::spawn(move || {
            write_command(port, sender_clone);
        });
    }

    let display_thread = thread::spawn(move || {
        display_positions(receiver);
    });

    display_thread.join().unwrap();
}
