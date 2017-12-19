use std::collections::{HashMap, LinkedList};
use std::ops::{Add, Mul, Rem};
use std::str::FromStr;
use utils;

pub fn recover_frequency(program: &str) -> Option<i64> {
    let mut sound_processor = SoundProcessor::new(parse_program(program)?);
    Some(sound_processor.run())
}

pub fn count_sends(program: &str) -> Option<u32> {
    let mut program_0 = ParallelProgram::new(parse_program(program)?, 0);
    let mut program_1 = ParallelProgram::new(parse_program(program)?, 1);
    while !program_0.is_locked() || !program_1.is_locked() {
        while !program_0.is_waiting() {
            program_0.step();
        }
        while !program_1.is_waiting() {
            program_1.step();
        }
        if let Some(value) = program_1.get_sent() {
            program_0.receive(value);
        }
        if let Some(value) = program_0.get_sent() {
            program_1.receive(value);
        }
    }
    Some(program_1.sent_count)
}

fn parse_program(program: &str) -> Option<Vec<Instruction>> {
    let mut instructions = vec![];
    for line in program.lines() {
        instructions.push(Instruction::from_str(line).ok()?);
    }
    Some(instructions)
}

struct ParallelProgram {
    program: Vec<Instruction>,
    registers: HashMap<char, i64>,
    sent_queue: LinkedList<i64>,
    program_counter: usize,
    sent_count: u32,
}

impl ParallelProgram {
    fn new(program: Vec<Instruction>, id: i64) -> ParallelProgram {
        let mut program = ParallelProgram {
            program: program,
            registers: HashMap::new(),
            sent_queue: LinkedList::new(),
            program_counter: 0,
            sent_count: 0,
        };
        program.registers.insert('p', id);
        program
    }

    fn step(&mut self) {
        match self.program[self.program_counter] {
            Instruction::Snd(ref register) => {
                let value = get_register_value(&mut self.registers, *register);
                self.sent_queue.push_back(value);
                self.sent_count += 1;
                self.program_counter += 1;
            }
            Instruction::AritIns(ref register, ref value, ref op) => {
                let register_value = get_register_value(&mut self.registers, *register);
                let value = get_value(&mut self.registers, value);
                self.registers.insert(*register, op(register_value, value));
                self.program_counter += 1;
            }
            Instruction::Jgz(ref condition, ref value) => {
                if get_value(&mut self.registers, condition) > 0 {
                    let jump = get_value(&mut self.registers, value);
                    self.program_counter = (self.program_counter as i64 + jump) as usize;
                } else {
                    self.program_counter += 1;
                }
            }
            Instruction::Rcv(_) => {}
        }
    }
    fn receive(&mut self, value: i64) -> bool {
        match self.program[self.program_counter] {
            Instruction::Rcv(ref register) => {
                self.registers.insert(*register, value);
                self.program_counter += 1;
                true
            }
            _ => false,
        }
    }
    fn is_waiting(&self) -> bool {
        match self.program[self.program_counter] {
            Instruction::Rcv(_) => true,
            _ => false,
        }
    }
    fn is_locked(&self) -> bool {
        self.is_waiting() && self.sent_queue.is_empty()
    }
    fn get_sent(&mut self) -> Option<i64> {
        self.sent_queue.pop_front()
    }
}

struct SoundProcessor {
    program: Vec<Instruction>,
    registers: HashMap<char, i64>,
    last_sound: i64,
    program_counter: usize,
}

impl SoundProcessor {
    fn new(program: Vec<Instruction>) -> SoundProcessor {
        SoundProcessor {
            program: program,
            registers: HashMap::new(),
            last_sound: 0,
            program_counter: 0,
        }
    }

