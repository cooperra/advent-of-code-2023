use advent_of_code_2023::day02::{Game, RGBCount};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut sum: u32 = 0;
    let bag = RGBCount { red: 12, green: 13, blue: 14 };
    for line in stdin.lock().lines() {
        let game = Game::parse(&line.unwrap());
        if is_possible(&bag, &game) {
            sum += game.num;
        }
    }
    println!("{}", sum);
}

fn is_possible(bag: &RGBCount, game: &Game) -> bool {
    game.draws.iter().all(|draw| draw.red <= bag.red && draw.green <= bag.green && draw.blue <= bag.blue)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let bag = RGBCount { red: 12, green: 13, blue: 14 };
        assert_eq!(is_possible( &bag, &Game::parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")), true);
        assert_eq!(is_possible( &bag, &Game::parse("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")), true);
        assert_eq!(is_possible( &bag, &Game::parse("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")), false);
        assert_eq!(is_possible( &bag, &Game::parse("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red")), false);
        assert_eq!(is_possible( &bag, &Game::parse("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")), true);
        // total = 8
    }
}
