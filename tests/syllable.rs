use syllit::count_syllables;

#[test]
fn match_syllable() {
    assert_eq!(count_syllables("Take"), 1);
    assert_eq!(count_syllables("Bee"), 1);
    assert_eq!(count_syllables("Taking"), 2);
    assert_eq!(count_syllables("Redo"), 2);
    assert_eq!(count_syllables("Dusted"), 2);
    assert_eq!(count_syllables("Worrying"), 3);
}
