use regex::*;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<i64> {
    let pattern = Regex::new(r"mul\((?<lhs>\d+)\,(?<rhs>\d+)\)").unwrap();
    let result = pattern
        .captures_iter(input)
        .filter_map(|c| match (c.name("lhs"), c.name("rhs")) {
            (Some(lhs), Some(rhs)) => Some((lhs.as_str(), rhs.as_str())),
            _ => None,
        })
        .filter_map(
            |(lhs, rhs)| match (lhs.parse::<i64>(), rhs.parse::<i64>()) {
                (Ok(lhs), Ok(rhs)) => Some((lhs, rhs)),
                _ => None,
            },
        )
        .fold(0, |acc, (lhs, rhs)| acc + lhs * rhs);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
