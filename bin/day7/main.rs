use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::cell::RefCell;
use itertools::Itertools;
#[derive(Debug, Clone)]
struct Program {
    name: String,
    children: Vec<Rc<RefCell<Program>>>,
    weight: u32,
}
impl PartialEq for Program {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Program {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
impl Program {
    fn new(name: &str, weight: u32) -> Program {
        Program {
            name: String::from(name),
            children: Vec::new(),
            weight: weight,
        }
    }
    fn add_child(&mut self, child: &Rc<RefCell<Program>>) {
        self.children.push(Rc::clone(child));
    }
    fn print_tree(&self, depth: usize) {
        let indent = " ".repeat(depth * 2);
        println!("{}Program: {}", indent, self.name);
        for child in &self.children {
            println!("{}  Contains: {}, weight: {}", indent, child.borrow().name, child.borrow().weight);
        }
        for child in &self.children {
            child.borrow().print_tree(depth + 1);
        }
    }
    fn total_weight(&self) -> u32 {
        self.weight + self.children.iter().map(|child| child.borrow().total_weight()).sum::<u32>()
    }

    fn find_wrong_weight(&self) -> i32 {
        let mut weights = HashMap::new();
        for child in &self.children {
            let total_weight = child.borrow().total_weight();
            weights.entry(total_weight).or_insert_with(Vec::new).push(child);
        }

        // If all children are balanced, return 0 indicating no adjustment needed
        if weights.len() == 1 {
            return 0;
        }

        // Find the unbalanced child and the target weight
        let (&target_weight, &unbalanced_weight) = {
            let (target, unbalanced) = weights.iter().minmax_by_key(|&(_, v)| v.len()).into_option().unwrap();
                (unbalanced.0, target.0)
        };

        let unbalanced_program = weights.get(&unbalanced_weight).unwrap().first().unwrap();
        let weight_diff = target_weight as i32 - unbalanced_weight as i32;
        let correction = unbalanced_program.borrow().find_wrong_weight();

        if correction == 0 {
            // If the children of the unbalanced program are balanced, adjust its weight
            unbalanced_program.borrow().weight as i32 + weight_diff
        } else {
            // Otherwise, propagate the correction up
            correction
        }
    }
}
fn parse(input: &str) -> (HashMap<&str, Vec<&str>>, HashSet<&str>, HashMap<&str, u32>) {
    let lines: Vec<&str> = input.lines().collect();
    let mut parents = HashMap::new();
    let mut names = HashSet::new();
    let mut programs: HashMap<&str, u32> = HashMap::new();
    for line in lines {
        if let Some((left, right)) = line.split_once(") -> ") {
            let (name, weight) = left.split_once(" (").unwrap();
            let weight = weight.parse::<u32>().unwrap();
            programs.insert(name, weight);
            let branches: Vec<&str> = right.split(", ").collect();
            parents.insert(name, branches);
            names.insert(name);
        } else {
            let (name, weight) = line.trim_end_matches(")").split_once(" (").unwrap();
            let weight = weight.parse::<u32>().unwrap();
            programs.insert(name, weight);
        }
    }
    (parents, names, programs)
}
fn part1(input: &str) -> String {
    let (towers, names, _) = parse(input);

    for name in names.iter() {
        if towers.values().all(|branches| !branches.contains(name)) {
            return name.to_string();
        }
    }

    String::new()
}
fn part2(input: &str) -> i32 {
    let (parents, _, disks) = parse(input);
    let mut programs = HashMap::new();

    // Create program nodes
    for (name, weight) in disks {
        programs.insert(name, Rc::new(RefCell::new(Program::new(name, weight))));
    }

    // Build the tree by setting children for each program
    for (parent, children) in parents {
        let parent_node = programs.get(&parent).expect("Parent not found");
        for child in children {
            let child_node = programs.get(&child).expect("Child not found");
            parent_node.borrow_mut().add_child(child_node);
        }
    }

    // Find the bottom program and calculate the weight correction needed
    let bottom = part1(input);
    programs.get(&bottom.as_str())
        .map_or(0, |root| root.borrow().find_wrong_weight())
}
fn main() {
    let input = include_str!("input7.txt");
    println!("{}", part2(input));
}