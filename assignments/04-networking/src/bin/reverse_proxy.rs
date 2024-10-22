use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        eprintln!("Usage: {} <proxy-from> <proxy-to>", args[0]);
        std::process::exit(2);
    }

    let proxy_addr = &args[1];
    let origin_addr = &args[2];
}
