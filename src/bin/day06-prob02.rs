#![allow(unused_imports)]

#[macro_use]
extern crate maplit;

use std::collections::HashMap;

fn main() {
    let result = std::fs::read_to_string("src/bin/day06.txt")
        .map(|file| {
            let orbits = file
                .split('\n')
                .filter(|line| !line.is_empty())
                .map(|line| {
                    let pieces = line.split(')').collect::<Vec<&str>>();
                    Orbit::new(pieces[0], pieces[1])
                })
                .collect::<Vec<Orbit>>();

            num_transfers(&orbits)
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

fn num_transfers(orbits: &Vec<Orbit>) -> u32 {
    let mut orbit_map: HashMap<&str, &str> = HashMap::new();
    for orbit in orbits {
        orbit_map.insert(orbit.satellite, orbit.center);
    }

    let mut you_orbits = orbit_list("YOU", &orbit_map);
    you_orbits.reverse();
    let you_orbits = you_orbits;

    let mut san_orbits = orbit_list("SAN", &orbit_map);
    san_orbits.reverse();
    let san_orbits = san_orbits;

    let mut i = 0;
    while i < you_orbits.len().min(san_orbits.len()) && you_orbits[i] == san_orbits[i] {
        i += 1;
    }
    let common_count = i;

    ((you_orbits.len() - common_count - 1) + (san_orbits.len() - common_count - 1)) as u32
}

fn orbit_list<'a>(satellite: &'a str, orbit_map: &HashMap<&'a str, &'a str>) -> Vec<&'a str> {
    let mut lst: Vec<&'a str> = Vec::new();
    let mut curr = satellite.clone();
    while curr != "COM" {
        lst.push(curr);
        curr = orbit_map.get(curr).unwrap();
    }
    lst
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_orbit_list_com() {
        assert_eq!(
            orbit_list(
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
            Vec::new() as Vec<&str>
        )
    }

    #[test]
    fn test_orbit_list_d() {
        assert_eq!(
            orbit_list(
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
            vec!["D", "C", "B"]
        )
    }

    #[test]
    fn test_orbit_list_l() {
        assert_eq!(
            orbit_list(
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
            vec!["L", "K", "J", "E", "D", "C", "B"]
        )
    }

    #[test]
    fn test_num_transfers() {
        assert_eq!(
            num_transfers(&vec![
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
                Orbit::new("K", "YOU"),
                Orbit::new("I", "SAN"),
            ]),
            4
        );
    }
}
