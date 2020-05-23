pub enum Bearing {
    North,
    South,
    East,
    West,
}

pub enum Direction {
    Left,
    Right,
}

impl Bearing {
    pub fn turn(&self, direction: &Direction) -> Bearing {
        match *self {
            Bearing::North => match *direction {
                Direction::Left => return Bearing::West,
                Direction::Right => return Bearing::East,
            },
            Bearing::South => match *direction {
                Direction::Left => return Bearing::East,
                Direction::Right => return Bearing::West,
            },
            Bearing::East => match *direction {
                Direction::Left => return Bearing::North,
                Direction::Right => return Bearing::South,
            },
            Bearing::West => match *direction {
                Direction::Left => return Bearing::South,
                Direction::Right => return Bearing::North,
            },
        }
    }
}
