use std::collections::HashMap;

advent_of_code::solution!(11);

fn blink_in_place(stones: &mut Vec<u64>, cache: &mut HashMap<u64, usize>) {
    let mut stones_to_insert = Vec::new();
    {
        let mut stones_iter = stones.iter_mut().enumerate();

        while let Some((i, stone)) = stones_iter.next() {
            if *stone == 0 {
                *stone = 1;
            } else if ((*stone as f64).log10().floor() + 1.) as u64 % 2 == 0 {
                let original_stone = *stone;
                let num_digits = (*stone as f64).log10().floor() as u32 + 1;

                *stone = original_stone / (10u64.pow(num_digits / 2));
                stones_to_insert.push((i + 1, original_stone % (10u64.pow(num_digits / 2))));
            } else {
                *stone *= 2024;
            }
        }
    }

    let mut offset = 0;
    for (i, stone) in stones_to_insert {
        stones.insert(i + offset, stone);
        offset += 1;
    }
}

fn blink(stone: u64) -> Vec<u64> {
    let mut new_stones = Vec::new();

    if stone == 0 {
        new_stones.push(1);
    } else if ((stone as f64).log10().floor() + 1.) as u64 % 2 == 0 {
        let num_digits = (stone as f64).log10().floor() as u32 + 1;

        new_stones.push(stone / (10u64.pow(num_digits / 2)));
        new_stones.push(stone % (10u64.pow(num_digits / 2)))
    } else {
        new_stones.push(stone * 2024);
    }

    new_stones
}

fn run(input: &str, n: u32) -> usize {
    let stones: Vec<u64> = input
        .lines()
        .nth(0)
        .unwrap()
        .split(' ')
        .map(|stone| stone.parse::<u64>().unwrap())
        .collect();

    let mut cache = HashMap::<u64, HashMap<u32, Vec<u64>>>::new();

    let mut to_visit = stones.iter().copied().rev().collect::<Vec<u64>>();

    let mut stones_count = 0;
    while let Some(outer_stone) = to_visit.pop() {
        let mut depth_to_visit = vec![outer_stone];

        let mut current_path = Vec::<u64>::new();

        let mut depth = 0;
        while let Some(stone) = depth_to_visit.pop() {
            current_path.push(stone);

            if depth > n {
                break;
            }

            if let Some(cached_traversal) = cache.get_mut(&outer_stone) {
                let max_depth = n - depth;
                let mut highest_depth = depth;
                for (cached_depth, _) in cached_traversal.iter() {
                    let found_depth = highest_depth.max(*cached_depth).min(max_depth);
                    if found_depth < max_depth {
                        highest_depth = found_depth;
                    }
                }

                if let Some(cached_result) = cached_traversal.get(&highest_depth) {
                    depth = highest_depth;
                    depth_to_visit.extend(cached_result);
                } else {
                    let blink_result = blink(stone);
                    depth_to_visit.extend(blink_result.iter());
                    cached_traversal.insert(depth, blink_result);
                }
            } else {
                let blink_result = blink(stone);
                depth_to_visit.extend(blink_result.iter());
                cache.insert(outer_stone, HashMap::from([(depth, blink_result)]));
            }

            depth += 1
        }
    }

    println!("{:#?}", cache);

    for stone in stones.iter() {
        stones_count += cache[stone][&n].len();
    }

    stones_count
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
        assert_eq!(result, None);
    }
}
