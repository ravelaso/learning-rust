use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        // std::process::exit(1);
        return;
    }
    let filename = &args[1];
    println!("Processing {filename}");
}
