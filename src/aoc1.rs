pub fn sum_digits(digits: &str) -> Option<u32> {
    sum_digits_impl(digits, 1)
}

pub fn sum_digits_half(digits: &str) -> Option<u32> {
    sum_digits_impl(digits, digits.len() / 2)
}

fn sum_digits_impl(digits: &str, step: usize) -> Option<u32> {
    let length = digits.len();
    let mut sum = 0;
    for i in 0..length {
        let c = digits.chars().nth(i)?;
        if c == digits.chars().nth((i + step) % length)? {
            sum += c.to_digit(10)?;
        }
    }
    Some(sum)
}

#[test]
fn test_invalid() {
    assert_eq!(None, sum_digits_impl("....", 1))
}

#[test]
fn test_examples() {
    assert_eq!(Some(3), sum_digits("1122"));
    assert_eq!(Some(4), sum_digits("1111"));
    assert_eq!(Some(0), sum_digits("1234"));
    assert_eq!(Some(9), sum_digits("91212129"));
}

#[test]
fn test_examples_half() {
    assert_eq!(Some(6), sum_digits_half("1212"));
    assert_eq!(Some(0), sum_digits_half("1221"));
    assert_eq!(Some(4), sum_digits_half("123425"));
    assert_eq!(Some(12), sum_digits_half("123123"));
    assert_eq!(Some(4), sum_digits_half("12131415"));
}
