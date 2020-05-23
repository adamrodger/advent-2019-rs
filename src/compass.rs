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
                Direction::Left => Bearing::West,
                Direction::Right => Bearing::East,
            },
            Bearing::South => match *direction {
                Direction::Left => Bearing::East,
                Direction::Right => Bearing::West,
            },
            Bearing::East => match *direction {
                Direction::Left => Bearing::North,
                Direction::Right => Bearing::South,
            },
            Bearing::West => match *direction {
                Direction::Left => Bearing::South,
                Direction::Right => Bearing::North,
            },
        }
    }
}
