/// Structure representing the coordinate for something on the engine.
#[derive(Clone, PartialEq, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    /// Make a new Coord from x and y coordinates.
    fn new(x: usize, y: usize) -> Coord {
        Coord { x: x, y: y }
    }

    /// Return a cordinate north/up from this coordinate.
    fn north(&self) -> Option<Coord> {
        if self.y == 0 {
            return None;
        }
        Some(Coord {
            x: self.x,
            y: self.y - 1,
        })
    }

    /// Return a cordinate northeast/up-right from this coordinate.
    fn northeast(&self) -> Option<Coord> {
        if self.y == 0 {
            return None;
        }
        Some(Coord {
            x: self.x + 1,
            y: self.y - 1,
        })
    }

    /// Return a cordinate east/right from this coordinate.
    fn east(&self) -> Option<Coord> {
        Some(Coord {
            x: self.x + 1,
            y: self.y,
        })
    }

    /// Return a cordinate southeast/down-right from this coordinate.
    fn southeast(&self) -> Option<Coord> {
        Some(Coord {
            x: self.x + 1,
            y: self.y + 1,
        })
    }

    /// Return a cordinate south/down from this coordinate.
    fn south(&self) -> Option<Coord> {
        Some(Coord {
            x: self.x,
            y: self.y + 1,
        })
    }

    /// Return a cordinate southwest/down-left from this coordinate.
    fn southwest(&self) -> Option<Coord> {
        if self.x == 0 {
            return None;
        }
        Some(Coord {
            x: self.x - 1,
            y: self.y + 1,
        })
    }

    /// Return a cordinate west/left from this coordinate.
    fn west(&self) -> Option<Coord> {
        if self.x == 0 {
            return None;
        }
        Some(Coord {
            x: self.x - 1,
            y: self.y,
        })
    }

    /// Return a cordinate northwest/up-left from this coordinate.
    fn northwest(&self) -> Option<Coord> {
        if self.x == 0 || self.y == 0 {
            return None;
        }
        Some(Coord {
            x: self.x - 1,
            y: self.y - 1,
        })
    }

    /// Get surrounding coordinates. Only return coords in range if maxes included.
    fn get_surrounding_coords(&self, max_x: Option<usize>, max_y: Option<usize>) -> Vec<Coord> {
        let mut coords: Vec<Coord> = Vec::new();
        match self.north() {
            None => (),
            Some(coord) => coords.push(coord),
        }
        match self.northeast() {
            None => (),
            Some(coord) => coords.push(coord),
        }
        match self.east() {
            None => (),
            Some(coord) => coords.push(coord),
        }
        match self.southeast() {
            None => (),
            Some(coord) => coords.push(coord),
        }
        match self.south() {
            None => (),
            Some(coord) => coords.push(coord),
        }
        match self.southwest() {
            None => (),
            Some(coord) => coords.push(coord),
        }
        match self.west() {
            None => (),
            Some(coord) => coords.push(coord),
        }
        match self.northwest() {
            None => (),
            Some(coord) => coords.push(coord),
        }
        match max_x {
            None => (),
            Some(max_x) => coords.retain(|x| x.x <= max_x),
        }
        match max_y {
            None => (),
            Some(max_y) => coords.retain(|x| x.y <= max_y),
        }
        coords
    }
}

/// Structure representing a digit in a part number.
#[derive(PartialEq, Debug)]
struct Digit {
    character: char,
    value: u32,
    coord: Coord,
    touching_symbols: Vec<Symbol>,
}

impl Digit {
    /// Make a new Digit from a character and x and y coordinates.
    fn new(character: char, x: usize, y: usize) -> Digit {
        let value: u32 = character
            .to_digit(10)
            .unwrap_or_else(|| panic!("failed to parse: {}", character));
        Digit {
            character: character,
            value: value,
            coord: Coord::new(x, y),
            touching_symbols: Vec::new(),
        }
    }

    /// Get touching symbols for this digit.
    fn get_touching_symbols(
        &mut self,
        symbols: &[Symbol],
        max_x: Option<usize>,
        max_y: Option<usize>,
    ) {
        let surrounding_coords = self.coord.get_surrounding_coords(max_x, max_y);
        for coord in surrounding_coords {
            for symbol in symbols.iter() {
                if symbol.coord == coord {
                    self.touching_symbols.push(symbol.clone());
                }
            }
        }
    }
}

