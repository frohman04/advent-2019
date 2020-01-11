#![allow(unused_imports)]

#[macro_use]
extern crate maplit;

use std::collections::HashMap;

fn main() {
    let result = std::fs::read_to_string("src/bin/day06.txt")
        .map(|file| {
            let orbits = file
                .lines()
                .filter(|line| !line.is_empty())
                .map(|line| {
                    let pieces = line.split(')').collect::<Vec<&str>>();
                    Orbit::new(pieces[0], pieces[1])
                })
                .collect::<Vec<Orbit>>();

            verify(&orbits)
        })
        .expect("Unable to open file");

    println!("{}", result);
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Orbit<'a> {
    center: &'a str,
    satellite: &'a str,
}

impl<'a> Orbit<'a> {
    pub fn new(center: &'a str, satellite: &'a str) -> Orbit<'a> {
        Orbit { center, satellite }
    }
}

fn verify(orbits: &Vec<Orbit>) -> i32 {
    let mut orbit_map: HashMap<&str, &str> = HashMap::new();
    for orbit in orbits {
        orbit_map.insert(orbit.satellite, orbit.center);
    }

    let mut total = 0;
    for satellite in orbit_map.keys() {
        total += orbit_count(satellite, &orbit_map);
    }

    total
}

fn orbit_count(satellite: &str, orbit_map: &HashMap<&str, &str>) -> i32 {
    let mut count = 0;
    let mut curr = satellite.clone();
    while curr != "COM" {
        count += 1;
        curr = orbit_map.get(curr).unwrap();
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_orbit_count_com() {
        assert_eq!(
            orbit_count(
                "COM",
                &hashmap!(
                    "B" => "COM",
                    "C" => "B",
                    "D" => "C",
                    "E" => "D",
                    "F" => "E",
                    "G" => "B",
                    "H" => "G",
                    "I" => "D",
                    "J" => "E",
                    "K" => "J",
                    "L" => "K",
                )
            ),
            0
        )
    }

    #[test]
    fn test_orbit_count_d() {
        assert_eq!(
            orbit_count(
                "D",
                &hashmap!(
                    "B" => "COM",
                    "C" => "B",
                    "D" => "C",
                    "E" => "D",
                    "F" => "E",
                    "G" => "B",
                    "H" => "G",
                    "I" => "D",
                    "J" => "E",
                    "K" => "J",
                    "L" => "K",
                )
            ),
            3
        )
    }

    #[test]
    fn test_orbit_count_l() {
        assert_eq!(
            orbit_count(
                "L",
                &hashmap!(
                    "B" => "COM",
                    "C" => "B",
                    "D" => "C",
                    "E" => "D",
                    "F" => "E",
                    "G" => "B",
                    "H" => "G",
                    "I" => "D",
                    "J" => "E",
                    "K" => "J",
                    "L" => "K",
                )
            ),
            7
        )
    }

    #[test]
    fn test_verify() {
        assert_eq!(
            verify(&vec![
                Orbit::new("COM", "B"),
                Orbit::new("B", "C"),
                Orbit::new("C", "D"),
                Orbit::new("D", "E"),
                Orbit::new("E", "F"),
                Orbit::new("B", "G"),
                Orbit::new("G", "H"),
                Orbit::new("D", "I"),
                Orbit::new("E", "J"),
                Orbit::new("J", "K"),
                Orbit::new("K", "L"),
            ]),
            42
        );
    }
}
