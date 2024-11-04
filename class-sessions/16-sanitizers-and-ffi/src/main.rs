mod ffi_curl;
use ffi_curl::run_curl;

fn get_dangling_pointer() -> *const i32 {
    let x = 42;
    x as _
}

fn main() {
    // println!("Hello, world!");
    // run_curl();
    unsafe {
        println!("random value: {}", *get_dangling_pointer());
    }
}
