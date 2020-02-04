use std::collections::{HashMap, VecDeque};

fn main() {
    let result = std::fs::read_to_string("src/bin/day14.txt")
        .map(|file| {
            let reactions = file
                .lines()
                .filter(|line| !line.is_empty())
                .map(|line| Reaction::from(line))
                .collect::<Vec<Reaction>>();
            solve(get_processing_order(reactions))
        })
        .expect("Unable to open file");

    println!("{}", result);
}

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, Hash, Clone)]
struct Unit(String);

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, Clone)]
struct Ingredient {
    quant: u32,
    unit: Unit,
}

impl Ingredient {
    fn new(quant: u32, unit: Unit) -> Ingredient {
        Ingredient { quant, unit }
    }
}

impl From<&str> for Ingredient {
    fn from(raw: &str) -> Self {
        let pieces: Vec<&str> = raw.trim().split_whitespace().collect();
        let quant: u32 = pieces[0].parse::<u32>().unwrap();
        let unit: Unit = Unit(pieces[1].to_string());
        Ingredient::new(quant, unit)
    }
}

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, Clone)]
struct Reaction {
    reactants: Vec<Ingredient>,
    product: Ingredient,
}

impl Reaction {
    fn new(reactants: Vec<Ingredient>, product: Ingredient) -> Reaction {
        Reaction { reactants, product }
    }
}

impl From<&str> for Reaction {
    fn from(raw: &str) -> Self {
        let pieces: Vec<&str> = raw.split("=>").collect();
        let reactants: Vec<Ingredient> = pieces[0].split(",").map(|piece| piece.into()).collect();
        let product: Ingredient = pieces[1].into();
        Reaction::new(reactants, product)
    }
}

fn solve(reactions: Vec<Reaction>) -> u32 {
    let mut needed: HashMap<Unit, u32> = HashMap::new();
    needed.insert(Unit("FUEL".to_string()), 1);

    for reaction in reactions {
        let min_quant = needed.get(&reaction.product.unit).unwrap();
        let mult = (*min_quant as f32 / reaction.product.quant as f32).ceil() as u32;
        for ing in reaction.reactants {
            let quant = needed.entry(ing.unit).or_insert(0);
            *quant += ing.quant * mult;
        }
    }

    *needed.get(&Unit("ORE".to_string())).unwrap()
}

/// Determine the order that reactions should be processed.  If processed in order, the last
/// reaction processed will produce the desired answer.
fn get_processing_order(reactions: Vec<Reaction>) -> Vec<Reaction> {
    let dependents: Vec<(Unit, Unit)> = reactions
        .iter()
        .flat_map(|reaction| {
            reaction
                .reactants
                .iter()
                .map(move |reactant| (reactant.unit.clone(), reaction.product.unit.clone()))
        })
        .collect();

    let reaction_map: HashMap<Unit, Reaction> = reactions
        .iter()
        .map(|reaction| (reaction.product.unit.clone(), reaction.clone()))
        .collect();

    topo_sort(dependents)
        .iter()
        .rev()
        .filter(|unit| **unit != Unit("ORE".to_string()))
        .map(|unit| reaction_map.get(unit).unwrap().clone())
        .collect()
}

