use std::collections::HashMap;
use utils;

pub fn compute_severity(scanner: &str) -> Option<u32> {
    let scanner_rules = parse_scanner(scanner)?;
    Some(scanner_rules.iter().fold(0, |acc, (distance, depth)| {
        if is_caught(*distance, *depth, 0) {
            acc + (*distance) * (*depth)
        } else {
            acc
        }
    }))
}

pub fn compute_delay(scanner: &str) -> Option<u32> {
    let scanner_rules = parse_scanner(scanner)?;
    let mut delay = 0;
    loop {
        if scanner_rules.iter().fold(true, |acc, (distance, depth)| {
            acc && !is_caught(*distance, *depth, delay)
        })
        {
            return Some(delay);
        }
        delay += 1;
    }
}

fn parse_scanner(scanner: &str) -> Option<HashMap<u32, u32>> {
    let mut scanner_rules = HashMap::new();
    for line in scanner.lines() {
        let rules = utils::parse_numbers(line, ':')?;
        scanner_rules.insert(rules[0], rules[1]);
    }
    Some(scanner_rules)
}

fn is_caught(distance: u32, depth: u32, delay: u32) -> bool {
    match depth {
        1 => true,
        x => (distance + delay) % (2 * (x - 1)) == 0,
    }
}

#[test]
fn tests_examples() {
    assert_eq!(Some(24), compute_severity("0: 3\n1: 2\n4: 4\n6: 4"));
    assert_eq!(Some(10), compute_delay("0: 3\n1: 2\n4: 4\n6: 4"));
}
