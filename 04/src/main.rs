//! Day 04: Scratchcards

use std::collections::HashSet;

/// Structure representing a scratchcard.
#[derive(Debug)]
struct Scratchcard {
    /// Number/name of the card
    card_number: String,
    /// Winning numbers (left side) on the scratchcard.
    winning_numbers: HashSet<u8>,
    /// Numbers (right side) on the scratchcard.
    numbers: HashSet<u8>,
}

impl Scratchcard {
    /// Make a new empty scratchcard.
    fn new() -> Scratchcard {
        Self {
            card_number: String::new(),
            winning_numbers: HashSet::new(),
            numbers: HashSet::new(),
        }
    }

    /// Make a new scratchcard from a String.
    fn new_from_string(string: &str) -> Self {
        let (card_number, winning_nums_and_nums_raw) =
            string.split_once(": ").expect("failed to split ': '");
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
            card_number: card_number.to_string(),
            winning_numbers,
            numbers,
        }
    }

    /// Calculate points for matching numbers.
    fn calculate_score(&self) -> u32 {
        let matching_nums = self.winning_numbers.intersection(&self.numbers).count();
        if matching_nums <= 0 {
            return 0;
        }
        2u32.pow(matching_nums as u32 - 1)
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

fn main() {
    part1("example.txt");
    part1("input.txt");
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
    fn test_scratchcard_new_from_string() {
        let scratch =
            Scratchcard::new_from_string("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        let winning_nums = HashSet::from([41, 48, 83, 86, 17]);
        let nums = HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]);
        assert_eq!(scratch.card_number, "Card 1");
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
}
