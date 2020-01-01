const INPUT: &str = include_str!("../input/2019/day1.txt");

pub fn parse_input() -> Vec<i32> {
    INPUT.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn part1() -> i32 {
    let input = parse_input();
    input.iter().map(|n| (n / 3) - 2).sum()
}

pub fn part2() -> i32 {
    let input = parse_input();
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

    #[test]
    fn day01_part1() {
        assert_eq!(part1(), 3305301);
    }

    #[test]
    fn day01_part2() {
        assert_eq!(part2(), 4955106);
    }
}
