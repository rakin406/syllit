use std::env;
use std::process;

fn main() {
    let args: Vec<_> = env::args().collect();
    let vowels = "aeiou";
    let word = "HELLO";
    let mut syllables = 0;

    // Process CLI argument
    if args.len() > 1 {
        println!("The first argument is {}", args[1]);
    } else {
        println!("Give argument");
        process::exit(1);
    }

    // Find vowels in word
    for c in vowels.chars() {
        if word.to_ascii_lowercase().contains(c) {
            syllables = syllables + 1;
        }
    }

    println!("{}", syllables);
}
