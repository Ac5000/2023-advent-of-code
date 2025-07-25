/// Structure representing the coordinate for something on the engine.
#[derive(PartialEq, Debug)]
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
    fn north(&self) -> Coord {
        Coord {
            x: self.x,
            y: self.y - 1,
        }
    }

    /// Return a cordinate northeast/up-right from this coordinate.
    fn northeast(&self) -> Coord {
        Coord {
            x: self.x + 1,
            y: self.y - 1,
        }
    }

    /// Return a cordinate east/right from this coordinate.
    fn east(&self) -> Coord {
        Coord {
            x: self.x + 1,
            y: self.y,
        }
    }

    /// Return a cordinate southeast/down-right from this coordinate.
    fn southeast(&self) -> Coord {
        Coord {
            x: self.x + 1,
            y: self.y + 1,
        }
    }

    /// Return a cordinate south/down from this coordinate.
    fn south(&self) -> Coord {
        Coord {
            x: self.x,
            y: self.y + 1,
        }
    }

    /// Return a cordinate southwest/down-left from this coordinate.
    fn southwest(&self) -> Coord {
        Coord {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    /// Return a cordinate west/left from this coordinate.
    fn west(&self) -> Coord {
        Coord {
            x: self.x - 1,
            y: self.y,
        }
    }

    /// Return a cordinate northwest/up-left from this coordinate.
    fn northwest(&self) -> Coord {
        Coord {
            x: self.x - 1,
            y: self.y - 1,
        }
    }

    /// Get surrounding coordinates.
    fn get_surrounding_coords(&self) -> Vec<Coord> {
        let mut coords: Vec<Coord> = Vec::new();
        coords.push(self.north());
        coords.push(self.northeast());
        coords.push(self.east());
        coords.push(self.southeast());
        coords.push(self.south());
        coords.push(self.southwest());
        coords.push(self.west());
        coords.push(self.northwest());
        coords
    }
}

/// Structure representing a digit in a part number.
#[derive(PartialEq, Debug)]
struct Digit {
    character: char,
    value: u32,
    coord: Coord,
    touching_symbols: Option<Symbol>,
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
            touching_symbols: None,
        }
    }
}

/// Structure representing a part number made up of digits.
#[derive(PartialEq, Debug)]
struct Number {
    digits: Vec<Digit>,
}

impl Number {
    /// Make a new Number with empty digits.
    fn new() -> Number {
        Number { digits: Vec::new() }
    }

    /// Make a new Number from Vec<Digit>.
    fn new_from_digits(digits: Vec<Digit>) -> Number {
        Number { digits: digits }
    }
}

/// Structure representing a symbol on the engine.
#[derive(PartialEq, Debug)]
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
        }
    }
    // Enumerate lines of file
    // Enumerate chars of line
    // If "." continue
    // Check if ascii digit - save coordinate if true

    // println!("NUMBERS");
    // println!("numbers: {:#?}", numbers);
    // println!("\nSYMBOLS");
    // println!("symbols: {:#?}", symbols);
    sum
}

fn main() {
    println!("part1_result: {}", part1("example.txt"));
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

        let cord = Coord::new(0, 0);
        assert_eq!(cord.north(), Coord::new(0, 0));
    }
}
