extern crate num;
extern crate regex;

use std::cell::Cell;
use std::collections::HashSet;

use regex::Regex;

fn main() {
    let result = std::fs::read_to_string("src/bin/day12.txt")
        .map(|file| {
            let moons = file
                .lines()
                .filter(|line| !line.is_empty())
                .map(Moon::from_file)
                .collect::<Vec<Moon>>();
            run_n_bodies(moons)
        })
        .expect("Unable to open file");

    println!("{:?}", result);
}

fn run_n_bodies(moons: Vec<Moon>) -> i64 {
    let x_min = run_n_bodies_dim(moons.iter().map(|moon| moon.x.clone()).collect());
    println!(
        "Found duplication for X dimension after {} iterations",
        x_min
    );
    let y_min = run_n_bodies_dim(moons.iter().map(|moon| moon.y.clone()).collect());
    println!(
        "Found duplication for Y dimension after {} iterations",
        y_min
    );
    let z_min = run_n_bodies_dim(moons.iter().map(|moon| moon.z.clone()).collect());
    println!(
        "Found duplication for Z dimension after {} iterations",
        z_min
    );
    num::integer::lcm(num::integer::lcm(x_min as i64, y_min as i64), z_min as i64)
}

fn run_n_bodies_dim(moons: Vec<MoonDim>) -> i32 {
    let mut seen: HashSet<Vec<MoonDim>> = HashSet::new();
    seen.insert(moons.clone());

    let curr = Cell::new(moons.clone());
    let mut i: i32 = 0;
    loop {
        let new = n_bodies_step(curr.take());
        i += 1;

        if seen.contains(&new) {
            break;
        }

        seen.insert(new.clone());
        curr.replace(new);
    }
    i
}

fn n_bodies_step(moons: Vec<MoonDim>) -> Vec<MoonDim> {
    let mut moons: Vec<(MoonDim, MoonDim)> =
        moons.iter().map(|moon| (*moon, moon.clone())).collect();
    for start in 1..moons.len() {
        let base = start - 1;
        for i in start..moons.len() {
            if moons[i].0.pos > moons[base].0.pos {
                moons[i].1.vel -= 1;
                moons[base].1.vel += 1;
            } else if moons[i].0.pos < moons[base].0.pos {
                moons[i].1.vel += 1;
                moons[base].1.vel -= 1;
            }
        }

        moons[base].1.pos += moons[base].1.vel;
    }

    let last = moons.len() - 1;
    moons[last].1.pos += moons[last].1.vel;

    moons.iter().map(|(_, new)| *new).collect()
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Moon {
    x: MoonDim,
    y: MoonDim,
    z: MoonDim,
}

impl Moon {
    fn from_file(line: &str) -> Moon {
        let re = Regex::new(r"<x=(-?[0-9]+), y=(-?[0-9]+), z=(-?[0-9]+)>").unwrap();
        let capture = re.captures(line).unwrap();
        Moon {
            x: MoonDim {
                pos: capture.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                vel: 0,
            },
            y: MoonDim {
                pos: capture.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                vel: 0,
            },
            z: MoonDim {
                pos: capture.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                vel: 0,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct MoonDim {
    pos: i32,
    vel: i32,
}

#[cfg(test)]
mod test {
    use super::*;

    impl Moon {
        fn new(x: i32, y: i32, z: i32, d_x: i32, d_y: i32, d_z: i32) -> Moon {
            Moon {
                x: MoonDim::new(x, d_x),
                y: MoonDim::new(y, d_y),
                z: MoonDim::new(z, d_z),
            }
        }
    }

    impl MoonDim {
        fn new(pos: i32, vel: i32) -> MoonDim {
            MoonDim { pos, vel }
        }
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            Moon::from_file("<x=4, y=5, z=6>"),
            Moon {
                x: MoonDim::new(4, 0),
                y: MoonDim::new(5, 0),
                z: MoonDim::new(6, 0),
            }
        )
    }

    #[test]
    fn test_parse_negative() {
        assert_eq!(
            Moon::from_file("<x=-4, y=-5, z=-6>"),
            Moon {
                x: MoonDim::new(-4, 0),
                y: MoonDim::new(-5, 0),
                z: MoonDim::new(-6, 0),
            }
        )
    }

    fn n_bodies_step_moon(moons: Vec<Moon>) -> Vec<Moon> {
        let x: Vec<MoonDim> = n_bodies_step(moons.iter().map(|moon| moon.x.clone()).collect());
        let y: Vec<MoonDim> = n_bodies_step(moons.iter().map(|moon| moon.y.clone()).collect());
        let z: Vec<MoonDim> = n_bodies_step(moons.iter().map(|moon| moon.z.clone()).collect());

        (0..x.len())
            .map(|i| Moon {
                x: x[i].clone(),
                y: y[i].clone(),
                z: z[i].clone(),
            })
            .collect()
    }

    #[test]
    fn test_n_bodies_step_1() {
        assert_eq!(
            n_bodies_step_moon(vec![
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
    fn test_n_bodies_step_2() {
        assert_eq!(
            n_bodies_step_moon(vec![
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
    fn test_run_nbodies() {
        assert_eq!(
            run_n_bodies(vec![
                Moon::new(-1, 0, 2, 0, 0, 0),
                Moon::new(2, -10, -7, 0, 0, 0),
                Moon::new(4, -8, 8, 0, 0, 0),
                Moon::new(3, 5, -1, 0, 0, 0),
            ]),
            2772
        )
    }
}
