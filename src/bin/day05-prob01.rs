fn main() {
    let result = std::fs::read_to_string("src/bin/day05.txt")
        .map(|file| {
            let line = file
                .split("\n")
                .filter(|line| line.len() > 0)
                .collect::<Vec<&str>>()[0];
            let code = line
                .split(",")
                .map(|item| item.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            find_answer(code, vec![1])
        })
        .expect("Unable to open file");

    println!("{}", result);
}

fn find_answer(code: Vec<i32>, input: Vec<i32>) -> i32 {
    let (_code, output) = execute(code, None, input);
    for i in 0..(output.len() - 1) {
        let (pc, val) = output[i];
        if val != 0 {
            panic!("Unexpected output {} at PC {}", val, pc);
        }
    }
    output.last().unwrap().clone().1
}

#[derive(Debug, Clone)]
struct InitialState {
    pub noun: i32,
    pub verb: i32,
}

fn execute(
    mut code: Vec<i32>,
    init_state: Option<InitialState>,
    mut input: Vec<i32>,
) -> (Vec<i32>, Vec<(usize, i32)>) {
    let mut pc: usize = 0;
    if init_state.is_some() {
        let init_state = init_state.unwrap();
        code[1] = init_state.noun;
        code[2] = init_state.verb;
    }

    input.reverse();
    let mut output: Vec<(usize, i32)> = Vec::new();

    //    println!("{:?}", code);
    while {
        let raw_opcode = code[pc];
        let opcode = raw_opcode % 100;
        let param1_imm = raw_opcode % 1000 / 100 == 1;
        let param2_imm = raw_opcode % 10000 / 1000 == 1;
        let param3_imm = raw_opcode % 100000 / 10000 == 1;

        //        println!("opcode {} at {}", opcode, i);
        let run_again = if opcode == 99 {
            false
        } else if opcode == 1 {
            let arg1 = get_val(&code, pc + 1, param1_imm);
            let arg2 = get_val(&code, pc + 2, param2_imm);
            if param3_imm {
                panic!("Param 3 cannot be immediate mode for opcode 01");
            }
            let out_addr = code[pc + 3] as usize;
            code[out_addr] = arg1 + arg2;

            pc += 4;
            true
        } else if opcode == 2 {
            let arg1 = get_val(&code, pc + 1, param1_imm);
            let arg2 = get_val(&code, pc + 2, param2_imm);
            if param3_imm {
                panic!("Param 3 cannot be immediate mode for opcode 02");
            }
            let out_addr = code[pc + 3] as usize;
            code[out_addr] = arg1 * arg2;

            pc += 4;
            true
        } else if opcode == 3 {
            let write_addr = code[pc + 1] as usize;
            code[write_addr] = input.pop().unwrap();

            pc += 2;
            true
        } else if opcode == 4 {
            let value = get_val(&code, pc + 1, param1_imm);
            output.push((pc, value));

            pc += 2;
            true
        } else {
            panic!("Unexpected opcode {} at {}", raw_opcode, pc)
        };
        //        println!("{:?}", code);

        run_again
    } {}

    (code, output)
}

fn get_val(code: &Vec<i32>, i: usize, is_imm: bool) -> i32 {
    if is_imm {
        code[i]
    } else {
        code[code[i] as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            execute(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], None, vec![],),
            (vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], vec![])
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            execute(vec![1, 0, 0, 0, 99], None, vec![]),
            (vec![2, 0, 0, 0, 99], vec![])
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            execute(vec![2, 3, 0, 3, 99], None, vec![]),
            (vec![2, 3, 0, 6, 99], vec![])
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            execute(vec![2, 4, 4, 5, 99, 0], None, vec![]),
            (vec![2, 4, 4, 5, 99, 9801], vec![])
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            execute(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], None, vec![],),
            (vec![30, 1, 1, 4, 2, 5, 6, 0, 99], vec![])
        );
    }

    #[test]
    fn test_input() {
        assert_eq!(
            execute(vec![3, 1, 99], None, vec![42]),
            (vec![3, 42, 99], vec![])
        )
    }

    #[test]
    fn test_output() {
        assert_eq!(
            execute(vec![4, 3, 99, 42], None, vec![]),
            (vec![4, 3, 99, 42], vec![(0 as usize, 42)])
        )
    }

    #[test]
    fn test_add_pos_pos() {
        assert_eq!(
            execute(vec![1, 5, 6, 7, 99, 4, 5, 0], None, vec![]),
            (vec![1, 5, 6, 7, 99, 4, 5, 9], vec![])
        )
    }

    #[test]
    fn test_add_imm_pos() {
        assert_eq!(
            execute(vec![101, 1, 6, 7, 99, 4, 5, 0], None, vec![]),
            (vec![101, 1, 6, 7, 99, 4, 5, 6], vec![])
        )
    }

    #[test]
    fn test_add_pos_imm() {
        assert_eq!(
            execute(vec![1001, 5, 1, 7, 99, 4, 5, 0], None, vec![]),
            (vec![1001, 5, 1, 7, 99, 4, 5, 5], vec![])
        )
    }

    #[test]
    fn test_mul_pos_pos() {
        assert_eq!(
            execute(vec![2, 5, 6, 7, 99, 4, 5, 0], None, vec![]),
            (vec![2, 5, 6, 7, 99, 4, 5, 20], vec![])
        )
    }

    #[test]
    fn test_mul_imm_pos() {
        assert_eq!(
            execute(vec![102, 1, 6, 7, 99, 4, 5, 0], None, vec![]),
            (vec![102, 1, 6, 7, 99, 4, 5, 5], vec![])
        )
    }

    #[test]
    fn test_mul_pos_imm() {
        assert_eq!(
            execute(vec![1002, 5, 1, 7, 99, 4, 5, 0], None, vec![]),
            (vec![1002, 5, 1, 7, 99, 4, 5, 4], vec![])
        )
    }
}