    fn run(&mut self) -> i64 {
        loop {
            let instruction = &self.program[self.program_counter];
            match *instruction {
                Instruction::Snd(ref register) => {
                    let value = get_register_value(&mut self.registers, *register);
                    if value != 0 {
                        self.last_sound = value;
                    }
                }
                Instruction::AritIns(ref register, ref value, ref op) => {
                    let register_value = get_register_value(&mut self.registers, *register);
                    let value = get_value(&mut self.registers, value);
                    self.registers.insert(*register, op(register_value, value));
                }
                Instruction::Rcv(ref register) => {
                    let last_sound = self.last_sound;
                    if last_sound != 0 {
                        self.registers.insert(*register, last_sound);
                        return last_sound;
                    }
                }
                Instruction::Jgz(ref condition, ref value) => {
                    if get_value(&mut self.registers, condition) > 0 {
                        let jump = get_value(&mut self.registers, value);
                        self.program_counter = (self.program_counter as i64 + jump) as usize;
                        continue;
                    }
                }
            }
            self.program_counter += 1;
        }
    }
}

fn get_register_value(registers: &mut HashMap<char, i64>, register: char) -> i64 {
    if !registers.contains_key(&register) {
        registers.insert(register, 0);
    }
    *registers.get(&register).unwrap()
}

fn get_value(registers: &mut HashMap<char, i64>, operand: &Operand) -> i64 {
    match *operand {
        Operand::Register(register) => get_register_value(registers, register),
        Operand::Value(value) => value as i64,
    }
}

enum Instruction {
    Snd(char),
    AritIns(char, Operand, &'static Fn(i64, i64) -> i64),
    Rcv(char),
    Jgz(Operand, Operand),
}

impl Instruction {
    fn set(_: i64, x: i64) -> i64 {
        x
    }
}

impl FromStr for Instruction {
    type Err = utils::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.trim().split(' ');
        match tokens.next() {
            Some("snd") => Ok(Instruction::Snd(
                char::from_str(&tokens.next().ok_or(utils::Error)?)?,
            )),
            Some("set") => {
                Ok(Instruction::AritIns(
                    char::from_str(&tokens.next().ok_or(utils::Error)?)?,
                    Operand::from_str(tokens.next().ok_or(utils::Error)?)?,
                    &Instruction::set,
                ))
            }
            Some("add") => {
                Ok(Instruction::AritIns(
                    char::from_str(&tokens.next().ok_or(utils::Error)?)?,
                    Operand::from_str(tokens.next().ok_or(utils::Error)?)?,
                    &Add::<i64>::add,
                ))
            }
            Some("mul") => {
                Ok(Instruction::AritIns(
                    char::from_str(&tokens.next().ok_or(utils::Error)?)?,
                    Operand::from_str(tokens.next().ok_or(utils::Error)?)?,
                    &Mul::<i64>::mul,
                ))
            }
            Some("mod") => {
                Ok(Instruction::AritIns(
                    char::from_str(&tokens.next().ok_or(utils::Error)?)?,
                    Operand::from_str(tokens.next().ok_or(utils::Error)?)?,
                    &Rem::<i64>::rem,
                ))
            }
            Some("rcv") => Ok(Instruction::Rcv(
                char::from_str(&tokens.next().ok_or(utils::Error)?)?,
            )),
            Some("jgz") => {
                Ok(Instruction::Jgz(
                    Operand::from_str(tokens.next().ok_or(utils::Error)?)?,
                    Operand::from_str(tokens.next().ok_or(utils::Error)?)?,
                ))
            }
            _ => Err(utils::Error),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone)]
enum Operand {
    Register(char),
    Value(i32),
}

impl FromStr for Operand {
    type Err = utils::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = s.trim().parse() {
            Ok(Operand::Value(value))
        } else {
            Ok(Operand::Register(char::from_str(s)?))
        }
    }
}

#[test]
fn test_examples() {
    assert_eq!(
        Some(4),
        recover_frequency(
            "set a 1\nadd a 2\nmul a a\nmod a 5\nsnd a\nset a 0\nrcv a\njgz a -1\nset a 1\njgz a -2",
        )
    );
    assert_eq!(
        Some(3),
        count_sends("snd 1\nsnd 2\nsnd p\nrcv a\nrcv b\nrcv c\nrcv d")
    );
}
