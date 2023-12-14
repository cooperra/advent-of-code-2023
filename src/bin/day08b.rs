use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    io::{self, BufRead},
    rc::Rc,
};

use regex_macro::regex;

type Num = u32;
type Graph = HashMap<Node, (Node, Node)>;
type Node = Rc<String>;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap());
    let result = day08(lines);
    println!("{}", result);
}

pub fn day08(mut lines: impl Iterator<Item = impl AsRef<str>>) -> Num {
    let directions = lines.next().expect("No input!");
    lines.next();
    let (graph, keystore) = parse_graph(lines);
    let mut current_nodes: BinaryHeap<Reverse<(Num, &Node)>> = keystore
        .iter()
        .filter(|name| name.ends_with("AAA"))
        .map(|name| Reverse((0, name)))
        .collect();
    loop {
        let Reverse((mut current_steps_taken, mut current_node)) = current_nodes.pop().unwrap();
        current_steps_taken += counttoend(
            &mut current_node,
            directions.as_ref(),
            &graph,
            current_steps_taken,
        );
        current_nodes.push(Reverse((current_steps_taken, current_node)));
        if current_nodes.iter().all(|Reverse((this_steps, name))| {
            *this_steps == current_steps_taken && name.ends_with("Z")
        }) {
            return current_steps_taken;
        }
    }
}

fn counttoend<'a>(
    current_node: &mut &'a Node,
    directions: &str,
    graph: &'a Graph,
    start_step: Num,
) -> Num {
    let mut iter_count = 0;
    for instruction in directions
        .chars()
        .cycle()
        .skip(start_step.try_into().unwrap())
    {
        nextynext(current_node, &instruction, graph);
        //dbg!(&current_nodes);
        iter_count += 1;
        if current_node.ends_with("Z") {
            return iter_count;
        }
    }
    unreachable!();
}

fn nextynext<'a>(current_node: &mut &'a Node, instruction: &char, graph: &'a Graph) {
    let (left_opt, right_opt) = graph.get(&Rc::clone(&current_node)).unwrap();
    let next_node = match instruction.to_string().as_ref() {
        "L" => left_opt,
        "R" => right_opt,
        _ => panic!(),
    };
    *current_node = next_node;
}

fn parse_graph<S: AsRef<str>>(mut lines: impl Iterator<Item = S>) -> (Graph, HashSet<Node>) {
    let mut graph = HashMap::new();
    let mut keystore = HashSet::new();
    for line in lines {
        let re = regex!(r"[A-Z]+");
        let mut names = re.find_iter(line.as_ref()).map(|m| m.as_str());
        let mut key = Rc::new(names.next().expect("No key").to_string());
        let mut left = Rc::new(names.next().expect("No left").to_string());
        let mut right = Rc::new(names.next().expect("No right").to_string());
        // backing store dance
        // get or initialize the definitive Rc for each
        let left2 = keystore.get(&left).unwrap_or(&left);
        keystore.insert(Rc::clone(&left2));
        let right2 = keystore.get(&right).unwrap_or(&right);
        keystore.insert(Rc::clone(&right));
        let key2 = keystore.get(&key).unwrap_or(&key);
        keystore.insert(Rc::clone(&key2));

        graph.insert(key, (left, right));

        //, ("null", "null");  // TODO store in a set
        //let _ = graph.insert(key, (left, right));
    }
    (graph, keystore)
}
