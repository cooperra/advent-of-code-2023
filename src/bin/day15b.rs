use advent_of_code_2023::day15::*;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let result = day15b(lines.next().unwrap());
    println!("{}", result);
}

pub fn day15b(line: impl AsRef<str>) -> Num {
    let mut fac = Facility::new();
    // Process instructions
    for instruction in line.as_ref().split(",") {
        if let Some(label) = instruction.strip_suffix("-") {
            // do minus
            let box_id = hash(label) as usize;
            if let Some(slot_id) = fac.boxes[box_id]
                .iter()
                .position(|(this_label, _)| &label == this_label)
            {
                fac.boxes[box_id].remove(slot_id);
            }
        } else if let Some((label, focal)) = instruction.rsplit_once("=") {
            let box_id = hash(label) as usize;
            let lens_entry: LabeledLens = (label, focal.parse::<Lens>().unwrap());
            if let Some(slot_id) = fac.boxes[box_id]
                .iter()
                .position(|(this_label, _)| &label == this_label)
            {
                fac.boxes[box_id][slot_id] = lens_entry;
            } else {
                fac.boxes[box_id].push(lens_entry);
            }
        } else {
            panic!("Bad instruction.")
        }
    }

    // Calculate total lens power
    let mut sum = 0;
    for (box_idx, bx) in fac.boxes.iter().enumerate() {
        for (slot_idx, (_, focal)) in bx.iter().enumerate() {
            sum += lens_power(box_idx as u8, slot_idx as u8, *focal);
        }
    }
    sum
}

fn lens_power(box_idx: u8, slot_idx: u8, focal: u8) -> u32 {
    (box_idx as u32 + 1) * (slot_idx as u32 + 1) * focal as u32
}

#[cfg(test)]
mod test {
    use super::*;

    //#[test]
    //fn test_hash() {
    //    assert_eq!(hash(&"HASH"), 52);
    //}
}
