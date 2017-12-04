use std::collections::HashSet;
use std::iter::FromIterator;

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

pub fn verify_pass_phrase_anagram(pass_phrase: &str) -> bool {
    let mut words = HashSet::new();
    for word in pass_phrase.split(' ') {
        let mut word = word.chars().collect::<Vec<char>>();
        word.sort();
        let word = String::from_iter(word);
        if words.contains(&word) {
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

#[test]
fn test_examples_anagram() {
    assert!(verify_pass_phrase_anagram("abcde fghij"));
    assert!(!verify_pass_phrase_anagram("abcde xyz ecdab"));
    assert!(verify_pass_phrase_anagram("a ab abc abd abf abj"));
    assert!(verify_pass_phrase_anagram("iiii oiii ooii oooi oooo"));
    assert!(!verify_pass_phrase_anagram("oiii ioii iioi iiio"));
}
