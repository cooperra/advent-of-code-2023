use std::cmp::{max, Ordering};
use std::collections::BTreeMap;
use std::fmt;

use Card::*;
use HandType::*;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Clone)]
pub enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    //Jack,
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
            //"J" => Some(Jack),
            "T" => Some(Ten),
            "9" => Some(Nine),
            "8" => Some(Eight),
            "7" => Some(Seven),
            "6" => Some(Six),
            "5" => Some(Five),
            "4" => Some(Four),
            "3" => Some(Three),
            "2" => Some(Two),
            "J" => Some(Joker),
            _ => None,
        }
    }

    fn to_str(self: &Self) -> &'static str {
        match self {
            Ace => "A",
            King => "K",
            Queen => "Q",
            //Jack => "J",
            Ten => "T",
            Nine => "9",
            Eight => "8",
            Seven => "7",
            Six => "6",
            Five => "5",
            Four => "4",
            Three => "3",
            Two => "2",
            Joker => "J",
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
            Ace, King, Queen, Joker, Ten, Nine, Eight, Seven, Six, Five, Four, Three,
            Two, // Jack
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
        let mut most_numerous_card: Option<(u8, Card)> = None;
        let mut second_most_numerous_card: Option<(u8, Card)> = None;
        //println!("{:#?} {:#?}", &self, card_counts);
        for (card, count) in card_counts.iter().rev() {
            if let Some((curr_count, curr_card)) = most_numerous_card {
                if (count, card) > (&curr_count, &curr_card) {
                    second_most_numerous_card = most_numerous_card;
                    most_numerous_card = Some((*count, *card));
                } else if let Some((curr_count, curr_card)) = second_most_numerous_card {
                    if (count, card) > (&curr_count, &curr_card) {
                        second_most_numerous_card = Some((*count, *card));
                    }
                } else {
                    second_most_numerous_card = Some((*count, *card));
                }
            } else {
                most_numerous_card = Some((*count, *card));
            }
        }
        // Joker boosting
        let num_jokers = *card_counts.get(&Joker).unwrap_or(&0);
        if let Some((foo, Joker)) = most_numerous_card {
            most_numerous_card = Some(second_most_numerous_card.unwrap_or((0, Joker)));
            // Second card type doesn't matter once jokers are in play (it's actually not needed at all at this point for this problem)
            // that is, we will never choose a full-house or two pair when we can choose 4kind or 3kind respectivly
            second_most_numerous_card = Some((1, Joker));
        }
        most_numerous_card =
            most_numerous_card.map(|(count, card_type)| (count + num_jokers, card_type));

        match most_numerous_card.unwrap() {
            (5, _) => FiveKind,
            (4, _) => FourKind,
            (3, _) => match second_most_numerous_card.unwrap() {
                (2, _) => FullHouse,
                (1, _) => ThreeKind,
                _ => unreachable!(),
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
