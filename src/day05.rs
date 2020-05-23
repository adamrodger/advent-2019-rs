use crate::intcode::IntCodeEmulator;

const INPUT: &str = include_str!("../input/2019/day5.txt");

pub fn part1() -> i64 {
    let mut vm = IntCodeEmulator::from_input(INPUT);
    vm.stdin().push_back(1);

    vm.execute();

    let result = vm
        .stdout()
        .iter()
        .last()
        .expect("Expected output but received none");
    *result
}

pub fn part2() -> i64 {
    let mut vm = IntCodeEmulator::from_input(INPUT);
    vm.stdin().push_back(5);

    vm.execute();

    let result = vm
        .stdout()
        .iter()
        .last()
        .expect("Expected output but received none");
    *result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day05_part1() {
        assert_eq!(part1(), 7_988_899);
    }

    #[test]
    fn day05_part2() {
        assert_eq!(part2(), 13_758_663);
    }
}
