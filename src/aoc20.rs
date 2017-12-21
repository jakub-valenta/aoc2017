use std::str::FromStr;
use utils;

pub fn closest_particle(particles: &str) -> Option<u32> {
    let particles = particles
        .lines()
        .map(Particle::from_str)
        .collect::<Result<Vec<Particle>, _>>()
        .ok()?;
    Some(
        particles
            .iter()
            .enumerate()
            .min_by(|&(_, x), &(_, y)| {
                x.distance_from_start(1000).cmp(
                    &y.distance_from_start(1000),
                )
            })?
            .0 as u32,
    )
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone)]
struct Particle {
    position: (i32, i32, i32),
    vector: (i32, i32, i32),
    acceleration: (i32, i32, i32),
}

impl Particle {
    fn distance_from_start(&self, time: i32) -> u32 {
        let position = (
            Particle::calculate_displacement(
                self.position.0,
                self.vector.0,
                self.acceleration.0,
                time,
            ),
            Particle::calculate_displacement(
                self.position.1,
                self.vector.1,
                self.acceleration.1,
                time,
            ),
            Particle::calculate_displacement(
                self.position.2,
                self.vector.2,
                self.acceleration.2,
                time,
            ),
        );
        (position.0.abs() + position.1.abs() + position.2.abs()) as u32
    }

    fn calculate_displacement(s0: i32, v0: i32, a: i32, t: i32) -> i32 {
        s0 + v0 * t + a * t.pow(2) / 2
    }
}

impl FromStr for Particle {
    type Err = utils::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tupples = s.split(' ').map(tupple_from_str);
        Ok(Particle {
            position: tupples.next().ok_or(utils::Error)??,
            vector: tupples.next().ok_or(utils::Error)??,
            acceleration: tupples.next().ok_or(utils::Error)??,
        })
    }
}

fn tupple_from_str(s: &str) -> Result<(i32, i32, i32), utils::Error> {
    let values = utils::parse_numbers(
        s.split('<')
            .nth(1)
            .ok_or(utils::Error)?
            .split('>')
            .nth(0)
            .ok_or(utils::Error)?,
        ',',
    ).ok_or(utils::Error)?;
    Ok((values[0], values[1], values[2]))
}

#[test]
fn test_examples() {
    assert_eq!(
        Some(0),
        closest_particle(
            "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>\np=<4,0,0>, v=<0,0,0>, a=<-2,0,0>",
        )
    );
}

#[test]
fn test_particle_from_str() {
    assert_eq!(
        Ok(Particle {
            position: (3, 0, 0),
            vector: (2, 0, 0),
            acceleration: (-1, 0, 0),
        }),
        Particle::from_str("p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>")
    );
}
