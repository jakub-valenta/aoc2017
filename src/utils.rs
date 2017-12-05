use std::str::FromStr;

pub fn parse_numbers<T: FromStr>(digits: &str) -> Option<Vec<T>> {
    let mut row = vec![];
    if digits.len() > 0 {
        for item in digits.split(' ') {
            match item.parse::<T>() {
                Ok(x) => row.push(x),
                _ => return None,
            };
        }
    }
    Some(row)
}

#[test]
fn test_parse_numbers_error() {
    assert_eq!(None, parse_numbers::<i32>("....\nasdasd"));
    assert_eq!(None, parse_numbers::<u32>("-1 5"));
}

#[test]
fn test_parse_numbers() {
    assert_eq!(Some(vec![]), parse_numbers::<i32>(""));
    assert_eq!(Some(vec![-5, 6, 7]), parse_numbers::<i32>("-5 6 7"));
    assert_eq!(Some(vec![5, 6, 7]), parse_numbers::<u32>("5 6 7"));
}
