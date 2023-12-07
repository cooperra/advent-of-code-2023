use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;
use std::io::{self, BufRead};

struct RGBCount {
    red: u8,
    green: u8,
    blue: u8,
}

impl RGBCount {
    fn from_parse_rule(rgb_count: Pair<Rule>) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for color_count in rgb_count.into_inner() {
            match color_count.as_rule() {
                Rule::count => {
                    let mut num: u8 = 0;
                    for count_or_color in color_count.into_inner() {
                        match count_or_color.as_rule() {
                            Rule::num => num = count_or_color.as_str().parse::<u8>().unwrap(),
                            Rule::color => match count_or_color.as_str() {
                                "red" => red = num,
                                "green" => green = num,
                                "blue" => blue = num,
                                _ => panic!(),
                            },
                            _ => panic!(),
                        }
                    }
                },
                _ => panic!(),
            }
        }
        RGBCount { red, green, blue }
    }
}

struct Game {
    num: u32,
    draws: Vec<RGBCount>,
}

impl Game {
    fn parse(line: &str) -> Self {
        Self::from_parse_rule(Day2Parser::parse(Rule::game, line).expect("unsuccessful parse").next().unwrap())
    }

    fn from_parse_rule(game: Pair<Rule>) -> Self {
        let mut num = 0;
        let mut draws: Vec<RGBCount> = Vec::new();

        for rule in game.into_inner() {
            match rule.as_rule() {
                Rule::num => {
                    num = rule.as_str().parse::<u32>().unwrap();
                },
                Rule::drawlist => {
                    for rgb_rule in rule.into_inner() {
                        match rgb_rule.as_rule() {
                            Rule::drawstats => draws.push(RGBCount::from_parse_rule(rgb_rule)),
                            _ => panic!(),
                        }
                    }
                },
                _ => panic!(),
            }
        }

        Game { num, draws }
    }
}

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

#[derive(Parser)]
#[grammar = "day02.pest"]
pub struct Day2Parser;


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
