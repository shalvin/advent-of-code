use std::collections::{HashMap, HashSet};

advent_of_code::solution!(6);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Heading {
    N,
    E,
    S,
    W,
}

impl Heading {
    pub fn turn_right(&self) -> Heading {
        use Heading::*;

        match self {
            N => E,
            E => S,
            S => W,
            W => N,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Guard {
    pos: Pos,
    heading: Heading,
}

impl Guard {
    fn next_pos(&self) -> Pos {
        use Heading::*;

        match self.heading {
            N => Pos {
                y: self.pos.y - 1,
                ..self.pos
            },
            E => Pos {
                x: self.pos.x + 1,
                ..self.pos
            },
            S => Pos {
                y: self.pos.y + 1,
                ..self.pos
            },
            W => Pos {
                x: self.pos.x - 1,
                ..self.pos
            },
        }
    }
}

#[derive(Clone, Debug)]
struct Map {
    w: u32,
    h: u32,
    obstacles: HashSet<Pos>,
}

impl Map {
    fn is_pos_outside_bounds(&self, pos: &Pos) -> bool {
        pos.x < 0 || pos.y < 0 || pos.x >= self.w as i32 || pos.y >= self.h as i32
    }

    fn is_pos_obstacle(&self, pos: &Pos) -> bool {
        self.obstacles.contains(pos)
    }

    fn is_pos_occupiable(&self, pos: &Pos) -> bool {
        !self.is_pos_outside_bounds(pos) && !self.is_pos_obstacle(pos)
    }

    fn add_obstacle(&mut self, pos: Pos) -> bool {
        self.obstacles.insert(pos)
    }

    fn remove_obstacle(&mut self, pos: Pos) -> bool {
        self.obstacles.remove(&pos)
    }

    fn trace_path(&self, tracer: Guard) -> Option<HashSet<(Pos, Heading)>> {
        let mut ray = tracer;

        let mut ray_path: HashSet<(Pos, Heading)> = HashSet::new();
        let _ = ray_path.insert((ray.pos, ray.heading));

        let mut iterations = 0;
        loop {
            match step_guard(&self, &mut ray) {
                StepResult::OutOfBounds => return None,
                StepResult::Turned(_, _) => (),
                StepResult::Visited(pos) => {
                    if ray_path.contains(&(pos, ray.heading)) {
                        return Some(ray_path);
                    }

                    let _ = ray_path.insert((pos, ray.heading));
                }
            }

            if iterations > 140 * 140 * 8 {
                return Some(ray_path);
            }

            iterations += 1;
        }
    }
}

type VisitMap = HashMap<Pos, Vec<(usize, Heading)>>;

fn parse_map(input: &str) -> (Map, Guard) {
    let mut obstacles: HashSet<Pos> = HashSet::new();
    let mut guard = Guard {
        pos: Pos { x: 0, y: 0 },
        heading: Heading::N,
    };

    for (line_num, line) in input.lines().enumerate() {
        for (char_num, char) in line.chars().enumerate() {
            if char == '#' {
                let _ = obstacles.insert(Pos {
                    x: char_num as i32,
                    y: line_num as i32,
                });
            } else if char == '^' {
                guard.pos = Pos {
                    x: char_num as i32,
                    y: line_num as i32,
                };
            }
        }
    }

    let h = input.lines().count() as u32;
    let w = input.lines().nth(0).unwrap().len() as u32;

    (Map { w, h, obstacles }, guard)
}

enum StepResult {
    OutOfBounds,
    Turned(Pos, Heading),
    Visited(Pos),
}

fn step_guard(map: &Map, guard: &mut Guard) -> StepResult {
    use StepResult::*;

    let guard_new_pos = guard.next_pos();

    if map.is_pos_outside_bounds(&guard_new_pos) {
        return OutOfBounds;
    }

    if map.is_pos_obstacle(&guard_new_pos) {
        guard.heading = guard.heading.turn_right();
        return Turned(guard.pos, guard.heading);
    } else {
        guard.pos = guard_new_pos;
        return Visited(guard.pos);
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (map, mut guard) = parse_map(input);

    let mut visited: HashSet<Pos> = HashSet::new();
    visited.insert(guard.pos);

    loop {
        match step_guard(&map, &mut guard) {
            StepResult::OutOfBounds => return Some(visited.len() as u64),
            StepResult::Turned(_, _) => (),
            StepResult::Visited(pos) => {
                if !visited.contains(&pos) {
                    visited.insert(pos);
                }
            }
        }
    }
}

fn traverse_map(map: &Map, mut guard: &mut Guard) -> VisitMap {
    let mut visited: VisitMap = HashMap::from([(guard.pos, vec![(0, guard.heading)])]);
    let mut visit_time = 1;

    loop {
        match step_guard(&map, &mut guard) {
            StepResult::OutOfBounds => break,
            StepResult::Turned(pos, heading) => {
                match visited.get_mut(&pos) {
                    Some(visits) => visits.push((visit_time, heading)),
                    None => {
                        let _ = visited.insert(pos, vec![(visit_time, heading)]);
                    }
                }
                visit_time += 1;
            }
            StepResult::Visited(pos) => {
                match visited.get_mut(&pos) {
                    Some(visits) => visits.push((visit_time, guard.heading)),
                    None => {
                        let _ = visited.insert(pos, vec![(visit_time, guard.heading)]);
                    }
                }
                visit_time += 1;
            }
        }
    }

    visited
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut map, mut guard) = parse_map(input);
    let starting_pos = guard.pos;

    let visited = traverse_map(&map, &mut guard);

    let mut candidates: HashSet<Pos> = HashSet::new();

    let mut draw_path = None;
    let mut draw_counter = 0;

    for (pos, visits) in visited.iter() {
        for (time, heading) in visits {
            let sim_guard = Guard {
                pos: *pos,
                heading: *heading,
            };

            let pos_forward = sim_guard.next_pos();

            if map.is_pos_occupiable(&sim_guard.pos)
                && map.is_pos_occupiable(&pos_forward)
                && !candidates.contains(&pos_forward)
                && visited
                    .get(&pos_forward)
                    .filter(|v| v.iter().any(|(t, _)| *time > *t))
                    .is_none()
            {
                map.add_obstacle(pos_forward);

                if let Some(path) = map.trace_path(sim_guard) {
                    candidates.insert(pos_forward);

                    if draw_counter == 1 {
                        draw_path = Some(path);
                    }
                    draw_counter += 1;
                }

                map.remove_obstacle(pos_forward);
            }
        }
    }

    candidates.remove(&starting_pos);

    if false {
        use omage::colors::*;
        use omage::{Components, Config, Image};

        let width = 1350;
        let height = 1350;

        let config = Config::new(width, height, WHITE, Some(BLACK), "test.png", None);

        let mut image = Image::new();

        let main_path = visited
            .iter()
            .map(|(pos, visits)| {
                // if visits.iter().any(|(_, h)| *h == Heading::N) {
                //     Components::Circle(10 * pos.x as u32, 10 * pos.y as u32, 5, BLACK)
                // } else {
                Components::Rectangle(10, 10, 10 * pos.x as u32, 10 * pos.y as u32, BLACK)
                // }
            })
            .collect::<Vec<_>>();

        let obstacles = map
            .obstacles
            .iter()
            .map(|pos| Components::Rectangle(10, 10, 10 * pos.x as u32, 10 * pos.y as u32, RED))
            .collect::<Vec<_>>();

        let candidates = candidates
            .iter()
            .map(|pos| Components::Rectangle(10, 10, 10 * pos.x as u32, 10 * pos.y as u32, PURPLE))
            .collect::<Vec<_>>();

        // println!("{:#?}", draw_path);
        let ray = draw_path
            .unwrap()
            .iter()
            .map(|(pos, _)| {
                Components::Rectangle(10, 10, 10 * pos.x as u32, 10 * pos.y as u32, PURPLE)
            })
            .collect::<Vec<_>>();

        image
            .config(config)
            .init()
            .unwrap()
            .add_components(main_path.iter().map(|v| v).collect())
            .add_components(obstacles.iter().map(|v| v).collect())
            .add_components(candidates.iter().map(|v| v).collect())
            // .add_components(ray.iter().map(|v| v).collect())
            .add_component(&Components::Rectangle(
                10,
                10,
                10 * starting_pos.x as u32,
                10 * starting_pos.y as u32,
                GREEN,
            ))
            // .add_components(candidates.iter().map(|v| v).collect())
            .draw()
            .unwrap();
    }

    Some(candidates.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(19));
    }
}
