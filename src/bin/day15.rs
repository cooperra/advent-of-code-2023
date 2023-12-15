use std::io::{self, BufRead};

type Num = u64;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let result = day15(lines.next().unwrap());
    println!("{}", result);
}

pub fn day15(line: impl AsRef<str>) -> Num {
    let mut sum = 0;
    for chunk in line.as_ref().split(",") {
        let chunk_hash = hash(&chunk);
        println!("'{}' -> {}", chunk, chunk_hash);
        sum += chunk_hash as Num;
    }
    sum
}

fn hash(input: &str) -> u8 {
    let mut current: Num = 0;
    for c in input.chars() {
        let b = c as u8;
        current += b as Num;
        current *= 17;
        current %= 256;
    }
    current as u8
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash(&"HASH"), 52);
    }
}
