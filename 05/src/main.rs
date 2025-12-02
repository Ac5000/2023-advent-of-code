//! Day 05: If You Give A Seed A Fertilizer

/// Convert a seed number to a location number using all the conversions between.
fn part1(file_name: &str) -> u32 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    5
}
// 4294967295
// 4196563819
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

    // #[test]
    // fn part2_example01() {
    //     assert_eq!(part2("example.txt"), 30);
    // }
    //
    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2("input.txt"), 13114317);
    // }
}
