use aoc10;

pub fn count_used_squares(input: &str) -> u32 {
    create_memory_map(input).iter().fold(0, |acc, x| {
        acc + x.iter().fold(0, |acc, y| acc + y.count_ones())
    })
}

pub fn count_regions(input: &str) -> u32 {
    let mut memory: Vec<Vec<char>> = create_memory_map(input)
        .iter()
        .map(|x| {
            x.iter()
                .map(|y| format!("{:08b}", y))
                .fold(String::new(), |acc, z| acc + &z)
                .chars()
                .collect()
        })
        .collect();
    let mut region_count = 0;
    for x in 0..128 {
        for y in 0..128 {
            if memory[x][y] == '1' {
                region_count += 1;
                remove_neighbours(&mut memory, x, y);
            }
        }
    }
    region_count
}

fn remove_neighbours(memory: &mut Vec<Vec<char>>, x: usize, y: usize) {
    if memory[x][y] == '1' {
        memory[x][y] = '0';
        if x != 0 {
            remove_neighbours(memory, x - 1, y);
        }
        if x != 127 {
            remove_neighbours(memory, x + 1, y);
        }
        if y != 0 {
            remove_neighbours(memory, x, y - 1);
        }
        if y != 127 {
            remove_neighbours(memory, x, y + 1);
        }
    }
}

fn create_memory_map(input: &str) -> Vec<Vec<u8>> {
    (0..128)
        .map(|x| aoc10::knot_tying_hash_raw(&format!("{}-{}", input, x)))
        .collect()
}

#[test]
fn test_examples() {
    assert_eq!(8108, count_used_squares("flqrgnkx"));
    assert_eq!(1242, count_regions("flqrgnkx"));
}
