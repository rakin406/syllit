// TODO: Get CLI argument
fn main() {
    let vowels = "aeiou";
    let word = "Hello";
    let mut syllables = 0;

    // Find vowels in word
    for c in vowels.chars() {
        if word.contains(c) {
            syllables = syllables + 1;
        }
    }

    println!("{}", syllables);
}
