use std::collections::HashMap;

type ProgramRecord = (String, u32, Vec<String>);

pub fn find_bottom_program(programs: &str) -> Option<String> {
    Some(find_bottom_program_impl(programs)?.name)
}

pub fn correct_weight(programs: &str) -> Option<u32> {
    let bottom_program = find_bottom_program_impl(programs)?;
    Some(bottom_program.balance()?)
}

fn find_bottom_program_impl(programs: &str) -> Option<Program> {
    let mut records = HashMap::new();
    for line in programs.lines() {
        let record = parse_record(&line)?;
        records.insert(record.0.clone(), record);
    }
    let records = records;
    let mut programs = vec![];
    for (_name, record) in records.iter() {
        programs.push(Program::from(&record, &records)?);

    }
    let max = programs.iter().max_by(
        |&x, &y| x.tower_size().cmp(&y.tower_size()),
    )?;
    Some(max.clone())
}

fn parse_record(line: &str) -> Option<ProgramRecord> {
    let mut tokens = line.split(' ');
    let name = String::from(tokens.next()?);
    let weight =
        match u32::from_str_radix(tokens.next()?.trim_matches(|c| c == '(' || c == ')'), 10) {
            Ok(x) => Some(x),
            _ => None,
        }?;
    let children = if tokens.next().is_some() {
        tokens.map(|x| String::from(x.trim_matches(','))).collect()
    } else {
        vec![]
    };
    Some((name, weight, children))
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone)]
struct Program {
    name: String,
    weight: u32,
    children: Vec<Program>,
}

impl Program {
    fn new(name: String, weight: u32, children: Vec<Program>) -> Program {
        Program {
            name: name,
            weight: weight,
            children: children,
        }
    }

    fn from(record: &ProgramRecord, records: &HashMap<String, ProgramRecord>) -> Option<Program> {
        let mut children = vec![];
        for child in record.2.iter() {
            children.push(Program::from(records.get(child)?, records)?);
        }
        Some(Program::new(record.0.clone(), record.1, children))
    }

    fn tower_size(&self) -> u32 {
        1 + self.children.iter().fold(0, |acc, x| acc + x.tower_size())
    }

    fn tower_weight(&self) -> u32 {
        self.weight +
            self.children.iter().fold(
                0,
                |acc, x| acc + x.tower_weight(),
            )
    }

    fn balance(&self) -> Option<u32> {
        if self.is_balanced() {
            return None;
        }
        for child in self.children.iter() {
            if let Some(weight) = child.balance() {
                return Some(weight);
            }
        }
        Some(self.balance_children())
    }

    fn is_balanced(&self) -> bool {
        let weights = self.children
            .iter()
            .map(|x| x.tower_weight())
            .collect::<Vec<u32>>();
        if self.children.len() == 0 {
            true
        } else {
            let first = weights[0];
            for weight in weights.iter() {
                if *weight != first {
                    return false;
                }
            }
            true
        }
    }

    fn balance_children(&self) -> u32 {
        let weights = self.children
            .iter()
            .map(|x| x.tower_weight())
            .collect::<Vec<u32>>();
        let mut weight = weights[0];
        let different = weights
            .iter()
            .enumerate()
            .filter(|&(_, x)| *x != weight)
            .collect::<Vec<_>>();
        let unbalanced = if different.len() > 1 {
            weight = weights[1];
            &self.children[0]
        } else {
            &self.children[different[0].0]
        };
        weight -
            unbalanced.children.iter().fold(
                0,
                |acc, x| acc + x.tower_weight(),
            )
    }
}

#[test]
fn test_examples() {
    assert_eq!(
        Some(String::from("tknk")),
        find_bottom_program(
            "pbga (66)\nxhth (57)\nebii (61)\nhavc (66)\nktlj (57)\nfwft (72) -> ktlj, cntj, xhth\nqoyq (66)\npadx (45) -> pbga, havc, qoyq\ntknk (41) -> ugml, padx, fwft\njptl (61)\nugml (68) -> gyxo, ebii, jptl\ngyxo (61)\ncntj (57)",
        )
    );
    assert_eq!(
        Some(60),
        correct_weight(
            "pbga (66)\nxhth (57)\nebii (61)\nhavc (66)\nktlj (57)\nfwft (72) -> ktlj, cntj, xhth\nqoyq (66)\npadx (45) -> pbga, havc, qoyq\ntknk (41) -> ugml, padx, fwft\njptl (61)\nugml (68) -> gyxo, ebii, jptl\ngyxo (61)\ncntj (57)",
        )
    );
}

#[test]
fn test_parse_record() {
    assert_eq!(None, parse_record(""));
    assert_eq!(
        Some((String::from("Name"), 5, vec![])),
        parse_record("Name (5)")
    );
    assert_eq!(
        Some((
            String::from("Name"),
            5,
            vec![
                String::from("aaa"),
                String::from("bbb"),
                String::from("ccc"),
            ],
        )),
        parse_record("Name (5) -> aaa, bbb, ccc")
    );
}
