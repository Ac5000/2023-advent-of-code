use std::{
    collections::{HashMap, HashSet},
    fmt,
};
use utilities::{coord::Coord, grid::Grid};

/// Struct representing a digit on the grid.
#[derive(PartialEq, Eq, Debug)]
struct Digit {
    coord: Coord,
    value: u32,
}

impl Digit {
    /// Make new Digit from coordinate and value.
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

/// Struct that represents a number from the grid.
#[derive(PartialEq, Eq, Debug)]
struct Number {
    /// Vec of digits that make up the number.
    digits: Vec<Digit>,
    /// Value of the number to be used in calculations.
    value: u32,
    /// HashSet of coordinates for each digit in the number.
    digits_coords: HashSet<Coord>,
}

impl Number {
    /// Make a new empty Number.
    fn new() -> Self {
        Self {
            digits: Vec::new(),
            value: 0,
            digits_coords: HashSet::new(),
        }
    }

    /// Calculate and return the value of the number.
    fn get_value(&mut self) -> u32 {
        match self.value {
            0 => {
                let mut val: u32 = 0;
                for (tens, digit) in self.digits.iter().rev().enumerate() {
                    val = val + (digit.value * 10_u32.pow(tens as u32))
                }
                self.value = val;
                val
            }
            _ => self.value,
        }
    }

    /// Set the digits_coords HashSet.
    fn set_digits_coords(&mut self) {
        for digit in &self.digits {
            self.digits_coords.insert(digit.coord);
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

/// Get hashmap of digits from the grid.
fn get_digits(schematic: &Grid) -> HashMap<Coord, u32> {
    let mut digits = HashMap::new();
    for (coord, character) in schematic.char_map.iter() {
        if character.is_ascii_digit() {
            match digits.insert(
                *coord,
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
    for (coord, character) in schematic.char_map.iter() {
        if !character.is_ascii_digit() && character != &'.' {
            match symbols.insert(*coord, *character) {
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
                        num.set_digits_coords();
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
                num.set_digits_coords();
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

/// Get two numbers that have a gear surrounding Coord in their coordinates.
/// swap_remove's the numbers from the original vec.
fn get_numbers_touching_gear(numbers: &mut Vec<Number>, gear: Coord) -> Option<(Number, Number)> {
    let mut num1_index: Option<usize> = None;
    let mut num2_index: Option<usize> = None;

    for (index, number) in numbers.iter().enumerate() {
        if !gear
            .get_surrounding_coords()
            .is_disjoint(&number.digits_coords)
        {
            if num1_index.is_none() {
                num1_index = Some(index);
            } else if num2_index.is_none() {
                num2_index = Some(index);
            } else {
                // More than 2 numbers found touching gear, return None.
                return None;
            }
        }
    }

    if num1_index.is_some() && num2_index.is_some() {
        let index2 = num2_index.unwrap();
        let num2 = numbers.swap_remove(index2);
        let index1 = num1_index.unwrap();
        let num1 = numbers.swap_remove(index1);
        Some((num1, num2))
    } else {
        None
    }
}

/// Get gear ratio of pairs of numbers touching a * symbol.
fn part2(file_name: &str) -> u32 {
    let schematic = Grid::new_from_file(file_name);
    let digits = get_digits(&schematic);
    let symbols = get_symbols(&schematic);
    let gears = get_gears(&symbols);
    let mut numbers = get_numbers(&schematic, digits);
    let mut touching_nums: Vec<(Number, Number)> = Vec::new();
    for gear in gears {
        if let Some((num1, num2)) = get_numbers_touching_gear(&mut numbers, gear) {
            touching_nums.push((num1, num2));
        }
    }
    let mut sum: u32 = 0;
    for num_pair in &touching_nums {
        sum = sum + num_pair.0.value * num_pair.1.value;
    }
    println!("Part 2 Gear ratio: {}", sum);
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
        assert_eq!(part2("input.txt"), 75847567)
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

        let coord1 = Coord::new(0, 0);
        let coord2 = Coord::new(1, 0);
        let coord3 = Coord::new(2, 0);

        let expected = Number {
            value: 467,
            digits: vec![
                Digit::new(coord1, 4),
                Digit::new(coord2, 6),
                Digit::new(coord3, 7),
            ],
            digits_coords: HashSet::from([coord1, coord2, coord3]),
        };
        // println!("numbers:{:#?}", numbers);
        assert_eq!(numbers.len(), 10);
        assert!(numbers.contains(&expected));
    }

    #[test]
    fn test_get_gears() {
        let example_grid = Grid::new_from_file("example.txt");
        let symbols = get_symbols(&example_grid);
        let gears = get_gears(&symbols);
        assert!(gears.contains(&Coord::new(3, 1)));
        assert!(gears.contains(&Coord::new(3, 4)));
        assert!(gears.contains(&Coord::new(5, 8)));
    }
}
