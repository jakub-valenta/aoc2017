use aoc10;

pub fn count_used_squares(input: &str) -> u32 {
    let mut count = 0;
    for i in 0..128 {
        let hash = aoc10::knot_tying_hash_raw(&format!("{}-{}", input, i));
        count += hash.iter().fold(0, |acc, x| acc + x.count_ones());
    }
    count
}

#[test]
fn test_examples() {
    assert_eq!(8108, count_used_squares("flqrgnkx"));
}
