use std::collections::HashMap;

fn main() {
    let result = std::fs::read_to_string("src/bin/day03.txt")
        .map(|file| {
            let lines = file
                .split("\n")
                .filter(|line| line.len() > 0)
                .collect::<Vec<&str>>();
            let code = lines
                .iter()
                .map(|line| {
                    line.split(",")
                        .map(|item| Move::from_text(item))
                        .collect::<Vec<Move>>()
                })
                .collect::<Vec<Vec<Move>>>();

            let wire1 = code[0].clone();
            let wire2 = code[1].clone();

            closest_intersection(wire1, wire2)
        })
        .expect("Unable to open file");

    println!("{}", result)
}

fn closest_intersection(wire1: Vec<Move>, wire2: Vec<Move>) -> u32 {
    let mut path: HashMap<(i32, i32), u32> = HashMap::new();

    for point in Tracer::new(wire1) {
        if !path.contains_key(&point.0) {
            path.insert(point.0, point.1);
        }
    }

    let mut intersections: Vec<((i32, i32), u32)> = Vec::new();
    for point in Tracer::new(wire2) {
        if path.contains_key(&point.0) && !((point.0).0 == 0 && (point.0).1 == 0) {
            intersections.push((point.0, point.1 + path.get(&point.0).unwrap().clone()));
        }
    }

    let mut best: u32 = std::u32::MAX;
    for point in intersections {
        let dist = point.1;
        if dist < best {
            best = dist;
        }
    }

    return best as u32;
}

struct Tracer {
    wire: Vec<Move>,
    curr_move: usize,
    curr_offset: u32,
    curr_x: i32,
    curr_y: i32,
    dist: u32,
    last: bool,
}

impl Tracer {
    pub fn new(wire: Vec<Move>) -> Tracer {
        Tracer {
            wire,
            curr_move: 0,
            curr_offset: 0,
            curr_x: 0,
            curr_y: 0,
            dist: 0,
            last: false,
        }
    }
}

