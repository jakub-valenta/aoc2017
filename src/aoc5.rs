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

#[test]
fn test_examples() {
    assert_eq!(Some(0), process_instructions(""));
    assert_eq!(Some(5), process_instructions("0 3 0 1 -3"));
}
