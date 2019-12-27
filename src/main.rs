mod day01;

fn main() {
    let contents = include_str!("../inputs/day01.txt");

    println!("Day 1 - Part 1 - {}", day01::part1(&contents));
    println!("Day 1 - Part 2 - {}", day01::part2(&contents));
}
