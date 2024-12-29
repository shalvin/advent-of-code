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

#[derive(Clone, Copy, Debug)]
enum Cmds {
    Do,
    Dont,
    Mul(i64, i64),
}

pub fn part_two(input: &str) -> Option<i64> {
    let pattern = Regex::new(r"(?<cmd>[\w\']+)\((?<params>\d+,\d+|)\)").unwrap();

    let cmds: Vec<Cmds> = pattern
        .captures_iter(input)
        .filter_map(|caps| {
            let (_, [cmd, params]) = caps.extract();
            if cmd.ends_with("do") {
                Some(Cmds::Do)
            } else if cmd.ends_with("don't") {
                Some(Cmds::Dont)
            } else if cmd.ends_with("mul") {
                let (lhs, rhs) = params.split_once(",").unwrap();
                Some(Cmds::Mul(
                    lhs.parse::<i64>().unwrap(),
                    rhs.parse::<i64>().unwrap(),
                ))
            } else {
                None
            }
        })
        .collect();

    let mut enabled = true;
    let mut sum = 0;
    for cmd in cmds.iter() {
        use Cmds::*;
        match cmd {
            Do => enabled = true,
            Dont => enabled = false,
            Mul(lhs, rhs) => {
                if enabled {
                    sum += lhs * rhs;
                }
            }
        }
    }

    Some(sum)
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(54));
    }
}
