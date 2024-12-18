use std::{collections::HashMap, iter::zip};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            (split.next().unwrap(), split.next().unwrap())
        })
        .map(|row| (row.0.parse::<i32>().unwrap(), row.1.parse::<i32>().unwrap()))
        .unzip();

    left.sort();
    right.sort();

    Some(zip(left, right).fold(0, |acc, (l, r)| acc + (r - l).abs()) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (left, right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            (split.next().unwrap(), split.next().unwrap())
        })
        .map(|row| (row.0.parse::<i32>().unwrap(), row.1.parse::<i32>().unwrap()))
        .unzip();

    let mut cache: HashMap<i32, usize> = HashMap::new();
    let mut score = 0;
    for id in left.iter() {
        let count = match cache.get(id) {
            Some(count) => *count,
            None => {
                let count = right.iter().filter(|rid| **rid == *id).count();
                cache.insert(*id, count);
                count
            }
        };
        score += *id as usize * count;
    }

    Some(score as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
