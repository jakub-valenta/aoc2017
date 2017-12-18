
pub fn spinlock(step_length: &str) -> Option<u32> {
    let step_length = step_length.trim().parse::<usize>().ok()?;
    let mut pointer = 0;
    let mut buffer = vec![0];
    for i in 1..2018 {
        pointer = (pointer + step_length) % buffer.len() + 1;
        buffer.insert(pointer, i)
    }
    Some(buffer[(pointer + 1) % buffer.len()])
}

pub fn angry_spinlock(step_length: &str) -> Option<u32> {
    let step_length = step_length.trim().parse::<u32>().ok()?;
    let mut pointer = 0;
    (1..50000000)
        .filter(|x| {
            pointer = (pointer + step_length) % x + 1;
            pointer == 1
        })
        .last()
}

#[test]
fn test_examples() {
    assert_eq!(Some(638), spinlock("3"));
}
