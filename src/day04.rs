const START: u32 = 347_312;
const END: u32 = 805_915 + 1; // +1 so we can use exclusive ranges which are much more performant

pub fn part1() -> usize {
    (START..END)
        .map(digits)
        .filter(|digits| is_in_order(digits) && has_multiple(digits))
        .count()
}

pub fn part2() -> usize {
    (START..END)
        .map(digits)
        .filter(|digits| is_in_order(digits) && has_double(digits))
        .count()
}

/// splits a number into its digits
fn digits(mut n: u32) -> [usize; 6] {
    let mut digits = [0; 6];

    for i in (0..6).rev() {
        digits[i] = (n % 10) as usize;
        n /= 10;
    }

    digits
}

/// makes sure the digits are in ascending order
fn is_in_order(digits: &[usize; 6]) -> bool {
    for i in 1..6 {
        if digits[i] < digits[i - 1] {
            return false;
        }
    }

    true
}

/// makes sure there's at least one repeated digit in the number
fn has_multiple(digits: &[usize; 6]) -> bool {
    for i in 1..6 {
        if digits[i] == digits[i - 1] {
            return true;
        }
    }

    false
}

/// makes sure there's a group of exactly 2 matching digits in the number
fn has_double(digits: &[usize; 6]) -> bool {
    let mut consecutive = 1;

    for i in 1..6 {
        if digits[i] == digits[i - 1] {
            consecutive += 1;
        } else if consecutive == 2 {
            return true;
        } else {
            consecutive = 1;
        }
    }

    consecutive == 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day04_part1() {
        assert_eq!(part1(), 594);
    }

    #[test]
    fn day04_part2() {
        assert_eq!(part2(), 364);
    }
}
