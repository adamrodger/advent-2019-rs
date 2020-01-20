use crate::intcode::{IntCodeEmulator, YieldReason};
use crate::points::{Bearing, Direction, Point2D};
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/2019/day11.txt");

#[derive(PartialEq)]
enum Colour {
    Black = 0,
    White = 1
}

pub fn part1() -> usize {
    let colours = run_program(Colour::Black);
    colours.keys().len()
}

pub fn part2() -> String {
    let colours = run_program(Colour::White);
    let painted = colours.iter().filter(|p| p.1 == &Colour::White).map(|p| p.0).collect::<Vec<&Point2D>>();

    let width: usize = painted.iter().map(|p| p.x as usize).max().unwrap() + 1 + 1; // add one for line breaks
    let height: usize = painted.iter().map(|p| p.y as usize).max().unwrap();

    let mut chars = vec![' '; (height + 1) * width - 1];

    for panel in painted {
        let index = panel.y as usize * width + panel.x as usize;
        chars[index] = '█';
    }

    for y in 0..height {
        chars[y * width + width - 1] = '\n';
    }

    chars.into_iter().collect()
}

fn run_program(starting_colour: Colour) -> HashMap<Point2D, Colour> {
    let mut vm = IntCodeEmulator::from_input(INPUT);

    let mut position = Point2D::zero();
    let mut bearing = Bearing::North;

    let mut colours = HashMap::new();
    colours.insert(position, starting_colour);

    loop {
        let colour = colours.entry(position).or_insert(Colour::Black);
        match colour {
            Colour::Black => vm.stdin().push_back(0),
            Colour::White => vm.stdin().push_back(1),
        };

        // tell the VM about the current square
        let result = vm.execute_until_yield();
        if result == YieldReason::Halted {
            break;
        }

        // paint the square
        let paint = vm.stdout().pop_front().expect("No colour available");
        let paint = match paint {
            0 => Colour::Black,
            1 => Colour::White,
            _ => panic!("Unexpected colour: {}", paint)
        };

        colours.insert(position, paint);

        // move to the next square
        let direction = vm.stdout().pop_front().expect("No direction available");
        let direction = match direction {
            0 => Direction::Left,
            1 => Direction::Right,
            _ => panic!("Unexpected direction: {}", direction)
        };

        bearing = bearing.turn(&direction);
        position = position.move_bearing(&bearing);
    }

    colours
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_part1() {
        assert_eq!(part1(), 1907);
    }

    #[test]
    fn day11_part2() {
        let expected = vec!["  ██  ███  ████ █  █ ████  ██  ████  ██ ",
                            " █  █ █  █ █    █ █     █ █  █ █    █  █",
                            " █  █ ███  ███  ██     █  █    ███  █   ",
                            " ████ █  █ █    █ █   █   █ ██ █    █ ██",
                            " █  █ █  █ █    █ █  █    █  █ █    █  █",
                            " █  █ ███  ████ █  █ ████  ███ █     ███"];

        assert_eq!(part2(), expected.join("\n"));
    }
}
