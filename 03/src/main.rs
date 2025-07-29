use std::collections::HashMap;

/// Structure representing the coordinate for something on the engine.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

/// Get max x and y values for the grid.
fn get_grid_maxes(file_contents: &String) -> (usize, usize) {
    let mut max_x: usize = 0;
    let max_y: usize = file_contents.lines().count();
    for line in file_contents.lines() {
        let x_count = line.chars().count();
        if x_count > max_x {
            max_x = x_count;
        }
    }
    (max_x, max_y)
}

/// Create the hashmap grid.
fn make_grid(file_contents: String) -> HashMap<Coord, char> {
    let mut grid = HashMap::new();
    for (y, line) in file_contents.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            match grid.insert(Coord::new(x, y), character) {
                None => (),
                Some(e) => panic!("Key was already present and contained: {}", e),
            }
        }
    }
    grid
}

/// Get hashmap of digits.
fn get_digits(file_contents: &String) -> HashMap<Coord, u32> {
    let mut digits = HashMap::new();
    for (y, line) in file_contents.lines().enumerate() {
        for (x, character) in line.chars().enumerate().filter(|(_, x)| x.is_ascii_digit()) {
            match digits.insert(
                Coord::new(x, y),
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
            .filter(|(_, x)| !x.is_ascii_digit() && x != &'.')
        {
            match symbols.insert(Coord::new(x, y), character) {
                None => (),
                Some(e) => panic!("Key was already present and contained: {}", e),
            }
        }
    }
    symbols
}

/// Get vec of digits that touch a symbol.
fn get_touching_symbols(digits: &HashMap<Coord, u32>, symbols: HashMap<Coord, char>) -> Vec<Coord> {
    let mut touching: Vec<Coord> = Vec::new();
    for digit in digits {
        let surrounding = digit.0.get_surrounding_coords(None, None);
        for coord in surrounding {
            match symbols.get(&coord) {
                None => (),
                Some(_) => touching.push(*digit.0),
            }
        }
    }
    touching
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
                    None => number = Some(vec![Coord::new(x, y)]),
                    Some(ref mut num) => num.push(Coord::new(x, y)),
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
    let (max_x, max_y) = get_grid_maxes(&file_contents);
    let digits = get_digits(&file_contents);
    let symbols = get_symbols(&file_contents);
    let numbers = get_numbers(&file_contents);
    // let grid = make_grid(file_contents);
    let touching_symbols = get_touching_symbols(&digits, symbols);
    let touching_nums = get_touching_numbers(&numbers, touching_symbols);
    let mut sum: u32 = 0;
    for num in &touching_nums {
        sum = sum + convert_coords_to_u32(&num, &digits);
    }
    println!("sum: {}", sum);
    sum
}

fn main() {
    part1("example.txt");
    part1("input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_grid_maxes() {
        let file_contents = std::fs::read_to_string("example.txt").expect("Couldn't open file");
        assert_eq!(get_grid_maxes(&file_contents), (10, 10));
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
        let numbers = get_numbers(&file_contents);
        let expected = vec![Coord::new(0, 0), Coord::new(1, 0), Coord::new(2, 0)];
        let expected2 = vec![Coord::new(7, 5), Coord::new(8, 5)];
        let expected3 = vec![Coord::new(7, 2), Coord::new(8, 2), Coord::new(9, 2)];
        assert!(numbers.contains(&expected));
        assert!(numbers.contains(&expected2));
        assert!(numbers.contains(&expected3));
    }

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
