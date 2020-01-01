use crate::intcode::IntCodeEmulator;

const INPUT: &str = include_str!("../input/2019/day2.txt");

pub fn parse_input() -> Vec<i64> {
    INPUT.trim().split(',').map(|l| l.parse().expect("Unable to parse input")).collect()
}

pub fn part1() -> i64 {
    let input = parse_input();
    run(&input, 12, 2)
}

pub fn part2() -> i64 {
    let input = parse_input();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let output = run(&input, noun, verb);

            if output == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    panic!("Correct noun/verb combo not found");
}

fn run(input: &Vec<i64>, noun: i64, verb: i64) -> i64 {
    let mut program = input.clone();
    program[1] = noun;
    program[2] = verb;

    let mut vm = IntCodeEmulator::new(program);
    vm.execute();

    let result = vm.ram()[0];
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02_part1() {
        assert_eq!(part1(), 6627023);
    }

    #[test]
    fn day02_part2() {
        assert_eq!(part2(), 4019);
    }
}
