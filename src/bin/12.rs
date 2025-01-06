advent_of_code::solution!(12);
use std::collections::HashSet;

#[derive(Debug)]
struct Region {
    points: HashSet<(usize, usize)>,
}

type Map = Vec<Vec<char>>;

fn find_area(map: &Map, x: usize, y: usize, label: char) -> HashSet<(usize, usize)> {
    let mut to_visit: Vec<(usize, usize)> = vec![(x, y)];
    let mut visited = HashSet::<(usize, usize)>::new();

    while let Some((x, y)) = to_visit.pop() {
        visited.insert((x, y));

        let neighbours = [
            (x + 1, y),
            (x, y + 1),
            (x.wrapping_sub(1), y),
            (x, y.wrapping_sub(1)),
        ]
        .into_iter()
        .filter_map(|(nx, ny)| {
            if let Some(neighbour) = map.get(ny).and_then(|row| row.get(nx)) {
                if !visited.contains(&(nx, ny)) && *neighbour == label {
                    Some((nx, ny))
                } else {
                    None
                }
            } else {
                None
            }
        });

        to_visit.extend(neighbours);
    }

    visited
}

pub fn part_one(input: &str) -> Option<u64> {
    let char_map: Map = input.lines().map(|line| line.chars().collect()).collect();

    let mut regions: Vec<Region> = Vec::new();
    for (y, line) in char_map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            let region_exists = regions.iter().any(|region| region.points.contains(&(x, y)));

            if region_exists {
                continue;
            } else {
                let new_region = Region {
                    points: find_area(&char_map, x, y, *c),
                };

                regions.push(new_region);
            }
        }
    }

    let mut total_price = 0;
    for region in regions.iter() {
        let perimeter = region.points.iter().fold(0, |acc, (x, y)| {
            acc + [
                (*x + 1, *y),
                (*x, *y + 1),
                (x.wrapping_sub(1), *y),
                (*x, y.wrapping_sub(1)),
            ]
            .into_iter()
            .filter(|point| !region.points.contains(point))
            .count()
        });
        let area = region.points.len();

        total_price += perimeter * area;
    }

    Some(total_price as u64)
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
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
