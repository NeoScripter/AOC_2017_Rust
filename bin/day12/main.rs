use std::collections::{HashMap, HashSet};
#[derive(Debug, Clone)]
struct Graph {
    pipes: HashMap<u32, HashSet<u32>>,
}
impl Graph {
    fn new() -> Self {
        Self {
            pipes: HashMap::new(),
        }
    }
    fn add_edges(&mut self, node1: u32, node2: u32) {
        self.pipes.entry(node1).or_insert(HashSet::new()).insert(node2);
        self.pipes.entry(node2).or_insert(HashSet::new()).insert(node1);
    }
    fn find_connections(&self, start: u32) -> usize {
        let mut cache = HashSet::new();
        self.connection_search(&mut cache, start)
    }
    fn connection_search(&self, cache: &mut HashSet<u32>, current: u32) -> usize {
        let mut con_count = 0;
        let available = self.pipes.get(&current).unwrap();
        for &pipe in available {
            if cache.insert(pipe) {
                con_count += 1;
                con_count += self.connection_search(cache, pipe);
            }
        }
        con_count
    }
    fn find_subgraphs(&self) -> usize {
        let mut seen = HashSet::new();
        let mut groups = 0;
        for &node in self.pipes.keys() {
            if seen.insert(node) {
                groups += self.find_group(&mut seen, node)
            }
        }
        groups
    }
    fn find_group(&self, seen: &mut HashSet<u32>, node: u32) -> usize {
        let available = self.pipes.get(&node).unwrap();
        for &pipe in available {
            if seen.insert(pipe) {
                self.find_group(seen, pipe);
            }
        }
        1
    }
    fn dfs(&self, node: u32, visited: &mut HashSet<u32>) {
        if visited.insert(node) {
            if let Some(neighbors) = self.pipes.get(&node) {
                for &neighbor in neighbors {
                    self.dfs(neighbor, visited);
                }
            }
        }
    }

    fn count_subgraphs(&self) -> usize {
        let mut visited = HashSet::new();
        let mut subgraph_count = 0;

        for &node in self.pipes.keys() {
            if !visited.contains(&node) {
                self.dfs(node, &mut visited);
                subgraph_count += 1;
            }
        }

        subgraph_count
    }
}

fn part1(input: &str) -> usize {
    let mut graph = Graph::new();
    input.lines().for_each(|line| {
        let (left, right) = line.split_once(" <-> ").unwrap();
        let node1 = left.parse::<u32>().unwrap();
        let right: Vec<u32> = right.split(", ").map(|x| x.parse::<u32>().unwrap()).collect();
        for node2 in right {
            graph.add_edges(node1, node2);
            graph.add_edges(node2, node1);
        }
    });
    graph.count_subgraphs()
}

fn main() {
    let input = include_str!("input12.txt");
    println!("{}", part1(input));
}