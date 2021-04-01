use std::env;
use std::process;

// Syllabification

fn main() {
    let args: Vec<_> = env::args().collect();

    // Process CLI argument
    if args.len() > 1 {
        println!("Syllables: {}", count_vowels(&args[1]));
    } else {
        println!("Give argument");
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
