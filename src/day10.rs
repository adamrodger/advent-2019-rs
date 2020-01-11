use crate::points::Point2D;
use std::collections::HashSet;

const INPUT: &str = include_str!("../input/2019/day10.txt");

pub fn part1() -> usize {
    let asteroids = parse_input();

    asteroids.iter()
             .map(|origin| visible_asteroids(origin, &asteroids))
             .map(|visible| visible.len())
             .max()
             .expect("Unable to find visible asteroids")
}

pub fn part2() -> usize {
    unimplemented!();
}

/// parse the locations of all asteriods in the input
fn parse_input() -> Vec<Point2D> {
    let mut points = Vec::new();

    for (y, line) in INPUT.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                points.push(Point2D::new(x as i32, y as i32));
            }
        }
    }

    points
}

/// gets the collection of all visible asteroids from the origin asteroid
fn visible_asteroids<'a>(origin: &'a Point2D, asteroids: &'a [Point2D]) -> HashSet<&'a Point2D>{
    let mut visible: HashSet<&Point2D> = HashSet::new();
    let mut vectors: HashSet<u64> = HashSet::new();

    for destination in asteroids.iter() {
        if origin == destination {
            continue;
        }

        let vector = vector(origin, destination);
        let vector = (vector * 1024.0 * 1024.0) as u64; // hack because f64 can't be used in HashSet

        if !vectors.contains(&vector) {
            visible.insert(destination);
        }

        vectors.insert(vector);
    }

    visible
}

/// calculate the vector in radians from the origin point to the destination point
fn vector(origin: &Point2D, dest: &Point2D) -> f64
{
    let delta = *dest - *origin;
    let dx = delta.x as f64;
    let dy = delta.y as f64;
    dy.atan2(dx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_part1() {
        assert_eq!(part1(), 280);
    }

    #[test]
    fn day10_part2() {
        assert_eq!(part2(), 706);
    }
}
