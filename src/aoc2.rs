use std::num::ParseIntError;
use std::str::FromStr;

pub fn checksum(digits: &str) -> Result<u32, ParseIntError> {
    let mut checksum = 0;
    let values = parse_file(digits)?;
    for row in values.iter() {
        let (min, max) = row.iter().fold((u32::max_value(), 0), |acc, item| {
            (u32::min(*item, acc.0), u32::max(*item, acc.1))
        });
        checksum += max - min;
    }
    Ok(checksum)
}

pub fn checksum_div(digits: &str) -> Result<u32, ParseIntError> {
    let mut checksum = 0;
    let values = parse_file(digits)?;
    for mut row in values {
        row.sort_by(|x, y| x.cmp(y).reverse());
        for i in 0..row.len() {
            for j in i + 1..row.len() {
                let x = row[i];
                let y = row[j];
                if x % y == 0 {
                    checksum += x / y;
                }
            }
        }
    }
    Ok(checksum)
}

fn parse_file(digits: &str) -> Result<Vec<Vec<u32>>, ParseIntError> {
    let mut file = vec![];
    for line in digits.lines() {
        let mut row = vec![];
        for item in line.split(' ') {
            row.push(u32::from_str(item)?);
        }
        file.push(row);
    }
    Ok(file)
}

#[test]
fn test_invalid() {
    assert!(checksum("....\nasdasd").is_err())
}

#[test]
fn test_examples() {
    assert_eq!(Ok(18), checksum("5 1 9 5\n7 5 3\n2 4 6 8"));
    assert_eq!(Ok(0), checksum("5 5 5 5"));
    assert_eq!(Ok(0), checksum(""));
}

#[test]
fn test_parse_file() {
    assert!(parse_file("....\nasdasd").is_err());
    assert_eq!(
        Ok(vec![vec![5, 1], vec![7], vec![2, 4, 6]]),
        parse_file("5 1\n7\n2 4 6")
    );
}

#[test]
fn test_examples_div() {
    assert_eq!(Ok(9), checksum_div("5 9 2 8\n9 4 7 3\n3 8 6 5"));
    assert_eq!(Ok(1), checksum_div("5 5"));
    assert_eq!(Ok(0), checksum_div(""));
}
