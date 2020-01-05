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
    mass / 3 - 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(fuel_for(12), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(fuel_for(14), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(fuel_for(1969), 654);
    }

    #[test]
    fn test4() {
        assert_eq!(fuel_for(100756), 33583);
    }
}
