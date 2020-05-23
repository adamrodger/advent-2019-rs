use crate::intcode::IntCodeEmulator;

const INPUT: &str = include_str!("../input/2019/day9.txt");

pub fn part1() -> i64 {
    let mut vm = IntCodeEmulator::from_input(INPUT);
    vm.stdin().push_back(1);

    vm.execute();

    if *vm.stdout().back().unwrap() == 0 {
        panic!("Failed with output: {:?}", vm.stdout());
    }

    vm.stdout().pop_back().expect("No output produced")
}

pub fn part2() -> i64 {
    let mut vm = IntCodeEmulator::from_input(INPUT);
    vm.stdin().push_back(2);

    vm.execute();

    vm.stdout().pop_back().expect("No output produced")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day09_part1() {
        assert_eq!(part1(), 4_006_117_640);
    }

    #[test]
    fn day09_part2() {
        assert_eq!(part2(), 88231);
    }
}
