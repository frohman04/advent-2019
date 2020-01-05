fn main() {
    let result = std::fs::read_to_string("src/bin/day02.txt")
        .map(|file| {
            let line = file
                .split("\n")
                .filter(|line| line.len() > 0)
                .collect::<Vec<&str>>()[0];
            let mut code = line
                .split(",")
                .map(|item| item.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            code[1] = 12;
            code[2] = 2;

            let post = run(code);

            post[0]
        })
        .expect("Unable to open file");

    println!("{}", result);
}

fn run(mut code: Vec<u32>) -> Vec<u32> {
    let mut i: usize = 0;

    //    println!("{:?}", code);
    while {
        let opcode = code[i];
        //        println!("opcode {} at {}", opcode, i);
        let run_again = if opcode == 99 {
            false
        } else if opcode == 1 {
            let arg1 = code[code[i + 1] as usize];
            let arg2 = code[code[i + 2] as usize];
            let out_addr = code[i + 3] as usize;
            code[out_addr] = arg1 + arg2;
            true
        } else if opcode == 2 {
            let arg1 = code[code[i + 1] as usize];
            let arg2 = code[code[i + 2] as usize];
            let out_addr = code[i + 3] as usize;
            code[out_addr] = arg1 * arg2;
            true
        } else {
            panic!("Unexpected opcode {} at {}", opcode, i)
        };
        //        println!("{:?}", code);

        i += 4;

        run_again
    } {}

    code
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            run(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(run(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test3() {
        assert_eq!(run(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test4() {
        assert_eq!(run(vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test5() {
        assert_eq!(
            run(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
