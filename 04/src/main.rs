//! Day 04: Scratchcards

use std::collections::{HashMap, HashSet};

/// Structure representing a scratchcard.
#[derive(Clone, Debug)]
struct Scratchcard {
    /// Number of the card
    card_number: u8,
    /// Winning numbers (left side) on the scratchcard.
    winning_numbers: HashSet<u8>,
    /// Numbers (right side) on the scratchcard.
    numbers: HashSet<u8>,
    /// Copies of this card.
    copies: u32,
}

impl Scratchcard {
    /// Make a new empty scratchcard.
    fn new() -> Scratchcard {
        Self {
            card_number: 0,
            winning_numbers: HashSet::new(),
            numbers: HashSet::new(),
            copies: 1,
        }
    }

    /// Make a new scratchcard from a String.
    fn new_from_string(string: &str) -> Self {
        let (card_number_raw, winning_nums_and_nums_raw) =
            string.split_once(": ").expect("failed to split ': '");
        let card_number_split: Vec<&str> = card_number_raw.split_ascii_whitespace().collect();
        let card_number =
            u8::from_str_radix(card_number_split[1], 10).expect("Failed to convert card_number.");
        let (winning_nums_raw, nums_raw) = winning_nums_and_nums_raw
            .split_once(" | ")
            .expect("failed to split ' | '");
        let winning_numbers = HashSet::from_iter(
            winning_nums_raw
                .split_ascii_whitespace()
                .map(|s| u8::from_str_radix(s, 10).expect("Failed to convert str to u8.")),
        );
        let numbers = HashSet::from_iter(
            nums_raw
                .split_ascii_whitespace()
                .map(|s| u8::from_str_radix(s, 10).expect("Failed to convert str to u8.")),
        );
        Self {
            card_number,
            winning_numbers,
            numbers,
            copies: 1,
        }
    }

    /// Calculate points for matching numbers.
    fn calculate_score(&self) -> u32 {
        let matching_nums = self.matching_numbers();
        if matching_nums <= 0 {
            return 0;
        }
        2u32.pow(matching_nums as u32 - 1)
    }

    /// Get number of matching numbers for part2.
    fn matching_numbers(&self) -> u8 {
        self.winning_numbers.intersection(&self.numbers).count() as u8
    }

    /// Increment copy count by value.
    fn increment(&mut self, value: u32) {
        self.copies = self.copies + value;
    }

    /// Get the keys for the cards to increment based on matching_numbers and
    /// how much to increment based on this cards copies. (key, qty)
    fn keys_and_qty_to_increment(&self) -> HashMap<u8, u32> {
        let mut ret: HashMap<u8, u32> = HashMap::new();
        for i in 0..self.matching_numbers() {
            ret.insert(self.card_number + 1 + i, self.copies);
        }
        ret
    }
}

/// Get points for matching cards.
fn part1(file_name: &str) -> u32 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let mut scratch_cards: Vec<Scratchcard> = Vec::new();
    for line in file_contents.lines() {
        scratch_cards.push(Scratchcard::new_from_string(line));
    }
    let mut score: u32 = 0;
    for card in scratch_cards {
        score = score + card.calculate_score();
    }
    println!("Part 1 Score = {score}");
    score
}

/// Cards multiply by winning numbers. Get total count of cards.
fn part2(file_name: &str) -> u32 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let mut card_map: HashMap<u8, Scratchcard> = HashMap::new();
    for line in file_contents.lines() {
        let scratch_card = Scratchcard::new_from_string(line);
        card_map.insert(scratch_card.card_number, scratch_card);
    }

    for i in 1..card_map.len() + 1 {
        let i = i as u8;
        let keys_qty = card_map
            .get(&i)
            .expect("Failed to find card.")
            .keys_and_qty_to_increment();
        for (key, qty) in keys_qty {
            card_map
                .get_mut(&key)
                .expect("Failed to find card to increment.")
                .increment(qty);
        }
    }

    let mut sum: u32 = 0;
    for (_, card) in card_map {
        sum = sum + card.copies;
    }
    println!("Part 2 Sum = {sum}");
    sum
}

fn main() {
    part1("example.txt");
    part1("input.txt");
    part2("example.txt");
    part2("input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example01() {
        assert_eq!(part1("example.txt"), 13);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input.txt"), 24706);
    }

    #[test]
    fn part2_example01() {
        assert_eq!(part2("example.txt"), 30);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input.txt"), 13114317);
    }

    #[test]
    fn test_scratchcard_new_from_string() {
        let scratch =
            Scratchcard::new_from_string("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        let winning_nums = HashSet::from([41, 48, 83, 86, 17]);
        let nums = HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]);
        assert_eq!(scratch.card_number, 1);
        assert_eq!(scratch.winning_numbers, winning_nums);
        assert_eq!(scratch.numbers, nums);
    }

    #[test]
    fn test_scratchcard_score_matches() {
        let mut scratch = Scratchcard::new();
        let winning_nums = HashSet::from([41, 48, 83, 86, 17]);
        let nums = HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]);
        scratch.winning_numbers = winning_nums;
        scratch.numbers = nums;
        assert_eq!(scratch.calculate_score(), 8);
    }

    #[test]
    fn test_scratchcard_score_no_matches() {
        let mut scratch = Scratchcard::new();
        let winning_nums = HashSet::from([1, 2, 3, 4, 5]);
        let nums = HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]);
        scratch.winning_numbers = winning_nums;
        scratch.numbers = nums;
        assert_eq!(scratch.calculate_score(), 0);
    }

    #[test]
    fn test_scratchcard_copy_inc() {
        let mut scratch = Scratchcard::new();
        assert_eq!(scratch.copies, 1);
        scratch.increment(1);
        assert_eq!(scratch.copies, 2);
        scratch.increment(5);
        assert_eq!(scratch.copies, 7);
    }

    #[test]
    fn test_scratchcard_keys_to_inc() {
        let mut scratch = Scratchcard::new();
        let winning_nums = HashSet::from([41, 48, 83, 86, 17]);
        let nums = HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]);
        scratch.card_number = 1;
        scratch.winning_numbers = winning_nums;
        scratch.numbers = nums;
        assert_eq!(scratch.matching_numbers(), 4);
        let expected = HashMap::from([(2, 1), (3, 1), (4, 1), (5, 1)]);
        assert_eq!(scratch.keys_and_qty_to_increment(), expected);
        scratch.copies = 5;
        let expected = HashMap::from([(2, 5), (3, 5), (4, 5), (5, 5)]);
        assert_eq!(scratch.keys_and_qty_to_increment(), expected);
    }

    #[test]
    fn test_scratchcard_keys_to_inc_no_matches() {
        let mut scratch = Scratchcard::new();
        let winning_nums = HashSet::from([1, 2, 3, 4, 5]);
        let nums = HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]);
        scratch.card_number = 1;
        scratch.winning_numbers = winning_nums;
        scratch.numbers = nums;
        assert_eq!(scratch.matching_numbers(), 0);
        let expected = HashMap::new();
        assert_eq!(scratch.keys_and_qty_to_increment(), expected);
    }
}
