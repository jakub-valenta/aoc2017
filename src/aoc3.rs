
pub fn distance(cell: u32) -> Option<u32> {
    if cell == 0 {
        return None;
    } else if cell == 1 {
        return Some(0);
    }
    let mut layer = (cell as f32).sqrt().ceil() as u32;
    if layer % 2 == 0 {
        layer += 1;
    }
    let layer_start = (layer as f32 - 2.0).powi(2) as u32 + 1;
    let zero = layer_start + layer / 2 - 1;
    let diff_from_zero = (cell as i32 - zero as i32).abs() as u32 % (layer - 1);
    Some(
        layer / 2 + u32::min(diff_from_zero, layer - diff_from_zero - 1),
    )
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
