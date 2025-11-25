//! Module for making a coordinate on a grid or map. Useful for a lot of the AoC
//! problems I did in the past.

use std::{
    collections::HashSet,
    fmt,
    hash::Hash,
    ops::{Add, Sub},
};

/// Structure representing a coordinate on the grid.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coord {
    /// X coordinate.
    pub x: i32,
    /// Y coordinate.
    pub y: i32,
}

impl Coord {
    /// Make a new Coord from x and y coordinates.
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }

    /// Return a cordinate north/up from this coordinate.
    pub const fn north(&self) -> Self {
        Self::new(self.x, self.y - 1)
    }

    /// Return a cordinate northeast/up-right from this coordinate.
    pub const fn northeast(&self) -> Self {
        Self::new(self.x + 1, self.y - 1)
    }

    /// Return a cordinate east/right from this coordinate.
    pub const fn east(&self) -> Self {
        Self::new(self.x + 1, self.y)
    }

    /// Return a cordinate southeast/down-right from this coordinate.
    pub const fn southeast(&self) -> Self {
        Self::new(self.x + 1, self.y + 1)
    }

    /// Return a cordinate south/down from this coordinate.
    pub const fn south(&self) -> Self {
        Self::new(self.x, self.y + 1)
    }

    /// Return a cordinate southwest/down-left from this coordinate.
    pub const fn southwest(&self) -> Self {
        Self::new(self.x - 1, self.y + 1)
    }

    /// Return a cordinate west/left from this coordinate.
    pub const fn west(&self) -> Self {
        Self::new(self.x - 1, self.y)
    }

    /// Return a cordinate northwest/up-left from this coordinate.
    pub const fn northwest(&self) -> Self {
        Self::new(self.x - 1, self.y - 1)
    }

    /// Get surrounding coordinates.
    pub fn get_surrounding_coords(&self) -> HashSet<Coord> {
        HashSet::from([
            self.north(),
            self.northeast(),
            self.east(),
            self.southeast(),
            self.south(),
            self.southwest(),
            self.west(),
            self.northwest(),
        ])
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
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Coord {
    type Output = Self;

    /// Subtract two coordinates to see the x,y distance between them.
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
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
        let expected = HashSet::from([
            Coord::new(1, 0), // N
            Coord::new(2, 0), // NE
            Coord::new(2, 1), // E
            Coord::new(2, 2), // SE
            Coord::new(1, 2), // S
            Coord::new(0, 2), // SW
            Coord::new(0, 1), // W
            Coord::new(0, 0), // NW
        ]);
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
