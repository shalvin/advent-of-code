advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    let line_max = input.len();
    let char_max = input[1].len();

    let search_word = "XMAS";

    let mut match_count = 0;
    for line_idx in 0..line_max {
        for char_idx in 0..char_max {
            let c = input[line_idx].chars().nth(char_idx).unwrap();

            if c != search_word.chars().nth(0).unwrap() {
                continue;
            }

            let mut matches = [true; 8];
            for radius in 1..search_word.len() {
                let (line_idx, char_idx, radius, line_max, char_max) = (
                    line_idx as i64,
                    char_idx as i64,
                    radius as i64,
                    line_max as i64,
                    char_max as i64,
                );
                let (line_lower, char_lower, line_upper, char_upper) = (
                    (line_idx - radius),
                    (char_idx - radius),
                    (line_idx + radius),
                    (char_idx + radius),
                );

                let search_locations = [
                    (line_lower, char_lower),
                    (line_lower, char_idx),
                    (line_lower, char_upper),
                    (line_idx, char_lower),
                    (line_idx, char_upper),
                    (line_upper, char_lower),
                    (line_upper, char_idx),
                    (line_upper, char_upper),
                ];

                for (i, (line_loc, char_loc)) in search_locations.into_iter().enumerate() {
                    if !matches[i] {
                        continue;
                    }

                    if line_loc < 0
                        || char_loc < 0
                        || line_loc > line_max - 1
                        || char_loc > char_max - 1
                    {
                        matches[i] = false;
                        continue;
                    }

                    let c = input[line_loc as usize]
                        .chars()
                        .nth(char_loc as usize)
                        .unwrap();
                    if search_word.chars().nth(radius as usize).unwrap() != c {
                        matches[i] = false;
                    }
                }
            }

            match_count += matches.into_iter().filter(|m| *m).count();
        }
    }

    Some(match_count as u64)
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
