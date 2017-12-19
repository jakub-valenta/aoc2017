use std::collections::HashMap;
use std::str::FromStr;
use utils;

pub fn recover_frequency(program: &str) -> Option<i64> {
    let mut sound_processor = SoundProcessor::new(parse_program(program)?);
    Some(sound_processor.run())
}

fn parse_program(program: &str) -> Option<Vec<Instruction>> {
    let mut instructions = vec![];
    for line in program.lines() {
        instructions.push(Instruction::from_str(line).ok()?);
    }
    Some(instructions)
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

    fn get_register_value(&mut self, register: char) -> i64 {
        if !self.registers.contains_key(&register) {
            self.registers.insert(register, 0);
        }
        *self.registers.get(&register).unwrap()
    }

    fn get_value(&mut self, operand: &Operand) -> i64 {
        match *operand {
            Operand::Register(register) => self.get_register_value(register),
            Operand::Value(value) => value as i64,
        }
    }

    fn run(&mut self) -> i64 {
        loop {
            let instruction = self.program[self.program_counter].clone();
            match instruction {
                Instruction::Snd(register) => {
                    let value = self.get_register_value(register);
                    if value != 0 {
                        self.last_sound = value;
                    }
                }
                Instruction::Set(register, value) => {
                    let value = self.get_value(&value);
                    self.registers.insert(register, value);
                }
                Instruction::Add(register, value) => {
                    let value = self.get_register_value(register) + self.get_value(&value);
                    self.registers.insert(register, value);
                }
                Instruction::Mul(register, value) => {
                    let value = self.get_register_value(register) * self.get_value(&value);
                    self.registers.insert(register, value);
                }
                Instruction::Mod(register, value) => {
                    let value = self.get_register_value(register) % self.get_value(&value);
                    self.registers.insert(register, value);
                }
                Instruction::Rcv(register) => {
                    let last_sound = self.last_sound;
                    if last_sound != 0 {
                        self.registers.insert(register, last_sound);
                        return last_sound;
                    }
                }
                Instruction::Jgz(register, value) => {
                    if self.get_register_value(register) != 0 {
                        let jump = self.get_value(&value);
                        self.program_counter = (self.program_counter as i64 + jump) as usize;
                        continue;
                    }
                }
            }
            self.program_counter += 1;
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone)]
enum Instruction {
    Snd(char),
    Set(char, Operand),
    Add(char, Operand),
    Mul(char, Operand),
    Mod(char, Operand),
    Rcv(char),
    Jgz(char, Operand),
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
                Ok(Instruction::Set(
                    char::from_str(&tokens.next().ok_or(utils::Error)?)?,
                    Operand::from_str(tokens.next().ok_or(utils::Error)?)?,
                ))
            }
            Some("add") => {
                Ok(Instruction::Add(
                    char::from_str(&tokens.next().ok_or(utils::Error)?)?,
                    Operand::from_str(tokens.next().ok_or(utils::Error)?)?,
                ))
            }
            Some("mul") => {
                Ok(Instruction::Mul(
                    char::from_str(&tokens.next().ok_or(utils::Error)?)?,
                    Operand::from_str(tokens.next().ok_or(utils::Error)?)?,
                ))
            }
            Some("mod") => {
                Ok(Instruction::Mod(
                    char::from_str(&tokens.next().ok_or(utils::Error)?)?,
                    Operand::from_str(tokens.next().ok_or(utils::Error)?)?,
                ))
            }
            Some("rcv") => Ok(Instruction::Rcv(
                char::from_str(&tokens.next().ok_or(utils::Error)?)?,
            )),
            Some("jgz") => {
                Ok(Instruction::Jgz(
                    char::from_str(&tokens.next().ok_or(utils::Error)?)?,
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
}
