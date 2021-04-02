use syllit::count_syllables;

#[test]
fn match_syllable() {
    assert_eq!(count_syllables("Batman"), 2);
}
