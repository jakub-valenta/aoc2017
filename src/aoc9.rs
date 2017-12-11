
pub fn compute_score(stream: &str) -> Option<u32> {
    let mut score = 0;
    let mut level = 0;
    let mut in_garbage = false;
    let mut skip = false;
    for c in stream.chars() {
        if skip {
            skip = false;
            continue;
        }
        if in_garbage {
            match c {
                '!' => skip = true,
                '>' => in_garbage = false,
                _ => continue,
            }
        } else {
            match c {
                '{' => {
                    level += 1;
                    score += level;
                }
                '}' => level -= 1,
                '<' => in_garbage = true,
                ',' => continue,
                _ => return None,
            }
        }
    }
    Some(score)
}


#[test]
fn test_examples() {
    assert_eq!(Some(1), compute_score("{}"));
    assert_eq!(Some(6), compute_score("{{{}}}"));
    assert_eq!(Some(5), compute_score("{{},{}}"));
    assert_eq!(Some(16), compute_score("{{{},{},{{}}}}"));
    assert_eq!(Some(1), compute_score("{<a>,<a>,<a>,<a>}"));
    assert_eq!(Some(9), compute_score("{{<ab>},{<ab>},{<ab>},{<ab>}}"));
    assert_eq!(Some(9), compute_score("{{<!!>},{<!!>},{<!!>},{<!!>}}"));
    assert_eq!(Some(3), compute_score("{{<a!>},{<a!>},{<a!>},{<ab>}}"));
}
