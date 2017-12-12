use utils;

pub fn knot_tying_hash(instructions: &str) -> Option<u32> {
    let instructions = utils::parse_numbers::<usize>(instructions)?;
    Some(knot_tying_hash_impl(&instructions, 256))
}

pub fn knot_tying_hash_impl(instructions: &Vec<usize>, string_length: u32) -> u32 {
    let mut string: Vec<u32> = (0..string_length).collect();
    let modulo = string.len();
    let mut pointer = 0;
    let mut skip_size = 0;
    for instruction in instructions.iter() {
        for i in 0..(instruction / 2) {
            string.swap(
                (pointer + i) % modulo,
                (pointer + instruction - i - 1) % modulo,
            );
        }
        pointer = (pointer + instruction + skip_size) % modulo;
        skip_size += 1;
    }
    string[0] * string[1]
}

#[test]
fn test_examples() {
    assert_eq!(12, knot_tying_hash_impl("3 4 1 5", 5));
}
