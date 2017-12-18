
pub fn spinlock(step_length: &str) -> Option<u32> {
    let step_length = step_length.trim().parse::<usize>().ok()?;
    let mut pointer = 0;
    let mut buffer = vec![0];
    for i in 1..2018 {
        pointer = (pointer + step_length + 1) % buffer.len();
        buffer.insert(pointer, i)
    }
    Some(buffer[(pointer + 1) % buffer.len()])
}

#[test]
fn test_examples() {
    assert_eq!(Some(638), spinlock("3"));
}