/// Structure representing a part number made up of digits.
#[derive(PartialEq, Debug)]
struct Number {
    digits: Vec<Digit>,
    is_part_num: Option<bool>,
    value: u32,
}

impl Number {
    /// Make a new Number with empty digits.
    fn new() -> Number {
        Number {
            digits: Vec::new(),
            is_part_num: None,
            value: 0,
        }
    }

    /// Get touching symbols.
    fn get_touching_symbols(
        &mut self,
        symbols: &[Symbol],
        max_x: Option<usize>,
        max_y: Option<usize>,
    ) {
        for digit in &mut self.digits {
            digit.get_touching_symbols(symbols, max_x, max_y);
            if !digit.touching_symbols.is_empty() {
                self.is_part_num = Some(true);
            }
        }
        if self.is_part_num.is_none() {
            self.is_part_num = Some(false);
        }
    }

    /// Turn digits into a number we can use.
    fn get_value(&mut self) -> u32 {
        for (tens, digit) in self.digits.iter().rev().enumerate() {
            self.value = self.value + (digit.value * 10_u32.pow(tens as u32));
        }
        self.value
    }
}

/// Structure representing a symbol on the engine.
#[derive(Clone, PartialEq, Debug)]
struct Symbol {
    character: char,
    coord: Coord,
}

impl Symbol {
    /// Make a new Symbol from a character and x and y coordinates.
    fn new(character: char, x: usize, y: usize) -> Symbol {
        Symbol {
            character: character,
            coord: Coord::new(x, y),
        }
    }
}

/// Get sum of numbers touching a symbol.
fn part1(file_name: &str) -> u32 {
    let mut sum: u32 = 0;
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;

    // Read the file into string.
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");

    // Read the file string line by line with a Y axis value for the grid.
    for (y, line) in file_contents.lines().enumerate() {
        // Initialize a new part number since they can't cross lines.
        let mut number: Option<Number> = None;

        // Read the line character by character with an X axis value for the grid.
        for (x, character) in line.chars().enumerate() {
            // Periods do not count as a symbol.
            if character == '.' {
                // If we were making a number, push it to the numbers and clear.
                if number.is_some() {
                    numbers.push(number.expect("number expected."));
                    number = None;
                }
                continue;
            }

            // Track symbols for later use.
            if !character.is_ascii_digit() {
                // If we were making a number, push it to the numbers and clear.
                if number.is_some() {
                    numbers.push(number.expect("number expected."));
                    number = None;
                }
                // Add the symbol to the vec.
                symbols.push(Symbol::new(character, x, y));
                continue;
            }

            // Capture vec of numbers using digits we find.
            if character.is_ascii_digit() {
                // Initialize new number if we aren't already building one. Otherwise
                // push the digit to the number.
                match number {
                    None => {
                        let mut temp = Number::new();
                        temp.digits.push(Digit::new(character, x, y));
                        number = Some(temp)
                    }
                    Some(ref mut num) => num.digits.push(Digit::new(character, x, y)),
                }
            }
            max_x = if x > max_x { x } else { max_x };
        }
        max_y = if y > max_y { y } else { max_y };
    }

    for number in &mut numbers {
        number.get_touching_symbols(&symbols, Some(max_x), Some(max_y));
        number.get_value();
        match number.is_part_num {
            None => !panic!("I don't think I should get here?"),
            Some(b) => match b {
                false => (),
                true => sum = sum + number.value,
            },
        }
    }

    // println!("NUMBERS");
    // println!("numbers: {:#?}", numbers);
    // println!("\nSYMBOLS");
    // println!("symbols: {:#?}", symbols);
    sum
}

fn main() {
    // 536667 is too low.
    println!("part1_result: {}", part1("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example01() {
        assert_eq!(part1("example.txt"), 4361);
    }

    #[test]
    fn test_get_surrounding_coords() {
        let cord = Coord::new(1, 1);
        assert_eq!(cord.north(), Some(Coord::new(1, 0)));
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
        assert_eq!(cord.get_surrounding_coords(None, None), expected);

        let cord = Coord::new(0, 0);
        assert_eq!(cord.north(), None);
        let expected: Vec<Coord> = vec![
            Coord::new(1, 0), // E
            Coord::new(1, 1), // SE
            Coord::new(0, 1), // S
        ];
        assert_eq!(cord.get_surrounding_coords(None, None), expected);
        assert_eq!(cord.get_surrounding_coords(Some(0), Some(0)), vec![]);
    }
}
