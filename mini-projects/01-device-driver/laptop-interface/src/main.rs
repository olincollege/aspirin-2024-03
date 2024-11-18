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

fn write_command(port_name: &str, sender: Sender<(String, String)>, player: usize) {
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
        let mut position = [0, 0];
        loop {
            let bytes_read = sp_blocking_read(port, buf.as_mut_ptr(), buf.len(), 1000);
            if bytes_read > 0 {
                // Format byte as binary string with "buttons:" prefix
                let current_states = ButtonStates {
                    northwest: (buf[0] & 0b1000) != 0,
                    southwest: (buf[0] & 0b0100) != 0,
                    southeast: (buf[0] & 0b0010) != 0,
                    northeast: (buf[0] & 0b0001) != 0,
                };
                if current_states != previous_states {
                    // Send messages for changed buttons
                    if current_states.northwest != previous_states.northwest
                        && !current_states.northwest
                    {
                        position[0] -= 1;
                        position[1] += 1;
                        sender
                            .send((
                                player.to_string(),
                                format!("({}, {})", position[0], position[1]),
                            ))
                            .unwrap();
                    }
                    if current_states.southwest != previous_states.southwest
                        && !current_states.southwest
                    {
                        position[0] -= 1;
                        position[1] -= 1;
                        sender
                            .send((
                                player.to_string(),
                                format!("({}, {})", position[0], position[1]),
                            ))
                            .unwrap();
                    }
                    if current_states.southeast != previous_states.southeast
                        && !current_states.southeast
                    {
                        position[0] += 1;
                        position[1] -= 1;
                        sender
                            .send((
                                player.to_string(),
                                format!("({}, {})", position[0], position[1]),
                            ))
                            .unwrap();
                    }
                    if current_states.northeast != previous_states.northeast
                        && !current_states.northeast
                    {
                        position[0] += 1;
                        position[1] += 1;
                        sender
                            .send((
                                player.to_string(),
                                format!("({}, {})", position[0], position[1]),
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
            println!("Player {} Position: {}", port_name, position);
        }
    }
}

const CONTROLLER_PORTS: [&str; 1] = ["/dev/ttyACM0"];

fn main() {
    let (sender, receiver) = unbounded();
    let mut threads = Vec::new();
    for (index, port) in CONTROLLER_PORTS.iter().enumerate() {
        let sender_clone = sender.clone();
        threads.push(thread::spawn(move || {
            write_command(port, sender_clone, index);
        }));
    }

    let display_thread = thread::spawn(move || {
        display_positions(receiver);
    });
    threads.push(display_thread);

    // Join all threads
    for thread in threads {
        thread.join().unwrap();
    }
}
