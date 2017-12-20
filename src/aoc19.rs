

pub fn walk_path(path: &str) -> Option<(String, u32)> {
    let map = path.lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut position = Position::new(
        0,
        map[0].iter().enumerate().find(|&(_, item)| *item == '|')?.0,
    );
    let mut direction = Direction::new();
    let mut sign = String::new();
    let mut steps = 0;
    loop {
        steps += 1;
        position = direction.step(&position);
        match map[position.x][position.y] {
            ' ' => break,
            '|' => continue,
            '-' => continue,
            '+' => {
                let left = direction.left();
                let left_next = left.step(&position);
                direction = if map[left_next.x][left_next.y] == ' ' {
                    direction.right()
                } else {
                    left
                }
            }
            c => sign.push(c),
        }
    }
    Some((sign, steps))
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Default, Hash, Eq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position { x: x, y: y }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Default, Hash, Eq)]
struct Direction {
    x: isize,
    y: isize,
}

impl Direction {
    fn new() -> Direction {
        Direction { x: 1, y: 0 }
    }
    fn left(&self) -> Direction {
        Direction {
            x: -self.y,
            y: self.x,
        }
    }
    fn right(&self) -> Direction {
        Direction {
            x: self.y,
            y: -self.x,
        }
    }
    fn step(&self, position: &Position) -> Position {
        Position::new(
            (position.x as isize + self.x) as usize,
            (position.y as isize + self.y) as usize,
        )
    }
}

#[test]
fn test_examples() {
    assert_eq!(
        Some((String::from("ABCDEF"), 38)),
        walk_path(
            "     |          \n     |  +--+    \n     A  |  C    \n F---|----E|--+ \n     |  |  |  D \n     +B-+  +--+ ",
        )
    );
}

#[test]
fn test_direction_left() {
    let mut direction = Direction::new();
    assert_eq!(Direction { x: 1, y: 0 }, direction);
    direction = direction.left();
    assert_eq!(Direction { x: 0, y: 1 }, direction);
    direction = direction.left();
    assert_eq!(Direction { x: -1, y: 0 }, direction);
    direction = direction.left();
    assert_eq!(Direction { x: 0, y: -1 }, direction);
}

#[test]
fn test_direction_right() {
    let mut direction = Direction::new();
    assert_eq!(Direction { x: 1, y: 0 }, direction);
    direction = direction.right();
    assert_eq!(Direction { x: 0, y: -1 }, direction);
    direction = direction.right();
    assert_eq!(Direction { x: -1, y: 0 }, direction);
    direction = direction.right();
    assert_eq!(Direction { x: 0, y: 1 }, direction);
}
