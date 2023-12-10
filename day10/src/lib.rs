use std::fmt::Display;
use ndarray::Array2;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Orientation {
    N,
    E,
    S,
    W,
}

#[derive(Debug)]
struct OrientationError(String);

impl OrientationError {
    fn new(orientation: &Orientation, tile: &Tile) -> Self {
        Self(format!("orientation: {orientation:?}, tile: {tile:?}"))
    }
}

impl Display for OrientationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Orientation {
    fn change(&self, tile: Tile) -> Result<Orientation, OrientationError> {
        match tile {
            Tile::NS => match self {
                Self::N => Ok(Self::N),
                Self::S => Ok(Self::S),
                _ => Err(OrientationError::new(self, &tile)),
            },
            Tile::EW => match self {
                Self::E => Ok(Self::E),
                Self::W => Ok(Self::W),
                _ => Err(OrientationError::new(self, &tile)),
            },
            Tile::NE => match self {
                Self::S => Ok(Self::E),
                Self::W => Ok(Self::N),
                _ => Err(OrientationError::new(self, &tile)),
            },
            Tile::NW => match self {
                Self::S => Ok(Self::W),
                Self::E => Ok(Self::N),
                _ => Err(OrientationError::new(self, &tile)),
            },
            Tile::SE => match self {
                Self::N => Ok(Self::E),
                Self::W => Ok(Self::S),
                _ => Err(OrientationError::new(self, &tile)),
            },
            Tile::SW => match self {
                Self::N => Ok(Self::W),
                Self::E => Ok(Self::S),
                _ => Err(OrientationError::new(self, &tile)),
            },
            _ => Err(OrientationError::new(self, &tile)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tile {
    NS,
    EW,
    NE,
    NW,
    SE,
    SW,
    Ground,
    S,
}
/*
    | is a vertical pipe connecting north and south.
    - is a horizontal pipe connecting east and west.
    L is a 90-degree bend connecting north and east.
    J is a 90-degree bend connecting north and west.
    7 is a 90-degree bend connecting south and west.
    F is a 90-degree bend connecting south and east.
    . is ground; there is no pipe in this tile.
    S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
*/
impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self::NS),
            '-' => Ok(Self::EW),
            'L' => Ok(Self::NE),
            'J' => Ok(Self::NW),
            '7' => Ok(Self::SW),
            'F' => Ok(Self::SE),
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::S),
            c => Err(c),
        }
    }
}

#[derive(Debug)]
pub struct Walker<'a> {
    maze: &'a Array2<Tile>,
    orientation: Orientation,
    position: Point,
}

impl<'a> Walker<'a> {
    pub fn new(maze: &'a Array2<Tile>, start: Point) -> (Self, Self) {
        let mut orientations = Vec::with_capacity(2);
        let left = maze[[start.y, start.x - 1]];
        let right = maze[[start.y, start.x + 1]];
        let up = maze[[start.y + 1, start.x]];
        let down = maze[[start.y - 1, start.x]];

        if up == Tile::NS || up == Tile::SE || up == Tile::SW {
            orientations.push(Orientation::N);
        }
        if down == Tile::NS || down == Tile::NE || down == Tile::NW {
            orientations.push(Orientation::S);
        }
        if left == Tile::EW || left == Tile::NE || left == Tile::SE {
            orientations.push(Orientation::W);
        }
        if right == Tile::EW || right == Tile::NW || right == Tile::SW {
            orientations.push(Orientation::E);
        }
        assert_eq!(orientations.len(), 2);
        let walker1 = Self {
            maze,
            orientation: orientations[0],
            position: start,
        };
        let walker2 = Self {
            maze,
            orientation: orientations[1],
            position: start,
        };
        (walker1, walker2)
    }

    pub fn position(&self) -> Point {
        self.position
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    pub fn walk(&mut self) {
        // advance
        match self.orientation {
            Orientation::N => self.position.y -= 1,
            Orientation::S => self.position.y += 1,
            Orientation::E => self.position.x += 1,
            Orientation::W => self.position.x -= 1,
        }
        // set new orientation
        self.orientation = self
            .orientation
            .change(self.maze[[self.position.y, self.position.x]])
            .unwrap();
    }
}
