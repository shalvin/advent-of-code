use std::collections::HashMap;

advent_of_code::solution!(11);

type BlinkCache = HashMap<(u32, u64), usize>;

fn blink_recurse(n: u32, stone: u64, mut cache: &mut BlinkCache) -> usize {
    if n == 0 {
        return 1;
    }

    if let Some(cached_count) = cache.get(&(n, stone)) {
        return *cached_count;
    }

    let count = if stone == 0 {
        blink_recurse(n - 1, 1, &mut cache)
    } else if ((stone as f64).log10().floor() + 1.) as u64 % 2 == 0 {
        let num_digits = (stone as f64).log10().floor() as u32 + 1;

        blink_recurse(n - 1, stone / (10u64.pow(num_digits / 2)), &mut cache)
            + blink_recurse(n - 1, stone % (10u64.pow(num_digits / 2)), &mut cache)
    } else {
        blink_recurse(n - 1, stone * 2024, &mut cache)
    };

    cache.insert((n, stone), count);

    count
}

fn run(input: &str, n: u32) -> usize {
    let stones: Vec<u64> = input
        .lines()
        .nth(0)
        .unwrap()
        .split(' ')
        .map(|stone| stone.parse::<u64>().unwrap())
        .collect();

    let mut cache = BlinkCache::new();

    let stone_count = stones
        .iter()
        .fold(0, |acc, stone| acc + blink_recurse(n, *stone, &mut cache));

    stone_count as usize
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(run(input, 25) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(run(input, 75) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }
}
