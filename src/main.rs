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
            println!("Syllables: {}", count_syllables(&args[1]));
        }
    } else {
        println!("Please give argument.");
        println!("Try -h option.");
        process::exit(1);
    }
}

/// Return if letter is vowel or not.
/// This also considers 'y' as a vowel.
fn is_vowel(letter: char) -> bool {
    let vowels = "aeiouy";
    for c in vowels.chars() {
        if c == letter.to_ascii_lowercase() {
            return true;
        }
    }
    return false;
}

/// Return count of syllables in word
fn count_syllables(word: &str) -> i32 {
    let mut count = 0;
    let mut prev_char = '\0';

    // Find syllables in word
    for c in word.to_ascii_lowercase().chars() {
        if is_vowel(c) {
            // Vowel team
            if c != prev_char && !is_vowel(prev_char) {
                count = count + 1;
            }
        }
        prev_char = c;
    }

    // Magic e syllable, consonant 'l' must not exist before 'e'
    let last_two: Vec<char> = word.chars().rev().take(2).collect();
    if last_two[0] == 'e' && last_two[1] != 'l' {
        count = count - 1;
    }

    return count;
}
