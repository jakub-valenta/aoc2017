use std::collections::HashSet;
use utils;

pub fn detect_cycle(input: &str) -> Option<u32> {
    let mut steps = 0;
    let mut memory_banks = utils::parse_numbers::<u32>(input)?;
    let mut states = HashSet::new();
    while !states.contains(&memory_banks) {
        steps += 1;
        let next = redistribute(&memory_banks);
        states.insert(memory_banks);
        memory_banks = next;
    }
    Some(steps)
}

fn redistribute(banks: &Vec<u32>) -> Vec<u32> {
    if banks.len() <= 1 {
        return banks.clone();
    }
    let (offset, max) = banks.iter().enumerate().fold(
        (banks.len(), 0),
        |acc, bank_value| if *bank_value.1 > acc.1 {
            (bank_value.0, *bank_value.1)
        } else {
            acc
        },
    );
    let mut redistributed = banks.clone();
    redistributed[offset] = 0;
    for i in 0..(max as usize) {
        let index = (i + offset + 1) % banks.len();
        redistributed[index] += 1;
    }
    redistributed
}

#[test]
fn test_invalid() {
    assert_eq!(None, detect_cycle("1 asdsda"));
}

#[test]
fn test_examples() {
    assert_eq!(Some(1), detect_cycle(""));
    assert_eq!(Some(5), detect_cycle("0 2 7 0"));
}

#[test]
fn test_redistribute() {
    assert_eq!(vec![2, 4, 1, 2], redistribute(&vec![0, 2, 7, 0]));
    assert_eq!(vec![3, 1, 2, 3], redistribute(&vec![2, 4, 1, 2]));
    assert_eq!(vec![0, 2, 3, 4], redistribute(&vec![3, 1, 2, 3]));
    assert_eq!(vec![1, 3, 4, 1], redistribute(&vec![0, 2, 3, 4]));
    assert_eq!(vec![1, 3, 0, 1], redistribute(&vec![0, 2, 3, 0]));
    assert_eq!(vec![0, 0, 3, 1], redistribute(&vec![0, 2, 2, 0]));
}
