use std::env;
use std::process;

// Syllabification

fn main() {
    let args: Vec<_> = env::args().collect();

    // Process CLI argument
    if args.len() > 1 {
        if args[1] == "-h" || args[1] == "--help" {
            // Help info
            println!("Usage: ./syllit <word>");
            println!("Example: ./syllit hello");
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

    // Find vowels in word
    for i in word.to_ascii_lowercase().chars() {
        for j in vowels.chars() {
            if i == j {
                // Magic e syllable
                if i == word.chars().last().unwrap() {
                    if i == 'e' {
                        continue;
                    }
                }
                count = count + 1;
                break;
            }
        }
    }

    return count;
}
