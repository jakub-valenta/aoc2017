use std::str::FromStr;
use utils;

pub fn dance(moves: &str) -> Option<String> {
    Some(
        one_dance(&parse_moves(moves)?, "abcdefghijklmnop".chars().collect())
            .iter()
            .collect(),
    )
}

pub fn dance_whole_night(moves: &str) -> Option<String> {
    let moves = parse_moves(moves)?;
    Some(
        (0..(1000000000 % find_loop(&moves)))
            .fold("abcdefghijklmnop".chars().collect(), |dancers, _| {
                one_dance(&moves, dancers)
            })
            .iter()
            .collect(),
    )
}

fn one_dance(dance: &Vec<Move>, dancers: Vec<char>) -> Vec<char> {
    dance.iter().fold(dancers, |acc, step| step.step(&acc))
}

fn find_loop(moves: &Vec<Move>) -> u32 {
    let mut steps = 0;
    let mut dancers = "abcdefghijklmnop".chars().collect();
    loop {
        steps += 1;
        dancers = one_dance(&moves, dancers);
        if dancers.iter().collect::<String>() == "abcdefghijklmnop" {
            return steps;
        }
    }
}

fn parse_moves(moves: &str) -> Option<Vec<Move>> {
    let mut dance = vec![];
    for token in moves.split(',') {
        dance.push(Move::from_str(token).ok()?);
    }
    Some(dance)
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone)]
enum Move {
    Spin(u32),
    Exchange(u32, u32),
    Partner(char, char),
}

impl Move {
    fn step(&self, dancers: &Vec<char>) -> Vec<char> {
        match *self {
            Move::Spin(length) => {
                let mut moved = vec![];
                let dancer_count = dancers.len();
                for i in 0..dancer_count {
                    moved.push(
                        dancers[(i + dancers.len() - length as usize) % dancer_count],
                    );
                }
                moved
            }
            Move::Exchange(left, right) => {
                let mut moved = dancers.clone();
                moved.swap(left as usize, right as usize);
                moved
            }
            Move::Partner(left, right) => {
                dancers
                    .iter()
                    .map(|x| if *x == left {
                        right
                    } else if *x == right {
                        left
                    } else {
                        *x
                    })
                    .collect()
            }
        }
    }
}

impl FromStr for Move {
    type Err = utils::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(0) {
            Some('s') => Ok(Move::Spin(s.split_at(1).1.parse()?)),
            Some('x') => {
                let mut tokens = s.split_at(1).1.split('/');
                Ok(Move::Exchange(
                    tokens.next().ok_or(utils::Error)?.parse()?,
                    tokens.next().ok_or(utils::Error)?.parse()?,
                ))
            }
            Some('p') => Ok(Move::Partner(
                s.chars().nth(1).ok_or(utils::Error)?,
                s.chars().nth(3).ok_or(utils::Error)?,
            )),
            _ => Err(utils::Error),
        }
    }
}

#[test]
fn test_examples() {
    assert_eq!(
        Some(String::from("opaedcbfghijklmn")),
        dance("s2,x4/5,pe/b")
    );
}

#[test]
fn test_from_str() {
    assert_eq!(Err(utils::Error), Move::from_str(""));
    assert_eq!(Err(utils::Error), Move::from_str("a3/4"));
    assert_eq!(Err(utils::Error), Move::from_str("sa"));
    assert_eq!(Ok(Move::Spin(10)), Move::from_str("s10"));
    assert_eq!(Ok(Move::Exchange(3, 10)), Move::from_str("x3/10"));
    assert_eq!(Ok(Move::Partner('a', 'c')), Move::from_str("pa/c"));
}
