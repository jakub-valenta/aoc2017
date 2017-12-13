use std::str::FromStr;

pub fn merge_args(args: &Vec<String>, first: usize, separator: &str) -> String {
    if first >= args.len() {
        return String::new();
    }
    let first_item = args[first].clone();
    args.iter().skip(first + 1).fold(first_item, |acc, item| {
        acc + separator + item
    })
}

pub fn parse_numbers<T: FromStr>(digits: &str, delimiter: char) -> Option<Vec<T>> {
    let mut row = vec![];
    let digits = digits.trim();
    if digits.len() > 0 {
        for item in digits.split(delimiter) {
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
    assert_eq!(None, parse_numbers::<i32>("....\nasdasd", ' '));
    assert_eq!(None, parse_numbers::<u32>("-1 5", ' '));
}

#[test]
fn test_parse_numbers() {
    assert_eq!(Some(vec![]), parse_numbers::<i32>("", ' '));
    assert_eq!(Some(vec![-5, 6, 7]), parse_numbers::<i32>("-5 6 7 ", ' '));
    assert_eq!(Some(vec![5, 6, 7]), parse_numbers::<u32>("5 6 7", ' '));
}

#[test]
fn test_merge_args() {
    assert_eq!("", merge_args(&vec![], 0, " "));
    assert_eq!("x", merge_args(&vec![String::from("x")], 0, " "));
    assert_eq!(
        "y z",
        merge_args(
            &vec![String::from("x"), String::from("y"), String::from("z")],
            1,
            " ",
        )
    );
}
