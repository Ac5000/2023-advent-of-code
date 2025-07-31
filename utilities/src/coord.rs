//! Module for making a coordinate on a grid or map. Useful for a lot of the AoC
//! problems I did in the past.

use std::{
    fmt,
    ops::{Add, Sub},
};

/// Structure representing a coordinate on the grid.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    /// Make a new Coord from x and y coordinates.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }

    /// Return a cordinate north/up from this coordinate.
    pub fn north(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    /// Return a cordinate northeast/up-right from this coordinate.
    pub fn northeast(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y - 1,
        }
    }

    /// Return a cordinate east/right from this coordinate.
    pub fn east(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    /// Return a cordinate southeast/down-right from this coordinate.
    pub fn southeast(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }

    /// Return a cordinate south/down from this coordinate.
    pub fn south(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    /// Return a cordinate southwest/down-left from this coordinate.
    pub fn southwest(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    /// Return a cordinate west/left from this coordinate.
    pub fn west(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    /// Return a cordinate northwest/up-left from this coordinate.
    pub fn northwest(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y - 1,
        }
    }

    /// Get surrounding coordinates. Only return coords in range if maxes included.
    pub fn get_surrounding_coords(&self) -> Vec<Self> {
        let mut coords: Vec<Self> = Vec::new();
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

impl fmt::Display for Coord {
    /// Format the coordinate to print out nicely.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Coord {
    type Output = Self;

    /// Add two coordinates together to make a third. Useful for offsetting a distance.
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Coord {
    type Output = Self;

    /// Subtract two coordinates to see the x,y distance between them.
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_coord() {
        let coord = Coord::new(1, 1);
        assert_eq!(coord.x, 1);
        assert_eq!(coord.y, 1);
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

    #[test]
    fn test_add_coords() {
        let coord = Coord::new(1, 1);
        let coord2 = Coord::new(1, 1);
        assert_eq!(coord + coord2, Coord::new(2, 2));
    }

    #[test]
    fn test_sub_coords() {
        let coord = Coord::new(1, 1);
        let coord2 = Coord::new(1, 1);
        assert_eq!(coord - coord2, Coord::new(0, 0));
    }
}
