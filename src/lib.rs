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
pub fn count_syllables(word: &str) -> i32 {
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
    } else if last_two[0] == 'd' && last_two[1] == 'e' {
        // Don't count "ed"
        // TODO: "Batted" has two syllables, it has to count "ed". Gotta solve
        // this problem.
        count = count - 1;
    }

    return count;
}
