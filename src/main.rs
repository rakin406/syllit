use std::env;
use std::process;

// Syllabification

fn main() {
    let args: Vec<_> = env::args().collect();

    // Process CLI argument
    if args.len() > 1 {
        if args[1] == "-h" || args[1] == "--help" {
            // Help info
            println!("Usage: cargo run <word>");
            println!("Example: cargo run hello");
        } else {
            println!("Syllables: {}", count_vowels(&args[1]));
        }
    } else {
        println!("Please give argument.");
        println!("Try -h option.");
        process::exit(1);
    }
}

fn count_vowels(word: &str) -> i32 {
    let vowels = "aeiou";
    let mut count = 0;

    for c in vowels.chars() {
        if word.to_ascii_lowercase().contains(c) {
            count = count + 1;
        }
    }

    return count;
}
