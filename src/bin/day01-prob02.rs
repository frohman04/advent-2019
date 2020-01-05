fn main() {
    let fuels = std::fs::read_to_string("src/bin/day01.txt")
        .map(|file| {
            file.split("\n")
                .filter(|line| line.len() > 0)
                .map(|val| val.parse::<i32>().map(fuel_for).ok().unwrap())
                .collect::<Vec<i32>>()
        })
        .expect("Unable to open file");
    println!("{}", fuels.iter().sum::<i32>());
}

fn fuel_for(mass: i32) -> i32 {
    let mut fuel: i32 = 0;
    let mut latest = mass.clone();
    while {
        latest = fuel_for_single(latest);
        fuel += latest;
        latest > 0
    } {}
    fuel
}

fn fuel_for_single(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel < 0 {
        0
    } else {
        fuel
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_single1() {
        assert_eq!(fuel_for_single(12), 2);
    }

    #[test]
    fn test_single2() {
        assert_eq!(fuel_for_single(14), 2);
    }

    #[test]
    fn test_single3() {
        assert_eq!(fuel_for_single(1969), 654);
    }

    #[test]
    fn test_single4() {
        assert_eq!(fuel_for_single(100756), 33583);
    }

    #[test]
    fn test_single5() {
        assert_eq!(fuel_for_single(1), 0);
    }

    #[test]
    fn test1() {
        assert_eq!(fuel_for(14), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(fuel_for(1969), 966);
    }

    #[test]
    fn test3() {
        assert_eq!(fuel_for(100756), 50346);
    }
}
