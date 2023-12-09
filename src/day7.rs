use std::fs;
use std::collections::HashMap;
use std::cmp::Ordering;

enum HandType {
    FiveOfAKind,
    FourOfAKind,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

impl HandType {
    fn get_value(&self) -> i32 {
        match self {
            HandType::FiveOfAKind => 6,
            HandType::FourOfAKind => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }

    fn get_hand_type(hand: &str) -> HandType {
        let mut cards = HashMap::new();
        for c in hand.chars() {
            cards.entry(c).and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        match cards.len() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            1 => HandType::FiveOfAKind,
            n => {
                let c = cards.values().max().unwrap();
                match n {
                    2 => {
                        // 4, 1
                        // 3, 2
                        if c == &4 { HandType::FourOfAKind } else { HandType::ThreeOfAKind }
                    }
                    3 => {
                        // 2, 2, 1
                        // 3, 1, 1
                        if c == &3 { HandType::ThreeOfAKind } else { HandType::TwoPair }
                    }
                    _ => HandType::HighCard
                }
            }
        }
    }
}


struct Hand {
    order: String,
    hand_type: HandType,
    bid: usize
}

impl Hand {
    fn create(hand: &str, bid: &str) -> Hand {
        Hand {
            order: String::from(hand),
            hand_type: HandType::get_hand_type(hand),
            bid: bid.parse::<usize>().unwrap()
        }
        
    }

    fn card_value(card: char) -> i32 {
        match  card {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => 0
        }
    }

    fn cmp(&self, other: &Hand) -> Ordering {
        let self_value = self.hand_type.get_value();
        let other_value = other.hand_type.get_value();

        if self_value < other_value {
            return Ordering::Less;
        }

        if self_value > other_value {
            return Ordering::Greater;
        }

        for c in self.order.chars().zip(other.order.chars()) {
            let s = Hand::card_value(c.0);
            let o = Hand::card_value(c.1);

            match s.cmp(&o) {
                Ordering::Equal => continue,
                Ordering::Less => return Ordering::Less,
                Ordering::Greater => return Ordering::Greater
            };
        }

        Ordering::Equal
    }
}

pub fn part1() {
    let content = fs::read_to_string("input/day7.txt").unwrap();
    let mut hands: Vec<_> = content.lines()
        .map(|l| {
            let lv: Vec<_> = l.split(' ').collect();
            return Hand::create(lv[0], lv[1]);
        })
        .collect();


    hands.sort_by(|u, v| u.cmp(v));
    let val = hands.iter()
        .enumerate()
        .fold(0, |acc, (i, h)| acc + (h.bid*(i+1)));

    println!("{val}");
}