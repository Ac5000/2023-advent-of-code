const DIGITS: [(&str, &str, u32); 9] = [
    ("one", "1", 1),
    ("two", "2", 2),
    ("three", "3", 3),
    ("four", "4", 4),
    ("five", "5", 5),
    ("six", "6", 6),
    ("seven", "7", 7),
    ("eight", "8", 8),
    ("nine", "9", 9),
];

fn part1(file_name: &str) -> u32 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");

    let mut sum: u32 = 0;

    for line in file_contents.lines() {
        let mut first_digit: Option<u32> = None;
        let mut second_digit: Option<u32> = None;
        for character in line.chars() {
            if character.is_ascii_digit() {
                if first_digit.is_none() {
                    first_digit = character.to_digit(10);
                }
                second_digit = character.to_digit(10);
            }
        }
        sum = sum + (10 * first_digit.expect("oops")) + second_digit.expect("oops2");
    }
    sum
}

/// Returns the calibration value for the line.
fn find_num_in_line(line: &str) -> u32 {
    // Index, value
    let mut first_digit: (usize, u32) = (line.len(), 0);
    // Index, value
    let mut second_digit: (usize, u32) = (0, 0);

    for digit in DIGITS {
        let first_word = line.find(digit.0);
        let first_digit_str = line.find(digit.1);
        let second_word = line.rfind(digit.0);
        let second_digit_str = line.rfind(digit.1);

        // Check for words.
        if first_word.is_some_and(|x| x < first_digit.0) {
            first_digit = (first_word.unwrap(), digit.2);
        }
        // Check for digits.
        if first_digit_str.is_some_and(|x| x < first_digit.0) {
            first_digit = (first_digit_str.unwrap(), digit.2);
        }

        // Check for words.
        if second_word.is_some_and(|x| x >= second_digit.0) {
            second_digit = (second_word.unwrap(), digit.2);
        }
        // Check for digits.
        if second_digit_str.is_some_and(|x| x >= second_digit.0) {
            second_digit = (second_digit_str.unwrap(), digit.2);
        }
    }
    let sum = (first_digit.1 * 10) + second_digit.1;
    // Without zero, sum always has to be at least 11.
    assert!(sum > 10);
    sum
}

fn part2(file_name: &str) -> u32 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");

    let mut sum: u32 = 0;

    for line in file_contents.lines() {
        sum = sum + find_num_in_line(line);
    }
    sum
}

fn main() {
    println!("part1_result: {}", part1("input.txt"));
    println!("part2_result: {}", part2("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example01() {
        assert_eq!(part1("example.txt"), 142);
    }

    #[test]
    fn test_find_num_in_line() {
        assert_eq!(find_num_in_line("two1nine"), 29);
        assert_eq!(find_num_in_line("eightwothree"), 83);
        assert_eq!(find_num_in_line("abcone2threexyz"), 13);
        assert_eq!(find_num_in_line("xtwone3four"), 24);
        assert_eq!(find_num_in_line("4nineeightseven2"), 42);
        assert_eq!(find_num_in_line("zoneight234"), 14);
        assert_eq!(find_num_in_line("7pqrstsixteen"), 76);
        assert_eq!(find_num_in_line("6sevenpqrstsixteen"), 66);
        assert_eq!(find_num_in_line("67sevenpqrstsixteen7"), 67);
        assert_eq!(find_num_in_line("1qlbcrdgg"), 11);
    }

    #[test]
    fn part2_example01() {
        assert_eq!(part2("example2.txt"), 281);
    }
}
