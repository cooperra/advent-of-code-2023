use core::ops::Range;
use regex::{Match, Regex};
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};
use std::rc::Rc;

//impl PartialEq for Match {
//    fn eq(
//}
#[derive(Hash, PartialEq, Eq, Debug)]
struct MyMatch {
    text: String,
    range: Range<usize>,
    // This is only used to make hashes differ.
    row_num: usize,
}

impl MyMatch {
    fn from_match(m: &Match, row_num: usize) -> Self {
        Self {
            text: m.as_str().to_owned(),
            range: m.range(),
            row_num,
        }
    }
}

type Num = Rc<MyMatch>;
type Sym = Rc<MyMatch>;
#[derive(Hash, PartialEq, Eq)]
struct Pair {
    num: Num,
    sym: Sym,
}
#[derive(Default, Debug)]
struct LineData {
    nums: HashMap<usize, Num>,
    syms: Vec<Sym>,
}

fn main() {
    let mut num_to_syms: HashMap<Num, HashSet<Sym>> = Default::default();
    let mut sym_to_nums: HashMap<Sym, HashSet<Num>> = Default::default();
    {
        let stdin = io::stdin();
        let mut prev_line: LineData = Default::default();
        let mut curr_line: LineData;
        for (line_num, line) in stdin.lock().lines().enumerate() {
            let (nums, syms): (Vec<Num>, Vec<Sym>) = scrape_items(&line.unwrap(), line_num);
            curr_line = build_index(nums, syms);
            let mut pairs: HashSet<Pair> = HashSet::new();
            collect_pairs_oneline(&mut pairs, &curr_line);
            collect_pairs_twoline(&mut pairs, &prev_line, &curr_line);
            for Pair { num, sym } in pairs.into_iter() {
                num_to_syms
                    .entry(Rc::clone(&num))
                    .or_default()
                    .insert(Rc::clone(&sym));
                sym_to_nums.entry(Rc::clone(&sym)).or_default().insert(num);
            }

            prev_line = curr_line;
        }
    }
    let result: u32 = get_result(num_to_syms.keys());
    println!("{}", result);
}

fn collect_pairs_twoline(
    mut pairs: &mut HashSet<Pair>,
    prev_line: &LineData,
    curr_line: &LineData,
) {
    collect_pairs_vertically(&mut pairs, &prev_line.syms, &curr_line.nums);
    collect_pairs_vertically(&mut pairs, &curr_line.syms, &prev_line.nums);
}

fn collect_pairs_oneline(mut pairs: &mut HashSet<Pair>, line: &LineData) {
    collect_pairs_horizontally(&mut pairs, &line);
}

fn collect_pairs_vertically(
    pairs: &mut HashSet<Pair>,
    row_a_syms: &Vec<Sym>,
    row_b_nums: &HashMap<usize, Num>,
) {
    for sym in row_a_syms.iter() {
        for pos in adjacent_cols_other_line(&sym) {
            if let Some(num) = row_b_nums.get(&pos) {
                pairs.insert(Pair {
                    num: Rc::clone(&num),
                    sym: Rc::clone(&sym),
                });
            }
        }
    }
}

fn collect_pairs_horizontally(pairs: &mut HashSet<Pair>, row: &LineData) {
    let row_syms = &row.syms;
    let row_nums = &row.nums;
    for sym in row_syms.iter() {
        for pos in adjacent_cols_same_line(&sym) {
            if let Some(num) = row_nums.get(&pos) {
                pairs.insert(Pair {
                    num: Rc::clone(&num),
                    sym: Rc::clone(&sym),
                });
            }
        }
    }
    //println!("{:#?}", row);
}

fn adjacent_cols_other_line(sym: &MyMatch) -> Range<usize> {
    Range {
        start: match sym.range.start {
            0 => 0,
            start => start - 1,
        },
        end: sym.range.end + 1,
    }
}

fn adjacent_cols_same_line(sym: &MyMatch) -> Vec<usize> {
    let mut cols = Vec::new();
    if sym.range.start > 0 {
        cols.push(sym.range.start - 1);
    }
    // range.end is exclusive, so no need to +1
    cols.push(sym.range.end);
    return cols;
}

fn get_result<'a>(mut nums: impl Iterator<Item = &'a Num>) -> u32 {
    let mut sum = 0;
    for n in &mut nums {
        sum += &n.text.parse::<u32>().unwrap();
    }
    sum
}

fn scrape_items(line: &str, line_num: usize) -> (Vec<Num>, Vec<Sym>) {
    let re = Regex::new(r"(?:(?<num>[0-9]+)|(?<sym>[^.]))").unwrap();
    let mut nums = Vec::new();
    let mut syms = Vec::new();
    for capture in re.captures_iter(&line) {
        if let Some(sym) = capture.name("sym") {
            syms.push(Rc::new(MyMatch::from_match(&sym, line_num)));
        }
        if let Some(num) = capture.name("num") {
            nums.push(Rc::new(MyMatch::from_match(&num, line_num)));
        }
    }
    return (nums, syms);
}

fn build_index(nums: Vec<Num>, syms: Vec<Sym>) -> LineData {
    let mut result = LineData {
        nums: HashMap::new(),
        syms,
    };
    for num in nums.into_iter() {
        for idx in num.range.to_owned() {
            result.nums.insert(idx, Rc::clone(&num));
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal() {
        {
            let sym = MyMatch {
                text: "*".to_string(),
                range: 0..1,
                row_num: 0,
            };
            assert_eq!(vec![1], adjacent_cols_same_line(&sym));
        }
        {
            let sym = MyMatch {
                text: "*".to_string(),
                range: 1..2,
                row_num: 0,
            };
            assert_eq!(vec![0, 2], adjacent_cols_same_line(&sym));
        }
    }

    #[test]
    fn vertical() {
        {
            let sym = MyMatch {
                text: "*".to_string(),
                range: 0..1,
                row_num: 0,
            };
            assert_eq!(0..2, adjacent_cols_other_line(&sym));
        }
        {
            let sym = MyMatch {
                text: "*".to_string(),
                range: 1..2,
                row_num: 0,
            };
            assert_eq!(0..3, adjacent_cols_other_line(&sym));
        }
    }
}
