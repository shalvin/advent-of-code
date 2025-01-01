advent_of_code::solution!(5);

use petgraph::{csr::IndexType, graph::node_index, prelude::*, visit::Walker};

type DependencyDefs = Vec<(u32, u32)>;
type UpdateDefs = Vec<Vec<u32>>;

fn parse_p1(input: &str) -> (DependencyDefs, UpdateDefs) {
    let mut first_half = true;
    let mut dependency_defs: Vec<String> = Vec::new();
    let mut update_defs: Vec<String> = Vec::new();

    for line in input.lines() {
        if first_half {
            if line.is_empty() {
                first_half = false;
            } else {
                dependency_defs.push(line.to_string());
            }
        } else {
            update_defs.push(line.to_string());
        }
    }

    let dependencies: DependencyDefs = dependency_defs
        .into_iter()
        .map(|def| {
            let split = def.split_once('|').unwrap();
            (
                split.0.parse::<u32>().unwrap(),
                split.1.parse::<u32>().unwrap(),
            )
        })
        .collect();

    let updates: UpdateDefs = update_defs
        .into_iter()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    (dependencies, updates)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (dependency_defs, update_defs) = parse_p1(input);

    let dependency_graph: StableDiGraph<u32, ()> = StableDiGraph::from_edges(dependency_defs);

    let mut sum = 0;
    for update in update_defs.into_iter() {
        let mut is_okay = true;

        let subgraph = dependency_graph.filter_map(
            |i, n| {
                if update.contains(&(i.index() as u32)) {
                    Some(*n)
                } else {
                    None
                }
            },
            |_, e| Some(*e),
        );

        for (i, page) in update.iter().skip(1).enumerate() {
            let previous_pages: Vec<_> = update[0..=i].iter().copied().collect();

            let node = node_index(*page as usize);
            let dfs = Dfs::new(&subgraph, node);
            let dependent_page = dfs
                .iter(&subgraph)
                .find(|n| previous_pages.contains(&(n.index() as u32)));

            if let Some(dep_page) = dependent_page {
                println!("Found dependent page for {}: {:#?}", page, dep_page);
                is_okay = false;
                break;
            }
        }

        if is_okay {
            sum += update[(update.len() / 2) as usize];
        }
    }

    Some(sum)
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
