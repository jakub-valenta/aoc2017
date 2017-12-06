use utils;

pub fn process_instructions(input: &str) -> Option<u32> {
    let mut steps = 0;
    let mut pc = 0;
    let mut instructions = utils::parse_numbers::<i32>(input)?;
    while (pc as usize) < instructions.len() {
        steps += 1;
        let index = pc as usize;
        pc += instructions[index];
        instructions[index] += 1;
    }
    Some(steps)
}

pub fn process_instructions_strange(input: &str) -> Option<u32> {
    let mut steps = 0;
    let mut pc = 0;
    let mut instructions = utils::parse_numbers::<i32>(input)?;
    while (pc as usize) < instructions.len() {
        steps += 1;
        let index = pc as usize;
        let offset = instructions[index];
        pc += offset;
        if offset >= 3 {
            instructions[index] -= 1;
        } else {
            instructions[index] += 1;
        }
    }
    Some(steps)
}

#[test]
fn test_examples() {
    assert_eq!(Some(0), process_instructions(""));
    assert_eq!(Some(5), process_instructions("0 3 0 1 -3"));
    assert_eq!(Some(0), process_instructions_strange(""));
    assert_eq!(Some(10), process_instructions_strange("0 3 0 1 -3"));
}
