/// Return if letter is vowel or not.
pub fn is_vowel(letter: char) -> bool {
    let vowels = "aeiou";
    for c in vowels.chars() {
        if c == letter.to_ascii_lowercase() {
            return true;
        }
    }
    return false;
}

/// Return count of syllables in word
pub fn count_syllables(word: &str) -> i32 {
    let mod_word = word.to_ascii_lowercase();
    let mut prev_char = '\0';
    let mut count = 0;

    // Find syllables in word
    for c in mod_word.chars() {
        if is_vowel(c) || c == 'y' {
            // Vowel team
            if c != prev_char && !is_vowel(prev_char) && prev_char != 'y' {
                count = count + 1;
            }
        }
        prev_char = c;
    }

    if count > 1 {
        // Magic e syllable, consonant 'l' must not exist before 'e'
        let last_two: Vec<char> = mod_word.chars().rev().take(2).collect();
        if last_two[0] == 'e' && last_two[1] != 'l' {
            count = count - 1;
        }

        // Example: Deactivate
        if mod_word.contains("ea") {
            count = count + 1;
        }

        // Example: Worrying
        if mod_word.contains("ying") {
            count = count + 1;
        }
    }

    return count;
}