impl Iterator for Tracer {
    type Item = ((i32, i32), u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.last {
            return None;
        }

        if !(self.curr_move == 0 && self.curr_offset == 0) {
            let curr_move = self.wire[self.curr_move];

            if self.curr_offset == curr_move.distance {
                self.curr_offset = 0;
                self.curr_move += 1;
            }

            if self.curr_move == self.wire.len() {
                self.last = true;
            }

            match curr_move.direction {
                Direction::Up => self.curr_y += 1,
                Direction::Right => self.curr_x += 1,
                Direction::Down => self.curr_y -= 1,
                Direction::Left => self.curr_x -= 1,
            }
            self.dist += 1;
        }

        self.curr_offset += 1;

        Some(((self.curr_x, self.curr_y), self.dist))
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Move {
    direction: Direction,
    distance: u32,
}

impl Move {
    #[cfg(test)]
    pub fn new(direction: Direction, distance: u32) -> Move {
        Move {
            direction,
            distance,
        }
    }

    pub fn from_text(text: &str) -> Move {
        let direction = Direction::from_text(text.chars().next().unwrap());
        let distance = text.get(1..).unwrap().parse::<u32>().ok().unwrap();
        Move {
            direction,
            distance,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    pub fn from_text(dir: char) -> Direction {
        match dir {
            'L' => Direction::Left,
            'U' => Direction::Up,
            'R' => Direction::Right,
            'D' => Direction::Down,
            _ => panic!(format!("Unknown direction: {}", dir)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_direction_l() {
        assert_eq!(Direction::from_text('L'), Direction::Left)
    }

    #[test]
    fn test_parse_direction_u() {
        assert_eq!(Direction::from_text('U'), Direction::Up)
    }

    #[test]
    fn test_parse_direction_r() {
        assert_eq!(Direction::from_text('R'), Direction::Right)
    }

    #[test]
    fn test_parse_direction_d() {
        assert_eq!(Direction::from_text('D'), Direction::Down)
    }

    #[test]
    fn test_parse_move() {
        assert_eq!(
            Move::from_text("L1234"),
            Move {
                direction: Direction::Left,
                distance: 1234
            }
        )
    }

    #[test]
    fn test_tracer() {
        assert_eq!(
            Tracer::new(vec![
                Move::new(Direction::Right, 8),
                Move::new(Direction::Up, 5),
                Move::new(Direction::Left, 5),
                Move::new(Direction::Down, 3)
            ])
            .collect::<Vec<((i32, i32), u32)>>(),
            vec![
                ((0, 0), 0),
                ((1, 0), 1),
                ((2, 0), 2),
                ((3, 0), 3),
                ((4, 0), 4),
                ((5, 0), 5),
                ((6, 0), 6),
                ((7, 0), 7),
                ((8, 0), 8),
                ((8, 1), 9),
                ((8, 2), 10),
                ((8, 3), 11),
                ((8, 4), 12),
                ((8, 5), 13),
                ((7, 5), 14),
                ((6, 5), 15),
                ((5, 5), 16),
                ((4, 5), 17),
                ((3, 5), 18),
                ((3, 4), 19),
                ((3, 3), 20),
                ((3, 2), 21),
            ]
        )
    }

    #[test]
    fn test_closest_intersection1() {
        assert_eq!(
            closest_intersection(
                vec![
                    Move::new(Direction::Right, 8),
                    Move::new(Direction::Up, 5),
                    Move::new(Direction::Left, 5),
                    Move::new(Direction::Down, 3)
                ],
                vec![
                    Move::new(Direction::Up, 7),
                    Move::new(Direction::Right, 6),
                    Move::new(Direction::Down, 4),
                    Move::new(Direction::Left, 4)
                ]
            ),
            30
        )
    }

    #[test]
    fn test_closest_intersection2() {
        assert_eq!(
            closest_intersection(
                vec![
                    Move::new(Direction::Right, 75),
                    Move::new(Direction::Down, 30),
                    Move::new(Direction::Right, 83),
                    Move::new(Direction::Up, 83),
                    Move::new(Direction::Left, 12),
                    Move::new(Direction::Down, 49),
                    Move::new(Direction::Right, 71),
                    Move::new(Direction::Up, 7),
                    Move::new(Direction::Left, 72),
                ],
                vec![
                    Move::new(Direction::Up, 62),
                    Move::new(Direction::Right, 66),
                    Move::new(Direction::Up, 55),
                    Move::new(Direction::Right, 34),
                    Move::new(Direction::Down, 71),
                    Move::new(Direction::Right, 55),
                    Move::new(Direction::Down, 58),
                    Move::new(Direction::Right, 83),
                ]
            ),
            610
        )
    }

    #[test]
    fn test_closest_intersection3() {
        assert_eq!(
            closest_intersection(
                vec![
                    Move::new(Direction::Right, 98),
                    Move::new(Direction::Up, 47),
                    Move::new(Direction::Right, 26),
                    Move::new(Direction::Down, 63),
                    Move::new(Direction::Right, 33),
                    Move::new(Direction::Up, 87),
                    Move::new(Direction::Left, 62),
                    Move::new(Direction::Down, 20),
                    Move::new(Direction::Right, 33),
                    Move::new(Direction::Up, 53),
                    Move::new(Direction::Right, 51),
                ],
                vec![
                    Move::new(Direction::Up, 98),
                    Move::new(Direction::Right, 91),
                    Move::new(Direction::Down, 20),
                    Move::new(Direction::Right, 16),
                    Move::new(Direction::Down, 67),
                    Move::new(Direction::Right, 40),
                    Move::new(Direction::Up, 7),
                    Move::new(Direction::Right, 15),
                    Move::new(Direction::Up, 6),
                    Move::new(Direction::Right, 7),
                ]
            ),
            410
        )
    }
}
