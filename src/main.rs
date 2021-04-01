// TODO: Get CLI argument
fn main() {
    let vowels = "aeiou";
    let word = "HELLO";
    let mut syllables = 0;

    // Find vowels in word
    for c in vowels.chars() {
        if word.to_ascii_lowercase().contains(c) {
            syllables = syllables + 1;
        }
    }

    println!("{}", syllables);
}
