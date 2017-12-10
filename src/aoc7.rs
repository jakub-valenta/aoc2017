use std::collections::HashMap;

type ProgramRecord = (String, u32, Vec<String>);

pub fn find_bottom_program(programs: &str) -> Option<String> {
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
    let max = programs.iter().fold((String::new(), 0), |acc, x| {
        let tower_size = x.tower_size();
        if acc.1 < tower_size {
            (x.name.clone(), tower_size)
        } else {
            acc
        }
    });
    if max.1 == 0 { None } else { Some(max.0) }
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

#[derive(Debug, PartialEq, PartialOrd, Eq)]
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
}

#[test]
fn test_examples() {
    assert_eq!(
        Some(String::from("tknk")),
        find_bottom_program(
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
