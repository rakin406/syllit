use std::env;
use std::process;

mod lib;

fn main() {
    let args: Vec<_> = env::args().collect();

    // Process CLI argument
    if args.len() > 1 {
        if args[1] == "-h" || args[1] == "--help" {
            // Help info
            println!("Usage: ./syllit <word>");
            println!("Example: ./syllit hello");
        } else {
            println!("Syllables: {}", lib::count_syllables(&args[1]));
        }
    } else {
        println!("Please give argument.");
        println!("Try -h option.");
        process::exit(1);
    }
}
