//! Module for making a grid or map. Having done AoC once before, I know that having
//! a reusable base for making grids is useful.
use std::{collections::HashMap, fmt};

use crate::color_text::cyan;
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

    /// Separator on the x axis legend.
    const X_LEGEND_SEP: char = '|';

    /// Fill character on the x axis legend.
    const X_LEGEND_FILL: char = ' ';

    /// Separator on the y axis legend.
    const Y_LEGEND_SEP: &str = " - ";

    /// Get y legend width since we need to know it for aligning the x legend.
    fn get_y_legend_width(&self) -> usize {
        self.max_y.to_string().len() + Self::Y_LEGEND_SEP.len()
    }

    /// Make legend for x axis of the grid for display purpose.
    fn x_legend(&self) -> String {
        let mut legend: String = "".to_string();

        // Offset each row of the legend by the y axis legend width.
        let offset: &str = &" ".repeat(self.get_y_legend_width() as usize);

        for i in (0..self.max_x.to_string().len()).rev() {
            let mut line: String = offset.to_string();
            for j in 0..self.max_x + 1 {
                match j.to_string().chars().rev().nth(i) {
                    None => line.push(Self::X_LEGEND_FILL),
                    Some(c) => line.push(c),
                }
            }
            legend.push_str("\n");
            legend.push_str(&line);
        }
        legend.push_str("\n");
        // Separator line
        let line = offset.to_string()
            + &Self::X_LEGEND_SEP
                .to_string()
                .repeat(self.max_x as usize + 1)
            + "\n";
        legend.push_str(&line);
        cyan(&legend)
        // legend
    }
}

impl fmt::Display for Grid {
    /// Format the grid to print out nicely with a legend and colors.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = self.max_y.to_string().len();
        write!(f, "{}", self.x_legend())?;
        for y in 0..self.max_y + 1 {
            let y_legend: String = cyan(&format!("{:>width$}{}", y, Self::Y_LEGEND_SEP));
            write!(f, "{y_legend}")?;
            for x in 0..self.max_x + 1 {
                write!(
                    f,
                    "{}",
                    self.char_map
                        .get(&Coord::new(x, y))
                        .expect("Didn't find char to print.")
                )?;
            }
            if y < self.max_y {
                write!(f, "\n")?
            }
        }
        Ok(())
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
        assert_eq!(grid.char_map.get(&Coord::new(0, 2)), None);
        assert_eq!(grid.char_map.get(&Coord::new(0, 0)), Some(&'1'));
        assert_eq!(grid.char_map.get(&Coord::new(1, 0)), Some(&'2'));
        assert_eq!(grid.char_map.get(&Coord::new(2, 0)), Some(&'3'));
        assert_eq!(grid.char_map.get(&Coord::new(0, 1)), Some(&'4'));
        assert_eq!(grid.char_map.get(&Coord::new(1, 1)), Some(&'5'));
        assert_eq!(grid.char_map.get(&Coord::new(2, 1)), Some(&'6'));
        assert_eq!(grid.char_map.get(&Coord::new(3, 1)), None);
    }

    #[test]
    fn test_get_y_legend_width() {
        let string: String = "123\n456".to_string();
        let grid = Grid::new_from_string(&string);
        assert_eq!(grid.get_y_legend_width(), 4)
    }

    // #[test]
    // fn test_grid_display() {
    //     let string: String = "01234567890\n01234567890".to_string();
    //     let grid = Grid::new_from_string(&string);
    //     println!("{grid}");
    //     assert_eq!(grid.to_string(), string)
    // }
}
