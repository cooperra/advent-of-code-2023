use std::cmp::{max, Ordering};
use std::collections::BTreeMap;
use std::fmt;

use Card::*;
use HandType::*;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Clone)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Clone)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl Card {
    fn from_str(card: &str) -> Option<Self> {
        match card {
            "A" => Some(Ace),
            "K" => Some(King),
            "Q" => Some(Queen),
            "J" => Some(Jack),
            "T" => Some(Ten),
            "9" => Some(Nine),
            "8" => Some(Eight),
            "7" => Some(Seven),
            "6" => Some(Six),
            "5" => Some(Five),
            "4" => Some(Four),
            "3" => Some(Three),
            "2" => Some(Two),
            _ => None,
        }
    }

    fn to_str(self: &Self) -> &'static str {
        match self {
            Ace => "A",
            King => "K",
            Queen => "Q",
            Jack => "J",
            Ten => "T",
            Nine => "9",
            Eight => "8",
            Seven => "7",
            Six => "6",
            Five => "5",
            Four => "4",
            Three => "3",
            Two => "2",
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
}

impl Hand {
    pub fn from_str(line: &str) -> Option<Self> {
        if line.len() != 5 {
            return None;
        }
        let mut cards: [Option<Card>; 5] = [None; 5];
        for (idx, char) in line.chars().enumerate().take(5) {
            cards[idx] = Card::from_str(&char.to_string());
        }
        if !cards.iter().all(Option::is_some) {
            return None;
        }
        Some(Self {
            cards: cards.map(Option::unwrap),
        })
    }

    pub fn get_card_counts(self: &Self) -> BTreeMap<Card, u8> {
        let mut card_counts = BTreeMap::new();
        for card_type in [
            Ace, King, Queen, Jack, Ten, Nine, Eight, Seven, Six, Five, Four, Three, Two,
        ] {
            let count = self.cards.iter().filter(|c| *c == &card_type).count() as u8;
            if count > 0 {
                //println!("count {} {}", card_type, count);
                *card_counts.entry(card_type).or_default() = count;
            }
        }
        return card_counts;
    }

    pub fn get_type(self: &Self) -> HandType {
        let card_counts = self.get_card_counts();
        let mut most_numerous_card = None;
        let mut second_most_numerous_card = None;
        //println!("{:#?} {:#?}", &self, card_counts);
        for (card, count) in card_counts.iter().rev() {
            if let Some(current) = most_numerous_card {
                if (count, card) > current {
                    second_most_numerous_card = most_numerous_card;
                    most_numerous_card = Some((count, card));
                } else if let Some(current) = second_most_numerous_card {
                    if (count, card) > current {
                        second_most_numerous_card = Some((count, card));
                    }
                } else {
                    second_most_numerous_card = Some((count, card));
                }
            } else {
                most_numerous_card = Some((count, card));
            }
        }

        match most_numerous_card.unwrap() {
            (5, _) => FiveKind,
            (4, _) => FourKind,
            (3, _) => match second_most_numerous_card.unwrap() {
                (2, _) => FullHouse,
                (1, _) => ThreeKind,
                _ => {
                    unreachable!()
                }
            },
            (2, _) => match second_most_numerous_card.unwrap() {
                (2, _) => TwoPair,
                (1, _) => OnePair,
                _ => unreachable!(),
            },
            (1, _) => HighCard,
            _ => unreachable!(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.get_type(), self.cards).cmp(&(other.get_type(), other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// impl ToString for Hand {
//     fn to_string(self: &Self) -> String {
//         self.cards.map(|ref c| c.to_str()).join("")
//     }
// }

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cards.map(|ref c| c.to_str()).join(""))
    }
}
