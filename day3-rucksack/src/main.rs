use std::fmt::{Debug, Write};
use std::hash::Hash;
use std::{str::FromStr, collections::HashSet};

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 Answer: {}", part1(input));
    println!("Part 2 Answer: {}", part2(input));
}

/** Part 1:
    Each rucksack has two large compartments. All items of a given type are meant to go into exactly one of the two compartments.

    Every item type is identified by a single lowercase or uppercase letter (that is, a and A refer to different types of items).

    The list of items for each rucksack is given as characters all on a single line. A given rucksack always has the same number
    of items in each of its two compartments, so the first half of the characters represent items in the first compartment,
    while the second half of the characters represent items in the second compartment.

    Lowercase item types a through z have priorities 1 through 26.
    Uppercase item types A through Z have priorities 27 through 52.

    Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?
*/
fn part1(input: &str) -> u32 {
    input.lines()
        .map(|l| l.parse::<Rucksack>().unwrap())
        .map(|r| {
            let items = r.get_mispacked_items();
            if items.len() != 1 {
                panic!("Found {} items in rucksack {:?}: {:?}", items.len(), r, items);
            }
            items.first().unwrap().priority()
        })
        .sum()
}

struct Rucksack(Compartment, Compartment);
struct Compartment(HashSet<Item>);

#[derive(PartialEq, Eq, Hash, Clone)]
struct Item(char);

impl FromStr for Rucksack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_at(s.len()/2);
        Ok(Self(a.parse()?, b.parse()?))
    }
}

impl FromStr for Compartment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().map(|c| Item(c)).collect()))
    }
}

impl Debug for Compartment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|i| i.0).collect::<String>())
    }
}

impl Rucksack {
    fn get_mispacked_items(&self) -> Vec<&Item> {
        self.0.0.intersection(&self.1.0).collect()
    }

    fn get_all_items(&self) -> HashSet<Item> {
        &self.0.0 | &self.1.0
    }
}

impl Debug for Rucksack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = self.get_all_items().iter().map(|i| i.0).collect::<String>();
        write!(f, "Rucksack{{{:?}/{:?}}} ({})", self.0, self.1, items)
    }
}

impl Item {
    fn priority(&self) -> u32 {
        match self.0 {
            x @ 'a'..='z' => (x as u32 - 'a' as u32) + 1,
            x @ 'A'..='Z' => (x as u32 - 'A' as u32) + 27,
            _ => 0
        }
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.0)
    }
}

/** Part 2:
    For safety, the Elves are divided into groups of three. Every Elf carries a
    badge that identifies their group. For efficiency, within each group of
    three Elves, the badge is the only item type carried by all three Elves.

    The only way to tell which item type is the right one is by finding the one
    item type that is common between all three Elves in each group.

    Every set of three lines in your list corresponds to a single group,
    but each group can have a different badge item type.

    Find the item type that corresponds to the badges of each three-Elf group. What is the sum of the priorities of those item types?
*/
fn part2(input: &str) -> u32 {
    input.lines()
        .map(|l| l.parse::<Rucksack>().unwrap())
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|group| {
            group.iter()
                .map(|r| r.get_all_items())
                .reduce(|a, b| &a & &b)
                .unwrap()
                .iter()
                .map(Item::priority)
                .sum::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priority() {
        assert_eq!(1, Item('a').priority());
        assert_eq!(26, Item('z').priority());
        assert_eq!(27, Item('A').priority());
        assert_eq!(52, Item('Z').priority());
    }

    #[test]
    fn part1_example1() {
        let input = include_str!("example1.txt");
        let expected = 157;
        assert_eq!(expected, part1(input));
    }

    #[test]
    fn part2_example1() {
        let input = include_str!("example1.txt");
        let expected = 70;
        assert_eq!(expected, part2(input));
    }
}