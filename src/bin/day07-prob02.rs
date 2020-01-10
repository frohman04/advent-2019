extern crate itertools;

use std::collections::VecDeque;

use itertools::Itertools;

fn main() {
    let result = std::fs::read_to_string("src/bin/day07.txt")
        .map(|file| {
            let line = file
                .split("\n")
                .filter(|line| line.len() > 0)
                .collect::<Vec<&str>>()[0];
            let code = line
                .split(",")
                .map(|item| item.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            find_answer(code)
        })
        .expect("Unable to open file");

    println!("{}", result);
}

#[macro_export(local_inner_macros)]
macro_rules! deque {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(deque!(@single $rest)),*]));

    ($($key:expr,)+) => { deque!($($key),+) };
    ($($key:expr),*) => {
        {
            let _cap = deque!(@count $($key),*);
            let mut _set = ::std::collections::VecDeque::with_capacity(_cap);
            $(
                let _ = _set.push_back($key);
            )*
            _set
        }
    };
}

fn find_answer(code: Vec<i32>) -> i32 {
    (5..=9)
        .permutations(5)
        .map(|phases| {
            let mut amp_a = new_proc(code.clone(), phases[0]);
            let mut amp_b = new_proc(code.clone(), phases[1]);
            let mut amp_c = new_proc(code.clone(), phases[2]);
            let mut amp_d = new_proc(code.clone(), phases[3]);
            let mut amp_e = new_proc(code.clone(), phases[4]);

            let mut init_input = 0;
            while {
                let (_, mut output) = amp_a.execute(deque!(init_input));
                let (_, mut output) = amp_b.execute(deque!(output.pop_front().unwrap()));
                let (_, mut output) = amp_c.execute(deque!(output.pop_front().unwrap()));
                let (_, mut output) = amp_d.execute(deque!(output.pop_front().unwrap()));
                let (state, mut output) = amp_e.execute(deque!(output.pop_front().unwrap()));

                init_input = output.pop_front().unwrap();
                state != ProcessorState::Halted
            } {}

            init_input
        })
        .max()
        .unwrap()
}

fn new_proc(code: Vec<i32>, phase: i32) -> Processor {
    let mut proc = Processor::new(code);
    let (state, _) = proc.execute(deque!(phase));
    assert_eq!(state, ProcessorState::IoWait);
    proc
}

#[derive(Debug, Clone)]
struct Processor {
    code: Vec<i32>,
    pc: usize,
}

impl Processor {
    fn new(code: Vec<i32>) -> Processor {
        Processor { code, pc: 0 }
    }

