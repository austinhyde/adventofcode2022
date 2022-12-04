use std::{ops::RangeInclusive, str::FromStr};

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 Answer: {}", part1(input));
    println!("Part 2 Answer: {}", part2(input));
}

/** Part 1:

*/
fn part1(input: &str) -> usize {
    input.lines()
        .map(|l| l.parse::<Pair>().unwrap())
        .filter(|p| p.has_full_overlap())
        .count()
}

struct Pair(Assignment, Assignment);
impl FromStr for Pair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(",").ok_or(s.to_owned())?;
        Ok(Pair(a.parse()?, b.parse()?))
    }
}
impl Pair {
    fn has_full_overlap(&self) -> bool {
        self.0.is_contained_by(&self.1) || self.1.is_contained_by(&self.0)
    }

    fn has_any_overlap(&self) -> bool {
        self.0.overlaps(&self.1) || self.1.overlaps(&self.0)
    }
}


struct Assignment(RangeInclusive<u32>);
impl FromStr for Assignment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once("-").ok_or(s.to_owned())?;
        Ok(Self(RangeInclusive::new(a.parse().or(Err(a.to_owned()))?, b.parse().or(Err(b.to_owned()))?)))
    }
}
impl Assignment {
    fn is_contained_by(&self, other: &Assignment) -> bool {
        self.0.start() >= other.0.start() && self.0.end() <= other.0.end()
    }
    fn overlaps(&self, other: &Assignment) -> bool {
        other.0.contains(self.0.start()) || other.0.contains(self.0.end())
    }
}

/** Part 2:

*/
fn part2(input: &str) -> usize {
    input.lines()
        .map(|l| l.parse::<Pair>().unwrap())
        .filter(|p| p.has_any_overlap())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = include_str!("example1.txt");
        let expected = 2;
        assert_eq!(expected, part1(input));
    }

    #[test]
    fn part2_example1() {
        let input = include_str!("example1.txt");
        let expected = 4;
        assert_eq!(expected, part2(input));
    }
}