use std::collections::BinaryHeap;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 Answer: {}", part1(input));
    println!("Part 2 Answer: {}", part2(input));
}

/** Part 1:
    In case the Elves get hungry and need extra snacks,
    they need to know which Elf to ask: they'd like to know how many
    Calories are being carried by the Elf carrying the most Calories.
    In the example above, this is 24000 (carried by the fourth Elf).
*/
fn part1(input: &str) -> u32 {
    input.lines().fold((0, 0), |(curr, max), line| {
        if line != "" {
            (curr + line.parse::<u32>().unwrap(), max)
        } else {
            (0, max.max(curr))
        }
    }).1
}

/** Part 2:
    Find the top three Elves carrying the most Calories.
    How many Calories are those Elves carrying in total?
*/
fn part2(input: &str) -> u32 {
    let mut heap = BinaryHeap::new();
    let mut curr = 0;
    for line in input.lines() {
        if line == "" {
            heap.push(curr);
            curr = 0;
        } else {
            curr += line.parse::<u32>().unwrap();
        }
    }
    heap.push(curr);
    return heap.pop().unwrap()+heap.pop().unwrap()+heap.pop().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = include_str!("example1.txt");
        let expected = 24000;
        assert_eq!(expected, part1(input));
    }

    #[test]
    fn part2_example1() {
        let input = include_str!("example1.txt");
        let expected = 45000;
        assert_eq!(expected, part2(input));
    }
}