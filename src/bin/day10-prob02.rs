use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

fn main() {
    let result = std::fs::read_to_string("src/bin/day10.txt")
        .map(|file| {
            let points = parse(file);
            let (_, _, idx) = find_best(&points);
            let blasted = get_blasted(&points, idx, 200);
            blasted.0 * 100 + blasted.1
        })
        .expect("Unable to open file");

    println!("{:?}", result);
}

fn parse(file: String) -> Vec<(usize, usize)> {
    let grid = file
        .lines()
        .map(|line| line.chars().map(|char| char == '#').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    let mut points: Vec<(usize, usize)> = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] {
                points.push((x, y));
            }
        }
    }

    points
}

fn num_visible(points: &Vec<(usize, usize)>, idx: usize) -> usize {
    let base = points[idx];

    let mut slopes: HashSet<i32> = HashSet::new();
    for point in points
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != idx)
        .map(|(_, p)| p)
    {
        let dir = (base.1 as f32 - point.1 as f32).atan2(base.0 as f32 - point.0 as f32);
        let hashable = (dir * 1_000f32) as i32;
        slopes.insert(hashable);
    }

    slopes.len()
}

fn find_best(points: &Vec<(usize, usize)>) -> (usize, (usize, usize), usize) {
    let (dist, point, idx) = points
        .iter()
        .enumerate()
        .map(|(idx, point)| (num_visible(&points, idx), point, idx))
        .max_by_key(|(dist, _, _)| *dist)
        .unwrap();
    (dist, point.clone(), idx)
}

fn blast_order(points: &Vec<(usize, usize)>, idx: usize) -> Vec<(usize, usize)> {
    let base = points[idx];

    // build map of direction -> vec[distances]
    let mut slopes: HashMap<i32, BinaryHeap<Reverse<Point>>> = HashMap::new();
    for point in points
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != idx)
        .map(|(_, p)| p)
    {
        let (_, dir_n, dist) = calc_delta(&base, point);
        let dir_n_hashable = (dir_n * 1_000f32) as i32;
        let dist_hashable = (dist * 1_000f32) as i32;

        let ps = Point {
            dist_hash: dist_hashable,
            point: point.clone(),
        };
        slopes
            .entry(dir_n_hashable)
            .or_insert(BinaryHeap::new())
            .push(Reverse(ps));
    }

    let mut saw_asteroid = true;
    let mut blasted: Vec<(usize, usize)> = Vec::new();
    let sorted_keys = slopes
        .keys()
        .sorted()
        .map(|key| key.clone())
        .collect::<Vec<i32>>();
    //    println!(
    //        "{}",
    //        sorted_keys
    //            .iter()
    //            .map(|dir_hashed| (
    //                (*dir_hashed as f32) / 1000f32,
    //                slopes
    //                    .get(dir_hashed)
    //                    .unwrap()
    //                    .clone()
    //                    .iter()
    //                    .map(|p| p.0.clone())
    //                    .map(|p| (p.dist_hash as f32 / 1000f32, p.point))
    //                    .collect::<Vec<(f32, (usize, usize))>>()
    //            ))
    //            .collect::<Vec<(f32, Vec<(f32, (usize, usize))>)>>()
    //            .iter()
    //            .map(|p| format!("{:?}", p))
    //            .join("\n")
    //    );
    while saw_asteroid {
        saw_asteroid = false;
        for dir in sorted_keys.iter() {
            if let Some(point) = slopes.get_mut(dir).unwrap().pop() {
                saw_asteroid = true;
                blasted.push(point.0.point);
            }
        }
    }

    blasted
}

fn calc_delta(base: &(usize, usize), point: &(usize, usize)) -> (f32, f32, f32) {
    let dir_e = (-(point.1 as f32) - -(base.1 as f32)).atan2(point.0 as f32 - base.0 as f32);
    let dir_n = ((-dir_e + (std::f32::consts::PI / 2f32)) * 180f32 / std::f32::consts::PI) % 360f32;
    let dir_n = if dir_n < 0f32 { dir_n + 360f32 } else { dir_n };

    let dist = ((point.1 as f32 - base.1 as f32).powi(2)
        + (point.0 as f32 - base.0 as f32).powi(2))
    .sqrt();

    //    println!(
    //        "{:?} -> {:?}   ccw_e: {} cw_n: {} dist: {}",
    //        base,
    //        point,
    //        dir_e * 180f32 / std::f32::consts::PI,
    //        dir_n,
    //        dist
    //    );

    (dir_e * 180f32 / std::f32::consts::PI, dir_n, dist)
}

