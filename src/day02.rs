use crate::intcode::IntCodeEmulator;

const INPUT: &str = include_str!("../input/2019/day2.txt");

pub fn input_generator() -> Vec<i64> {
    INPUT.trim().split(',').map(|l| l.parse().expect("Unable to parse input")).collect()
}

pub fn part1(program: &Vec<i64>) -> i64 {
    let mut program = program.clone();
    program[1] = 12;
    program[2] = 2;

    let mut vm = IntCodeEmulator::new(program);
    vm.execute();

    let result = vm.ram()[0];
    result
}

pub fn part2(_program: &Vec<i64>) -> i64 {
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02_part1() {
        assert_eq!(part1(&input_generator()), 6627023);
    }

    #[test]
    fn day02_part2() {
        assert_eq!(part2(&input_generator()), 4019);
    }
}
