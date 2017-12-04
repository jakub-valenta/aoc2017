use std::num::ParseIntError;
use std::str::FromStr;

pub fn checksum(digits: &str) -> Result<u32, ParseIntError> {
    let mut checksum = 0;
    for line in digits.lines() {
        let (min, max) = line.split(' ').map(u32::from_str).fold(
            Ok((u32::max_value(), 0)),
            |acc, item| {
                let item = item?;
                let (min, max) = acc?;
                Ok((u32::min(item, min), u32::max(item, max)))
            },
        )?;
        checksum += max - min;
    }
    Ok(checksum)
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
