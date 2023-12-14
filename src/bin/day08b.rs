use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
    rc::Rc,
};

use regex_macro::regex;

type Num = u32;

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
    let mut current_nodes: Vec<&Rc<String>> =
        keystore.iter().filter(|name| name.ends_with("A")).collect();
    dbg!(current_nodes.len());

    let mut iter_count = 0;
    for instruction in directions.as_ref().chars().cycle() {
        for current_node in current_nodes.iter_mut() {
            let (left_opt, right_opt) = graph.get(&Rc::clone(&current_node)).unwrap();
            let next_node = match instruction.to_string().as_ref() {
                "L" => left_opt,
                "R" => right_opt,
                _ => panic!(),
            };
            *current_node = next_node;
        }
        dbg!(&current_nodes);
        iter_count += 1;
        if current_nodes.iter().all(|name| name.ends_with("Z")) {
            return iter_count;
        }
    }
    unreachable!();
}

fn parse_graph<S: AsRef<str>>(
    mut lines: impl Iterator<Item = S>,
) -> (
    HashMap<Rc<String>, (Rc<String>, Rc<String>)>,
    HashSet<Rc<String>>,
) {
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
