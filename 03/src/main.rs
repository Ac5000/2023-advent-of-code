use std::{collections::HashMap, fmt};
use utilities::{coord::Coord, grid::Grid};

#[derive(PartialEq, Eq, Debug)]
struct Digit {
    coord: Coord,
    value: u32,
}

impl fmt::Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "( {} @ {} )", self.value, self.coord)
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Symbol {
    coord: Coord,
    character: char,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "( {} @ {} )", self.character, self.coord)
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Number {
    digits: Vec<Digit>,
    value: i32,
}

impl Number {
    fn new() -> Self {
        Self {
            digits: Vec::new(),
            value: 0,
        }
    }

    fn get_value(&mut self) -> i32 {
        match self.value {
            0 => {
                let mut val: i32 = 0;
                for (tens, digit) in self.digits.iter().rev().enumerate() {
                    val = val + (digit.value as i32 * 10_i32.pow(tens as u32))
                }
                self.value = val;
                val
            }
            _ => self.value,
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "( {} - [", self.value)?;
        for digit in &self.digits {
            write!(f, "{} ", digit)?
        }
        write!(f, "]")?;
        Ok(())
    }
}

/// Get hashmap of digits.
fn get_digits(file_contents: &String) -> HashMap<Coord, u32> {
    let mut digits = HashMap::new();
    for (y, line) in file_contents.lines().enumerate() {
        for (x, character) in line.chars().enumerate().filter(|(_, c)| c.is_ascii_digit()) {
            match digits.insert(
                Coord::new(x as i32, y as i32),
                character.to_digit(10).expect("failed to convert to digit."),
            ) {
                None => (),
                Some(e) => panic!("Key was already present and contained: {}", e),
            }
        }
    }
    digits
}

/// Get hashmap of symbols.
fn get_symbols(file_contents: &String) -> HashMap<Coord, char> {
    let mut symbols = HashMap::new();
    for (y, line) in file_contents.lines().enumerate() {
        for (x, character) in line
            .chars()
            .enumerate()
            .filter(|(_, c)| !c.is_ascii_digit() && c != &'.')
        {
            match symbols.insert(Coord::new(x as i32, y as i32), character) {
                None => (),
                Some(e) => panic!("Key was already present and contained: {}", e),
            }
        }
    }
    symbols
}

/// Get vec of digits that touch a symbol.
fn get_touching_symbols(
    digits: &HashMap<Coord, u32>,
    symbols: &HashMap<Coord, char>,
) -> Vec<Coord> {
    let mut touching: Vec<Coord> = Vec::new();
    for digit in digits {
        let surrounding = digit.0.get_surrounding_coords();
        for coord in surrounding {
            match symbols.get(&coord) {
                None => (),
                Some(_) => touching.push(*digit.0),
            }
        }
    }
    touching
}

/// Get a vec of numbers in the grid.
fn get_numbers2(file_contents: &String) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();
    for (y, line) in file_contents.lines().enumerate() {
        let mut number: Option<Number> = None;
        for (x, character) in line.chars().enumerate() {
            if !character.is_ascii_digit() {
                match number {
                    None => {
                        continue;
                    }
                    Some(mut num) => {
                        num.get_value();
                        numbers.push(num);
                        number = None;
                    }
                }
                continue;
            } else {
                match number {
                    None => {
                        let mut num = Number::new();
                        let digit: Digit = Digit {
                            coord: Coord::new(x as i32, y as i32),
                            value: character.to_digit(10).expect("Failed to convert digit."),
                        };
                        num.digits.push(digit);
                        number = Some(num);
                    }
                    Some(ref mut num) => {
                        let digit: Digit = Digit {
                            coord: Coord::new(x as i32, y as i32),
                            value: character.to_digit(10).expect("Failed to convert digit."),
                        };
                        num.digits.push(digit);
                    }
                }
            }
        }
        match number {
            None => (),
            Some(mut num) => {
                num.get_value();
                numbers.push(num);
            }
        }
    }
    numbers
}

/// Get a vec of vec of coordinates that make up the various numbers on grid.
fn get_numbers(file_contents: &String) -> Vec<Vec<Coord>> {
    let mut numbers: Vec<Vec<Coord>> = Vec::new();
    for (y, line) in file_contents.lines().enumerate() {
        let mut number: Option<Vec<Coord>> = None;
        for (x, character) in line.chars().enumerate() {
            if !character.is_ascii_digit() {
                match number {
                    None => continue,
                    Some(ref n) => {
                        numbers.push(n.to_vec());
                        number = None;
                    }
                }
            } else {
                match number {
                    None => number = Some(vec![Coord::new(x as i32, y as i32)]),
                    Some(ref mut num) => num.push(Coord::new(x as i32, y as i32)),
                }
            }
        }
        // Need to push in-process numbers at end of lines.
        match number {
            None => (),
            Some(ref n) => {
                numbers.push(n.to_vec());
            }
        }
    }
    numbers
}

/// Get numbers that include a Coord in their vec.
fn get_touching_numbers(numbers: &Vec<Vec<Coord>>, touching: Vec<Coord>) -> Vec<Vec<Coord>> {
    let mut touching_nums: Vec<Vec<Coord>> = Vec::new();
    for number in numbers {
        for digit in number {
            if touching.contains(&digit) && !touching_nums.contains(number) {
                touching_nums.push(number.to_vec());
            }
        }
    }
    touching_nums
}

