use std::collections::{HashMap, HashSet};
use std::f64::consts::PI;

use itertools::Itertools;

use crate::points::Point2D;

const INPUT: &str = include_str!("../input/2019/day10.txt");
const ROTATION: f64 = PI / 2.0;
const FULL_CIRCLE: f64 = PI * 2.0;
const ACCURACY: f64 = 100_000_000.0; // vectors are accurate to 8dp

pub fn part1() -> usize {
    let asteroids = parse_input();

    asteroids.iter()
             .map(|origin| visible_asteroids(origin, &asteroids))
             .map(|visible| visible.len())
             .max()
             .expect("Unable to find visible asteroids")
}

pub fn part2() -> i32 {
    let asteroids = parse_input();
    let base = Point2D::new(20, 18); // starting position worked out from part 1

    // group asteroids by their vector from the base
    let vectors: HashMap<u64, Vec<&Point2D>> = asteroids.iter()
                                                        .map(|asteroid| (vector(&base, asteroid), asteroid))
                                                        .into_iter()
                                                        .into_group_map();

    let mut destroyed = HashSet::with_capacity(200);

    // destroy the closest asteroid in each vector group, starting at due-north and looping round clockwise
    for vector in vectors.keys().sorted().cycle() {
        let remaining: Vec<&Point2D> = vectors[vector].iter()
                                                      .filter(|&asteroid| !destroyed.contains(asteroid))
                                                      .map(|x| *x)
                                                      .collect();

        if remaining.len() == 0 {
            continue;
        }

        let closest = remaining.iter()
                               .min_by_key(|target| (target.x - base.x).abs() + (target.y - base.y).abs())
                               .unwrap();
        destroyed.insert(*closest);

        if destroyed.len() == 200 {
            return closest.x * 100 + closest.y;
        }
    }

    panic!("Didn't find 200 asteroids to destroy");
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

        if !vectors.contains(&vector) {
            visible.insert(destination);
        }

        vectors.insert(vector);
    }

    visible
}

/// calculate the vector in radians from the origin point to the destination point from a north bearing
/// returns the result in radians to 8dp as u64 because f64 can't be used in HashSet
fn vector(origin: &Point2D, dest: &Point2D) -> u64
{
    let delta = *origin - *dest; // x and y are flipped for this grid so this appears backwards but it's not
    let dx = delta.x as f64;
    let dy = delta.y as f64;
    let vector = dy.atan2(dx) - ROTATION; // atan2 is from due-east, so rotate to due-north

    if vector < 0.0 {
        // vectors on the 'left' half of the circle will be negative radians, so add 1 full loop to make them always positive
        ((vector + FULL_CIRCLE) * ACCURACY) as u64
    }
    else {
        (vector * ACCURACY) as u64
    }
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
