#![allow(unused_imports)]

#[macro_use]
extern crate maplit;
#[macro_use]
extern crate num_derive;

use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};

fn main() {
    let result = std::fs::read_to_string("src/bin/day13.txt")
        .map(|file| {
            let line = file
                .lines()
                .filter(|line| !line.is_empty())
                .collect::<Vec<&str>>()[0];
            let code = line
                .split(',')
                .map(|item| item.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            run(code)
                .tiles
                .values()
                .filter(|tile| **tile == Tile::Block)
                .count()
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

fn run(code: Vec<i64>) -> World {
    let mut world = World::new();

    let mut proc = Processor::new(code);
    while {
        let (state, output) = proc.execute(deque!());

        world.add_output(output);

        state != ProcessorState::Halted
    } {}

    world
}

#[derive(Debug, Clone)]
struct World {
    tiles: HashMap<(i64, i64), Tile>,
}

impl World {
    fn new() -> World {
        World {
            tiles: HashMap::new(),
        }
    }

    fn add_output(&mut self, output: VecDeque<i64>) {
        output.iter().chunks(3).into_iter().for_each(|mut chunk| {
            self.add_tile(
                *chunk.next().unwrap(),
                *chunk.next().unwrap(),
                *chunk.next().unwrap(),
            )
        });
    }

    fn add_tile(&mut self, x: i64, y: i64, tile_i: i64) {
        self.tiles
            .insert((x, y), num::FromPrimitive::from_i64(tile_i).unwrap());
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq, FromPrimitive)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    HorizontalPaddle = 3,
    Ball = 4,
}

#[derive(Debug, Clone)]
struct Processor {
    code: Vec<i64>,
    pc: usize,
    rel: i64,
}

impl Processor {
    pub fn new(code: Vec<i64>) -> Processor {
        Processor {
            code,
            pc: 0,
            rel: 0,
        }
    }

    pub fn execute(&mut self, mut input: VecDeque<i64>) -> (ProcessorState, VecDeque<i64>) {
        let mut output: VecDeque<i64> = VecDeque::new();
        let mut state: Option<ProcessorState> = None;

        let debug_pc = false;
        let debug_code = false;

        if debug_code {
            println!("{:?} -> {:?} -> {:?}", input, self.code, output);
        }
        while {
            let raw_opcode = self.code[self.pc];
            let (opcode, (param1_mode, param2_mode, param3_mode)) =
                Processor::parse_opcode(raw_opcode);

            let run_again = if opcode == 99 {
                if debug_pc {
                    println!("PC {}: halt", self.pc);
                }
                state = Some(ProcessorState::Halted);
                false
            } else if opcode == 1 {
                // add in1 in2 out_addr
                let (arg1, arg2, out_addr) =
                    self.parse_inst_data_data_addr(param1_mode, param2_mode, param3_mode);

                if debug_pc {
                    println!("PC {}: {} + {} -> {}", self.pc, arg1, arg2, out_addr);
                }
                self.code[out_addr] = arg1 + arg2;

                self.pc += 4;
                true
            } else if opcode == 2 {
                // mul in1 in2 out_addr
                let (arg1, arg2, out_addr) =
                    self.parse_inst_data_data_addr(param1_mode, param2_mode, param3_mode);

                if debug_pc {
                    println!("PC {}: {} * {} -> {}", self.pc, arg1, arg2, out_addr);
                }
                self.code[out_addr] = arg1 * arg2;

                self.pc += 4;
                true
            } else if opcode == 3 {
                // input write_addr
                let write_addr = self.parse_inst_addr(param1_mode);

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
                let value = self.parse_inst_data(param1_mode);

                if debug_pc {
                    println!("PC {}: output {}", self.pc, value);
                }
                output.push_back(value);

                self.pc += 2;
                true
            } else if opcode == 5 {
                // jump-if-true cond addr
                let (cond, addr) = self.parse_inst_data_data(param1_mode, param2_mode);

                if debug_pc {
                    println!("PC {}: jnz {} {}", self.pc, cond, addr);
                }

                if cond != 0 {
                    self.pc = addr as usize;
                } else {
                    self.pc += 3;
                }
                true
            } else if opcode == 6 {
                // jump-if-false cond addr
                let (cond, addr) = self.parse_inst_data_data(param1_mode, param2_mode);

                if debug_pc {
                    println!("PC {}: jz {} {}", self.pc, cond, addr);
                }

                if cond == 0 {
                    self.pc = addr as usize;
                } else {
                    self.pc += 3;
                }
                true
            } else if opcode == 7 {
                // less-than val1 val2 out_addr
                let (arg1, arg2, out_addr) =
                    self.parse_inst_data_data_addr(param1_mode, param2_mode, param3_mode);

                if debug_pc {
                    println!("PC {}: {} < {} -> {}", self.pc, arg1, arg2, out_addr);
                }
                self.code[out_addr] = if arg1 < arg2 { 1 } else { 0 };

                self.pc += 4;
                true
            } else if opcode == 8 {
                // equals val1 val2 out_addr
                let (arg1, arg2, out_addr) =
                    self.parse_inst_data_data_addr(param1_mode, param2_mode, param3_mode);

                if debug_pc {
                    println!("PC {}: {} == {} -> {}", self.pc, arg1, arg2, out_addr);
                }
                self.code[out_addr] = if arg1 == arg2 { 1 } else { 0 };

                self.pc += 4;
                true
            } else if opcode == 9 {
                // rel val
                let arg1 = self.parse_inst_data(param1_mode);

                if debug_pc {
                    println!("PC {}: rel({}) + {}", self.pc, self.rel, arg1)
                }
                self.rel += arg1;

                self.pc += 2;
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

    fn parse_opcode(raw_opcode: i64) -> (i64, (ParamMode, ParamMode, ParamMode)) {
        (
            raw_opcode % 100,
            (
                Processor::parse_mode(raw_opcode, 100),
                Processor::parse_mode(raw_opcode, 1_000),
                Processor::parse_mode(raw_opcode, 10_000),
            ),
        )
    }

    fn parse_mode(raw_opcode: i64, offset: i64) -> ParamMode {
        let raw_mode = raw_opcode % (offset * 10) / offset;
        match raw_mode {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            _ => panic!(
                "Got unknown parameter mode for opcode {} for pos {} ({})",
                raw_opcode,
                offset / 100,
                raw_mode
            ),
        }
    }

    fn parse_inst_addr(&mut self, param1_mode: ParamMode) -> usize {
        (self.get_addr(1, param1_mode))
    }

    fn parse_inst_data(&mut self, param1_mode: ParamMode) -> i64 {
        let param1_addr = self.get_addr(1, param1_mode);
        (self.code[param1_addr])
    }

    fn parse_inst_data_data(
        &mut self,
        param1_mode: ParamMode,
        param2_mode: ParamMode,
    ) -> (i64, i64) {
        let param1_addr = self.get_addr(1, param1_mode);
        let param2_addr = self.get_addr(2, param2_mode);
        (self.code[param1_addr], self.code[param2_addr])
    }

    fn parse_inst_data_data_addr(
        &mut self,
        param1_mode: ParamMode,
        param2_mode: ParamMode,
        param3_mode: ParamMode,
    ) -> (i64, i64, usize) {
        let param1_addr = self.get_addr(1, param1_mode);
        let param2_addr = self.get_addr(2, param2_mode);
        let param3_addr = self.get_addr(3, param3_mode);
        (self.code[param1_addr], self.code[param2_addr], param3_addr)
    }

    fn get_addr(&mut self, offset: usize, mode: ParamMode) -> usize {
        let addr = match mode {
            ParamMode::Position => self.code[self.pc + offset] as usize,
            ParamMode::Immediate => self.pc + offset,
            ParamMode::Relative => (self.rel + self.code[self.pc + offset]) as usize,
        };
        self.ensure_capacity(addr);
        addr
    }

    fn ensure_capacity(&mut self, addr: usize) {
        if addr >= self.code.len() {
            let additional = addr - self.code.len();
            //            println!("Growing memory by {}", additional + 1);
            self.code.reserve(additional);
            for _ in 0..=additional {
                self.code.push(0);
            }
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum ParamMode {
    Immediate,
    Position,
    Relative,
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

    #[test]
    fn test_quine() {
        assert_eq!(
            Processor::new(vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99
            ])
            .execute(deque!()),
            (
                ProcessorState::Halted,
                deque!(109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99)
            )
        )
    }

    #[test]
    fn test_large_number() {
        assert_eq!(
            Processor::new(vec![104, 1125899906842624, 99]).execute(deque!()),
            (ProcessorState::Halted, deque!(1125899906842624))
        )
    }

    #[test]
    fn test_203_micro_1() {
        assert_eq!(
            Processor::new(vec![109, -1, 4, 1, 99]).execute(deque!()),
            (ProcessorState::Halted, deque!(-1))
        )
    }

    #[test]
    fn test_203_micro_2() {
        assert_eq!(
            Processor::new(vec![109, -1, 104, 1, 99]).execute(deque!()),
            (ProcessorState::Halted, deque!(1))
        )
    }

    #[test]
    fn test_203_micro_3() {
        assert_eq!(
            Processor::new(vec![109, -1, 204, 1, 99]).execute(deque!()),
            (ProcessorState::Halted, deque!(109))
        )
    }

    #[test]
    fn test_203_micro_4() {
        assert_eq!(
            Processor::new(vec![109, 1, 9, 2, 204, -6, 99]).execute(deque!()),
            (ProcessorState::Halted, deque!(204))
        )
    }

    #[test]
    fn test_203_micro_5() {
        assert_eq!(
            Processor::new(vec![109, 1, 109, 9, 204, -6, 99]).execute(deque!()),
            (ProcessorState::Halted, deque!(204))
        )
    }

    #[test]
    fn test_203_micro_6() {
        assert_eq!(
            Processor::new(vec![109, 1, 209, -1, 204, -106, 99]).execute(deque!()),
            (ProcessorState::Halted, deque!(204))
        )
    }

    #[test]
    fn test_203_micro_7() {
        assert_eq!(
            Processor::new(vec![109, 1, 3, 3, 204, 2, 99]).execute(deque!(42)),
            (ProcessorState::Halted, deque!(42))
        )
    }

    #[test]
    fn test_203_micro_8() {
        assert_eq!(
            Processor::new(vec![109, 1, 203, 2, 204, 2, 99]).execute(deque!(42)),
            (ProcessorState::Halted, deque!(42))
        )
    }

    #[test]
    fn test_output_to_world() {
        let mut world = World::new();
        world.add_output(deque!(1, 2, 3, 6, 5, 4));
        assert_eq!(
            world.tiles,
            hashmap!((1, 2) => Tile::HorizontalPaddle, (6, 5) => Tile::Ball)
        )
    }
}
