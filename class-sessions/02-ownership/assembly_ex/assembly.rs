#[no_mangle]
#[inline(never)]
fn read(x: i32) {
    println!("read: {}", x);
}

fn main() {
    let x = 5;
    read(x);
}