/// Perform a topological sort on pairs of (unit -> unit) dependencies.  Returns topologically
/// sorted list such that the first item has no dependencies.
fn topo_sort(edges: Vec<(Unit, Unit)>) -> Vec<Unit> {
    let mut neighbors: HashMap<Unit, Vec<Unit>> = HashMap::new();
    for (source, sink) in edges.iter() {
        let ns = neighbors.entry(source.clone()).or_insert(Vec::new());
        ns.push(sink.clone());

        neighbors.entry(sink.clone()).or_insert(Vec::new());
    }
    let neighbors = neighbors;

    let mut indegree: HashMap<Unit, u32> = HashMap::new();
    for n in neighbors.keys() {
        indegree.insert(n.clone(), 0);
    }
    for ns in neighbors.values() {
        for n in ns {
            let count = indegree.entry(n.clone()).or_insert(0);
            *count += 1;
        }
    }

    let mut ordered: Vec<Unit> = Vec::new();
    let mut queue: VecDeque<Unit> = VecDeque::new();
    for unit in indegree.keys() {
        if *indegree.get(unit).unwrap() == 0 {
            queue.push_front(unit.clone());
        }
    }
    while !queue.is_empty() {
        let curr = queue.pop_back().unwrap();
        ordered.push(curr.clone());

        for unit in neighbors.get(&curr).unwrap() {
            let count = indegree.get_mut(unit).unwrap();
            *count -= 1;

            if *count == 0 {
                queue.push_front(unit.clone());
            }
        }
    }

    ordered
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ingredient_from_str_1() {
        assert_eq!(
            Ingredient::from("10 A"),
            Ingredient::new(10, Unit("A".to_string()))
        )
    }

    #[test]
    fn test_ingredient_from_str_2() {
        assert_eq!(
            Ingredient::from(" 10 FUEL "),
            Ingredient::new(10, Unit("FUEL".to_string()))
        )
    }

    #[test]
    fn test_reaction_from_str_1() {
        assert_eq!(
            Reaction::from("7 A => 1 C"),
            Reaction::new(
                vec![(Ingredient::new(7, Unit("A".to_string())))],
                Ingredient::new(1, Unit("C".to_string())),
            )
        )
    }

    #[test]
    fn test_reaction_from_str_2() {
        assert_eq!(
            Reaction::from("3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"),
            Reaction::new(
                vec![
                    Ingredient::new(3, Unit("DCFZ".to_string())),
                    Ingredient::new(7, Unit("NZVS".to_string())),
                    Ingredient::new(5, Unit("HKGWZ".to_string())),
                    Ingredient::new(10, Unit("PSHF".to_string()))
                ],
                Ingredient::new(8, Unit("KHKGT".to_string())),
            )
        )
    }

    #[test]
    fn test_topo_sort() {
        assert_eq!(
            topo_sort(vec![
                (Unit("ORE".to_string()), Unit("A".to_string())),
                (Unit("ORE".to_string()), Unit("B".to_string())),
                (Unit("A".to_string()), Unit("C".to_string())),
                (Unit("B".to_string()), Unit("C".to_string())),
                (Unit("A".to_string()), Unit("D".to_string())),
                (Unit("C".to_string()), Unit("D".to_string())),
                (Unit("A".to_string()), Unit("E".to_string())),
                (Unit("D".to_string()), Unit("E".to_string())),
                (Unit("A".to_string()), Unit("FUEL".to_string())),
                (Unit("E".to_string()), Unit("FUEL".to_string()))
            ]),
            vec![
                Unit("ORE".to_string()),
                Unit("A".to_string()),
                Unit("B".to_string()),
                Unit("C".to_string()),
                Unit("D".to_string()),
                Unit("E".to_string()),
                Unit("FUEL".to_string())
            ]
        )
    }

    #[test]
    fn test_get_processing_order() {
        assert_eq!(
            get_processing_order(vec![
                Reaction::from("10 ORE => 10 A"),
                Reaction::from("1 ORE => 1 B"),
                Reaction::from("7 A, 1 B => 1 C"),
                Reaction::from("7 A, 1 C => 1 D"),
                Reaction::from("7 A, 1 D => 1 E"),
                Reaction::from("7 A, 1 E => 1 FUEL")
            ]),
            vec![
                Reaction::from("7 A, 1 E => 1 FUEL"),
                Reaction::from("7 A, 1 D => 1 E"),
                Reaction::from("7 A, 1 C => 1 D"),
                Reaction::from("7 A, 1 B => 1 C"),
                Reaction::from("1 ORE => 1 B"),
                Reaction::from("10 ORE => 10 A")
            ]
        )
    }

    #[test]
    fn test_solve() {
        assert_eq!(
            solve(vec![
                Reaction::from("7 A, 1 E => 1 FUEL"),
                Reaction::from("7 A, 1 D => 1 E"),
                Reaction::from("7 A, 1 C => 1 D"),
                Reaction::from("7 A, 1 B => 1 C"),
                Reaction::from("1 ORE => 1 B"),
                Reaction::from("10 ORE => 10 A")
            ]),
            31
        )
    }

    #[test]
    fn test_solve_all_1() {
        assert_eq!(
            solve(get_processing_order(vec![
                Reaction::from("9 ORE => 2 A"),
                Reaction::from("8 ORE => 3 B"),
                Reaction::from("7 ORE => 5 C"),
                Reaction::from("3 A, 4 B => 1 AB"),
                Reaction::from("5 B, 7 C => 1 BC"),
                Reaction::from("4 C, 1 A => 1 CA"),
                Reaction::from("2 AB, 3 BC, 4 CA => 1 FUEL")
            ])),
            165
        )
    }

    #[test]
    fn test_solve_all_2() {
        assert_eq!(
            solve(get_processing_order(vec![
                Reaction::from("157 ORE => 5 NZVS"),
                Reaction::from("165 ORE => 6 DCFZ"),
                Reaction::from("44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL"),
                Reaction::from("12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ"),
                Reaction::from("179 ORE => 7 PSHF"),
                Reaction::from("177 ORE => 5 HKGWZ"),
                Reaction::from("7 DCFZ, 7 PSHF => 2 XJWVT"),
                Reaction::from("165 ORE => 2 GPVTF"),
                Reaction::from("3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT")
            ])),
            13312
        )
    }

    #[test]
    fn test_solve_all_3() {
        assert_eq!(
            solve(get_processing_order(vec![
                Reaction::from("2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG"),
                Reaction::from("17 NVRVD, 3 JNWZP => 8 VPVL"),
                Reaction::from("53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL"),
                Reaction::from("22 VJHF, 37 MNCFX => 5 FWMGM"),
                Reaction::from("139 ORE => 4 NVRVD"),
                Reaction::from("144 ORE => 7 JNWZP"),
                Reaction::from("5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC"),
                Reaction::from("5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV"),
                Reaction::from("145 ORE => 6 MNCFX"),
                Reaction::from("1 NVRVD => 8 CXFTF"),
                Reaction::from("1 VJHF, 6 MNCFX => 4 RFSQX"),
                Reaction::from("176 ORE => 6 VJHF")
            ])),
            180697
        )
    }

    #[test]
    fn test_solve_all_4() {
        assert_eq!(
            solve(get_processing_order(vec![
                Reaction::from("171 ORE => 8 CNZTR"),
                Reaction::from(
                    "7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL"
                ),
                Reaction::from("114 ORE => 4 BHXH"),
                Reaction::from("14 VRPVC => 6 BMBT"),
                Reaction::from("6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL"),
                Reaction::from(
                    "6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT"
                ),
                Reaction::from("15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW"),
                Reaction::from("13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW"),
                Reaction::from("5 BMBT => 4 WPTQ"),
                Reaction::from("189 ORE => 9 KTJDG"),
                Reaction::from("1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP"),
                Reaction::from("12 VRPVC, 27 CNZTR => 2 XDBXC"),
                Reaction::from("15 KTJDG, 12 BHXH => 5 XCVML"),
                Reaction::from("3 BHXH, 2 VRPVC => 7 MZWV"),
                Reaction::from("121 ORE => 7 VRPVC"),
                Reaction::from("7 XCVML => 6 RJRHP"),
                Reaction::from("5 BHXH, 4 VRPVC => 5 LTCX")
            ])),
            2210736
        )
    }
}
