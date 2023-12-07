use std::io::{self, BufRead};
use std::cmp::{max};
use advent_of_code_2023::day02::{Game, RGBCount};

fn main() {
    let stdin = io::stdin();
    let mut sum: u32 = 0;
    for line in stdin.lock().lines() {
        let game = Game::parse(&line.unwrap());
        let min_bag = min_bag(&game);
        sum += min_bag.power();
    }
    println!("{}", sum);
}

fn min_bag(game: &Game) -> RGBCount {
    let mut bag = RGBCount { red: 0, green: 0, blue: 0 };
    for draw in &game.draws {
        bag.red = max(bag.red, draw.red);
        bag.green = max(bag.green, draw.green);
        bag.blue = max(bag.blue, draw.blue);
    }
    return bag;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(min_bag(&Game::parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")), RGBCount::new(4, 2, 6));
        assert_eq!(min_bag(&Game::parse("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")), RGBCount::new(1, 3, 4));
        assert_eq!(min_bag(&Game::parse("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")), RGBCount::new(20, 13, 6));
        assert_eq!(min_bag(&Game::parse("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red")), RGBCount::new(14, 3, 15));
        assert_eq!(min_bag(&Game::parse("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")), RGBCount::new(6, 3, 2));
        // sum of powers = 2286
    }
}
