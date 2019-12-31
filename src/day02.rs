use crate::intcode::IntCodeEmulator;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.trim().split(',').map(|l| l.parse().expect("Unable to parse input")).collect()
}

#[aoc(day2, part1)]
pub fn part1(program: &Vec<i64>) -> i64 {
    let mut program = program.clone();
    program[1] = 12;
    program[2] = 2;

    let mut vm = IntCodeEmulator::new(program);
    vm.execute();

    let result = vm.ram()[0];
    result
}

#[aoc(day2, part2)]
pub fn part2(_program: &Vec<i64>) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2019/day2.txt");

    #[test]
    fn day02_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 6627023);
    }

    #[test]
    fn day02_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 4019);
    }
}
