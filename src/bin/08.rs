use std::collections::{HashMap, HashSet};

use bevy_math::{IVec2, Vec2};

advent_of_code::solution!(8);

struct Map {
    size: IVec2,
    antennas: HashMap<char, Vec<IVec2>>,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let w = input.lines().nth(0).unwrap().len();
        let h = input.lines().count();

        let mut antenna_map = HashMap::<char, Vec<IVec2>>::new();

        for (line_i, line) in input.lines().enumerate() {
            for (char_i, c) in line.chars().enumerate().filter(|(_, c)| *c != '.') {
                let antenna_pos = IVec2::new(char_i as i32, line_i as i32);

                if let Some(antennas) = antenna_map.get_mut(&c) {
                    antennas.push(antenna_pos);
                } else {
                    antenna_map.insert(c, vec![antenna_pos]);
                }
            }
        }

        Self {
            size: IVec2::new(w as i32, h as i32),
            antennas: antenna_map,
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = Map::from_str(input);
    let pos_zero = IVec2::new(0, 0);

    let mut antinodes = HashSet::<IVec2>::new();

    for (_, antennas) in map.antennas {
        for (i, antenna) in antennas.iter().enumerate() {
            for (_, other_antenna) in antennas.iter().enumerate().filter(|(k, _)| *k != i) {
                let delta = other_antenna - antenna;
                let antinodes_within_bounds = [other_antenna + delta, antenna - delta]
                    .into_iter()
                    .filter(|pos| pos.cmpge(pos_zero).all() && pos.cmplt(map.size).all());

                antinodes.extend(antinodes_within_bounds);
            }
        }
    }

    if false {
        for y in 0..map.size.y {
            let line = (0..map.size.x)
                .into_iter()
                .map(|x| {
                    if antinodes.iter().any(|pos| *pos == IVec2::new(x, y)) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>();

            println!("{}", line);
        }
    }

    Some(antinodes.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = Map::from_str(input);
    let pos_zero = IVec2::new(0, 0);

    let mut antinodes = HashSet::<IVec2>::new();

    for (_, antennas) in map.antennas {
        for (i, antenna) in antennas.iter().enumerate() {
            for (_, other_antenna) in antennas.iter().enumerate().filter(|(k, _)| *k != i) {
                let delta = other_antenna - antenna;

                let mut count = 1;
                loop {
                    let antinodes_within_bounds =
                        [antenna + delta * count, other_antenna - delta * count]
                            .into_iter()
                            .filter(|pos| pos.cmpge(pos_zero).all() && pos.cmplt(map.size).all());

                    if antinodes_within_bounds.clone().count() > 0 {
                        antinodes.extend(antinodes_within_bounds);
                    } else {
                        break;
                    }

                    count += 1;
                }
            }
        }
    }

    if true {
        for y in 0..map.size.y {
            let line = (0..map.size.x)
                .into_iter()
                .map(|x| {
                    if antinodes.iter().any(|pos| *pos == IVec2::new(x, y)) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>();

            println!("{}", line);
        }
    }

    Some(antinodes.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
