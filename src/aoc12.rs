use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;
use utils;

pub fn connected_programs(pipe_network: &str) -> Option<u32> {
    let pipes = parse_pipe_networkpipe_network(pipe_network)?;
    Some(pipes.count_connected(0))
}

pub fn program_groups(pipe_network: &str) -> Option<u32> {
    let pipes = parse_pipe_networkpipe_network(pipe_network)?;
    Some(pipes.group_count())
}

fn parse_pipe_networkpipe_network(pipe_network: &str) -> Option<Pipes> {
    let mut pipes = Pipes::new();
    for line in pipe_network.lines() {
        let (source, destinations) = parse_pipe(&line)?;
        destinations.iter().for_each(|x| pipes.add_pipe(source, *x));
    }
    Some(pipes)
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
        self.find_group(node).len() as u32
    }

    fn find_group(&self, node: u32) -> HashSet<u32> {
        let mut visited: HashSet<u32> = HashSet::new();
        let mut queue: LinkedList<u32> = LinkedList::new();
        queue.push_back(node);
        while queue.len() > 0 {
            let node = queue.pop_front().unwrap();
            if !visited.contains(&node) {
                visited.insert(node);
                for child in self.pipes.get(&node).unwrap() {
                    if !visited.contains(child) {
                        queue.push_back(*child);
                    }
                }
            }
        }
        visited
    }

    fn group_count(&self) -> u32 {
        let mut groups = 0;
        let mut nodes: HashSet<&u32> = self.pipes.keys().collect();
        while !nodes.is_empty() {
            let visited = self.find_group(**nodes.iter().next().unwrap());
            nodes.retain(|x| !visited.contains(x));
            groups += 1;
        }
        groups
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
    assert_eq!(
        Some(2),
        program_groups(
            "0 <-> 2\n1 <-> 1\n2 <-> 0, 3, 4\n3 <-> 2, 4\n4 <-> 2, 3, 6\n5 <-> 6\n6 <-> 4, 5",
        )
    );
}

#[test]
fn test_parse_pipe() {
    assert_eq!(Some((0, vec![2])), parse_pipe("0 <-> 2"));
    assert_eq!(Some((4, vec![2, 3, 6])), parse_pipe("4 <-> 2, 3, 6"));
}
