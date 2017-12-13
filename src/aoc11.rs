use std::cmp;
use std::str::FromStr;

pub fn find_shortest_path(path: &str) -> Option<u32> {
    let directions = path.split(',')
        .map(Direction::from_str)
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    let mut start = Hex::new();
    directions.iter().for_each(|x| start.step(&x));
    Some(cmp::max(
        cmp::max(start.x.abs(), start.y.abs()),
        start.z.abs(),
    ) as u32)
}

struct Error;

enum Direction {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl Direction {
    fn coordinates(&self) -> (i32, i32, i32) {
        match *self {
            Direction::North => (0, 1, -1),
            Direction::NorthEast => (1, 0, -1),
            Direction::SouthEast => (1, -1, 0),
            Direction::South => (0, -1, 1),
            Direction::SouthWest => (-1, 0, 1),
            Direction::NorthWest => (-1, 1, 0),
        }
    }
}

impl FromStr for Direction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_ref() {
            "n" => Ok(Direction::North),
            "ne" => Ok(Direction::NorthEast),
            "se" => Ok(Direction::SouthEast),
            "s" => Ok(Direction::South),
            "sw" => Ok(Direction::SouthWest),
            "nw" => Ok(Direction::NorthWest),
            _ => Err(Error),
        }
    }
}

struct Hex {
    x: i32,
    y: i32,
    z: i32,
}

impl Hex {
    fn new() -> Hex {
        Hex { x: 0, y: 0, z: 0 }
    }

    fn step(&mut self, direction: &Direction) {
        let (x, y, z) = direction.coordinates();
        self.x += x;
        self.y += y;
        self.z += z;
    }
}

#[test]
fn test_examples() {
    assert_eq!(Some(3), find_shortest_path("ne,ne,ne"));
    assert_eq!(Some(0), find_shortest_path("ne,ne,sw,sw"));
    assert_eq!(Some(2), find_shortest_path("ne,ne,s,s"));
    assert_eq!(Some(3), find_shortest_path("se,sw,se,sw,sw"));
}
