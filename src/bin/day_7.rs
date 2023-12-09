use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::Iterator;

use itertools::Itertools;

use aoc23::get_input;

#[derive(Debug)]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum Kind {
    High,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

#[derive(Debug)]
struct Bet {
    card: String,
    kind: Kind,
    bid: i64,
}


impl PartialEq for Bet {
    fn eq(&self, other: &Self) -> bool {
        return self.kind == other.kind;
    }
}

impl Eq for Bet {}

impl PartialOrd for Bet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Bet {
    fn cmp(&self, other: &Self) -> Ordering {
        let CARD_SCORES: HashMap<char, i32> = HashMap::from([
            ('A', 13), ('K', 12), ('Q', 11), ('J', 10), ('T', 9), ('9', 8), ('8', 7), ('7', 6), ('6', 5), ('5', 4), ('4', 3), ('3', 2), ('2', 1)
        ]);
        let order = self.kind.cmp(&other.kind);
        if order == Ordering::Equal {
            let chars1 = self.card.chars().collect_vec();
            let chars2 = other.card.chars().collect_vec();
            for (idx, char) in chars1.iter().enumerate() {
                let score1 = CARD_SCORES[&char];
                let score2 = CARD_SCORES[&chars2[idx]];
                if score1 > score2 {
                    return Ordering::Greater;
                }
                if score1 < score2 {
                    return Ordering::Less;
                }
            }
        }
        return order;
    }
}

fn get_kind(card: String) -> Kind {
    let chars: Vec<_> = card.chars().collect::<Vec<_>>();
    let mut sorted = chars.clone();
    sorted.sort();
    sorted.iter().dedup_with_count().map(|x| x.0).collect_vec();
    let mut counts = sorted.iter().dedup_with_count().map(|x| x.0).collect_vec();
    counts.sort();
    let kind = match counts[..] {
        [5] => {
            Kind::Five
        }
        [1, 4] => {
            Kind::Four
        }
        [2, 3] => {
            Kind::FullHouse
        }
        [1, 1, 3] => {
            Kind::Three
        }
        [1, 2, 2] => {
            Kind::TwoPair
        }
        [1, 1, 1, 2] => {
            Kind::OnePair
        }
        [1, 1, 1, 1, 1 ] => {
            Kind::High
        }
        _ => { todo!("unmatched pattern {:?}", counts) }
    };
    return kind;
}

fn main() {
    let input = get_input(7);

    let mut bets: Vec<Bet> = input.lines().map(|line| Bet {
        card: line.split_once(" ").unwrap().0.parse().unwrap(),
        kind: get_kind(line.split_once(" ").unwrap().0.parse().unwrap()),
        bid: line.split_once(" ").unwrap().1.parse().unwrap(),
    }).collect();

    bets.sort();

    println!("{:?}", bets.iter().enumerate().map(|(idx, bet)| (idx as i64 + 1) * bet.bid).sum::<i64>());
}