use std::collections::HashSet;
use std::error::Error;
use std::iter::FromIterator;
use std::str::FromStr;
use crate::points::Point2D;

const INPUT: &str = include_str!("../input/2019/day3.txt");

pub fn part1() -> i32 {
    let wires: Vec<Path> = parse_input();
    let wire_one: &Vec<Point2D> = &wires[0].locations;
    let wire_two: &Vec<Point2D> = &wires[1].locations;

    let locations_one: HashSet<&Point2D> = HashSet::from_iter(wire_one.iter());
    let locations_two: HashSet<&Point2D> = HashSet::from_iter(wire_two.iter());

    let intersections: HashSet<&&Point2D> = locations_one.intersection(&locations_two).collect();

    let closest: i32 = intersections.iter()
                                    .map(|i| i.manhattan_distance())
                                    .min()
                                    .expect("No intersections found");
    closest
}

pub fn part2() -> usize {
    let wires: Vec<Path> = parse_input();
    let wire_one: &Vec<Point2D> = &wires[0].locations;
    let wire_two: &Vec<Point2D> = &wires[1].locations;

    let locations_one: HashSet<&Point2D> = HashSet::from_iter(wire_one.iter());
    let locations_two: HashSet<&Point2D> = HashSet::from_iter(wire_two.iter());

    let intersections: HashSet<&&Point2D> = locations_one.intersection(&locations_two).collect();

    let closest = intersections.iter()
                               .map(|i| wire_one.iter().position(|x| x == **i).unwrap() + 1 + wire_two.iter().position(|x| x == **i).unwrap() + 1)
                               .min()
                               .expect("No intersections found");
    closest
}

fn parse_input() -> Vec<Path> {
    INPUT.trim().lines().map(|l| l.parse().unwrap()).collect()
}

struct Path {
    locations: Vec<Point2D>
}

impl FromStr for Path {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s.split(',').map(|m| m.parse::<Move>().unwrap()).collect();
        Ok(Path::new(&moves))
    }
}

impl Path {
    fn new(moves: &Vec<Move>) -> Path {
        let mut locations = Vec::new();
        let mut current = Point2D::zero();

        for m in moves {
            for _ in 0..m.steps {
                current += m.direction.delta();
                locations.push(current);
            }
        }

        Path { locations }
    }
}

struct Move {
    direction: Direction,
    steps: u32
}

impl FromStr for Move {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = s[0..1].parse()?;
        let steps = s[1..].parse()?;

        Ok(Move { direction, steps })
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn delta(&self) -> Point2D {
        match *self {
            Direction::Up => Point2D::new(0, -1),
            Direction::Down => Point2D::new(0, 1),
            Direction::Left => Point2D::new(-1, 0),
            Direction::Right => Point2D::new(1, 0)
        }
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(format!("Unable to parse direction {}", s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day03_part1() {
        assert_eq!(part1(), 1285);
    }

    #[test]
    fn day03_part2() {
        assert_eq!(part2(), 14228);
    }
}
