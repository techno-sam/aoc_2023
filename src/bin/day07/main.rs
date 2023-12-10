use std::{cmp::Ordering, collections::HashMap, fs};

fn main() {
    println!("AOC 2023 Day 7");
    let content = fs::read_to_string("src/bin/day07/input.txt").expect("Could not find input");
    let lines = content.trim().split("\n");
    let mut hands: Vec<(Hand, u64)> = lines.map(|l| l.trim().split_once(" ").map(|(a, b)| (a.trim(), b.trim())))
        .map(|o| o.unwrap())
        .map(|(card_str, count_str)| (Hands::parse_hand(card_str), count_str.parse::<u64>().unwrap()))
        .collect();
    hands.sort_by(|(hand_a, _), (hand_b, _)| compare_hands(*hand_a, *hand_b));
    println!("Sorted hands:");
    for hand in &hands {
        println!("\t[{:?}]: {}", hand.0, hand.1);
    }
    let sum_winnings: u64 = hands.iter().map(|(_, bid)| bid).enumerate().map(|(idx, bid)| (idx as u64 + 1) * bid).sum();
    println!("Sum of winnings: {}", sum_winnings);
}


type Hand = [Card; 5];

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Card {
    A, // val 13
    K,
    Q,
    T,
    _9,
    _8,
    _7,
    _6,
    _5,
    _4,
    _3,
    _2,
    J,  // val 0
}
impl Card {
    fn index(&self) -> usize {
        return *self as usize;
    }

    fn value(&self) -> u64 {
        return 13 - (self.index() as u64);
    }

    fn parse(chr: char) -> Option<Card> {
        return match chr {
            'A' => Some(Card::A),
            'K' => Some(Card::K),
            'Q' => Some(Card::Q),
            'J' => Some(Card::J),
            'T' => Some(Card::T),
            '9' => Some(Card::_9),
            '8' => Some(Card::_8),
            '7' => Some(Card::_7),
            '6' => Some(Card::_6),
            '5' => Some(Card::_5),
            '4' => Some(Card::_4),
            '3' => Some(Card::_3),
            '2' => Some(Card::_2),
            _ => None
        };
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
enum Hands {
    FiveKind,  // AAAAA
    FourKind,  // AA8AA
    FullHouse, // 23332
    ThreeKind, // TTT98
    TwoPair,   // 23432
    OnePair,   // A23A4
    HighCard,  // 23456
}
impl Hands {
    #[allow(dead_code)]
    fn from_str(data: &str) -> Hands {
        return Hands::from(Hands::parse_hand(data));
    }

    fn parse_hand(data: &str) -> Hand {
        assert_eq!(5, data.len());
        let cards: Vec<Card> = data.chars().into_iter().map(|c| Card::parse(c).unwrap()).collect();
        return [cards[0], cards[1], cards[2], cards[3], cards[4]];
    }

    fn from(hand: Hand) -> Hands {
        let mut joker_count: u64 = 0;
        let mut counts: HashMap<Card, u64> = HashMap::new();
        for card in hand {
            if card == Card::J {
                joker_count += 1;
            } else {
                counts.insert(card, counts.get(&card).or(Some(&0u64)).unwrap() + 1);
            }
        }
        let mut vals: Vec<u64> = counts.values().map(|v| *v).collect();
        vals.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap()); // reverse sort
        while vals.len() < 2 {
            vals.push(0);
        }
        if vals[0]+joker_count >= 5 { // WARN: modified
            return Hands::FiveKind;
        } else if vals[0]+joker_count >= 4 { // WARN: modified
            return Hands::FourKind;
        } else if vals[0] == 3 && vals[1] == 2 { // NOTE: original
            return Hands::FullHouse;
        } else if (3-vals[0]) + (2-vals[1]) <= joker_count { // vals[0]+part of joker >= 3 && vals[1] + part of joker >= 2
                                                             // jokers needed = (3-vals[0]) + (2-vals[1])
            return Hands::FullHouse;
        } else if vals[0]+joker_count >= 3 { // WARN: modified
            return Hands::ThreeKind;
        } else if vals[0] == 2 && vals[1] == 2 { // NOTE: original
            return Hands::TwoPair;
        } else if (2-vals[0]) + (2-vals[1]) <= joker_count { // jokers_needed = (2-vals[0]) + (2-vals[1])
            return Hands::TwoPair;
        } else if vals[0]+joker_count >= 2 { // WARN: modified
            return Hands::OnePair;
        } else { // NOTE: original
            return Hands::HighCard;
        }
    }
}

fn compare_hands(a: Hand, b: Hand) -> Ordering {
    let a_hands: Hands = Hands::from(a);
    let b_hands: Hands = Hands::from(b);
    let ord = b_hands.partial_cmp(&a_hands).expect("The hands aren't ordered, :why:");
    if ord == Ordering::Equal {
        for i in 0..5_usize {
            let av = a[i].value();
            let bv = b[i].value();
            let val_ord = av.partial_cmp(&bv).unwrap();
            if val_ord != Ordering::Equal {
                return val_ord;
            }
        }
        return Ordering::Equal;
    } else {
        return ord;
    }
}

#[test]
fn hand_parsing() {
    assert_eq!(Hands::FiveKind, Hands::from_str("AAAAA"));
    assert_eq!(Hands::FourKind, Hands::from_str("AA8AA"));
    assert_eq!(Hands::FullHouse, Hands::from_str("23332"));
    assert_eq!(Hands::ThreeKind, Hands::from_str("TTT98"));
    assert_eq!(Hands::TwoPair, Hands::from_str("23432"));
    assert_eq!(Hands::OnePair, Hands::from_str("A23A4"));
    assert_eq!(Hands::HighCard, Hands::from_str("23456"));
}

#[test]
fn hand_cmp() {
    assert_eq!(Ordering::Greater, compare_hands(Hands::parse_hand("33332"), Hands::parse_hand("2AAAA")));
}

#[test]
fn joker_parsing() {
    assert_eq!(Hands::FourKind, Hands::from_str("T55J5"));
    assert_eq!(Hands::FourKind, Hands::from_str("KTJJT"));
    assert_eq!(Hands::FourKind, Hands::from_str("QQQJA"));
}
