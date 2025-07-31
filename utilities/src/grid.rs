//! Module for making a grid or map. Having done AoC once before, I know that having
//! a reusable base for making grids is useful.
use std::collections::HashMap;

use crate::coord::Coord;

/// Structure representing a grid/map/2D array.
#[derive(Debug)]
pub struct Grid {
    pub char_map: HashMap<Coord, char>,
    pub max_x: i32,
    pub max_y: i32,
}

impl Grid {
    /// Make a new empty grid.
    pub fn new() -> Self {
        Self {
            char_map: HashMap::new(),
            max_x: 0,
            max_y: 0,
        }
    }

    /// Make a new grid from a String.
    pub fn new_from_string(string: &String) -> Self {
        let mut max_x: i32 = 0;
        let mut max_y: i32 = 0;
        let mut char_map: HashMap<Coord, char> = HashMap::new();

        for (y, line) in string.lines().enumerate() {
            if y as i32 > max_y {
                max_y = y as i32;
            }
            for (x, character) in line.chars().enumerate() {
                if x as i32 > max_x {
                    max_x = x as i32;
                }
                char_map.insert(Coord::new(x as i32, y as i32), character);
            }
        }

        Self {
            char_map: char_map,
            max_x: max_x,
            max_y: max_y,
        }
    }

    /// Make a new grid from a file.
    pub fn new_from_file(file_name: &str) -> Self {
        let file_contents = std::fs::read_to_string(file_name).expect("Couldn't open file");
        Self::new_from_string(&file_contents)
    }

    /// Set max x and y for the grid.
    pub fn set_max_sizes(mut self) {
        for key in self.char_map.keys() {
            if key.x > self.max_x {
                self.max_x = key.x;
            }
            if key.y > self.max_y {
                self.max_y = key.y;
            }
        }
    }

    /// Grid contains the coordinate.
    pub fn has_coord(&self, coord: &Coord) -> bool {
        self.char_map.contains_key(coord)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_grid() {
        let grid = Grid::new();
        assert!(grid.char_map.is_empty());
        assert_eq!(grid.max_x, 0);
        assert_eq!(grid.max_y, 0);
    }

    #[test]
    fn test_new_grid_from_string() {
        let string: String = "123\n456".to_string();
        let grid = Grid::new_from_string(&string);
        assert_eq!(grid.max_x, 2);
        assert_eq!(grid.max_y, 1);
        assert_eq!(grid.char_map.get(&Coord { x: 0, y: 2 }), None);
        assert_eq!(grid.char_map.get(&Coord { x: 0, y: 0 }), Some(&'1'));
        assert_eq!(grid.char_map.get(&Coord { x: 1, y: 0 }), Some(&'2'));
        assert_eq!(grid.char_map.get(&Coord { x: 2, y: 0 }), Some(&'3'));
        assert_eq!(grid.char_map.get(&Coord { x: 0, y: 1 }), Some(&'4'));
        assert_eq!(grid.char_map.get(&Coord { x: 1, y: 1 }), Some(&'5'));
        assert_eq!(grid.char_map.get(&Coord { x: 2, y: 1 }), Some(&'6'));
        assert_eq!(grid.char_map.get(&Coord { x: 3, y: 1 }), None);
    }
}
