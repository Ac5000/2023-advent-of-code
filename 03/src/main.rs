use std::{collections::HashMap, fmt};
use utilities::{coord::Coord, grid::Grid};

#[derive(PartialEq, Eq, Debug)]
struct Digit {
    coord: Coord,
    value: u32,
}

impl Digit {
    fn new(coord: Coord, value: u32) -> Self {
        Self {
            coord: coord,
            value: value,
        }
    }
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

impl Number {
    fn digit_coords(&self) -> impl Iterator<Item = Coord> + '_ {
        self.digits.iter().map(|digit| digit.coord)
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

/// Get hashmap of digits from the grid.
fn get_digits(schematic: &Grid) -> HashMap<Coord, u32> {
    let mut digits = HashMap::new();
    for (chord, character) in schematic.char_map.iter() {
        if character.is_ascii_digit() {
            match digits.insert(
                *chord,
                character.to_digit(10).expect("failed to convert to digit."),
            ) {
                None => (),
                Some(e) => panic!("Key was already present and contained: {}", e),
            }
        }
    }
    digits
}

/// Get hashmap of symbols from the grid.
fn get_symbols(schematic: &Grid) -> HashMap<Coord, char> {
    let mut symbols = HashMap::new();
    for (chord, character) in schematic.char_map.iter() {
        if !character.is_ascii_digit() && character != &'.' {
            match symbols.insert(*chord, *character) {
                None => (),
                Some(e) => panic!("Key was already present and contained: {}", e),
            }
        }
    }
    symbols
}

/// Get a vec of numbers in the grid from the hashmap of digits.
fn get_numbers(schematic: &Grid, digits: HashMap<Coord, u32>) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();
    for y in 0..schematic.max_y + 1 {
        let mut number: Option<Number> = None;
        for x in 0..schematic.max_x + 1 {
            let coord = Coord::new(x, y);
            if let Some(value) = digits.get(&coord) {
                match number {
                    None => {
                        let mut num = Number::new();
                        num.digits.push(Digit::new(coord, *value));
                        number = Some(num);
                    }
                    Some(ref mut num) => {
                        num.digits.push(Digit::new(coord, *value));
                    }
                }
            } else {
                match number {
                    None => {}
                    Some(mut num) => {
                        num.get_value();
                        numbers.push(num);
                    }
                }
                number = None;
            }
        }
        // Push numbers touching end of row.
        match number {
            None => {}
            Some(mut num) => {
                num.get_value();
                numbers.push(num);
            }
        }
    }
    numbers
}

/// Get numbers that include a Coord in their vec.
fn get_touching_numbers(numbers: Vec<Number>, symbols: &HashMap<Coord, char>) -> Vec<Number> {
    let mut touching_nums: Vec<Number> = Vec::new();

    let mut symbol_coords: Vec<Coord> = Vec::new();
    for coord in symbols.keys() {
        for coord2 in coord.get_surrounding_coords() {
            symbol_coords.push(coord2);
        }
    }

    for number in numbers {
        for digit in &number.digits {
            if symbol_coords.contains(&digit.coord) && !touching_nums.contains(&number) {
                touching_nums.push(number);
                break;
            }
        }
    }
    touching_nums
}

/// Get sum of numbers touching a symbol.
fn part1(file_name: &str) -> u32 {
    let schematic = Grid::new_from_file(file_name);
    let digits = get_digits(&schematic);
    let symbols = get_symbols(&schematic);
    let numbers = get_numbers(&schematic, digits);
    let touching_nums = get_touching_numbers(numbers, &symbols);
    let mut sum: u32 = 0;
    for num in &touching_nums {
        sum = sum + num.value as u32;
    }
    println!("sum: {}", sum);
    sum
}

/// Get coords for all * symbols.
fn get_gears(symbols: &HashMap<Coord, char>) -> Vec<Coord> {
    let mut gears: Vec<Coord> = Vec::new();
    for (coord, symbol) in symbols.iter() {
        if symbol == &'*' {
            gears.push(*coord);
        }
    }
    gears
}

/// Get numbers that include a surrounding Coord in their digits.
fn get_numbers_touching_gears(numbers: Vec<Number>, gears: Vec<Coord>) -> Vec<Number> {
    let mut touching_nums: Vec<Number> = Vec::new();

    let mut symbol_coords: Vec<Coord> = Vec::new();
    for gear in gears {
        for coord in gear.get_surrounding_coords() {
            symbol_coords.push(coord);
        }
    }

    for number in numbers {
        for digit in &number.digits {
            if symbol_coords.contains(&digit.coord) && !touching_nums.contains(&number) {
                touching_nums.push(number);
                break;
            }
        }
    }
    touching_nums
}

/// Get gear ratio of pairs of numbers touching a * symbol.
fn part2(file_name: &str) -> u32 {
    let schematic = Grid::new_from_file(file_name);
    let digits = get_digits(&schematic);
    let symbols = get_symbols(&schematic);
    let gears = get_gears(&symbols);
    let numbers = get_numbers(&schematic, digits);
    let touching_nums = get_touching_numbers(numbers, &symbols);
    let mut sum: u32 = 0;
    for num in &touching_nums {
        sum = sum + num.value as u32;
    }
    println!("Gear ratio: {}", sum);
    sum
}

fn main() {
    part1("example.txt");
    part1("input.txt");
    part2("example2.txt");
    part2("input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example01() {
        assert_eq!(part1("example.txt"), 4361);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input.txt"), 539433)
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2("example2.txt"), 467835);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input.txt"), 99999999)
    }

    #[test]
    fn test_get_digits() {
        let example_grid = Grid::new_from_file("example.txt");
        let digits = get_digits(&example_grid);
        assert!(digits.contains_key(&Coord::new(0, 0)));
        assert!(digits.contains_key(&Coord::new(1, 9)));
        assert!(digits.contains_key(&Coord::new(7, 2)));
        assert!(digits.contains_key(&Coord::new(8, 2)));
        assert!(digits.contains_key(&Coord::new(9, 2)));
        assert!(!digits.contains_key(&Coord::new(0, 9)));
    }

    #[test]
    fn test_get_symbols() {
        let example_grid = Grid::new_from_file("example.txt");
        let symbols = get_symbols(&example_grid);
        assert!(symbols.contains_key(&Coord::new(3, 1)));
        assert!(symbols.contains_key(&Coord::new(6, 3)));
        assert!(!symbols.contains_key(&Coord::new(1, 1)));
    }

    #[test]
    fn test_get_numbers() {
        let example_grid = Grid::new_from_file("example.txt");
        let digits = get_digits(&example_grid);
        let numbers = get_numbers(&example_grid, digits);

        let expected = Number {
            value: 467,
            digits: vec![
                Digit::new(Coord::new(0, 0), 4),
                Digit::new(Coord::new(1, 0), 6),
                Digit::new(Coord::new(2, 0), 7),
            ],
        };
        println!("numbers:{:#?}", numbers);
        assert_eq!(numbers.len(), 10);
        assert!(numbers.contains(&expected));
    }
}