    fn execute(&mut self, mut input: VecDeque<i32>) -> (ProcessorState, VecDeque<i32>) {
        let mut output: VecDeque<i32> = VecDeque::new();
        let mut state: Option<ProcessorState> = None;

        let debug_pc = false;
        let debug_code = false;

        if debug_code {
            println!("{:?} -> {:?} -> {:?}", input, self.code, output);
        }
        while {
            let raw_opcode = self.code[self.pc];
            let opcode = raw_opcode % 100;
            let param1_imm = raw_opcode % 1000 / 100 == 1;
            let param2_imm = raw_opcode % 10000 / 1000 == 1;
            let param3_imm = raw_opcode % 100000 / 10000 == 1;

            let run_again = if opcode == 99 {
                if debug_pc {
                    println!("PC {}: halt", self.pc);
                }
                state = Some(ProcessorState::Halted);
                false
            } else if opcode == 1 {
                // add in1 in2 out_addr
                let arg1 = self.get_val(1, param1_imm);
                let arg2 = self.get_val(2, param2_imm);
                if param3_imm {
                    panic!(
                        "Param 3 cannot be immediate mode for opcode 01 at PC {}",
                        self.pc
                    );
                }
                let out_addr = self.code[self.pc + 3] as usize;

                if debug_pc {
                    println!("PC {}: {} + {} -> {}", self.pc, arg1, arg2, out_addr);
                }
                self.code[out_addr] = arg1 + arg2;

                self.pc += 4;
                true
            } else if opcode == 2 {
                // mul in1 in2 out_addr
                let arg1 = self.get_val(1, param1_imm);
                let arg2 = self.get_val(2, param2_imm);
                if param3_imm {
                    panic!(
                        "Param 3 cannot be immediate mode for opcode 02 at PC {}",
                        self.pc
                    );
                }
                let out_addr = self.code[self.pc + 3] as usize;

                if debug_pc {
                    println!("PC {}: {} * {} -> {}", self.pc, arg1, arg2, out_addr);
                }
                self.code[out_addr] = arg1 * arg2;

                self.pc += 4;
                true
            } else if opcode == 3 {
                // input write_addr
                if param1_imm {
                    panic!(
                        "Param 1 cannot be immediate mode for opcode 03 at PC {}",
                        self.pc
                    );
                }
                let write_addr = self.code[self.pc + 1] as usize;

                if input.is_empty() {
                    state = Some(ProcessorState::IoWait);
                    false
                } else {
                    let input_val = input.pop_front().unwrap();

                    if debug_pc {
                        println!("PC {}: read {} -> {}", self.pc, input_val, write_addr);
                    }
                    self.code[write_addr] = input_val;

                    self.pc += 2;
                    true
                }
            } else if opcode == 4 {
                // output read_addr
                let value = self.get_val(1, param1_imm);

                if debug_pc {
                    println!("PC {}: output {}", self.pc, value);
                }
                output.push_back(value);

                self.pc += 2;
                true
            } else if opcode == 5 {
                // jump-if-true cond addr
                let cond = self.get_val(1, param1_imm);
                let addr = self.get_val(2, param2_imm) as usize;

                if debug_pc {
                    println!("PC {}: jnz {} {}", self.pc, cond, addr);
                }

                if cond != 0 {
                    self.pc = addr;
                } else {
                    self.pc += 3;
                }
                true
            } else if opcode == 6 {
                // jump-if-false cond addr
                let cond = self.get_val(1, param1_imm);
                let addr = self.get_val(2, param2_imm) as usize;

                if debug_pc {
                    println!("PC {}: jz {} {}", self.pc, cond, addr);
                }

                if cond == 0 {
                    self.pc = addr;
                } else {
                    self.pc += 3;
                }
                true
            } else if opcode == 7 {
                // less-than val1 val2 out_addr
                let arg1 = self.get_val(1, param1_imm);
                let arg2 = self.get_val(2, param2_imm);
                if param3_imm {
                    panic!(
                        "Param 3 cannot be immediate mode for opcode 07 at PC {}",
                        self.pc
                    );
                }
                let out_addr = self.code[self.pc + 3] as usize;

                if debug_pc {
                    println!("PC {}: {} < {} -> {}", self.pc, arg1, arg2, out_addr);
                }
                self.code[out_addr] = if arg1 < arg2 { 1 } else { 0 };

                self.pc += 4;
                true
            } else if opcode == 8 {
                // equals val1 val2 out_addr
                let arg1 = self.get_val(1, param1_imm);
                let arg2 = self.get_val(2, param2_imm);
                if param3_imm {
                    panic!(
                        "Param 3 cannot be immediate mode for opcode 08 at PC {}",
                        self.pc
                    );
                }
                let out_addr = self.code[self.pc + 3] as usize;

                if debug_pc {
                    println!("PC {}: {} == {} -> {}", self.pc, arg1, arg2, out_addr);
                }
                self.code[out_addr] = if arg1 == arg2 { 1 } else { 0 };

                self.pc += 4;
                true
            } else {
                panic!("Unexpected opcode {} at {}", raw_opcode, self.pc)
            };
            if debug_code {
                println!("{:?} -> {:?} -> {:?}", input, self.code, output);
            }

            run_again
        } {}

        if state.is_none() {
            panic!("Execution ended without setting a state!")
        }
        (state.unwrap(), output)
    }

