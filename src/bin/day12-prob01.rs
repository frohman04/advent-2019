extern crate regex;

use regex::Regex;
use std::cell::Cell;

fn main() {
    let result = std::fs::read_to_string("src/bin/day12.txt")
        .map(|file| {
            let moons = file
                .lines()
                .filter(|line| !line.is_empty())
                .map(Moon::from_file)
                .collect::<Vec<Moon>>();
            run_n_bodies(moons, 1000)
        })
        .expect("Unable to open file");

    println!("{:?}", result);
}

fn run_n_bodies(moons: Vec<Moon>, num_steps: usize) -> i32 {
    let curr = Cell::new(moons.clone());
    for _ in 0..num_steps {
        curr.replace(n_bodies_step(curr.take()));
    }
    curr.into_inner()
        .iter()
        .map(|moon| moon.total_energy())
        .sum()
}

fn n_bodies_step(moons: Vec<Moon>) -> Vec<Moon> {
    let mut moons: Vec<(Moon, Moon)> = moons.iter().map(|moon| (*moon, moon.clone())).collect();
    for start in 1..moons.len() {
        let base = start - 1;
        for i in start..moons.len() {
            if moons[i].0.x > moons[base].0.x {
                moons[i].1.d_x -= 1;
                moons[base].1.d_x += 1;
            } else if moons[i].0.x < moons[base].0.x {
                moons[i].1.d_x += 1;
                moons[base].1.d_x -= 1;
            }

            if moons[i].0.y > moons[base].0.y {
                moons[i].1.d_y -= 1;
                moons[base].1.d_y += 1;
            } else if moons[i].0.y < moons[base].0.y {
                moons[i].1.d_y += 1;
                moons[base].1.d_y -= 1;
            }

            if moons[i].0.z > moons[base].0.z {
                moons[i].1.d_z -= 1;
                moons[base].1.d_z += 1;
            } else if moons[i].0.z < moons[base].0.z {
                moons[i].1.d_z += 1;
                moons[base].1.d_z -= 1;
            }
        }

        moons[base].1.x += moons[base].1.d_x;
        moons[base].1.y += moons[base].1.d_y;
        moons[base].1.z += moons[base].1.d_z;
    }

    let last = moons.len() - 1;
    moons[last].1.x += moons[last].1.d_x;
    moons[last].1.y += moons[last].1.d_y;
    moons[last].1.z += moons[last].1.d_z;

    moons.iter().map(|(_, new)| *new).collect()
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
    d_x: i32,
    d_y: i32,
    d_z: i32,
}

impl Moon {
    fn from_file(line: &str) -> Moon {
        let re = Regex::new(r"<x=(-?[0-9]+), y=(-?[0-9]+), z=(-?[0-9]+)>").unwrap();
        let capture = re.captures(line).unwrap();
        Moon {
            x: capture.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            y: capture.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            z: capture.get(3).unwrap().as_str().parse::<i32>().unwrap(),
            d_x: 0,
            d_y: 0,
            d_z: 0,
        }
    }

    fn potential_energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn kinetic_energy(&self) -> i32 {
        self.d_x.abs() + self.d_y.abs() + self.d_z.abs()
    }

    fn total_energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl Moon {
        fn new(x: i32, y: i32, z: i32, d_x: i32, d_y: i32, d_z: i32) -> Moon {
            Moon {
                x,
                y,
                z,
                d_x,
                d_y,
                d_z,
            }
        }
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            Moon::from_file("<x=4, y=5, z=6>"),
            Moon {
                x: 4,
                y: 5,
                z: 6,
                d_x: 0,
                d_y: 0,
                d_z: 0
            }
        )
    }

    #[test]
    fn test_parse_negative() {
        assert_eq!(
            Moon::from_file("<x=-4, y=-5, z=-6>"),
            Moon {
                x: -4,
                y: -5,
                z: -6,
                d_x: 0,
                d_y: 0,
                d_z: 0
            }
        )
    }

    #[test]
    fn test_nbodies_step_1() {
        assert_eq!(
            n_bodies_step(vec![
                Moon::new(-1, 0, 2, 0, 0, 0),
                Moon::new(2, -10, -7, 0, 0, 0),
                Moon::new(4, -8, 8, 0, 0, 0),
                Moon::new(3, 5, -1, 0, 0, 0),
            ]),
            vec![
                Moon::new(2, -1, 1, 3, -1, -1),
                Moon::new(3, -7, -4, 1, 3, 3),
                Moon::new(1, -7, 5, -3, 1, -3),
                Moon::new(2, 2, 0, -1, -3, 1),
            ]
        )
    }

    #[test]
    fn test_nbodies_step_2() {
        assert_eq!(
            n_bodies_step(vec![
                Moon::new(2, -1, 1, 3, -1, -1),
                Moon::new(3, -7, -4, 1, 3, 3),
                Moon::new(1, -7, 5, -3, 1, -3),
                Moon::new(2, 2, 0, -1, -3, 1),
            ]),
            vec![
                Moon::new(5, -3, -1, 3, -2, -2),
                Moon::new(1, -2, 2, -2, 5, 6),
                Moon::new(1, -4, -1, 0, 3, -6),
                Moon::new(1, -4, 2, -1, -6, 2),
            ]
        )
    }

    #[test]
    fn test_total_energy() {
        assert_eq!(Moon::new(2, 1, -3, -3, -2, 1).total_energy(), 36)
    }

    #[test]
    fn test_run_n_bodies_1() {
        assert_eq!(
            run_n_bodies(
                vec![
                    Moon::new(-1, 0, 2, 0, 0, 0),
                    Moon::new(2, -10, -7, 0, 0, 0),
                    Moon::new(4, -8, 8, 0, 0, 0),
                    Moon::new(3, 5, -1, 0, 0, 0),
                ],
                10
            ),
            179
        )
    }

    #[test]
    fn test_run_n_bodies_2() {
        assert_eq!(
            run_n_bodies(
                vec![
                    Moon::new(-8, -10, 0, 0, 0, 0),
                    Moon::new(5, 5, 10, 0, 0, 0),
                    Moon::new(2, -7, 3, 0, 0, 0),
                    Moon::new(9, -8, -3, 0, 0, 0),
                ],
                100
            ),
            1940
        )
    }
}
