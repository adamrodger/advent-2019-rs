pub fn part1(input: &str) -> u32 {
    let modules = input.lines().map(|l| l.parse::<u32>().unwrap());
    modules.map(|n| (n / 3) - 2).sum()
}

pub fn part2(input: &str) -> i32 {
    let modules = input.lines().map(|l| l.parse::<i32>().unwrap());
    let mut total = 0;

    for module in modules {
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
    #[test]
    fn part1() {
        let input = include_str!("../inputs/day01.txt");
        assert_eq!(super::part1(&input), 3305301);
    }

    #[test]
    fn part2() {
        let input = include_str!("../inputs/day01.txt");
        assert_eq!(super::part2(&input), 4955106);
    }
}
