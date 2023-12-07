use std::collections::HashMap;

#[derive(Debug)]
pub struct Game(Hands);

#[derive(Debug, PartialEq, Eq)]
struct Hand([Card; 5], Bid);

type Hands = Vec<Hand>;
type Card = u32;
type Bid = u64;

trait ParseCard {
    fn try_parse_card(&self) -> Option<Card>;
}

impl Game {
    pub fn create(input: &str) -> Self {
        let hands = input.lines()
            .map(Hand::create)
            .collect();

        Self(hands)
    }

    pub fn total_winnings(&self) -> u64 {
        let mut hands = self.0.iter().collect::<Vec<_>>();
        hands.sort();

        hands.into_iter()
            .enumerate()
            .map(|(i, hand)| hand.winnings(i as u64 + 1))
            .sum()
    }

    pub fn into_jacks_to_jokers(self) -> Self {
        let Self(mut hands) = self;

        for Hand(ref mut hand, _) in hands.iter_mut() {
            for i in 0..5 {
                if hand[i] == 11 {
                    hand[i] = 1;
                }
            }
        }

        Self(hands)
    }
}

impl Hand {
    fn create(input: &str) -> Self {
        let mut parts = input.split(" ");

        let hand = parts.next().unwrap()
            .chars()
            .filter_map(|c| c.try_parse_card())
            .collect::<Vec<_>>()
            .try_into().unwrap();

        let bid = parts
            .next().unwrap()
            .parse().unwrap();

        Self(hand, bid)
    }

    fn power(&self) -> u64 {
        let mut map = HashMap::new();
        for card in self.0 {
            if let Some(count) = map.get(&card) {
                map.insert(card, count + 1);
            } else {
                map.insert(card, 1u32);
            }
        }

        if let Some(joker_count) = map.remove(&1) {
            let (card, count) = map.iter()
                .fold((0, 0), card_with_highest_count);

            map.insert(card, count + joker_count);
        }

        let mut counts = map.values().collect::<Vec<_>>();

        counts.sort();
        counts.reverse();

        match counts[..] {
            [5] => 7,        // 5 of a kind
            [4, _] => 6,     // 4 of a kind
            [3, 2] => 5,     // full house
            [3, ..] => 4,    // 3 of a kind
            [2, 2, _] => 3,  // 2 pair
            [2, ..] => 2,    // 1 pair
            _ => 1,          // high card
        }
    }

    fn winnings(&self, rank: u64) -> u64 {
        self.1 * rank
    }
}

fn card_with_highest_count(acc: (Card, u32), card: (&Card, &u32)) -> (Card, u32) {
    let (card_to_beat, count_to_beat) = acc;
    let (&card, &count) = card;

    if count > count_to_beat {
        (card, count)
    } else {
        (card_to_beat, count_to_beat)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_power = self.power();
        let other_power = other.power();

        if self_power > other_power {
            Some(std::cmp::Ordering::Greater)
        } else if self_power < other_power {
            Some(std::cmp::Ordering::Less)
        } else {
            let self_hand = self.0;
            let other_hand = other.0;

            for i in 0..5 {
                if self_hand[i] > other_hand[i] {
                    return Some(std::cmp::Ordering::Greater)
                } else if self_hand[i] < other_hand[i] {
                    return Some(std::cmp::Ordering::Less)
                }
            }

            Some(std::cmp::Ordering::Equal)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl ParseCard for char {
    fn try_parse_card(&self) -> Option<Card> {
        if self.is_numeric() {
            Some(self.to_digit(10)?)
        } else {
            match self {
                'T' => Some(10),
                'J' => Some(11),
                'Q' => Some(12),
                'K' => Some(13),
                'A' => Some(14),
                _ => None,
            }
        }
    }
}
