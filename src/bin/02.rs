advent_of_code::solution!(2);

fn parse_reports(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|i| i.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}
// [1,2],[2,3],[3,4]
// 1      2     3

fn find_unsafe_index(report: &Vec<i32>) -> Option<usize> {
    let direction = report[1] < report[0];
    report
        .as_slice()
        .windows(2)
        .enumerate()
        .find(|(_, w)| {
            let delta = w[1] - w[0];
            !(delta.abs() >= 1 && delta.abs() <= 3 && (w[1] < w[0]) == direction)
        })
        .map(|(i, _)| i)
}

pub fn part_one(input: &str) -> Option<u64> {
    let reports: Vec<Vec<i32>> = parse_reports(input);

    let safe_reports = reports
        .iter()
        .filter(|report| find_unsafe_index(*report).is_none())
        .count();

    Some(safe_reports as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let reports: Vec<Vec<i32>> = parse_reports(input);

    let mut safe_reports = 0;
    for report in reports.iter() {
        let unsafe_index = find_unsafe_index(report);
        if let Some(idx) = unsafe_index {
            let idx = idx as i64;
            for i in (idx - 1).max(0)..=(idx + 1).min(report.len() as i64 - 1) {
                let mut dampened_report = report.clone();
                dampened_report.remove(i as usize);
                let unsafe_index = find_unsafe_index(&dampened_report);
                if unsafe_index.is_none() {
                    safe_reports += 1;
                    break;
                }
            }
        } else {
            safe_reports += 1;
        }
    }

    Some(safe_reports as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
