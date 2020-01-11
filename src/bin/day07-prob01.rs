extern crate itertools;

use itertools::Itertools;

fn main() {
    let result = std::fs::read_to_string("src/bin/day07.txt")
        .map(|file| {
            let line = file
                .split('\n')
                .filter(|line| !line.is_empty())
                .collect::<Vec<&str>>()[0];
            let code = line
                .split(',')
                .map(|item| item.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            find_answer(code)
        })
        .expect("Unable to open file");

    println!("{}", result);
}

fn find_answer(code: Vec<i32>) -> i32 {
    (0..=4)
        .permutations(5)
        .map(|phases| {
            let mut last_output = 0;
            for phase in phases {
                let input = vec![phase, last_output];
                let (_code, output) = execute(code.clone(), None, input);
                last_output = output.last().unwrap().1
            }
            last_output
        })
        .max()
        .unwrap()
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
    if let Some(init_state) = init_state {
        code[1] = init_state.noun;
        code[2] = init_state.verb;
    }

    let debug_pc = false;
    let debug_code = false;

    input.reverse();
    let mut output: Vec<(usize, i32)> = Vec::new();

    if debug_code {
        println!("{:?} -> {:?} -> {:?}", input, code, output);
    }
    while {
        let raw_opcode = code[pc];
        let opcode = raw_opcode % 100;
        let param1_imm = raw_opcode % 1_000 / 100 == 1;
        let param2_imm = raw_opcode % 10_000 / 1_000 == 1;
        let param3_imm = raw_opcode % 100_000 / 10_000 == 1;

        let run_again = if opcode == 99 {
            if debug_pc {
                println!("PC {}: halt", pc);
            }
            false
        } else if opcode == 1 {
            // add in1 in2 out_addr
            let arg1 = get_val(&code, pc + 1, param1_imm);
            let arg2 = get_val(&code, pc + 2, param2_imm);
            if param3_imm {
                panic!(
                    "Param 3 cannot be immediate mode for opcode 01 at PC {}",
                    pc
                );
            }
            let out_addr = code[pc + 3] as usize;

            if debug_pc {
                println!("PC {}: {} + {} -> {}", pc, arg1, arg2, out_addr);
            }
            code[out_addr] = arg1 + arg2;

            pc += 4;
            true
        } else if opcode == 2 {
            // mul in1 in2 out_addr
            let arg1 = get_val(&code, pc + 1, param1_imm);
            let arg2 = get_val(&code, pc + 2, param2_imm);
            if param3_imm {
                panic!(
                    "Param 3 cannot be immediate mode for opcode 02 at PC {}",
                    pc
                );
            }
            let out_addr = code[pc + 3] as usize;

            if debug_pc {
                println!("PC {}: {} * {} -> {}", pc, arg1, arg2, out_addr);
            }
            code[out_addr] = arg1 * arg2;

            pc += 4;
            true
        } else if opcode == 3 {
            // input write_addr
            if param1_imm {
                panic!(
                    "Param 1 cannot be immediate mode for opcode 03 at PC {}",
                    pc
                );
            }
            let write_addr = code[pc + 1] as usize;
            let input_val = input.pop().unwrap();

            if debug_pc {
                println!("PC {}: read {} -> {}", pc, input_val, write_addr);
            }
            code[write_addr] = input_val;

            pc += 2;
            true
        } else if opcode == 4 {
            // output read_addr
            let value = get_val(&code, pc + 1, param1_imm);

            if debug_pc {
                println!("PC {}: output {}", pc, value);
            }
            output.push((pc, value));

            pc += 2;
            true
        } else if opcode == 5 {
            // jump-if-true cond addr
            let cond = get_val(&code, pc + 1, param1_imm);
            let addr = get_val(&code, pc + 2, param2_imm) as usize;

            if debug_pc {
                println!("PC {}: jnz {} {}", pc, cond, addr);
            }

            if cond != 0 {
                pc = addr;
            } else {
                pc += 3;
            }
            true
        } else if opcode == 6 {
            // jump-if-false cond addr
            let cond = get_val(&code, pc + 1, param1_imm);
            let addr = get_val(&code, pc + 2, param2_imm) as usize;

            if debug_pc {
                println!("PC {}: jz {} {}", pc, cond, addr);
            }

            if cond == 0 {
                pc = addr;
            } else {
                pc += 3;
            }
            true
        } else if opcode == 7 {
            // less-than val1 val2 out_addr
            let arg1 = get_val(&code, pc + 1, param1_imm);
            let arg2 = get_val(&code, pc + 2, param2_imm);
            if param3_imm {
                panic!(
                    "Param 3 cannot be immediate mode for opcode 07 at PC {}",
                    pc
                );
            }
            let out_addr = code[pc + 3] as usize;

            if debug_pc {
                println!("PC {}: {} < {} -> {}", pc, arg1, arg2, out_addr);
            }
            code[out_addr] = if arg1 < arg2 { 1 } else { 0 };

            pc += 4;
            true
        } else if opcode == 8 {
            // equals val1 val2 out_addr
            let arg1 = get_val(&code, pc + 1, param1_imm);
            let arg2 = get_val(&code, pc + 2, param2_imm);
            if param3_imm {
                panic!(
                    "Param 3 cannot be immediate mode for opcode 08 at PC {}",
                    pc
                );
            }
            let out_addr = code[pc + 3] as usize;

            if debug_pc {
                println!("PC {}: {} == {} -> {}", pc, arg1, arg2, out_addr);
            }
            code[out_addr] = if arg1 == arg2 { 1 } else { 0 };

            pc += 4;
            true
        } else {
            panic!("Unexpected opcode {} at {}", raw_opcode, pc)
        };
        if debug_code {
            println!("{:?} -> {:?} -> {:?}", input, code, output);
        }

        run_again
    } {}

    (code, output)
}

fn get_val(code: &[i32], i: usize, is_imm: bool) -> i32 {
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

    #[test]
    fn test_equal_pos_1() {
        assert_eq!(
            execute(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], None, vec![8]).1,
            vec![(6, 1)]
        )
    }

    #[test]
    fn test_equal_pos_0() {
        assert_eq!(
            execute(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], None, vec![7]).1,
            vec![(6, 0)]
        )
    }

    #[test]
    fn test_lt_pos_1() {
        assert_eq!(
            execute(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], None, vec![7]).1,
            vec![(6, 1)]
        )
    }

    #[test]
    fn test_lt_pos_0() {
        assert_eq!(
            execute(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], None, vec![10]).1,
            vec![(6, 0)]
        )
    }

    #[test]
    fn test_equal_imm_1() {
        assert_eq!(
            execute(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], None, vec![8]).1,
            vec![(6, 1)]
        )
    }

    #[test]
    fn test_equal_imm_0() {
        assert_eq!(
            execute(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], None, vec![7]).1,
            vec![(6, 0)]
        )
    }

    #[test]
    fn test_lt_imm_1() {
        assert_eq!(
            execute(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], None, vec![7]).1,
            vec![(6, 1)]
        )
    }

    #[test]
    fn test_lt_imm_0() {
        assert_eq!(
            execute(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], None, vec![10]).1,
            vec![(6, 0)]
        )
    }

    #[test]
    fn test_jump_nz_pos_1() {
        assert_eq!(
            execute(
                vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                None,
                vec![10]
            )
            .1
            .last()
            .unwrap()
            .1,
            1
        )
    }

    #[test]
    fn test_jump_nz_pos_0() {
        assert_eq!(
            execute(
                vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                None,
                vec![0]
            )
            .1
            .last()
            .unwrap()
            .1,
            0
        )
    }

    #[test]
    fn test_jump_nz_imm_1() {
        assert_eq!(
            execute(
                vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
                None,
                vec![10]
            )
            .1
            .last()
            .unwrap()
            .1,
            1
        )
    }

    #[test]
    fn test_jump_nz_imm_0() {
        assert_eq!(
            execute(
                vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
                None,
                vec![0]
            )
            .1
            .last()
            .unwrap()
            .1,
            0
        )
    }

    #[test]
    fn test_all_lt_8() {
        assert_eq!(
            execute(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
                ],
                None,
                vec![7]
            )
            .1
            .last()
            .unwrap()
            .1,
            999
        )
    }

    #[test]
    fn test_all_eq_8() {
        assert_eq!(
            execute(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
                ],
                None,
                vec![8]
            )
            .1
            .last()
            .unwrap()
            .1,
            1000
        )
    }

    #[test]
    fn test_all_gt_8() {
        assert_eq!(
            execute(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
                ],
                None,
                vec![9]
            )
            .1
            .last()
            .unwrap()
            .1,
            1001
        )
    }

    #[test]
    fn test_find_answer_1() {
        assert_eq!(
            find_answer(vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0
            ]),
            43210
        )
    }

    #[test]
    fn test_find_answer_2() {
        assert_eq!(
            find_answer(vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0
            ]),
            54321
        )
    }

    #[test]
    fn test_find_answer_3() {
        assert_eq!(
            find_answer(vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
            ]),
            65210
        )
    }
}