/// Convert vec of coords to a u32 using a HashMap of digits.
fn convert_coords_to_u32(coords: &Vec<Coord>, digits: &HashMap<Coord, u32>) -> u32 {
    let mut val: u32 = 0;
    for (tens, coord) in coords.iter().rev().enumerate() {
        val =
            val + (digits.get(coord).expect("coord not found in digits") * 10_u32.pow(tens as u32))
    }
    val
}

/// Get sum of numbers touching a symbol.
fn part1(file_name: &str) -> u32 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let digits = get_digits(&file_contents);
    let symbols = get_symbols(&file_contents);
    let numbers = get_numbers(&file_contents);
    let touching_symbols = get_touching_symbols(&digits, &symbols);
    let touching_nums = get_touching_numbers(&numbers, touching_symbols);
    let mut sum: u32 = 0;
    for num in &touching_nums {
        sum = sum + convert_coords_to_u32(&num, &digits);
    }
    println!("sum: {}", sum);
    sum
}

/// Get coords for all * symbols.
fn get_gears(symbols: &HashMap<Coord, char>) -> HashMap<Coord, char> {
    let mut gears: HashMap<Coord, char> = HashMap::new();
    for symbol in symbols {
        if symbol.1 == &'*' {
            match gears.insert(*symbol.0, *symbol.1) {
                None => (),
                Some(e) => panic!("Key was already present and contained: {}", e),
            }
        }
    }
    gears
}

/// Get gear ratio of pairs of numbers touching a * symbol.
fn part2(file_name: &str) -> u32 {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
    let digits = get_digits(&file_contents);
    let symbols = get_symbols(&file_contents);
    let gears = get_gears(&symbols);
    let numbers = get_numbers(&file_contents);
    let touching_symbols = get_touching_symbols(&digits, &gears);
    let touching_nums = get_touching_numbers(&numbers, touching_symbols);
    let mut sum: u32 = 0;
    for num in &touching_nums {
        sum = sum + convert_coords_to_u32(&num, &digits);
    }
    println!("Gear ratio: {}", sum);
    sum
}

fn main() {
    // part1("example.txt");
    // part1("input.txt");
    // let file_contents = std::fs::read_to_string("example.txt").expect("Couldn't open file");
    // let nums = get_numbers2(&file_contents);
    // for num in nums {
    //     println!("num:{}", num);
    // }
    let grid: Grid = Grid::new_from_file("input.txt");
    println!("{}", grid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("input.txt"), 539433)
    }

    #[test]
    fn test_get_digits() {
        let file_contents = std::fs::read_to_string("example.txt").expect("Couldn't open file");
        let digits = get_digits(&file_contents);
        assert!(digits.contains_key(&Coord::new(0, 0)));
        assert!(digits.contains_key(&Coord::new(1, 9)));
        assert!(digits.contains_key(&Coord::new(7, 2)));
        assert!(digits.contains_key(&Coord::new(8, 2)));
        assert!(digits.contains_key(&Coord::new(9, 2)));
        assert!(!digits.contains_key(&Coord::new(0, 9)));
    }

    #[test]
    fn test_get_symbols() {
        let file_contents = std::fs::read_to_string("example.txt").expect("Couldn't open file");
        let symbols = get_symbols(&file_contents);
        assert!(symbols.contains_key(&Coord::new(3, 1)));
        assert!(symbols.contains_key(&Coord::new(6, 3)));
        assert!(!symbols.contains_key(&Coord::new(1, 1)));
    }

    #[test]
    fn test_get_numbers() {
        let file_contents = std::fs::read_to_string("example.txt").expect("Couldn't open file");
        let numbers = get_numbers2(&file_contents);
        // let expected = vec![Coord::new(0, 0), Coord::new(1, 0), Coord::new(2, 0)];
        // let expected2 = vec![Coord::new(7, 5), Coord::new(8, 5)];
        // let expected3 = vec![Coord::new(7, 2), Coord::new(8, 2), Coord::new(9, 2)];
        // assert!(numbers.contains(&expected));
        // assert!(numbers.contains(&expected2));
        let expected = Number {
            value: 467,
            digits: vec![
                Digit {
                    coord: Coord::new(0, 0),
                    value: 4,
                },
                Digit {
                    coord: Coord::new(1, 0),
                    value: 6,
                },
                Digit {
                    coord: Coord::new(2, 0),
                    value: 7,
                },
            ],
        };
        assert_eq!(numbers.len(), 10);
        assert!(numbers.contains(&expected));
    }

    #[test]
    fn part1_example01() {
        assert_eq!(part1("example.txt"), 4361);
    }

    #[test]
    fn test_get_surrounding_coords() {
        let cord = Coord::new(1, 1);
        assert_eq!(cord.north(), Coord::new(1, 0));
        let expected: Vec<Coord> = vec![
            Coord::new(1, 0), // N
            Coord::new(2, 0), // NE
            Coord::new(2, 1), // E
            Coord::new(2, 2), // SE
            Coord::new(1, 2), // S
            Coord::new(0, 2), // SW
            Coord::new(0, 1), // W
            Coord::new(0, 0), // NW
        ];
        assert_eq!(cord.get_surrounding_coords(), expected);
    }
}
