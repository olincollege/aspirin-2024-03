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
}

fn read_from_controller(port_name: &str, sender: Sender<(String, String)>) {
    let port_name_c = CString::new(port_name).expect("CString::new failed");
    let mut port: *mut c_void = ptr::null_mut();

    unsafe {
        if sp_get_port_by_name(port_name_c.as_ptr(), &mut port) == 0 {
            if sp_open(port, 0) == 0 {
                println!("Port {} opened successfully", port_name);

                let mut buf = [0u8; 1024];
                loop {
                    let bytes_read = sp_blocking_read(port, buf.as_mut_ptr(), buf.len(), 1000);
                    if bytes_read > 0 {
                        let position =
                            String::from_utf8_lossy(&buf[..bytes_read as usize]).to_string();
                        sender.send((port_name.to_string(), position)).unwrap();
                    }
                }

                sp_close(port);
                println!("Port {} closed successfully", port_name);
            } else {
                eprintln!("Failed to open port {}", port_name);
            }
            sp_free_port(port);
        } else {
            eprintln!("Failed to get port by name {}", port_name);
        }
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
            read_from_controller(port, sender_clone);
        });
    }

    let display_thread = thread::spawn(move || {
        display_positions(receiver);
    });

    display_thread.join().unwrap();
}
