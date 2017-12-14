use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;
use utils;

pub fn connected_programs(pipe_network: &str) -> Option<u32> {
    let mut pipes = Pipes::new();
    for line in pipe_network.lines() {
        let (source, destinations) = parse_pipe(&line)?;
        destinations.iter().for_each(|x| pipes.add_pipe(source, *x));
    }
    Some(pipes.count_connected(0))
}

fn parse_pipe(pipe_description: &str) -> Option<(u32, Vec<u32>)> {
    let mut tokens = pipe_description.split("<->");
    let source = tokens.next()?.trim().parse().ok()?;
    let destinations = utils::parse_numbers(tokens.next()?, ',')?;
    Some((source, destinations))
}

struct Pipes {
    pipes: HashMap<u32, Vec<u32>>,
}

impl Pipes {
    fn new() -> Pipes {
        Pipes { pipes: HashMap::new() }
    }

    fn add_pipe(&mut self, source: u32, destination: u32) {
        self.pipes.entry(source).or_insert(vec![]).push(destination);
        self.pipes.entry(destination).or_insert(vec![]).push(source);
    }

    fn count_connected(&self, node: u32) -> u32 {
        let mut visited: HashSet<u32> = HashSet::new();
        let mut queue: LinkedList<u32> = LinkedList::new();
        queue.push_back(node);
        loop {
            match queue.pop_front() {
                Some(node) => {
                    if !visited.contains(&node) {
                        visited.insert(node);
                        for child in self.pipes.get(&node).unwrap() {
                            if !visited.contains(child) {
                                queue.push_back(*child);
                            }
                        }
                    }
                }
                _ => return visited.len() as u32,
            }
        }
    }
}

#[test]
fn test_examples() {
    assert_eq!(
        Some(6),
        connected_programs(
            "0 <-> 2\n1 <-> 1\n2 <-> 0, 3, 4\n3 <-> 2, 4\n4 <-> 2, 3, 6\n5 <-> 6\n6 <-> 4, 5",
        )
    );
}

#[test]
fn test_parse_pipe() {
    assert_eq!(Some((0, vec![2])), parse_pipe("0 <-> 2"));
    assert_eq!(Some((4, vec![2, 3, 6])), parse_pipe("4 <-> 2, 3, 6"));
}
