use syllit::count_syllables;

#[test]
fn match_syllable1() {
    assert_eq!(count_syllables("Take"), 1);
}

#[test]
fn match_syllable2() {
    assert_eq!(count_syllables("Bee"), 1);
}

#[test]
fn match_syllable3() {
    assert_eq!(count_syllables("Taking"), 2);
}

#[test]
fn match_syllable4() {
    assert_eq!(count_syllables("Redo"), 2);
}

#[test]
fn match_syllable5() {
    assert_eq!(count_syllables("Dusted"), 2);
}

#[test]
fn match_syllable6() {
    assert_eq!(count_syllables("Worrying"), 3);
}