    fn get_val(&self, offset: usize, is_imm: bool) -> i32 {
        if is_imm {
            self.code[self.pc + offset]
        } else {
            self.code[self.code[self.pc + offset] as usize]
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum ProcessorState {
    /// The processor is currently waiting at an input instruction for additional data to arrive
    /// through the input deque
    IoWait,
    /// The processor encountered a HALT instruction and has stopped running
    Halted,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input_output() {
        assert_eq!(
            Processor::new(vec![3, 1, 4, 1, 99]).execute(deque!(42)),
            (ProcessorState::Halted, deque!(42))
        )
    }

    #[test]
    fn test_no_input_suspend() {
        assert_eq!(
            Processor::new(vec![3, 1, 4, 1, 99]).execute(deque!()),
            (ProcessorState::IoWait, deque!())
        )
    }

    #[test]
    fn test_add_pos_pos() {
        assert_eq!(
            Processor::new(vec![1, 7, 8, 9, 4, 9, 99, 4, 5, 0]).execute(deque!()),
            (ProcessorState::Halted, deque!(9))
        )
    }

    #[test]
    fn test_add_imm_pos() {
        assert_eq!(
            Processor::new(vec![101, 1, 8, 9, 4, 9, 99, 4, 5, 0]).execute(deque!()),
            (ProcessorState::Halted, deque!(6))
        )
    }

    #[test]
    fn test_add_pos_imm() {
        assert_eq!(
            Processor::new(vec![1001, 7, 1, 9, 4, 9, 99, 4, 5, 0]).execute(deque!()),
            (ProcessorState::Halted, deque!(5))
        )
    }

    #[test]
    fn test_mul_pos_pos() {
        assert_eq!(
            Processor::new(vec![2, 7, 8, 9, 4, 9, 99, 4, 5, 0]).execute(deque!()),
            (ProcessorState::Halted, deque!(20))
        )
    }

    #[test]
    fn test_mul_imm_pos() {
        assert_eq!(
            Processor::new(vec![102, 1, 8, 9, 4, 9, 99, 4, 5, 0]).execute(deque!()),
            (ProcessorState::Halted, deque!(5))
        )
    }

    #[test]
    fn test_mul_pos_imm() {
        assert_eq!(
            Processor::new(vec![1002, 7, 1, 9, 4, 9, 99, 4, 5, 0]).execute(deque!()),
            (ProcessorState::Halted, deque!(4))
        )
    }

    #[test]
    fn test_equal_pos_1() {
        assert_eq!(
            Processor::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]).execute(deque!(8)),
            (ProcessorState::Halted, deque!(1))
        )
    }

    #[test]
    fn test_equal_pos_0() {
        assert_eq!(
            Processor::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]).execute(deque!(7)),
            (ProcessorState::Halted, deque!(0))
        )
    }

    #[test]
    fn test_lt_pos_1() {
        assert_eq!(
            Processor::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]).execute(deque!(7)),
            (ProcessorState::Halted, deque!(1))
        )
    }

    #[test]
    fn test_lt_pos_0() {
        assert_eq!(
            Processor::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]).execute(deque!(10)),
            (ProcessorState::Halted, deque!(0))
        )
    }

    #[test]
    fn test_equal_imm_1() {
        assert_eq!(
            Processor::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]).execute(deque!(8)),
            (ProcessorState::Halted, deque!(1))
        )
    }

    #[test]
    fn test_equal_imm_0() {
        assert_eq!(
            Processor::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]).execute(deque!(7)),
            (ProcessorState::Halted, deque!(0))
        )
    }

    #[test]
    fn test_lt_imm_1() {
        assert_eq!(
            Processor::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]).execute(deque!(7)),
            (ProcessorState::Halted, deque!(1))
        )
    }

    #[test]
    fn test_lt_imm_0() {
        assert_eq!(
            Processor::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]).execute(deque!(10)),
            (ProcessorState::Halted, deque!(0))
        )
    }

    #[test]
    fn test_jump_nz_pos_1() {
        assert_eq!(
            Processor::new(vec![
                3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9
            ])
            .execute(deque!(10)),
            (ProcessorState::Halted, deque!(1))
        )
    }

    #[test]
    fn test_jump_nz_pos_0() {
        assert_eq!(
            Processor::new(vec![
                3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9
            ])
            .execute(deque!(0)),
            (ProcessorState::Halted, deque!(0))
        )
    }

    #[test]
    fn test_jump_nz_imm_1() {
        assert_eq!(
            Processor::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1])
                .execute(deque!(10)),
            (ProcessorState::Halted, deque!(1))
        )
    }

    #[test]
    fn test_jump_nz_imm_0() {
        assert_eq!(
            Processor::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1])
                .execute(deque!(10)),
            (ProcessorState::Halted, deque!(1))
        )
    }

    #[test]
    fn test_all_lt_8() {
        assert_eq!(
            Processor::new(vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ])
            .execute(deque!(7)),
            (ProcessorState::Halted, deque!(999))
        )
    }

    #[test]
    fn test_all_eq_8() {
        assert_eq!(
            Processor::new(vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ])
            .execute(deque!(8)),
            (ProcessorState::Halted, deque!(1000))
        )
    }

    #[test]
    fn test_all_gt_8() {
        assert_eq!(
            Processor::new(vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ])
            .execute(deque!(9)),
            (ProcessorState::Halted, deque!(1001))
        )
    }
}
