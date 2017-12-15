
pub fn count_matches(init_a: &str, init_b: &str) -> Option<u32> {
    let mut generator_a = Generator::new(init_a.trim().parse().ok()?, 16807, 1);
    let mut generator_b = Generator::new(init_b.trim().parse().ok()?, 48271, 1);
    Some((0..40000000).fold(0, |acc, _| {
        let a = generator_a.next() & 0xffff;
        let b = generator_b.next() & 0xffff;
        if a == b { acc + 1 } else { acc }
    }))
}

pub fn count_filtered(init_a: &str, init_b: &str) -> Option<u32> {
    let mut generator_a = Generator::new(init_a.trim().parse().ok()?, 16807, 4);
    let mut generator_b = Generator::new(init_b.trim().parse().ok()?, 48271, 8);
    Some((0..5000000).fold(0, |acc, _| {
        let a = generator_a.next() & 0xffff;
        let b = generator_b.next() & 0xffff;
        if a == b { acc + 1 } else { acc }
    }))
}

struct Generator {
    previous: u64,
    factor: u64,
    modulo: u64,
    filter: u32,
}

impl Generator {
    fn new(initial_value: u32, factor: u32, filter: u32) -> Generator {
        Generator {
            previous: initial_value as u64,
            factor: factor as u64,
            modulo: 2147483647,
            filter: filter,
        }
    }

    fn next(&mut self) -> u32 {
        loop {
            self.previous = (self.previous * self.factor) % self.modulo;
            if self.previous as u32 % self.filter == 0 {
                break;
            }
        }
        self.previous as u32
    }
}

#[test]
fn test_examples() {
    assert_eq!(Some(588), count_matches("65", "8921"));
    assert_eq!(Some(309), count_filtered("65", "8921"));
}
