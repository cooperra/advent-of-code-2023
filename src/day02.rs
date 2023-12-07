use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

pub struct RGBCount {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
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

pub struct Game {
    pub num: u32,
    pub draws: Vec<RGBCount>,
}

impl Game {
    pub fn parse(line: &str) -> Self {
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


#[derive(Parser)]
#[grammar = "day02.pest"]
pub struct Day2Parser;
