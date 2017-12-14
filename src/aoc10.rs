use utils;

pub fn knot_tying_hash_round(instructions: &str) -> Option<u32> {
    let instructions = utils::parse_numbers::<u8>(instructions, ',')?;
    let mut hash = KnotTypingHash::new(256);
    hash.compute(&instructions);
    Some(hash.hash[0] * hash.hash[1])
}

pub fn knot_tying_hash(text: &str) -> String {
    let dense_hash = knot_tying_hash_raw(text);
    dense_hash.iter().fold(String::new(), |mut acc, x| {
        acc.push_str(&format!("{:02x}", x));
        acc
    })
}

pub fn knot_tying_hash_raw(text: &str) -> Vec<u8> {
    let mut input: Vec<u8> = text.bytes().collect();
    input.append(&mut vec![17, 31, 73, 47, 23]);
    let mut hash = KnotTypingHash::new(256);
    hash.compute_dense_hash(&input)
}

struct KnotTypingHash {
    pointer: usize,
    skip_size: usize,
    hash: Vec<u32>,
}

impl KnotTypingHash {
    fn new(size: u32) -> KnotTypingHash {
        KnotTypingHash {
            pointer: 0,
            skip_size: 0,
            hash: (0..size).collect(),
        }
    }

    fn compute(&mut self, instructions: &Vec<u8>) {
        let string_length = self.hash.len();
        for instruction in instructions.iter() {
            let instruction = *instruction as usize;
            for i in 0..(instruction / 2) {
                self.hash.swap(
                    (self.pointer + i) % string_length,
                    (self.pointer + instruction - i - 1) % string_length,
                );
            }
            self.pointer = (self.pointer + instruction + self.skip_size) % string_length;
            self.skip_size = (self.skip_size + 1) % string_length;
        }
    }
    fn compute_dense_hash(&mut self, instructions: &Vec<u8>) -> Vec<u8> {
        for _ in 0..64 {
            self.compute(&instructions);
        }
        let mut dense_hash = vec![];
        for i in 0..16 {
            let slice = &self.hash[(i * 16)..((i + 1) * 16)];
            dense_hash.push(slice.iter().fold(0, |acc, x| acc ^ (*x as u8)));
        }
        dense_hash
    }
}

#[test]
fn test_examples() {
    assert_eq!(
        Some(826),
        knot_tying_hash_round("120,93,0,90,5,80,129,74,1,165,204,255,254,2,50,113")
    );
    assert_eq!(
        String::from("a2582a3a0e66e6e86e3812dcb672a272"),
        knot_tying_hash("")
    );
    assert_eq!(
        String::from("33efeb34ea91902bb2f59c9920caa6cd"),
        knot_tying_hash("AoC 2017")
    );
    assert_eq!(
        String::from("63960835bcdc130f0b66d7ff4f6a5a8e"),
        knot_tying_hash("1,2,4")
    );
    assert_eq!(
        String::from("3efbe78a8d82f29979031a4aa0b16a9d"),
        knot_tying_hash("1,2,3")
    );
}
