use crate::intcode::{IntCodeEmulator, YieldReason};
use itertools::Itertools;
use std::collections::VecDeque;

const INPUT: &str = include_str!("../input/2019/day7.txt");

pub fn part1() -> i64 {
    let program = IntCodeEmulator::parse_input(INPUT);
    let permutations = (0..5).permutations(5);

    let max = permutations
        .map(|p| run_in_series(&program, &p))
        .max()
        .expect("Unable to run IntCode VMs");

    max
}

pub fn part2() -> i64 {
    let program = IntCodeEmulator::parse_input(INPUT);
    let permutations = (5..10).permutations(5);
    let max = permutations
        .map(|p| run_feedback_loop(&program, &p))
        .max()
        .expect("Unable to run IntCode VMs");

    max
}

/// runs intcode VMs with the given IDs in series and returns the output from the final one
fn run_in_series(program: &Vec<i64>, ids: &[i64]) -> i64 {
    ids.iter().fold(0, |current, &id| {
        let mut vm = IntCodeEmulator::new(program.clone());
        vm.stdin().push_back(id);
        vm.stdin().push_back(current);

        vm.execute();

        let output = vm.stdout().pop_back().expect("No output produced");
        output
    })
}

/// runs intcode VMs with the given IDs in a feedback loop until none require any more input
/// then returns the final result
fn run_feedback_loop(program: &Vec<i64>, ids: &[i64]) -> i64 {
    let mut waiting: VecDeque<IntCodeEmulator> = ids
        .iter()
        .map(|&id| {
            let mut vm = IntCodeEmulator::new(program.clone());
            vm.stdin().push_back(id);
            vm
        })
        .collect();

    let mut value = 0;

    while let Some(mut vm) = waiting.pop_front() {
        vm.stdin().push_back(value);

        let result = vm.execute_until_yield();
        value = vm.stdout().pop_back().expect("No output produced");

        if let YieldReason::InputRequired = result {
            waiting.push_back(vm);
        }
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day07_part1() {
        assert_eq!(part1(), 272368);
    }

    #[test]
    fn day07_part2() {
        assert_eq!(part2(), 19741286);
    }
}
