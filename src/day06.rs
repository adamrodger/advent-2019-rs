use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input/2019/day6.txt");

pub fn part1() -> usize {
    let planets: HashMap<String, Planet> = INPUT
        .lines()
        .map(Planet::new)
        .map(|p| (p.id.clone(), p))
        .collect();

    planets
        .iter()
        .map(|p| p.1.steps_to_root(&planets).len())
        .sum()
}

pub fn part2() -> usize {
    let planets: HashMap<String, Planet> = INPUT
        .lines()
        .map(Planet::new)
        .map(|p| (p.id.clone(), p))
        .collect();

    let you = planets.get("YOU").expect("Unable to find planet YOU");
    let santa = planets.get("SAN").expect("Unable to find planet SAN");

    let you_path: HashSet<&Planet> = you.steps_to_root(&planets).into_iter().collect();
    let santa_path: HashSet<&Planet> = santa.steps_to_root(&planets).into_iter().collect();

    // by subtracting the common planets we'll be left with a path to a common
    // branch point, then we can count steps from YOU/SAN to common
    let common_planets = you_path.intersection(&santa_path).count();

    // -2 because the YOU and SAN planets themselves don't count
    (you_path.len() - common_planets) + (santa_path.len() - common_planets) - 2
}

#[derive(Hash, Eq, PartialEq)]
struct Planet {
    id: String,
    orbiting: String,
}

impl Planet {
    pub fn new(s: &str) -> Self {
        let mut split = s.split(')');
        Planet {
            orbiting: split.next().expect("No parent planet found").to_owned(),
            id: split.next().expect("No planet ID found").to_owned(),
        }
    }

    /// count the number of steps it takes to get back to route in a naive but good-enough way - i.e. totally unmemoized
    pub fn steps_to_root<'a>(&'a self, planets: &'a HashMap<String, Planet>) -> Vec<&'a Planet> {
        let mut orbits = vec![self];
        let mut parent = self.orbiting.clone();

        while let Some(p) = planets.get(&parent) {
            orbits.push(p);
            parent = p.orbiting.clone();
        }

        orbits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day06_part1() {
        assert_eq!(part1(), 254447);
    }

    #[test]
    fn day06_part2() {
        assert_eq!(part2(), 445);
    }
}
