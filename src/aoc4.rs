use std::collections::HashSet;

pub fn verify_pass_phrase(pass_phrase: &str) -> bool {
    let mut words = HashSet::new();
    for word in pass_phrase.split(' ') {
        if words.contains(word) {
            return false;
        }
        words.insert(word);
    }
    true
}

#[test]
fn test_examples() {
    assert!(verify_pass_phrase("aa bb cc dd ee"));
    assert!(!verify_pass_phrase("aa bb cc dd aa"));
    assert!(verify_pass_phrase("aa bb cc dd aaa"));
}
