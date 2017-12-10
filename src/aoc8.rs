use std::collections::HashMap;
use std::convert::AsRef;
use std::num::ParseIntError;
use std::str::FromStr;

pub fn find_max(program: &str) -> Option<i32> {
    let instructions = parse_program(program)?;
    let mut registers = HashMap::new();
    for instruction in instructions.iter() {
        instruction.process(&mut registers);
    }
    for (k, v) in registers.iter() {
        println!("({}, {})", k, v);
    }
    Some(*registers.iter().max_by(|&(_, l), &(_, r)| l.cmp(r))?.1)
}

fn parse_program(program: &str) -> Option<Vec<Instruction>> {
    let mut instructions = vec![];
    for line in program.lines() {
        instructions.push(Instruction::from_str(line).ok()?);
    }
    Some(instructions)
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone, Default)]
struct Error;

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Error {
        Error {}
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone)]
enum Operation {
    Inc,
    Dec,
}

impl Operation {
    fn compute(&self, left: i32, right: i32) -> i32 {
        match *self {
            Operation::Inc => left + right,
            Operation::Dec => left - right,
        }
    }
}

impl FromStr for Operation {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_ref() {
            "inc" => Ok(Operation::Inc),
            "dec" => Ok(Operation::Dec),
            _ => Err(Error),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone)]
enum Compare {
    LT,
    HT,
    EQ,
    NE,
    LE,
    HE,
}

impl Compare {
    fn evaluate(&self, left: i32, right: i32) -> bool {
        match *self {
            Compare::LT => left < right,
            Compare::HT => left > right,
            Compare::EQ => left == right,
            Compare::NE => left != right,
            Compare::LE => left <= right,
            Compare::HE => left >= right,
        }
    }
}

impl FromStr for Compare {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_ref() {
            "<" => Ok(Compare::LT),
            ">" => Ok(Compare::HT),
            "==" => Ok(Compare::EQ),
            "!=" => Ok(Compare::NE),
            "<=" => Ok(Compare::LE),
            ">=" => Ok(Compare::HE),
            _ => Err(Error),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone)]
struct Condition {
    register: String,
    compare: Compare,
    value: i32,
}

impl Condition {
    fn new(register: &str, compare: Compare, value: i32) -> Condition {
        Condition {
            register: String::from(register),
            compare: compare,
            value: value,
        }
    }

    fn evaluate(&self, registers: &HashMap<String, i32>) -> bool {
        self.compare.evaluate(
            *registers.get(&self.register).unwrap_or(&0),
            self.value,
        )
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone)]
struct Instruction {
    register: String,
    operation: Operation,
    value: i32,
    condition: Condition,
}

impl Instruction {
    fn new(register: &str, operation: Operation, value: i32, condition: Condition) -> Instruction {
        Instruction {
            register: String::from(register),
            operation: operation,
            value: value,
            condition: condition,
        }
    }

    fn process(&self, registers: &mut HashMap<String, i32>) {
        if self.condition.evaluate(registers) {
            let value = *registers.get(&self.register).unwrap_or(&0);
            registers.insert(
                self.register.clone(),
                self.operation.compute(value, self.value),
            );
        }
    }
}

impl FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(' ');
        let register = tokens.next().ok_or(Error {})?;
        let operation = Operation::from_str(tokens.next().ok_or(Error {})?)?;
        let value = i32::from_str_radix(tokens.next().ok_or(Error {})?, 10)?;
        tokens.next().ok_or(Error {})?;
        let cond_register = tokens.next().ok_or(Error {})?;
        let compare = Compare::from_str(tokens.next().ok_or(Error {})?)?;
        let cond_value = i32::from_str_radix(tokens.next().ok_or(Error {})?, 10)?;
        Ok(Instruction {
            register: String::from(register),
            operation: operation,
            value: value,
            condition: Condition::new(cond_register, compare, cond_value),
        })
    }
}

#[test]
fn test_examples() {
    assert_eq!(
        Some(1),
        find_max(
            "b inc 5 if a > 1\na inc 1 if b < 5\nc dec -10 if a >= 1\nc inc -20 if c == 10",
        )
    );
}

#[test]
fn test_from_str() {
    assert_eq!(Err(Error), Instruction::from_str(""));
    assert_eq!(
        Ok(Instruction::new(
            "b",
            Operation::Inc,
            5,
            Condition::new("a", Compare::HT, 1),
        )),
        Instruction::from_str("b inc 5 if a > 1")
    );
    assert_eq!(
        Ok(Instruction::new(
            "a",
            Operation::Inc,
            1,
            Condition::new("b", Compare::LT, 5),
        )),
        Instruction::from_str("a inc 1 if b < 5")
    );
    assert_eq!(
        Ok(Instruction::new(
            "c",
            Operation::Dec,
            -10,
            Condition::new("a", Compare::HE, 1),
        )),
        Instruction::from_str("c dec -10 if a >= 1")
    );
    assert_eq!(
        Ok(Instruction::new(
            "c",
            Operation::Inc,
            -20,
            Condition::new("c", Compare::EQ, 10),
        )),
        Instruction::from_str("c inc -20 if c == 10")
    );
}
