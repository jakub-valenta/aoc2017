pub fn sum_digits(digits: &str) -> Option<u32> {
    let length = digits.len();
    let mut sum = 0;
    for i in 0..length {
        let c = digits.chars().nth(i)?;
        if c == digits.chars().nth((i + 1) % length)? {
            sum += c.to_digit(10)?;
        }
    }
    Some(sum)
}

#[test]
fn test_invalid() {
    assert_eq!(None, sum_digits("...."))
}

#[test]
fn test_examples() {
    assert_eq!(Some(3), sum_digits("1122"));
    assert_eq!(Some(4), sum_digits("1111"));
    assert_eq!(Some(0), sum_digits("1234"));
    assert_eq!(Some(9), sum_digits("91212129"));
}
