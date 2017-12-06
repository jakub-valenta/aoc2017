pub fn distance(cell: u32) -> Option<u32> {
    Some(Position::new_from_cell(cell)?.distance_from_start())
}

pub fn calculate_bigger_than(limit: u32) -> Option<u32> {
    let mut cell = 1;
    let mut map = vec![];
    loop {
        let position = Position::new_from_cell(cell)?;
        let value = calculate_value(&position, &map);
        cell += 1;
        if value > limit {
            return Some(value);
        } else {
            map.push(Cell::new(position, value));
        }
    }
}

fn calculate_value(position: &Position, map: &Vec<Cell>) -> u32 {
    if position.is_start() {
        1
    } else {
        map.iter()
            .filter(|cell| position.is_neighbour(&cell.position))
            .fold(0, |acc, x| acc + x.value)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Default, Hash, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x: x, y: y }
    }

    fn new_from_cell(cell: u32) -> Option<Position> {
        match cell {
            0 => None,
            1 => Some(Position::new(0, 0)),
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
                    x if x <= layer_distance => Some(Position::new(layer_distance, x)),
                    x if x <= 3 * layer_distance => Some(Position::new(
                        layer_distance * 2 - x,
                        layer_distance,
                    )),
                    x if x <= 5 * layer_distance => Some(Position::new(
                        -layer_distance,
                        layer_distance * 4 - x,
                    )),
                    x => Some(Position::new(x - layer_distance * 6, -layer_distance)),
                }
            }
        }
    }

    fn is_neighbour(&self, position: &Position) -> bool {
        (self.x - position.x).abs() <= 1 && (self.y - position.y).abs() <= 1
    }

    fn distance_from_start(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }

    fn is_start(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Default, Hash, Eq)]
struct Cell {
    position: Position,
    value: u32,
}

impl Cell {
    fn new(position: Position, value: u32) -> Cell {
        Cell {
            position: position,
            value: value,
        }
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
    assert_eq!(None, Position::new_from_cell(0));
    assert_eq!(Some(Position::new(0, 0)), Position::new_from_cell(1));
    assert_eq!(Some(Position::new(1, 0)), Position::new_from_cell(2));
    assert_eq!(Some(Position::new(2, -1)), Position::new_from_cell(10));
    assert_eq!(Some(Position::new(2, 2)), Position::new_from_cell(13));
    assert_eq!(Some(Position::new(1, 2)), Position::new_from_cell(14));
    assert_eq!(Some(Position::new(-2, 2)), Position::new_from_cell(17));
    assert_eq!(Some(Position::new(-2, 1)), Position::new_from_cell(18));
    assert_eq!(Some(Position::new(-2, -1)), Position::new_from_cell(20));
    assert_eq!(Some(Position::new(-1, -2)), Position::new_from_cell(22));
    assert_eq!(Some(Position::new(0, -2)), Position::new_from_cell(23));
    assert_eq!(Some(Position::new(1, -2)), Position::new_from_cell(24));
    assert_eq!(Some(Position::new(2, -2)), Position::new_from_cell(25));
}

#[test]
fn test_calculate_value() {
    assert_eq!(1, calculate_value(&Position::new(0, 0), &vec![]));
    assert_eq!(
        1,
        calculate_value(
            &Position::new(0, 1),
            &vec![Cell::new(Position::new_from_cell(1).unwrap(), 1)],
        )
    );
    assert_eq!(
        2,
        calculate_value(
            &Position::new(0, 1),
            &vec![
                Cell::new(Position::new_from_cell(1).unwrap(), 1),
                Cell::new(Position::new_from_cell(2).unwrap(), 1),
            ],
        )
    );
}
