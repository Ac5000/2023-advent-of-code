const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
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

fn part2(file_name: &str) -> u32 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");

    let mut sum: u32 = 0;

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
    fn part2_example01() {
        assert_eq!(part2("example2.txt"), 281);
    }
}
