use std::fs;

fn main() {
    println!("AOC 2023 Day 4");

    let contents = fs::read_to_string("src/bin/day4/input.txt").expect("Failed to read file");
    let mut pile: CardPile = CardPile::load(&contents);
    let total: u32 = pile.calculate_total_points();
    println!("Part 1: {}", total);
    let copies: u32 = pile.calculate_copies();
    println!("Part 2: {}", copies);
}

struct CardPile {
    cards: Vec<Card>,
    total_copies: Option<u32>,
}

impl CardPile {
    fn load(txt: &str) -> CardPile {
        let mut pile = CardPile::new();
        let lines = txt.split("\n");
        for line in lines {
            if line.is_empty() {
                continue;
            }
            pile.push(Card::parse(line));
        }
        return pile;
    }

    fn new() -> CardPile {
        return CardPile { cards: vec![], total_copies: None };
    }

    fn push(&mut self, card: Card) {
        assert_eq!(self.cards.len() as u32, card.id - 1);
        self.cards.push(card);
    }

    fn get_immutable(&self, card_id: u32) -> &Card {
        return &self.cards[(card_id - 1) as usize];
    }

    fn get(&mut self, card_id: u32) -> &mut Card {
        return &mut self.cards[(card_id - 1) as usize];
    }

    fn calculate_total_points(&mut self) -> u32 {
        let mut total: u32 = 0;
        for id in 1u32..=(self.cards.len() as u32) {
            total += self.get(id).calculate_points();
        }
        return total;
    }

    fn calculate_copies(&mut self) -> u32 {
        if let Some(copies) = self.total_copies {
            return copies;
        }

        self.calculate_total_points();
        for id in 1u32..=(self.cards.len() as u32) {
            let current = self.get_immutable(id);
            let instances = current.instances;
            for offset in 1..=current.matches {
                let other: &mut Card = self.get(id + offset);
                other.instances += instances;
            }
        }

        let mut copies: u32 = 0;
        for id in 1u32..=(self.cards.len() as u32) {
            copies += self.get_immutable(id).instances;
        }
        self.total_copies = Some(copies);
        return copies;
    }
}

#[allow(dead_code)]
struct Card {
    id: u32,
    winning: Vec<u32>,
    have: Vec<u32>,
    sorted_winning: Vec<u32>,
    sorted_have: Vec<u32>,
    points: Option<u32>,
    instances: u32, // >= 1 (original + copies)
    matches: u32
}

fn nums_to_vec(nums: &str) -> Vec<u32> {
    return nums.split(" ")
        .filter(|t| !t.is_empty())
        .map(|t| t.parse::<u32>().unwrap())
        .collect();
}

impl Card {
    fn parse(text: &str) -> Card {
        let (card_id, numbers) = text.trim().split_once(": ").unwrap();
        let id: u32 = card_id.strip_prefix("Card ").unwrap().trim().parse::<u32>()
            .expect(&("Faild to parse id from ".to_owned() + card_id));
        let (winning, have) = numbers.split_once(" | ").unwrap();

        let win_vec: Vec<u32> = nums_to_vec(winning);
        let have_vec: Vec<u32> = nums_to_vec(have);

        let mut win_sorted: Vec<u32> = win_vec.clone();
        let mut have_sorted: Vec<u32> = have_vec.clone();
        win_sorted.sort_unstable();
        have_sorted.sort_unstable();

        return Card { id, winning: win_vec, have: have_vec, sorted_winning: win_sorted, sorted_have: have_sorted, points: None, instances: 1, matches: 0 };
    }

    fn calculate_points(&mut self) -> u32 {
        if let Some(points) = self.points {
            return points;
        }
        let mut points: u32 = 0;

        let mut have_idx: usize = 0;
        let mut win_idx: usize = 0;
        self.matches = 0;

        while have_idx < self.sorted_have.len() && win_idx < self.sorted_winning.len() {
            let have = self.sorted_have[have_idx];
            let win = self.sorted_winning[win_idx];

            if have == win {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
                self.matches += 1;
                have_idx += 1;
                win_idx += 1;
            } else if have < win {
                have_idx += 1;
            } else if win < have { // yes, this is equivalent to just else{}, but this is clearer
                win_idx += 1;
            }
        }

        self.points = Some(points);
        return points;
    }
}

#[test]
fn point_calculation() {
    assert_eq!(8, Card::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").calculate_points());
    assert_eq!(2, Card::parse("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19").calculate_points());
    assert_eq!(2, Card::parse("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1").calculate_points());
    assert_eq!(1, Card::parse("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83").calculate_points());
    assert_eq!(0, Card::parse("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36").calculate_points());
    assert_eq!(0, Card::parse("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11").calculate_points());
}

#[test]
fn instance_stacking() {
    let mut pile: CardPile = CardPile::load("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
");
    let total = pile.calculate_copies();
    assert_eq!(1, pile.get(1).instances);
    assert_eq!(2, pile.get(2).instances);
    assert_eq!(4, pile.get(3).instances);
    assert_eq!(8, pile.get(4).instances);
    assert_eq!(14, pile.get(5).instances);
    assert_eq!(1, pile.get(6).instances);
    assert_eq!(30, total);
}
