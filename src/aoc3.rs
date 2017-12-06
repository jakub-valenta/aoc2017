use std::collections::HashMap;

pub fn distance(cell: u32) -> Option<u32> {
    let (x, y) = calculate_position(cell)?;
    Some((x.abs() + y.abs()) as u32)
}

pub fn calculate_bigger_than(limit: u32) -> Option<u32> {
    let mut cell = 1;
    let mut map: HashMap<(i32, i32), u32> = HashMap::new();
    loop {
        let position = calculate_position(cell)?;
        let value = calculate_value(&position, &map);
        cell += 1;
        if value > limit {
            return Some(value);
        } else {
            map.insert(position, value);
        }
    }
}

fn calculate_position(cell: u32) -> Option<(i32, i32)> {
    match cell {
        0 => None,
        1 => Some((0, 0)),
        x => {
            let x = x as i32;
            let mut layer = (cell as f32).sqrt().ceil() as i32;
            if layer % 2 == 0 {
                layer += 1;
            }
            let layer_start = ((layer - 2) as f32).powi(2) as i32 + 1;
            let zero = layer_start + layer / 2 - 1;
            let layer_distance = layer / 2;
            match x - zero {
                x if x <= layer_distance => Some((layer_distance, x)),
                x if x <= 3 * layer_distance => Some((layer_distance * 2 - x, layer_distance)),
                x if x <= 5 * layer_distance => Some((-layer_distance, layer_distance * 4 - x)),
                x => Some((x - layer_distance * 6, -layer_distance)),
            }
        }
    }
}

fn calculate_value(position: &(i32, i32), map: &HashMap<(i32, i32), u32>) -> u32 {
    if *position == (0, 0) {
        1
    } else {
        map.iter()
            .filter_map(|record| {
                let (x, y) = *record.0;
                if (x - position.0).abs() <= 1 && (y - position.1).abs() <= 1 {
                    Some(record.1)
                } else {
                    None
                }
            })
            .fold(0, |acc, x| acc + x)
    }
}

#[test]
fn test_invalid() {
    assert_eq!(None, distance(0));
}

#[test]
fn test_examples() {
    assert_eq!(Some(0), distance(1));
    assert_eq!(Some(3), distance(10));
    assert_eq!(Some(3), distance(12));
    assert_eq!(Some(3), distance(14));
    assert_eq!(Some(4), distance(21));
    assert_eq!(Some(2), distance(23));
    assert_eq!(Some(31), distance(1024));
}

#[test]
fn test_examples2() {
    assert_eq!(Some(2), calculate_bigger_than(1));
    assert_eq!(Some(11), calculate_bigger_than(10));
    assert_eq!(Some(23), calculate_bigger_than(12));
    assert_eq!(Some(806), calculate_bigger_than(747));
}

#[test]
fn test_calculate_position() {
    assert_eq!(None, calculate_position(0));
    assert_eq!(Some((0, 0)), calculate_position(1));
    assert_eq!(Some((1, 0)), calculate_position(2));
    assert_eq!(Some((2, -1)), calculate_position(10));
    assert_eq!(Some((2, 2)), calculate_position(13));
    assert_eq!(Some((1, 2)), calculate_position(14));
    assert_eq!(Some((-2, 2)), calculate_position(17));
    assert_eq!(Some((-2, 1)), calculate_position(18));
    assert_eq!(Some((-2, -1)), calculate_position(20));
    assert_eq!(Some((-1, -2)), calculate_position(22));
    assert_eq!(Some((0, -2)), calculate_position(23));
    assert_eq!(Some((1, -2)), calculate_position(24));
    assert_eq!(Some((2, -2)), calculate_position(25));
}

#[test]
fn test_calculate_value() {
    assert_eq!(1, calculate_value(&(0, 0), &HashMap::new()));
}
