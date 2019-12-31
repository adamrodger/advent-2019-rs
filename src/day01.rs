#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    input.iter().map(|n| (n / 3) - 2).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    let mut total = 0;

    for module in input.iter() {
        let mut fuel = (module / 3) - 2;

        while fuel > 0 {
            total += fuel;
            fuel = (fuel / 3) - 2;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2019/day1.txt");

    #[test]
    fn day01_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 3305301);
    }

    #[test]
    fn day01_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 4955106);
    }
}