fn get_blasted(points: &Vec<(usize, usize)>, idx: usize, nth: usize) -> (usize, usize) {
    let order = blast_order(points, idx);
    order[nth - 1]
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
struct Point {
    dist_hash: i32,
    point: (usize, usize),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc_delta_1() {
        assert_eq!(calc_delta(&(2, 2), &(4, 2)), (0f32, 90f32, 2f32))
    }

    #[test]
    fn test_calc_delta_2() {
        assert_eq!(calc_delta(&(2, 2), &(2, 4)), (-90f32, 180f32, 2f32))
    }

    #[test]
    fn test_calc_delta_3() {
        assert_eq!(calc_delta(&(2, 2), &(0, 2)), (180f32, 270f32, 2f32))
    }

    #[test]
    fn test_calc_delta_4() {
        assert_eq!(calc_delta(&(2, 2), &(2, 0)), (90f32, 0f32, 2f32))
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(
                ".#..#
.....
#####
....#
...##"
                    .to_string()
            ),
            vec![
                (1, 0),
                (4, 0),
                (0, 2),
                (1, 2),
                (2, 2),
                (3, 2),
                (4, 2),
                (4, 3),
                (3, 4),
                (4, 4)
            ]
        )
    }

    #[test]
    fn test_num_visible_1() {
        assert_eq!(
            num_visible(
                &vec![
                    (1, 0),
                    (4, 0),
                    (0, 2),
                    (1, 2),
                    (2, 2),
                    (3, 2),
                    (4, 2),
                    (4, 3),
                    (3, 4),
                    (4, 4)
                ],
                0
            ),
            7
        )
    }

    #[test]
    fn test_num_visible_2() {
        assert_eq!(
            num_visible(
                &vec![
                    (1, 0),
                    (4, 0),
                    (0, 2),
                    (1, 2),
                    (2, 2),
                    (3, 2),
                    (4, 2),
                    (4, 3),
                    (3, 4),
                    (4, 4)
                ],
                1
            ),
            7
        )
    }

    #[test]
    fn test_num_visible_3() {
        assert_eq!(
            num_visible(
                &vec![
                    (1, 0),
                    (4, 0),
                    (0, 2),
                    (1, 2),
                    (2, 2),
                    (3, 2),
                    (4, 2),
                    (4, 3),
                    (3, 4),
                    (4, 4)
                ],
                2
            ),
            6
        )
    }

    #[test]
    fn test_num_visible_4() {
        assert_eq!(
            num_visible(
                &vec![
                    (1, 0),
                    (4, 0),
                    (0, 2),
                    (1, 2),
                    (2, 2),
                    (3, 2),
                    (4, 2),
                    (4, 3),
                    (3, 4),
                    (4, 4)
                ],
                3
            ),
            7
        )
    }

    #[test]
    fn test_num_visible_5() {
        assert_eq!(
            num_visible(
                &vec![
                    (1, 0),
                    (4, 0),
                    (0, 2),
                    (1, 2),
                    (2, 2),
                    (3, 2),
                    (4, 2),
                    (4, 3),
                    (3, 4),
                    (4, 4)
                ],
                4
            ),
            7
        )
    }

    #[test]
    fn test_num_visible_6() {
        assert_eq!(
            num_visible(
                &vec![
                    (1, 0),
                    (4, 0),
                    (0, 2),
                    (1, 2),
                    (2, 2),
                    (3, 2),
                    (4, 2),
                    (4, 3),
                    (3, 4),
                    (4, 4)
                ],
                5
            ),
            7
        )
    }

    #[test]
    fn test_num_visible_7() {
        assert_eq!(
            num_visible(
                &vec![
                    (1, 0),
                    (4, 0),
                    (0, 2),
                    (1, 2),
                    (2, 2),
                    (3, 2),
                    (4, 2),
                    (4, 3),
                    (3, 4),
                    (4, 4)
                ],
                6
            ),
            5
        )
    }

    #[test]
    fn test_num_visible_8() {
        assert_eq!(
            num_visible(
                &vec![
                    (1, 0),
                    (4, 0),
                    (0, 2),
                    (1, 2),
                    (2, 2),
                    (3, 2),
                    (4, 2),
                    (4, 3),
                    (3, 4),
                    (4, 4)
                ],
                7
            ),
            7
        )
    }

    #[test]
    fn test_num_visible_9() {
        assert_eq!(
            num_visible(
                &vec![
                    (1, 0),
                    (4, 0),
                    (0, 2),
                    (1, 2),
                    (2, 2),
                    (3, 2),
                    (4, 2),
                    (4, 3),
                    (3, 4),
                    (4, 4)
                ],
                8
            ),
            8
        )
    }

    #[test]
    fn test_num_visible_10() {
        assert_eq!(
            num_visible(
                &vec![
                    (1, 0),
                    (4, 0),
                    (0, 2),
                    (1, 2),
                    (2, 2),
                    (3, 2),
                    (4, 2),
                    (4, 3),
                    (3, 4),
                    (4, 4)
                ],
                9
            ),
            7
        )
    }

    #[test]
    fn test_find_best_1() {
        assert_eq!(
            find_best(&vec![
                (1, 0),
                (4, 0),
                (0, 2),
                (1, 2),
                (2, 2),
                (3, 2),
                (4, 2),
                (4, 3),
                (3, 4),
                (4, 4)
            ]),
            (8, (3, 4), 8)
        )
    }

    #[test]
    fn test_find_best_2() {
        assert_eq!(
            find_best(&vec![
                (6, 0),
                (8, 0),
                (0, 1),
                (3, 1),
                (5, 1),
                (2, 2),
                (3, 2),
                (4, 2),
                (5, 2),
                (6, 2),
                (7, 2),
                (8, 2),
                (1, 3),
                (3, 3),
                (5, 3),
                (6, 3),
                (7, 3),
                (1, 4),
                (4, 4),
                (2, 5),
                (7, 5),
                (9, 5),
                (0, 6),
                (3, 6),
                (8, 6),
                (1, 7),
                (2, 7),
                (4, 7),
                (7, 7),
                (8, 7),
                (9, 7),
                (0, 8),
                (1, 8),
                (5, 8),
                (8, 8),
                (1, 9),
                (6, 9),
                (7, 9),
                (8, 9),
                (9, 9)
            ]),
            (33, (5, 8), 33)
        )
    }

    #[test]
    fn test_find_best_3() {
        assert_eq!(
            find_best(&vec![
                (0, 0),
                (2, 0),
                (6, 0),
                (8, 0),
                (1, 1),
                (2, 1),
                (3, 1),
                (8, 1),
                (1, 2),
                (6, 2),
                (0, 3),
                (1, 3),
                (3, 3),
                (5, 3),
                (7, 3),
                (9, 3),
                (4, 4),
                (6, 4),
                (8, 4),
                (1, 5),
                (2, 5),
                (5, 5),
                (6, 5),
                (7, 5),
                (9, 5),
                (2, 6),
                (6, 6),
                (7, 6),
                (2, 7),
                (3, 7),
                (8, 7),
                (9, 7),
                (6, 8),
                (1, 9),
                (2, 9),
                (3, 9),
                (4, 9),
                (6, 9),
                (7, 9),
                (8, 9)
            ]),
            (35, (1, 2), 8)
        )
    }

    #[test]
    fn test_find_best_4() {
        assert_eq!(
            find_best(&vec![
                (1, 0),
                (4, 0),
                (7, 0),
                (8, 0),
                (9, 0),
                (0, 1),
                (1, 1),
                (2, 1),
                (3, 1),
                (5, 1),
                (6, 1),
                (7, 1),
                (9, 1),
                (4, 2),
                (5, 2),
                (6, 2),
                (8, 2),
                (2, 3),
                (3, 3),
                (4, 3),
                (6, 3),
                (7, 3),
                (9, 3),
                (0, 4),
                (1, 4),
                (3, 4),
                (4, 4),
                (6, 4),
                (8, 4),
                (4, 5),
                (5, 5),
                (6, 5),
                (9, 5),
                (2, 6),
                (4, 6),
                (7, 6),
                (9, 6),
                (0, 7),
                (3, 7),
                (5, 7),
                (7, 7),
                (8, 7),
                (9, 7),
                (1, 8),
                (2, 8),
                (6, 8),
                (7, 8),
                (9, 8),
                (5, 9),
                (7, 9)
            ]),
            (41, (6, 3), 20)
        )
    }

    fn large() -> Vec<(usize, usize)> {
        vec![
            (1, 0),
            (4, 0),
            (5, 0),
            (7, 0),
            (8, 0),
            (9, 0),
            (13, 0),
            (14, 0),
            (15, 0),
            (16, 0),
            (17, 0),
            (18, 0),
            (19, 0),
            (0, 1),
            (1, 1),
            (3, 1),
            (4, 1),
            (5, 1),
            (6, 1),
            (7, 1),
            (8, 1),
            (9, 1),
            (10, 1),
            (11, 1),
            (12, 1),
            (13, 1),
            (14, 1),
            (17, 1),
            (18, 1),
            (1, 2),
            (3, 2),
            (4, 2),
            (5, 2),
            (6, 2),
            (7, 2),
            (8, 2),
            (10, 2),
            (11, 2),
            (12, 2),
            (13, 2),
            (14, 2),
            (15, 2),
            (16, 2),
            (17, 2),
            (19, 2),
            (1, 3),
            (2, 3),
            (3, 3),
            (5, 3),
            (6, 3),
            (7, 3),
            (8, 3),
            (9, 3),
            (10, 3),
            (11, 3),
            (13, 3),
            (14, 3),
            (15, 3),
            (16, 3),
            (18, 3),
            (0, 4),
            (1, 4),
            (2, 4),
            (3, 4),
            (4, 4),
            (6, 4),
            (7, 4),
            (9, 4),
            (11, 4),
            (12, 4),
            (14, 4),
            (15, 4),
            (16, 4),
            (18, 4),
            (19, 4),
            (2, 5),
            (3, 5),
            (4, 5),
            (5, 5),
            (6, 5),
            (9, 5),
            (11, 5),
            (12, 5),
            (13, 5),
            (14, 5),
            (15, 5),
            (16, 5),
            (17, 5),
            (18, 5),
            (19, 5),
            (0, 6),
            (1, 6),
            (2, 6),
            (3, 6),
            (4, 6),
            (5, 6),
            (6, 6),
            (7, 6),
            (8, 6),
            (9, 6),
            (10, 6),
            (11, 6),
            (12, 6),
            (13, 6),
            (14, 6),
            (15, 6),
            (16, 6),
            (17, 6),
            (18, 6),
            (19, 6),
            (0, 7),
            (2, 7),
            (3, 7),
            (4, 7),
            (5, 7),
            (10, 7),
            (11, 7),
            (12, 7),
            (14, 7),
            (16, 7),
            (18, 7),
            (19, 7),
            (0, 8),
            (1, 8),
            (3, 8),
            (4, 8),
            (5, 8),
            (6, 8),
            (7, 8),
            (8, 8),
            (9, 8),
            (10, 8),
            (11, 8),
            (12, 8),
            (13, 8),
            (14, 8),
            (15, 8),
            (16, 8),
            (17, 8),
            (18, 8),
            (19, 8),
            (0, 9),
            (1, 9),
            (2, 9),
            (3, 9),
            (4, 9),
            (6, 9),
            (7, 9),
            (9, 9),
            (10, 9),
            (11, 9),
            (14, 9),
            (15, 9),
            (16, 9),
            (17, 9),
            (2, 10),
            (3, 10),
            (4, 10),
            (5, 10),
            (6, 10),
            (7, 10),
            (10, 10),
            (11, 10),
            (13, 10),
            (14, 10),
            (15, 10),
            (16, 10),
            (17, 10),
            (18, 10),
            (19, 10),
            (0, 11),
            (1, 11),
            (2, 11),
            (3, 11),
            (5, 11),
            (6, 11),
            (8, 11),
            (9, 11),
            (10, 11),
            (11, 11),
            (15, 11),
            (16, 11),
            (19, 11),
            (1, 12),
            (2, 12),
            (3, 12),
            (4, 12),
            (5, 12),
            (8, 12),
            (10, 12),
            (11, 12),
            (12, 12),
            (13, 12),
            (14, 12),
            (15, 12),
            (17, 12),
            (18, 12),
            (19, 12),
            (0, 13),
            (1, 13),
            (5, 13),
            (7, 13),
            (8, 13),
            (9, 13),
            (10, 13),
            (11, 13),
            (12, 13),
            (13, 13),
            (14, 13),
            (15, 13),
            (16, 13),
            (0, 14),
            (2, 14),
            (3, 14),
            (4, 14),
            (5, 14),
            (6, 14),
            (7, 14),
            (8, 14),
            (9, 14),
            (10, 14),
            (11, 14),
            (13, 14),
            (14, 14),
            (15, 14),
            (16, 14),
            (17, 14),
            (18, 14),
            (19, 14),
            (1, 15),
            (2, 15),
            (3, 15),
            (4, 15),
            (6, 15),
            (8, 15),
            (9, 15),
            (10, 15),
            (12, 15),
            (13, 15),
            (14, 15),
            (16, 15),
            (18, 15),
            (19, 15),
            (4, 16),
            (5, 16),
            (7, 16),
            (8, 16),
            (10, 16),
            (11, 16),
            (12, 16),
            (15, 16),
            (16, 16),
            (17, 16),
            (18, 16),
            (19, 16),
            (1, 17),
            (3, 17),
            (5, 17),
            (6, 17),
            (7, 17),
            (8, 17),
            (9, 17),
            (10, 17),
            (11, 17),
            (12, 17),
            (13, 17),
            (14, 17),
            (15, 17),
            (17, 17),
            (18, 17),
            (19, 17),
            (0, 18),
            (2, 18),
            (4, 18),
            (6, 18),
            (7, 18),
            (8, 18),
            (9, 18),
            (10, 18),
            (12, 18),
            (13, 18),
            (14, 18),
            (15, 18),
            (17, 18),
            (18, 18),
            (19, 18),
            (0, 19),
            (1, 19),
            (2, 19),
            (4, 19),
            (5, 19),
            (7, 19),
            (8, 19),
            (9, 19),
            (10, 19),
            (12, 19),
            (13, 19),
            (15, 19),
            (18, 19),
            (19, 19),
        ]
    }

    #[test]
    fn test_find_best_5() {
        assert_eq!(find_best(&large()), (210, (11, 13), 205))
    }

    #[test]
    fn test_blast_order() {
        assert_eq!(
            blast_order(
                &vec![
                    (1, 0),
                    (6, 0),
                    (7, 0),
                    (8, 0),
                    (9, 0),
                    (10, 0),
                    (14, 0),
                    (0, 1),
                    (1, 1),
                    (5, 1),
                    (6, 1),
                    (8, 1),
                    (9, 1),
                    (10, 1),
                    (11, 1),
                    (12, 1),
                    (15, 1),
                    (16, 1),
                    (0, 2),
                    (1, 2),
                    (5, 2),
                    (9, 2),
                    (11, 2),
                    (12, 2),
                    (13, 2),
                    (14, 2),
                    (15, 2),
                    (2, 3),
                    (8, 3),
                    (12, 3),
                    (13, 3),
                    (14, 3),
                    (2, 4),
                    (4, 4),
                    (10, 4),
                    (15, 4),
                    (16, 4)
                ],
                28
            )
            .iter()
            .take(9)
            .map(|d| d.clone())
            .collect::<Vec<(usize, usize)>>(),
            vec![
                (8, 1),
                (9, 0),
                (9, 1),
                (10, 0),
                (9, 2),
                (11, 1),
                (12, 1),
                (11, 2),
                (15, 1)
            ]
        )
    }

    #[test]
    fn test_get_blasted_1() {
        assert_eq!(get_blasted(&large(), 205, 1), (11, 12))
    }

    #[test]
    fn test_get_blasted_2() {
        assert_eq!(get_blasted(&large(), 205, 2), (12, 1))
    }

    #[test]
    fn test_get_blasted_3() {
        assert_eq!(get_blasted(&large(), 205, 3), (12, 2))
    }

    #[test]
    fn test_get_blasted_10() {
        assert_eq!(get_blasted(&large(), 205, 10), (12, 8))
    }

    #[test]
    fn test_get_blasted_20() {
        assert_eq!(get_blasted(&large(), 205, 20), (16, 0))
    }

    #[test]
    fn test_get_blasted_50() {
        assert_eq!(get_blasted(&large(), 205, 50), (16, 9))
    }

    #[test]
    fn test_get_blasted_100() {
        assert_eq!(get_blasted(&large(), 205, 100), (10, 16))
    }

    #[test]
    fn test_get_blasted_199() {
        assert_eq!(get_blasted(&large(), 205, 199), (9, 6))
    }

    #[test]
    fn test_get_blasted_200() {
        assert_eq!(get_blasted(&large(), 205, 200), (8, 2))
    }

    #[test]
    fn test_get_blasted_201() {
        assert_eq!(get_blasted(&large(), 205, 200), (8, 2))
    }

    #[test]
    fn test_get_blasted_299() {
        assert_eq!(get_blasted(&large(), 205, 299), (11, 1))
    }
}
