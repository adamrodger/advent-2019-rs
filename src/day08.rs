const INPUT: &str = include_str!("../input/2019/day8.txt");

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

const BLACK: u32 = 0;
const WHITE: u32 = 1;
const TRANSPARENT: u32 = 2;

pub fn part1() -> usize {
    let digits = parse_input();

    let layer = digits.chunks_exact(WIDTH * HEIGHT)
                      .min_by_key(|layer| count_chars(layer, 0))
                      .expect("Unable to parse layers");

    count_chars(layer, 1) * count_chars(layer, 2)
}

pub fn part2() -> String {
    let digits = parse_input();

    let mut output = String::with_capacity(WIDTH * HEIGHT + HEIGHT - 1); // extra height for newline chars

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let cell = digits.chunks_exact(WIDTH * HEIGHT)
                             .map(|layer| layer[y * WIDTH + x]) // extract the correct cell from each layer
                             .find(|&c| c != TRANSPARENT)       // same as First() in C# Linq
                             .map(|c| match c {
                                 BLACK => ' ',
                                 WHITE => '#',
                                 _ => panic!("Unexpected character")
                             })
                             .expect("No non-transparent pixel found");
            output.push(cell);
        }

        if y < HEIGHT - 1 {
            output.push('\n');
        }
    }

    output
}

fn parse_input() -> Vec<u32> {
    INPUT.trim()
         .chars()
         .map(|c| c.to_digit(10).expect("Unable to parse digit"))
         .collect()
}

fn count_chars(layer: &[u32], search: u32) -> usize {
    layer.iter().filter(|&c| *c == search).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day08_part1() {
        assert_eq!(part1(), 1690);
    }

    #[test]
    fn day08_part2() {
        // ZPZUB
        let expected = vec!["#### ###  #### #  # ###  ",
                            "   # #  #    # #  # #  # ",
                            "  #  #  #   #  #  # ###  ",
                            " #   ###   #   #  # #  # ",
                            "#    #    #    #  # #  # ",
                            "#### #    ####  ##  ###  "];

        assert_eq!(part2(), expected.join("\n"));
    }
}
