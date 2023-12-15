use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    io::{self, BufRead},
    rc::Rc,
};

use regex_macro::regex;

type Num = u64;
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
    let mut memoize_table: HashMap<(Num, Node), (Num, Node)> = HashMap::new();
    let mut current_nodes: BinaryHeap<Reverse<(Num, Node)>> = keystore
        .iter()
        .filter(|name| name.ends_with("A"))
        .map(|name| Reverse((0, Rc::clone(name))))
        .collect();
    loop {
        let Reverse((mut current_steps_taken, mut current_node)) = current_nodes.pop().unwrap();
        let steps_offset = current_steps_taken % directions.as_ref().chars().count() as Num;

        let steps_between_nodes;
        (steps_between_nodes, current_node) = counttoend_memoized(
            current_node,
            directions.as_ref(),
            &graph,
            steps_offset,
            &mut memoize_table,
        );
        current_steps_taken += steps_between_nodes;
        current_nodes.push(Reverse((current_steps_taken, current_node)));
        if current_nodes.iter().all(|Reverse((this_steps, name))| {
            *this_steps == current_steps_taken && name.ends_with("Z")
        }) {
            return current_steps_taken;
        }
    }
}

fn counttoend_memoized(
    start_node: Node,
    directions: &str,
    graph: &Graph,
    directions_offset: Num,
    memoize_table: &mut HashMap<(Num, Node), (Num, Node)>,
) -> (Num, Node) {
    let func_input = (directions_offset, Rc::clone(&start_node));
    if let Some((count, end_node)) = memoize_table.get(&func_input) {
        //dbg!("memoize hit!");
        //dbg!((&count, &end_node));
        return (*count, Rc::clone(end_node));
    }
    dbg!("memoize_miss :(");
    let (count, end_node) = counttoend(&start_node, directions.as_ref(), &graph, directions_offset);
    memoize_table.insert(func_input, (count, Rc::clone(&end_node)));
    dbg!(&memoize_table);
    // Take the data at this point and solve by hand.
    if memoize_table.keys().len() == 12 {
        let inst_len = directions.chars().count() as Num;
        for ((offset, start_name), (steps, dest_name)) in
            memoize_table.iter().filter(|x| x.1 .1.ends_with("A"))
        {
            println!(
                "({}) {} -> {} {}x",
                offset,
                start_name,
                dest_name,
                *steps as f64 / inst_len as f64,
            );
        }
        let initial_offsets: HashMap<Rc<String>, Num> = HashMap::from_iter(
            memoize_table
                .iter()
                .filter(|x| x.0 .1.ends_with("A"))
                .map(|x| (Rc::clone(&x.1 .1), x.1 .0)),
        );
        for ((offset, start_name), (steps, dest_name)) in
            memoize_table.iter().filter(|x| x.0 .1.ends_with("Z"))
        {
            println!(
                "t = ({} + {}n) * 293",
                //start_name,
                //dest_name,
                *initial_offsets.get(dest_name).unwrap() as f64 / 293_f64,
                *steps as f64 / 293_f64
            );
        }
        // All of the offsets are a prime number of iterations, and each are equal to their respective initial offset.
        // We're just going to multiply them all together.
        // There's also a prime number of instructions. It's painfully obvious.
        let final_answer = initial_offsets
            .values()
            .map(|x| *x / inst_len)
            .reduce(core::ops::Mul::mul)
            .unwrap()
            * inst_len;
        println!("Answer: {}", final_answer);
    }
    return (count, Rc::clone(end_node));
}

fn counttoend<'a>(
    start_node: &'a Node,
    directions: &str,
    graph: &'a Graph,
    directions_offset: Num,
) -> (Num, &'a Node) {
    let mut current_node = start_node;
    let mut iter_count = 0;
    for instruction in directions
        .chars()
        .cycle()
        .skip(directions_offset.try_into().unwrap())
    {
        nextynext(&mut current_node, &instruction, graph);
        //dbg!(&current_nodes);
        iter_count += 1;
        if current_node.ends_with("Z") {
            return (iter_count, current_node);
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
